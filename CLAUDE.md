<project_goal>
emit human-readable text for terminal viewing of rust documentation
</project_goal>

<project_overview>
Doccer is a Rust CLI tool that converts rustdoc JSON output into human-readable text optimized for terminal viewing. It supports multiple input sources:
- External crates from docs.rs
- Local crate documentation generation
- Standard library documentation (std, core, alloc)
- Local JSON files (deprecated)

The tool transforms rustdoc's machine-readable JSON into concise, ASCII-text representation showing function signatures and documentation comments.
</project_overview>

<architecture>
The project follows a two-phase architecture:

1. **Parsing Phase** (`src/parser/`):
   - `ItemParser` converts rustdoc JSON into structured `ParsedItem` types
   - `parser.rs` - Core parsing logic and rustdoc-types integration
   - `types.rs` - Internal representation types (ParsedItem, RustType, etc.)

2. **Rendering Phase** (`src/renderer/`):
   - `ParsedRenderer` converts structured data into human-readable text
   - `renderer.rs` - Main renderer implementation
   - `traits.rs` - Extensible `Render` trait with `RenderContext`
   - `components.rs` - Specialized rendering components (TypeRenderer, DocRenderer)
   - `renders.rs` - Modular render implementations for all `ParsedItem` variants

3. **CLI Interface** (`src/main.rs`):
   - Command-line argument parsing with clap
   - Input type resolution and data fetching
   - Integration with docs.rs, local crates, and stdlib docs
</architecture>

<commands>
    <test> cargo test </test>
    <build> cargo build --release </build>
    <lint> cargo clippy -- -D warnings </lint>
    <format> cargo fmt </format>
</commands>

<testing_approach>
The project uses a comprehensive testing strategy:

1. **Unit Tests** (`src/tests/`, `tests/unit/`):
   - Test individual components in isolation
   - Cover parsing, rendering, error handling, and type rendering
   - Use mockall for network testing
   - Focus on component-specific behavior

2. **Integration Tests** (`tests/integration_tests.rs`):
   - End-to-end testing with real test fixtures using cargo-insta
   - Snapshot-based testing with automatic diff generation
   - Interactive review process for approving changes
   - Test fixtures in `tests/fixtures/` (basic_types, complex, generics, etc.)

3. **Test Fixtures**:
   - `tests/fixtures/*/` - Small Rust crates for testing
   - `tests/snapshots/` - Insta snapshot files for expected outputs
   - Cover various Rust constructs (structs, enums, traits, generics, modules)
</testing_approach>

<development_workflow>
1. **Before making changes**: Run `cargo test` to ensure all tests pass
2. **After making changes**:
   - Run `cargo test` to verify functionality
   - Run `cargo clippy` to check for lint issues
   - Run `cargo fmt` to ensure consistent formatting
3. **For integration test failures**: 
   - Use `cargo insta review` to interactively review snapshot changes
   - Use `cargo insta accept` to accept all pending snapshots
   - Use `cargo insta reject` to reject pending snapshots
4. **For new features**: Add both unit tests and integration tests where appropriate
5. **Snapshot management**: When output format changes, review and approve snapshots carefully
</development_workflow>

<code_instructions>
    <comments>All comments should be written in context of the project as a
    declaration of non-intuitive parts of the code. Comments should never refer
    to the current implementation task.
        <bad_example>Indent to account for the expected format for trailing curly braces</bad_example>
        <good_example>The indentation at this level is decremented as we close a
        syntactic construct</good_example>

        Comments should be sparing and concise when used.
    </comments>

    <CRITICAL_RULES>
        NEVER hardcode test-specific behaviors into the main codebase. This includes:
        - Special handling for specific crate, module, struct, or function names
        - Conditional logic based on specific function or type names
        - Fixed output formats for specific test fixtures
        - Custom indentation rules for specific trait implementations

        The core implementation (src/main.rs) must be able to handle ANY Rust crate,
        not just our test fixtures. Hard-coding violates this principle and undermines
        the generality of the solution.

        Instead, use unit tests to verify that the generic implementation works
        correctly across all required use cases. If tests are failing because the
        implementation doesn't match specific expectations, either:
        1. Fix the general implementation to handle all cases correctly
        2. Update the test expectations to match the correct output

        This separation between code and tests is MANDATORY. Violating this rule
        will result in immediate rejection of any changes.
    </CRITICAL_RULES>

    <architecture_principles>
        - Maintain the two-phase architecture: Parse → Render
        - Keep parsers and renderers loosely coupled
        - Use the official rustdoc-types for JSON parsing
        - Prefer generic solutions over special-case handling
        - Maintain extensibility through trait-based design
        - Follow Rust best practices for error handling (anyhow, Result)
    </architecture_principles>

    <rendering_guidelines>
        - Output must be human-readable and optimized for terminal display
        - Maintain consistent indentation and formatting
        - Include documentation comments when available
        - Show function signatures clearly
        - Handle deprecation warnings appropriately
        - Support filtering by module paths
    </rendering_guidelines>
</code_instructions>

<known_issues>
    <version_compatibility>
        Project follows a single-version policy: each release supports exactly one FORMAT_VERSION.
        Current support: rustdoc-types v0.36.0 with FORMAT_VERSION 40.

        When updating to newer format versions:
        1. Update rustdoc-types dependency version
        2. Update SUPPORTED_VERSION constant in src/main.rs
        3. Update version compatibility table in README.md
        4. Test against all fixture files
        5. Update test fixtures if format changes
    </version_compatibility>

    <integration_tests>
        Some integration tests may fail due to formatting differences.
        Use the detailed diff output to understand and fix rendering issues.
        Focus on improving the generic implementation rather than hardcoding fixes.
    </integration_tests>
</known_issues>

<dependencies>
    <key_crates>
        - rustdoc-types: Official rustdoc JSON types
        - serde/serde_json: JSON serialization
        - clap: Command-line argument parsing
        - anyhow: Error handling
        - reqwest: HTTP client for docs.rs
        - rustdoc-json: Local crate documentation generation
        - tracing: Logging and debugging
    </key_crates>
</dependencies>
