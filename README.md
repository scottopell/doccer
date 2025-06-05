# Doccer

Convert Rust documentation from `cargo doc` JSON output to readable text for terminal viewing.

## Usage

There are three main ways to use Doccer:

### 1. Fetch documentation from docs.rs (default)

View documentation for a published crate directly from docs.rs:

```bash
doccer tokio
```

With specific version:

```bash
doccer serde --crate-version 1.0.0
```

### 2. View documentation for an existing JSON file

```bash
doccer path/to/your_crate.json
```

### 3. Generate documentation for a local crate

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

## Goal

Transforms rustdoc's machine-readable JSON into a concise, ASCII-text representation showing function signatures and documentation comments, optimized for terminal display.