use anyhow::{Context, Result};
use clap::Parser;
use rustdoc_types::{Crate, Deprecation, Id, Item, ItemEnum, Module, Visibility};
use serde::Deserialize;
use std::env;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use tracing::{debug, info};

#[cfg(test)]
mod tests;

// Now using official rustdoc-types structs

mod parser;
mod renderer;
use parser::*;
use renderer::*;


/// Types of input that can be provided to doccer
enum InputType {
    /// External crate from docs.rs
    ExternalCrate(String),
    /// Local JSON file
    /// TODO: Remove this local file support fully, it is deprecated.
    LocalFile(PathBuf),
    /// Local crate to generate docs for
    LocalCrate,
    /// Standard library documentation
    Stdlib {
        crate_name: String,          // "std", "core", "alloc"
        module_path: Option<String>, // "net", "collections::HashMap"
    },
}

/// Parse the module path from an input string like "std::net" or "core::mem"
fn parse_module_path(input: &str) -> Option<String> {
    let parts: Vec<&str> = input.split("::").collect();
    if parts.len() <= 1 {
        None
    } else {
        Some(parts[1..].join("::"))
    }
}

/// Resolve the input type based on the input string
fn resolve_input(input: &str) -> InputType {
    if input.starts_with("std::") || input == "std" {
        InputType::Stdlib {
            crate_name: "std".to_string(),
            module_path: parse_module_path(input),
        }
    } else if input.starts_with("core::") || input == "core" {
        InputType::Stdlib {
            crate_name: "core".to_string(),
            module_path: parse_module_path(input),
        }
    } else if input.starts_with("alloc::") || input == "alloc" {
        InputType::Stdlib {
            crate_name: "alloc".to_string(),
            module_path: parse_module_path(input),
        }
    } else if input.ends_with(".json") || Path::new(input).exists() {
        InputType::LocalFile(PathBuf::from(input))
    } else {
        InputType::ExternalCrate(input.to_string())
    }
}

// CLI Arguments structure
#[derive(Parser)]
#[command(
    author,
    version,
    about = "Convert rustdoc JSON to readable text",
    disable_version_flag = true
)]
struct Cli {
    /// Input: crate name (serde), stdlib module (std::net), JSON file, or leave empty for local crate
    input: Option<String>,

    /// Crate version (defaults to "latest", can also be a specific version like "1.0.0" or "~1" for semver matching)
    #[arg(short = 'V', long = "crate-version", default_value = "latest")]
    crate_version: String,

    /// Target platform (defaults to x86_64-unknown-linux-gnu)
    #[arg(short, long, default_value = "x86_64-unknown-linux-gnu")]
    target: String,

    /// Format version (defaults to latest)
    #[arg(short = 'f', long)]
    format_version: Option<String>,

    /// Path to the local crate or workspace (if provided, generates docs for a local crate)
    #[arg(long)]
    crate_path: Option<PathBuf>,

    /// Package name within workspace (required for workspaces when using --crate-path)
    #[arg(short, long)]
    package: Option<String>,

    /// Features to enable when generating documentation for a local crate (comma or space separated)
    #[arg(long)]
    features: Option<String>,

    /// Activate all available features when generating documentation for a local crate
    #[arg(long)]
    all_features: bool,

    /// Do not activate the default features when generating documentation for a local crate
    #[arg(long)]
    no_default_features: bool,

    /// Toolchain to use for stdlib docs (default: nightly)
    #[arg(long, help = "Toolchain to use for stdlib docs (default: nightly)")]
    toolchain: Option<String>,
}

/// Function to handle loading a documentation JSON from a file
fn load_from_file(file_path: &PathBuf) -> Result<String> {
    info!("Loading file: {}", file_path.to_string_lossy());

    // Read the JSON file
    fs::read_to_string(file_path)
        .with_context(|| format!("Failed to read file: {}", file_path.display()))
}

