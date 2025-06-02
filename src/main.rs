use anyhow::Result;
use clap::{Arg, Command};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

// Core data structures for modern rustdoc JSON format

#[derive(Debug, Deserialize)]
struct Crate {
    root: u32,
    #[serde(default)]
    crate_version: Option<String>,
    #[serde(default)]
    includes_private: bool,
    index: HashMap<String, Item>,
    #[serde(default)]
    paths: serde_json::Value, // Make this flexible
    #[serde(default)]
    external_crates: serde_json::Value, // Make this flexible
    #[serde(default)]
    format_version: u32,
}

#[derive(Debug, Deserialize)]
struct ExternalCrate {
    name: String,
    html_root_url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ItemSummary {
    crate_id: u32,
    path: Vec<String>,
    kind: String,
}

#[derive(Debug, Deserialize)]
struct Item {
    id: Option<u32>,
    crate_id: u32,
    name: Option<String>,
    span: Option<Span>,
    visibility: Visibility,
    docs: Option<String>,
    links: HashMap<String, serde_json::Value>,
    attrs: Vec<String>,
    deprecation: Option<Deprecation>,
    inner: serde_json::Value, // We'll handle this as raw JSON
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum Visibility {
    Simple(String),
    Restricted { restricted: RestrictedVisibility },
}

#[derive(Debug, Deserialize)]
struct RestrictedVisibility {
    parent: String,
    path: String,
}

#[derive(Debug, Deserialize)]
struct Span {
    filename: String,
    begin: (u32, u32),
    end: (u32, u32),
}

#[derive(Debug, Deserialize)]
struct Deprecation {
    since: Option<String>,
    note: Option<String>,
}

// Simplified structures for the modern format
#[derive(Debug, Deserialize)]
struct ModernFunction {
    sig: serde_json::Value,
    generics: serde_json::Value,
    header: serde_json::Value,
    has_body: bool,
}

#[derive(Debug, Deserialize)]
struct ModernStruct {
    kind: serde_json::Value,
    generics: serde_json::Value,
    impls: Vec<u32>,
}

#[derive(Debug, Deserialize)]
struct Module {
    is_crate: Option<bool>,
    items: Vec<u32>,
    is_stripped: Option<bool>,
}

// Text renderer implementation
struct TextRenderer {
    crate_data: Crate,
}

impl TextRenderer {
    fn new(crate_data: Crate) -> Self {
        Self { crate_data }
    }

    fn render(&self) -> Result<String> {
        let mut output = String::new();

        // Start with the root module
        let root_id = self.crate_data.root.to_string();
        if let Some(root_item) = self.crate_data.index.get(&root_id) {
            output.push_str(&format!(
                "# Crate: {}\n\n",
                root_item.name.as_deref().unwrap_or("unknown")
            ));

            if let Some(docs) = &root_item.docs {
                output.push_str(&format!("{}\n\n", docs));
            }

            self.render_item(&root_id, &mut output, 0)?;
        }

        Ok(output)
    }

