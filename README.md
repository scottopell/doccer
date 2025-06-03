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

## Future Enhancements

### External Crate References

The rustdoc JSON format includes information about external crate references, which could be used to:
- Show the origin of imported types
- Indicate when a trait is implemented from an external crate
- Distinguish between local and external types in function signatures

These references are currently not rendered in the terminal output as they would create noise without the ability to navigate hyperlinks in a text interface.

### Function Signature Analysis

The current implementation provides basic function signature rendering, but the rustdoc JSON contains more detailed information that could be used to:
- Show default values for parameters
- Display detailed generic constraint information
- Render complex lifetimes and associated types more accurately
- Distinguish between different kinds of method receivers (self, &self, &mut self)

Improvements in this area would make the signatures more precise while maintaining readability.