/// Function to fetch documentation JSON from docs.rs
fn fetch_from_docs_rs(
    name: &str,
    version: &str,
    target: &str,
    format_version: Option<&str>,
) -> Result<String> {
    // Build the URL based on the parameters
    let mut url = if target == "x86_64-unknown-linux-gnu" {
        // Default target can be omitted
        format!(
            "https://docs.rs/crate/{}/{}/json",
            name,
            // URL encode tilde for semver patterns
            version.replace("~", "%7E")
        )
    } else {
        format!(
            "https://docs.rs/crate/{}/{}/{}/json",
            name,
            // URL encode tilde for semver patterns
            version.replace("~", "%7E"),
            target
        )
    };

    // Add format version if specified
    if let Some(fv) = format_version {
        url.push('/');
        url.push_str(fv);
    }

    info!("Fetching documentation from: {}", url);

    // Docs.rs redirects to static.docs.rs, so we need to follow redirects
    let client = reqwest::blocking::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;

    // Print more detailed debugging information
    debug!("Sending request...");
    let response = client
        .get(&url)
        .header("User-Agent", concat!("doccer/", env!("CARGO_PKG_VERSION")))
        .header("Accept", "application/json, application/zstd")
        .send()
        .with_context(|| format!("Failed to fetch documentation from {}", url))?;

    if response.status().as_u16() == 404 {
        return Err(anyhow::anyhow!(
            "Documentation not found for crate '{}' version '{}' on target '{}'. \n\
             This could be because:\n\
             1. The crate doesn't exist\n\
             2. The version doesn't exist\n\
             3. The target isn't supported\n\
             4. The crate version was published before May 23, 2025\n\n\
             Note: docs.rs only generates JSON documentation for crates published after May 23, 2025.\n\
             Try a newer version or try a different crate like 'clap' (4.3.0+) which has JSON documentation.",
            name, version, target
        ));
    } else if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to fetch documentation: HTTP {}",
            response.status()
        ));
    }

    // Print the final URL after redirects
    let final_url = response.url().clone();
    debug!("Fetched from: {}", final_url);

    // Check if the response is zstandard compressed
    let content_type = response
        .headers()
        .get("content-type")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("")
        .to_string(); // Clone to avoid borrow issues

    debug!("Content-Type: {}", content_type);

    // Check if we need to append .json.zst to the URL if we got a redirect to a directory
    if final_url.path().ends_with("/") {
        debug!("URL ends with directory, retrying with .json.zst extension");
        let new_url = format!("{}json.zst", final_url);
        debug!("New URL: {}", new_url);

        let response = client
            .get(&new_url)
            .header("User-Agent", concat!("doccer/", env!("CARGO_PKG_VERSION")))
            .send()
            .with_context(|| format!("Failed to fetch documentation from {}", new_url))?;

        if response.status().as_u16() == 404 {
            return Err(anyhow::anyhow!(
                "Documentation not found for crate '{}' version '{}' on target '{}'. \n\
                 This could be because:\n\
                 1. The crate doesn't exist\n\
                 2. The version doesn't exist\n\
                 3. The target isn't supported\n\
                 4. The crate version was published before May 23, 2025\n\n\
                 Note: docs.rs only generates JSON documentation for crates published after May 23, 2025.\n\
                 Try a newer version or try a different crate like 'clap' (4.3.0+) which has JSON documentation.",
                name, version, target
            ));
        } else if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "Failed to fetch documentation: HTTP {}",
                response.status()
            ));
        }

        // Read response as bytes
        let bytes = response.bytes()?;
        debug!("Downloaded {} bytes", bytes.len());

        // For .json.zst URLs, always use zstd decompression
        debug!("Decompressing zstd data...");
        let decompressed =
            zstd::decode_all(io::Cursor::new(bytes)).context("Failed to decompress zstd data")?;

        return String::from_utf8(decompressed)
            .context("Failed to convert decompressed data to UTF-8");
    }

    // Read response as bytes for the original URL
    let bytes = response.bytes()?;
    debug!("Downloaded {} bytes", bytes.len());

    let json_content = if content_type.contains("application/zstd")
        || final_url.path().ends_with(".zst")
        || bytes.starts_with(&[0x28, 0xB5, 0x2F, 0xFD])
    {
        // zstd magic number
        debug!("Decompressing zstd data...");
        // Decompress with zstd
        let decompressed =
            zstd::decode_all(io::Cursor::new(bytes)).context("Failed to decompress zstd data")?;

        String::from_utf8(decompressed).context("Failed to convert decompressed data to UTF-8")?
    } else {
        // Just read the regular JSON content
        debug!("Using raw JSON content");
        String::from_utf8(bytes.to_vec()).context("Failed to convert response data to UTF-8")?
    };

    Ok(json_content)
}

