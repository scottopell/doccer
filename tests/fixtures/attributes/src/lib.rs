//! Comprehensive demonstration of Rust attributes and conditional compilation.
//! This module tests doccer's ability to parse and display various compiler
//! attributes that provide important metadata about API usage and behavior.

use std::fmt::Display;

// =============================================================================
// Conditional Compilation Attributes
// =============================================================================

/// Available only when the "advanced" feature is enabled
#[cfg(feature = "advanced")]
pub struct AdvancedFeature {
    pub data: Vec<u8>,
}

/// Available only on Unix-like systems
#[cfg(unix)]
pub fn unix_specific_function() -> i32 {
    42
}

/// Available only on Windows systems
#[cfg(windows)]
pub fn windows_specific_function() -> i32 {
    84
}

/// Available only in debug builds
#[cfg(debug_assertions)]
pub fn debug_only_function() {
    println!("Debug mode enabled");
}

/// Available only when targeting x86_64 architecture
#[cfg(target_arch = "x86_64")]
pub const X86_64_SPECIFIC: usize = 64;

/// Available only when targeting ARM architecture
#[cfg(target_arch = "aarch64")]
pub const ARM_SPECIFIC: usize = 64;

/// Complex conditional compilation
#[cfg(all(feature = "advanced", unix, debug_assertions))]
pub fn complex_conditional() -> bool {
    true
}

/// Alternative implementations based on feature flags
#[cfg(feature = "fast")]
pub fn algorithm_impl() -> &'static str {
    "fast implementation"
}

#[cfg(not(feature = "fast"))]
pub fn algorithm_impl() -> &'static str {
    "standard implementation"
}

// =============================================================================
// Performance Attributes
// =============================================================================

/// A function that should always be inlined
#[inline]
pub fn always_inline_me(x: i32) -> i32 {
    x * 2
}

/// A function that should be aggressively inlined
#[inline(always)]
pub fn force_inline(x: i32) -> i32 {
    x + 1
}

/// A function that should never be inlined
#[inline(never)]
pub fn never_inline(x: i32) -> i32 {
    x - 1
}

/// A function that is unlikely to be called (cold path)
#[cold]
pub fn error_handler() -> ! {
    panic!("Critical error occurred");
}

/// A function optimized for size rather than speed
#[cfg(feature = "optimize")]
pub fn size_optimized_function() -> Vec<u8> {
    vec![1, 2, 3, 4, 5]
}

// =============================================================================
// Usage Requirement Attributes
// =============================================================================

/// A function whose return value must be used
#[must_use]
pub fn important_calculation() -> i32 {
    42
}

/// A function with a custom must_use message
#[must_use = "the result contains important error information"]
pub fn fallible_operation() -> Result<i32, String> {
    Ok(100)
}

/// A struct that must be used once created
#[must_use = "Guards must be held for the duration of the critical section"]
pub struct CriticalSectionGuard {
    _private: (),
}

