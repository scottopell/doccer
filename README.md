# Doccer

Convert Rust documentation from `cargo doc` JSON output to readable text for terminal viewing.

## Usage

There are three main ways to use Doccer:

### 1. View documentation for an existing JSON file

```bash
cargo run target/doc/your_crate.json
```

### 2. Generate documentation for a local crate

This automatically runs the nightly compiler to generate documentation and displays it:

```bash
cargo run -- local-crate --crate-path /path/to/your/crate
```

For a workspace with multiple packages, specify the package:

```bash
cargo run -- local-crate --crate-path /path/to/workspace -p package_name
```

### 3. Fetch documentation from docs.rs

View documentation for a published crate directly from docs.rs:

```bash
cargo run -- docs-rs tokio
```

With specific version:

```bash
cargo run -- docs-rs serde --version 1.0.0
```

## Requirements

- To generate documentation for local crates, the nightly Rust compiler is required:
  ```
  rustup install nightly
  rustup component add --nightly rustfmt
  ```

## Goal

Transforms rustdoc's machine-readable JSON into a concise, ASCII-text representation showing function signatures and documentation comments, optimized for terminal display.