/// Function to filter a Crate structure to show only items in a specific module path
fn filter_by_module_path(crate_data: &mut Crate, module_path: &str) -> Result<()> {
    // Split module path into segments
    let segments: Vec<&str> = module_path.split("::").collect();

    // Start from the root module
    let mut current_module_id = crate_data.root;
    let mut current_module_name = "root".to_string();

    // Traverse the module hierarchy to find the target module
    for segment in &segments {
        let mut found = false;

        // Get the current module
        if let Some(current_module) = crate_data.index.get(&current_module_id) {
            // Check if it's a module
            if let ItemEnum::Module(module_data) = &current_module.inner {
                // Try to find the next segment in the module's items
                for item_id in &module_data.items {
                    if let Some(item) = crate_data.index.get(item_id) {
                        if let Some(name) = &item.name {
                            if name == segment {
                                // Found the next module in the path
                                current_module_id = *item_id;
                                current_module_name = name.clone();
                                found = true;
                                break;
                            }
                        }
                    }
                }
            }
        }

        if !found {
            return Err(anyhow::anyhow!(
                "Module '{}' not found in the path '{}'",
                segment,
                module_path
            ));
        }
    }

    // At this point, current_module_id points to the target module
    // Update the crate's root to point to the target module
    crate_data.root = current_module_id;

    // Filter the index to include only items that are part of the target module
    // Start by collecting all items related to the target module
    let mut items_to_keep: std::collections::HashSet<Id> = std::collections::HashSet::new();
    let mut queue = vec![current_module_id];

    // Breadth-first search to find all items in the target module and its submodules
    while let Some(module_id) = queue.pop() {
        items_to_keep.insert(module_id);

        if let Some(module_item) = crate_data.index.get(&module_id) {
            if let ItemEnum::Module(module_data) = &module_item.inner {
                for item_id in &module_data.items {
                    items_to_keep.insert(*item_id);

                    // If the item is a module, add it to the queue for further traversal
                    if let Some(item) = crate_data.index.get(item_id) {
                        if let ItemEnum::Module(_) = &item.inner {
                            queue.push(*item_id);
                        }
                    }
                }
            }
        }
    }

    // Remove items that are not part of the target module
    crate_data.index.retain(|k, _| items_to_keep.contains(k));

    // Update the crate's name to reflect the module path
    if let Some(root_item) = crate_data.index.get_mut(&crate_data.root) {
        if let Some(name) = &mut root_item.name {
            // Don't overwrite the name if the module path is just the crate name
            if !segments.is_empty() {
                *name = current_module_name;
            }
        }
    }

    Ok(())
}

/// Function to load standard library documentation from local rustup installation
fn load_stdlib_docs(crate_name: &str, toolchain: Option<&str>) -> Result<String> {
    let toolchain = toolchain.unwrap_or("nightly");

    // Get target triple for current system
    let target_triple = get_target_triple()?;

    let home_dir = match env::var("HOME") {
        Ok(home) => PathBuf::from(home),
        Err(_) => match dirs::home_dir() {
            Some(home) => home,
            None => return Err(anyhow::anyhow!("Could not determine home directory")),
        },
    };

    let json_path = home_dir
        .join(".rustup/toolchains")
        .join(format!("{}-{}", toolchain, target_triple))
        .join("share/doc/rust/json")
        .join(format!("{}.json", crate_name));

    if json_path.exists() {
        info!("Loading stdlib JSON from: {}", json_path.display());
        fs::read_to_string(json_path).context("Failed to read stdlib JSON")
    } else {
        Err(anyhow::anyhow!(
            "Standard library documentation not found at {}.\n\n\
             To view stdlib docs, install: rustup component add rust-docs-json --toolchain nightly\n\
             Then try: doccer {}",
            json_path.display(), crate_name
        ))
    }
}

/// Get the current system's target triple (e.g., x86_64-apple-darwin)
fn get_target_triple() -> Result<String> {
    // Try to get from rustc
    let output = std::process::Command::new("rustc")
        .args(["--version", "--verbose"])
        .output();

    match output {
        Ok(output) => {
            let output = String::from_utf8_lossy(&output.stdout);
            for line in output.lines() {
                if let Some(stripped) = line.strip_prefix("host: ") {
                    return Ok(stripped.to_string());
                }
            }
            Err(anyhow::anyhow!(
                "Could not determine target triple from rustc output"
            ))
        }
        Err(_) => {
            // Fallback: make a best guess based on OS/arch
            #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
            return Ok("x86_64-unknown-linux-gnu".to_string());

            #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
            return Ok("aarch64-unknown-linux-gnu".to_string());

            #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
            return Ok("x86_64-apple-darwin".to_string());

            #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
            return Ok("aarch64-apple-darwin".to_string());

            #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
            return Ok("x86_64-pc-windows-msvc".to_string());

            #[cfg(all(target_os = "windows", target_arch = "aarch64"))]
            return Ok("aarch64-pc-windows-msvc".to_string());

            #[cfg(not(any(
                all(
                    target_os = "linux",
                    any(target_arch = "x86_64", target_arch = "aarch64")
                ),
                all(
                    target_os = "macos",
                    any(target_arch = "x86_64", target_arch = "aarch64")
                ),
                all(
                    target_os = "windows",
                    any(target_arch = "x86_64", target_arch = "aarch64")
                )
            )))]
            Err(anyhow::anyhow!(
                "Could not determine target triple for current system"
            ))
        }
    }
}

