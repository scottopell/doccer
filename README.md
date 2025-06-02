# Doccer

Convert Rust documentation from `cargo doc` JSON output to readable text for terminal viewing.

## Usage

1. Generate JSON documentation:
```bash
cargo +nightly rustdoc --lib -- -Zunstable-options --output-format json
```

2. Convert to text:
```bash
cargo run target/doc/your_crate.json
```

## Goal

Transforms rustdoc's machine-readable JSON into a concise, ASCII-text representation showing function signatures and documentation comments, optimized for terminal display.