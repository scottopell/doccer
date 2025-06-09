# Doccer Architecture Refactor Plan

## Previous Phase Summary ✅ COMPLETED
Successfully migrated from custom deserialization structs to official `rustdoc-types` v0.36.0. The project now uses upstream types for all JSON parsing, providing better type safety and future compatibility.

## Phase 1 Summary ✅ COMPLETED
Successfully refactored the monolithic 2669-line `main.rs` into focused modules. Created a clean `src/parser/` module containing all parsing logic (`types.rs`, `parser.rs`, `mod.rs`). Extracted 1095 lines from main.rs while maintaining the two-phase architecture and ensuring all unit tests continue to pass.

## Phase 2 Summary ✅ COMPLETED
Successfully implemented trait-based renderer architecture. Extracted remaining ~1500 lines of rendering logic from main.rs into `src/renderer/` module with clean separation of concerns. Created extensible `Render` trait with `RenderContext`, specialized rendering components (`TypeRenderer`, `DocRenderer`), and modular render implementations for all `ParsedItem` variants. All unit tests (15/15) continue to pass with the new architecture.

## Current State Assessment

### ✅ Working Components
- Two-phase architecture: `ItemParser` → `ParsedRenderer`
- Official rustdoc-types integration
- Basic CLI functionality
- Unit tests passing (15/15)
- Module filtering and docs.rs integration
- **NEW**: Modular parser architecture with `src/parser/` module
- **NEW**: Clean separation of parsing types and logic
- **NEW**: Trait-based renderer architecture with `src/renderer/` module
- **NEW**: Extensible `Render` trait with `RenderContext` configuration
- **NEW**: Specialized rendering components (`TypeRenderer`, `DocRenderer`, etc.)

### ❌ Issues Identified
- **Integration tests failing (4/9)**: formatting inconsistencies
- **Rendering bugs**: indentation, type paths, missing implementations

## Next Phase Goals

1. **All integration tests passing** - Fix rendering to match expected output
2. **Best-practices printer architecture** - Clean, extensible, well-structured code
3. **Comprehensive unit tests** - Thorough testing of all printer components
4. **Maintainable codebase** - Proper separation of concerns and modularity

## Ground Rules
- NEVER hard-code test-specific behaviors into the main codebase
- Deviating from test fixture output IS ALLOWED if well-justified for human readability
- Focus on generic, extensible solutions that work for any Rust crate

## Architecture Refactor Plan

### Phase 1: Code Organization ✅ COMPLETED
**Goal**: Split monolithic main.rs into focused modules

**Tasks**:
1. **✅ Create `src/parser/` module**
   - ✅ Move `ItemParser` and parsing logic to `parser.rs`
   - ✅ Create `types.rs` for `ParsedItem`, `RustType`, etc.
   - ✅ Create `mod.rs` for clean module exports
   - ✅ Successfully extracted 1095 lines from main.rs

2. **Create `src/renderer/` module**
   - Move `ParsedRenderer` and all rendering logic
   - Create trait-based architecture for extensibility
   - Separate concerns: formatting, indentation, type rendering

3. **Create `src/cli.rs` module**
   - Move CLI argument parsing and input handling
   - Keep main.rs minimal and focused

4. **✅ Update `src/main.rs`**
   - ✅ Added clean parser module imports
   - ✅ Maintained two-phase architecture
   - ✅ Code compiles successfully with all unit tests passing

### Phase 2: Renderer Architecture ✅ COMPLETED
**Goal**: Implement best-practices printer using trait-based design

**Architecture Choice**: Trait-based approach (not visitor pattern)
- More idiomatic for Rust
- Easier to extend and test
- Clear separation of rendering concerns

**Design**:
```rust
// Core rendering trait
trait Render {
    fn render(&self, context: &RenderContext) -> String;
}

// Context for rendering configuration
struct RenderContext {
    depth: usize,
    show_private: bool,
    format: OutputFormat,
}

// Implement Render for each ParsedItem type
impl Render for ParsedFunction { ... }
impl Render for ParsedStruct { ... }
// etc.
```