/// Function to generate documentation JSON for a local crate using rustdoc-json crate
fn generate_local_crate_docs(
    crate_path: &Path,
    package: Option<&String>,
    features: Option<&String>,
    all_features: bool,
    no_default_features: bool,
) -> Result<String> {
    info!("Generating documentation for local crate...");

    // Ensure the crate path exists
    if !crate_path.exists() {
        return Err(anyhow::anyhow!(
            "Crate path does not exist: {}",
            crate_path.display()
        ));
    }

    // Find the manifest path (Cargo.toml)
    let manifest_path = if let Some(pkg) = package {
        // For workspace packages, find the specific package's Cargo.toml
        let potential_paths = [
            crate_path.join(format!("{}/Cargo.toml", pkg)),
            crate_path.join(format!("packages/{}/Cargo.toml", pkg)),
            crate_path.join(format!("crates/{}/Cargo.toml", pkg)),
            crate_path.join(format!("libs/{}/Cargo.toml", pkg)),
            crate_path.join(format!("services/{}/Cargo.toml", pkg)),
        ];

        let mut found_path = None;
        for path in &potential_paths {
            if path.exists() {
                found_path = Some(path.clone());
                break;
            }
        }

        found_path.unwrap_or_else(|| crate_path.join("Cargo.toml"))
    } else {
        // For single crates, use the main Cargo.toml
        crate_path.join("Cargo.toml")
    };

    // Verify the manifest path exists
    if !manifest_path.exists() {
        return Err(anyhow::anyhow!(
            "Cargo.toml not found at: {}",
            manifest_path.display()
        ));
    }

    info!("Using manifest path: {}", manifest_path.display());

    // Configure the rustdoc-json builder
    let mut builder = rustdoc_json::Builder::default()
        .toolchain("nightly")
        .manifest_path(manifest_path);

    // Apply package filter if specified
    if let Some(pkg) = package {
        builder = builder.package(pkg);
    }

    // Apply feature flags
    if let Some(feature_list) = features {
        // rustdoc-json expects features as a Vec<String>
        let feature_vec: Vec<String> = feature_list
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        builder = builder.features(feature_vec);
    }

    if all_features {
        builder = builder.all_features(true);
    }

    if no_default_features {
        builder = builder.no_default_features(true);
    }

    // Build the documentation
    let json_path = builder
        .build()
        .map_err(|e| anyhow::anyhow!("Failed to generate rustdoc JSON: {}", e))?;

    info!(
        "Successfully generated documentation at: {}",
        json_path.display()
    );

    // Read the generated JSON file
    fs::read_to_string(&json_path).with_context(|| {
        format!(
            "Failed to read generated JSON file: {}",
            json_path.display()
        )
    })
}

fn main() -> Result<()> {
    // Initialize tracing with environment filter (defaults to no output)
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();

    // Determine the input type based on CLI arguments
    let input_type = if cli.crate_path.is_some() {
        InputType::LocalCrate
    } else if let Some(input) = &cli.input {
        resolve_input(input)
    } else {
        // No input provided
        return Err(anyhow::anyhow!(
            "Missing input. Please provide either a crate name, a stdlib module (std::net), a JSON file path, or use --crate-path. Use --help for usage information."
        ));
    };

    // Process input based on type
    let json_content = match &input_type {
        InputType::LocalCrate => {
            // Local crate mode (if --crate-path is provided)
            if let Some(crate_path) = &cli.crate_path {
                generate_local_crate_docs(
                    crate_path,
                    cli.package.as_ref(),
                    cli.features.as_ref(),
                    cli.all_features,
                    cli.no_default_features,
                )?
            } else {
                return Err(anyhow::anyhow!(
                    "Missing --crate-path argument for local crate mode"
                ));
            }
        }
        InputType::LocalFile(path) => {
            // Local file mode
            load_from_file(path)?
        }
        InputType::ExternalCrate(name) => {
            // Docs.rs mode
            fetch_from_docs_rs(
                name,
                &cli.crate_version,
                &cli.target,
                cli.format_version.as_deref(),
            )?
        }
        InputType::Stdlib {
            crate_name,
            module_path: _,
        } => {
            // Standard library mode
            load_stdlib_docs(crate_name, cli.toolchain.as_deref())?
        }
    };

    // Parse the JSON content
    let mut crate_data: Crate =
        serde_json::from_str(&json_content).context("Failed to parse JSON documentation")?;

    // If this is a stdlib request with a module path, filter to that module
    if let InputType::Stdlib {
        crate_name: _,
        module_path: Some(ref path),
    } = input_type
    {
        filter_by_module_path(&mut crate_data, path)?;
    }

    // Two-phase approach: Parse then Render

    // Phase 1: Parse JSON into structured data
    let parser = ItemParser::new(&crate_data);
    let parsed_module = parser.parse_crate()?;

    // Phase 2: Render structured data to text
    let renderer = ParsedRenderer;
    let output = renderer.render(&parsed_module, crate_data.crate_version.as_deref());

    println!("{}", output);

    Ok(())
}