impl CriticalSectionGuard {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

impl Drop for CriticalSectionGuard {
    fn drop(&mut self) {
        // Cleanup critical section
    }
}

// =============================================================================
// Memory Layout Attributes
// =============================================================================

/// A struct with C-compatible memory layout
#[repr(C)]
pub struct CCompatibleStruct {
    pub a: u32,
    pub b: u16,
    pub c: u8,
}

/// A transparent wrapper around a single field
#[repr(transparent)]
pub struct TransparentWrapper(pub u64);

/// A struct with packed memory layout (no padding)
#[repr(packed)]
pub struct PackedStruct {
    pub a: u8,
    pub b: u32,
    pub c: u16,
}

/// An enum with explicit discriminant representation
#[repr(u8)]
pub enum StatusCode {
    Success = 0,
    Warning = 1,
    Error = 2,
    Critical = 3,
}

/// A union type (unsafe)
#[repr(C)]
pub union IntOrFloat {
    pub int_value: i32,
    pub float_value: f32,
}

// =============================================================================
// Compiler Hint Attributes
// =============================================================================

/// A deprecated function with a message
#[deprecated(since = "0.2.0", note = "Use `new_function` instead")]
pub fn old_function() -> i32 {
    42
}

/// A function that should not be mangled (for FFI)
#[no_mangle]
pub extern "C" fn c_exported_function() -> i32 {
    100
}

/// A function with a custom export name
#[export_name = "custom_name"]
pub fn renamed_export() -> i32 {
    200
}

/// A function that may unwind across FFI boundaries
#[cfg(feature = "unwind")]
pub extern "C" fn may_unwind() -> i32 {
    panic!("This function may unwind");
}

/// A function that should not unwind
#[cfg(feature = "unwind")]
pub fn no_unwind() -> i32 {
    42
}

// =============================================================================
// Documentation Attributes
// =============================================================================

/// A function with detailed documentation
#[doc = "This function performs a complex calculation"]
#[doc = ""]
#[doc = "# Examples"]
#[doc = ""]
#[doc = "```rust"]
#[doc = "let result = documented_function(5);"]
#[doc = "assert_eq!(result, 10);"]
#[doc = "```"]
pub fn documented_function(x: i32) -> i32 {
    x * 2
}

/// A function that is hidden from documentation
#[doc(hidden)]
pub fn internal_function() -> i32 {
    42
}

// =============================================================================
// Testing Attributes
// =============================================================================

/// A test function
#[test]
fn test_basic_functionality() {
    assert_eq!(always_inline_me(5), 10);
}

/// A test that should panic
#[test]
#[should_panic]
fn test_panic_expected() {
    panic!("Expected panic");
}

/// A test that should panic with a specific message
#[test]
#[should_panic(expected = "specific error")]
fn test_panic_with_message() {
    panic!("specific error occurred");
}

/// A test that should be ignored
#[test]
#[ignore]
fn test_ignored() {
    // This test is ignored by default
}

/// A test that should be ignored with a reason
#[test]
#[ignore = "requires special hardware"]
fn test_ignored_with_reason() {
    // This test requires special hardware
}

/// A benchmark function (requires nightly and test feature)
#[cfg(feature = "bench")]
fn bench_simple_operation() {
    // Benchmark would go here
}

// =============================================================================
// Custom Attributes and Derives
// =============================================================================

/// A struct with common derives
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommonStruct {
    pub field1: String,
    pub field2: i32,
}

/// A struct with serialization derives (would require serde)
#[derive(Debug, Clone)]
#[cfg(feature = "serde")]
pub struct SerializableStruct {
    pub name: String,
    pub value: i32,
}

/// A struct with custom attribute (would be processed by proc macro)
#[derive(Debug)]
#[cfg(feature = "custom")]
pub struct CustomAttributeStruct {
    pub data: Vec<u8>,
}

// =============================================================================
// Compiler Configuration
// =============================================================================

/// A function that warns about unused variables
#[allow(unused_variables)]
pub fn unused_variables_allowed(x: i32, y: i32) -> i32 {
    let unused = 42;
    x
}

/// A function that denies certain warnings
#[deny(unused_must_use)]
pub fn must_use_enforced() -> i32 {
    let _result = important_calculation(); // Explicitly use the result
    42
}

/// A function with specific lint configuration
#[warn(clippy::all)]
#[allow(clippy::too_many_arguments)]
pub fn lint_configured(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32, g: i32, h: i32) -> i32 {
    a + b + c + d + e + f + g + h
}

// =============================================================================
// Linkage Attributes
// =============================================================================

/// A function with external linkage
#[link_name = "external_function"]
extern "C" {
    fn linked_function() -> i32;
}

/// A static with specific linkage
#[link_section = ".data"]
static SPECIAL_DATA: i32 = 42;

// =============================================================================
// Trait Implementations with Attributes
// =============================================================================

impl Display for CCompatibleStruct {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CCompatibleStruct {{ a: {}, b: {}, c: {} }}", self.a, self.b, self.c)
    }
}

impl Display for TransparentWrapper {
    #[cold]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TransparentWrapper({})", self.0)
    }
}

// =============================================================================
// Module-level Attributes
// =============================================================================

/// A submodule with attributes
#[cfg(feature = "submodule")]
pub mod conditional_module {
    /// Function only available when submodule feature is enabled
    #[inline]
    pub fn submodule_function() -> &'static str {
        "submodule active"
    }
}

/// A module that's always compiled but with specific attributes
#[allow(dead_code)]
pub mod utility_module {
    /// A utility function that might not be used
    #[must_use]
    pub fn utility_function() -> bool {
        true
    }
    
    /// A deprecated utility
    #[deprecated(note = "Use utility_function instead")]
    pub fn old_utility() -> bool {
        false
    }
}