**Tasks**:
1. **✅ Extract `src/renderer/` module**
   - ✅ Moved `ParsedRenderer` and all rendering logic from main.rs (~1500 lines)
   - ✅ Created modular structure: `mod.rs`, `renderer.rs`, `traits.rs`, `components.rs`, `renders.rs`
   - ✅ Maintained existing functionality during extraction

2. **✅ Design `Render` trait and `RenderContext`**
   - ✅ Defined core rendering interface with `Render` trait
   - ✅ Created `RenderContext` with depth, visibility, and format configuration
   - ✅ Planned for future output formats (HTML, markdown, etc.)

3. **✅ Implement rendering components**
   - ✅ `TypeRenderer` for type signatures, visibility, generics, where clauses
   - ✅ `DocRenderer` for documentation formatting and deprecation notices
   - ✅ `IndentationHelper` for consistent spacing
   - ✅ Component-based architecture for reusability

4. **✅ Create specialized renderers**
   - ✅ Implemented `Render` for all `ParsedItem` variants
   - ✅ Clean, modular implementations with helper components
   - ✅ Updated unit tests to use new trait-based approach (15/15 passing)

### Phase 3: Fix Integration Tests 🎯 NEXT
**Goal**: Resolve all rendering bugs to pass integration tests

**Known Issues to Fix** (Current: 4/9 integration tests failing):
1. **Function signatures**: prevent truncation with `-> ...`
2. **Type path rendering**: `$crate::fmt::Formatter` → `std::fmt::Formatter`
3. **Indentation problems**: trait methods, impl blocks
4. **Missing trait implementations**: ensure all impls are rendered
5. **Re-exports section**: properly render `pub use` statements
6. **Documentation formatting**: consistent spacing and line breaks

**Tasks**:
1. **Debug each failing test**
   - Analyze expected vs actual output differences
   - Identify root cause of each formatting issue
   - Fix generic rendering logic (not test-specific hacks)

2. **Improve type rendering**
   - Fix path resolution for standard library types
   - Handle generic parameters correctly
   - Ensure complete function signatures

3. **Fix structural issues**
   - Correct indentation logic
   - Proper trait implementation rendering
   - Re-exports section generation

### Phase 4: Comprehensive Unit Testing 🔄 PENDING
**Goal**: Achieve thorough test coverage for all printer components

**Testing Strategy**:
1. **Component-level tests**: Test each renderer independently
2. **Integration-style tests**: Test renderer combinations
3. **Edge case tests**: Handle unusual or complex Rust constructs
4. **Regression tests**: Prevent future formatting regressions

**Tasks**:
1. **Create `src/renderer/tests/` module**
   - Unit tests for `TypeRenderer`
   - Unit tests for each `Render` implementation
   - Tests for indentation and formatting helpers

2. **Add property-based tests**
   - Generate diverse Rust constructs
   - Verify consistent formatting rules
   - Test edge cases automatically

3. **Improve existing test coverage**
   - Enhance `src/tests/mod.rs` unit tests
   - Add specific tests for recently fixed bugs
   - Ensure all code paths are tested

### Phase 5: Polish and Documentation 🔄 PENDING
**Goal**: Finalize architecture and prepare for future extensions

**Tasks**:
1. **Code quality improvements**
   - Run clippy and fix all warnings
   - Optimize performance where needed
   - Clean up dead code and unused imports

## Success Criteria

- 🔄 All integration tests pass (5/9 currently passing)
- ✅ Well-organized, modular codebase (parser + renderer modules completed)
- 🔄 Comprehensive unit test coverage (>90%)
- 🔄 Clean, idiomatic Rust code (some warnings remain)
- ✅ Extensible architecture for future enhancements (trait-based renderer)
- 🔄 No clippy warnings or dead code (some warnings remain)
- ✅ Clear separation between parsing and rendering concerns (both modules done)

## Benefits of This Approach

1. **Maintainability**: Clear module boundaries and single responsibilities
2. **Testability**: Each component can be tested in isolation
3. **Extensibility**: Easy to add new output formats or rendering features
4. **Robustness**: Comprehensive testing prevents regressions
5. **Performance**: Focused, efficient rendering logic
6. **Code Quality**: Follows Rust best practices and idioms



