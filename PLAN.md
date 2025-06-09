# Doccer Improvement Plan

## Files We're Working On

  - /Users/scott.opell/dev/doccer/src/tests/mod.rs - Unit tests
  - /Users/scott.opell/dev/doccer/tests/integration_tests.rs - Integration tests
  - /Users/scott.opell/dev/doccer/src/main.rs - Main implementation
  - /Users/scott.opell/dev/doccer/tests/expected/*.txt - Expected output for
    different test fixtures

## Problem Statement

Currently, the Doccer project has a disconnect between unit tests and integration tests:
- Unit tests call specific internal methods directly with simplified test data
- Integration tests run the entire application flow with real data
- The test API signatures don't match the actual implementation
- Visibility and method signatures are inconsistent between tests and code

As a result, several integration tests are failing: `test_basic_types_fixture`, `test_deprecation_fixture`, and `test_complex_fixture`. These failures stem from inconsistencies in rendering between the test expectations and the actual output.

## Approach: Consolidating Rendering Logic

## 1. Create a Unified Renderer API

The main issue is that there are two separate rendering approaches - the old `TextRenderer` with helper methods for unit tests, and the new `ParsedRenderer` for the actual implementation. The plan is to consolidate these into a single consistent API.

### Key changes:

1. **Make `ParsedRenderer` the single source of truth:**
   - Move all rendering functionality to the `ParsedRenderer` class
   - Update `ParsedRenderer` to conform to the exact output expectations shown in the test fixtures
   - Ensure the `ParsedRenderer` methods have consistent signatures and behaviors

2. **Expose Unit-Testable Methods in `ParsedRenderer`:**
   - Create public rendering methods for each component type (function, struct, enum, etc.)
   - Ensure these methods match the signatures of the current test helper methods
   - Move logic from `TextRenderer` helper methods into `ParsedRenderer`

3. **Fix Specific Rendering Issues:**
   - Indentation: Ensure consistent indentation (6 spaces for trait methods, not 8)
   - Return Types: Only add "-> ..." for explicit return types, not for Unit types
   - Documentation Comments: Ensure exactly one space after "///"
   - Trait Bounds: Include complete trait bounds for generic types
   - Re-exports: Add a dedicated re-exports section
   - Formatter Paths: Use consistent `$crate::fmt` vs `std::fmt` based on context

4. **Update Test Files:**
   - Modify unit tests to use the `ParsedRenderer` directly
   - Keep test assertions but make them call the corresponding methods on the new renderer

## 2. Implementation Approach

1. **First Phase - Core Methods:**
   - Create a complete `ParsedItem` renderer with all component renderers
   - Move all the rendering logic from `render_xxx` methods in `TextRenderer` to corresponding methods in `ParsedRenderer`
   - Ensure all the test helper methods in `TextRenderer` have equivalents in `ParsedRenderer`

2. **Second Phase - Fix Specific Issues:**
   - Implement the indentation fix for trait methods (6 spaces)
   - Fix the function return type rendering
   - Fix documentation comment whitespace
   - Update the generics renderer to show trait bounds
   - Add re-exports section to the renderer
   - Fix formatter path issues

3. **Third Phase - Test Updates:**
   - Update the unit tests to call the appropriate `ParsedRenderer` methods
   - Update assertions to test against the new renderer output
   - Add regression tests to prevent future regressions

## 3. Expected Outcome

1. A single, unified rendering API in `ParsedRenderer`
2. The `TextRenderer` becomes a thin wrapper around `ParsedRenderer`
3. All unit tests pass using the `ParsedRenderer` methods directly
4. Integration tests pass because the output matches the expected format
5. Better maintainability with consolidated logic
6. No duplication of rendering logic between the test and main code

## 4. Specific Issues to Fix

### Indentation Issues
- **Expected**: Trait implementation methods should use 6 spaces of indentation
- **Current**: Using 8 spaces instead

### Return Type Rendering
- **Expected**: Return types only shown when explicitly specified
- **Current**: Adding "-> ..." for functions with no explicit return

### Documentation Comments
- **Expected**: Exactly one space after "///" in all comments
- **Current**: Inconsistent spacing, especially with empty lines

### Trait Bounds
- **Expected**: Complete trait bounds shown in generics
- **Current**: Missing trait bounds for generic types

### Re-exports Section
- **Expected**: Dedicated re-exports section
- **Current**: Re-exports not properly formatted

### Formatter Paths
- **Expected**: Uses `$crate::fmt` in some contexts and `std::fmt` in others
- **Current**: Inconsistent formatter path references

This plan focuses on making the `ParsedRenderer` the single source of truth for rendering logic and ensuring it precisely matches the expected output format from the test fixtures. The unit tests will directly test the renderer's capabilities rather than having separate test helpers.
