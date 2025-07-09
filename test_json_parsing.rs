use rustdoc_types::Crate;

fn main() {
    // Test 1: Basic malformed JSON
    let malformed_json = r#"{"format_version":"#;
    let result: Result<Crate, _> = serde_json::from_str(malformed_json);
    println!("Test 1 - Malformed JSON:");
    match result {
        Ok(_) => println!("  ✓ Parsed successfully"),
        Err(e) => println!("  ✗ Error: {}", e),
    }
    
    // Test 2: Truncated JSON
    let truncated_json = r#"{"format_version":40,"index":{"#;
    let result: Result<Crate, _> = serde_json::from_str(truncated_json);
    println!("\nTest 2 - Truncated JSON:");
    match result {
        Ok(_) => println!("  ✓ Parsed successfully"),
        Err(e) => println!("  ✗ Error: {}", e),
    }
    
    // Test 3: Valid JSON
    let valid_json = r#"{
        "format_version": 40,
        "index": {
            "0": {
                "id": "0",
                "crate_id": 0,
                "name": "test",
                "span": null,
                "visibility": "public",
                "docs": null,
                "links": {},
                "attrs": [],
                "deprecation": null,
                "inner": "module"
            }
        },
        "paths": {
            "0": {
                "crate_id": 0,
                "path": ["test"]
            }
        },
        "external_crates": {},
        "root": "0"
    }"#;
    let result: Result<Crate, _> = serde_json::from_str(valid_json);
    println!("\nTest 3 - Valid JSON:");
    match result {
        Ok(_) => println!("  ✓ Parsed successfully"),
        Err(e) => println!("  ✗ Error: {}", e),
    }
}