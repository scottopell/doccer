# Doccer

Convert Rust documentation from `cargo doc` JSON output to readable text for terminal viewing.

## Usage

There are four main ways to use Doccer:

### 1. View Standard Library Documentation

View Rust's standard library documentation directly:

```bash
# View entire standard library
doccer std

# View specific modules
doccer std::net
doccer std::collections::HashMap
doccer core::mem
doccer alloc::vec::Vec
```

**Note:** To use this feature, you need to install the nightly Rust toolchain and the `rust-docs-json` component:
```
rustup toolchain install nightly
rustup component add rust-docs-json --toolchain nightly
```

### 2. Fetch documentation from docs.rs (default)

View documentation for a published crate directly from docs.rs:

```bash
doccer clap
```

With specific version:

```bash
doccer clap --crate-version 4.5.0
```

**Note:** docs.rs only began generating JSON documentation artifacts for crates published after May 23, 2025. Many older crates or versions won't have these artifacts available yet. They will become available as crates publish new versions.

Some popular crates with JSON documentation available:
- clap (4.3.0+)
- tokio (recent versions)
- serde (recent versions)

### 3. View documentation for an existing JSON file
> Note this is deprecated and should not be used, will be removed in the future.

```bash
doccer path/to/your_crate.json
```

### 4. Generate documentation for a local crate

This automatically runs the nightly compiler to generate documentation and displays it:

```bash
doccer --crate-path /path/to/your/crate
```

For a workspace with multiple packages, specify the package:

```bash
doccer --crate-path /path/to/workspace -p package_name
```

When generating documentation for crates that use feature flags, you can enable specific features:

```bash
doccer --crate-path /path/to/crate --features "feature1,feature2"
```

You can also use `--all-features` to enable all available features, or `--no-default-features` to disable default features:

```bash
doccer --crate-path /path/to/crate --all-features
doccer --crate-path /path/to/crate --no-default-features --features "specific_feature"
```

## Requirements

- To generate documentation for local crates, the nightly Rust compiler is required:
  ```
  rustup install nightly
  rustup component add --nightly rustfmt
  ```

- For standard library documentation, you'll need the `rust-docs-json` component:
  ```
  rustup component add rust-docs-json --toolchain nightly
  ```

## Version Compatibility

**Important:** Doccer is currently pinned to handle **rustdoc JSON format version 40** (rustdoc-types v0.36.0). This corresponds to Rust nightly builds from around March 2025.

As Rust nightly continues to evolve and the JSON format version increases beyond 40, we will need to update our `rustdoc-types` dependency to match the newer format. If you encounter parsing errors with newer nightly builds, this likely indicates the format version has changed and an update is needed.

To check your current nightly version:
```bash
rustc +nightly --version
```

If you're using a significantly newer nightly and experiencing issues, please check if there's an updated version of Doccer available, or consider using an older nightly that's compatible with format version 40.

## Goal

Transforms rustdoc's machine-readable JSON into a concise, ASCII-text representation showing function signatures and documentation comments, optimized for terminal display.