    fn render_item(&self, item_id: &str, output: &mut String, depth: usize) -> Result<()> {
        let item = match self.crate_data.index.get(item_id) {
            Some(item) => item,
            None => return Ok(()), // Skip items not in our index
        };

        let indent = "  ".repeat(depth);

        // Determine the kind from the inner object keys
        if let Some(inner_obj) = item.inner.as_object() {
            for (kind, inner_data) in inner_obj {
                match kind.as_str() {
                    "function" => {
                        self.render_function_simple(item, inner_data, output, &indent)?;
                    }
                    "struct" => {
                        self.render_struct_simple(item, inner_data, output, &indent)?;
                    }
                    "module" => {
                        if let Ok(module) = serde_json::from_value::<Module>(inner_data.clone()) {
                            self.render_module(item, &module, output, depth)?;
                        }
                    }
                    "enum" => {
                        self.render_enum_simple(item, output, &indent)?;
                    }
                    "trait" => {
                        self.render_trait_simple(item, output, &indent)?;
                    }
                    _ => {
                        // For other kinds, just show basic info
                        if let Some(name) = &item.name {
                            output.push_str(&format!("{}{}({})\n", indent, name, kind));
                            if let Some(docs) = &item.docs {
                                output.push_str(&format!("{}  {}\n", indent, docs));
                            }
                            output.push('\n');
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn render_function_simple(
        &self,
        item: &Item,
        _func_data: &serde_json::Value,
        output: &mut String,
        indent: &str,
    ) -> Result<()> {
        let mut signature = String::new();

        // Add visibility
        match &item.visibility {
            Visibility::Simple(vis) if vis == "public" => signature.push_str("pub "),
            _ => {}
        }

        signature.push_str("fn ");

        if let Some(name) = &item.name {
            signature.push_str(name);
        }

        signature.push_str("(...)"); // Simplified for now

        output.push_str(&format!("{}{}\n", indent, signature));

        // Add documentation
        if let Some(docs) = &item.docs {
            for line in docs.lines() {
                output.push_str(&format!("{}  /// {}\n", indent, line));
            }
        }

        output.push('\n');
        Ok(())
    }

    fn render_struct_simple(
        &self,
        item: &Item,
        _struct_data: &serde_json::Value,
        output: &mut String,
        indent: &str,
    ) -> Result<()> {
        let mut signature = String::new();

        // Add visibility
        match &item.visibility {
            Visibility::Simple(vis) if vis == "public" => signature.push_str("pub "),
            _ => {}
        }

        signature.push_str("struct ");

        if let Some(name) = &item.name {
            signature.push_str(name);
        }

        signature.push_str(" { ... }");

        output.push_str(&format!("{}{}\n", indent, signature));

        // Add documentation
        if let Some(docs) = &item.docs {
            for line in docs.lines() {
                output.push_str(&format!("{}  /// {}\n", indent, line));
            }
        }

        output.push('\n');
        Ok(())
    }

    fn render_module(
        &self,
        item: &Item,
        module: &Module,
        output: &mut String,
        depth: usize,
    ) -> Result<()> {
        let indent = "  ".repeat(depth);

        if depth > 0 {
            // Don't show "mod" for root crate
            if let Some(name) = &item.name {
                output.push_str(&format!("{}mod {}\n", indent, name));

                if let Some(docs) = &item.docs {
                    for line in docs.lines() {
                        output.push_str(&format!("{}  /// {}\n", indent, line));
                    }
                }
                output.push('\n');
            }
        }

        // Render all items in this module - convert integer IDs to strings
        for item_id in &module.items {
            let item_id_str = item_id.to_string();
            self.render_item(&item_id_str, output, depth + 1)?;
        }

        Ok(())
    }

    fn render_enum_simple(&self, item: &Item, output: &mut String, indent: &str) -> Result<()> {
        let mut signature = String::new();

        match &item.visibility {
            Visibility::Simple(vis) if vis == "public" => signature.push_str("pub "),
            _ => {}
        }

        signature.push_str("enum ");

        if let Some(name) = &item.name {
            signature.push_str(name);
        }

        signature.push_str(" { ... }");

        output.push_str(&format!("{}{}\n", indent, signature));

        if let Some(docs) = &item.docs {
            for line in docs.lines() {
                output.push_str(&format!("{}  /// {}\n", indent, line));
            }
        }

        output.push('\n');
        Ok(())
    }

    fn render_trait_simple(&self, item: &Item, output: &mut String, indent: &str) -> Result<()> {
        let mut signature = String::new();

        match &item.visibility {
            Visibility::Simple(vis) if vis == "public" => signature.push_str("pub "),
            _ => {}
        }

        signature.push_str("trait ");

        if let Some(name) = &item.name {
            signature.push_str(name);
        }

        signature.push_str(" { ... }");

        output.push_str(&format!("{}{}\n", indent, signature));

        if let Some(docs) = &item.docs {
            for line in docs.lines() {
                output.push_str(&format!("{}  /// {}\n", indent, line));
            }
        }

        output.push('\n');
        Ok(())
    }
}

fn main() -> Result<()> {
    let matches = Command::new("doccer")
        .about("Convert rustdoc JSON to readable text")
        .arg(
            Arg::new("input")
                .help("Input JSON file from rustdoc")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();

    // Read and parse the JSON file
    let json_content = fs::read_to_string(input_file)?;
    let crate_data: Crate = serde_json::from_str(&json_content)?;

    // Generate text output
    let renderer = TextRenderer::new(crate_data);
    let output = renderer.render()?;

    println!("{}", output);

    Ok(())
}
