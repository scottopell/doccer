use std::fs;
use std::path::Path;
use std::process::Command;

/// Test doccer against a fixture by comparing output with expected results
fn test_fixture(fixture_name: &str) {
    let json_path = format!("tests/{}.json", fixture_name);
    let expected_path = format!("tests/expected/{}.txt", fixture_name);

    // Ensure the JSON file exists
    assert!(
        Path::new(&json_path).exists(),
        "JSON fixture file not found: {}",
        json_path
    );

    // Ensure the expected output file exists
    assert!(
        Path::new(&expected_path).exists(),
        "Expected output file not found: {}",
        expected_path
    );

    // Run doccer on the JSON file
    let output = Command::new("./target/release/doccer")
        .arg(&json_path)
        .output()
        .expect("Failed to execute doccer");

    // Check that doccer executed successfully
    if !output.status.success() {
        panic!(
            "Doccer failed on {}: {}",
            fixture_name,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    // Read expected output
    let expected = fs::read_to_string(&expected_path).expect("Failed to read expected output file");

    // Compare actual output with expected
    let actual = String::from_utf8(output.stdout).expect("Doccer output is not valid UTF-8");

    if actual.trim() != expected.trim() {
        // Print detailed diff information
        println!("=== FIXTURE: {} ===", fixture_name);
        println!("Expected output:\n{}", expected);
        println!("Actual output:\n{}", actual);
        println!("=== END DIFF ===");

        panic!(
            "Output mismatch for fixture '{}'. See diff above.",
            fixture_name
        );
    }
}

#[test]
fn test_basic_types_fixture() {
    test_fixture("basic_types");
}

#[test]
fn test_generics_fixture() {
    test_fixture("generics");
}

#[test]
fn test_modules_fixture() {
    test_fixture("modules");
}

#[test]
fn test_complex_fixture() {
    test_fixture("complex");
}

#[test]
fn test_all_fixtures_exist() {
    let fixtures = ["basic_types", "generics", "modules", "complex"];

    for fixture in &fixtures {
        let json_path = format!("tests/{}.json", fixture);
        let expected_path = format!("tests/expected/{}.txt", fixture);

        assert!(
            Path::new(&json_path).exists(),
            "Missing JSON file: {}",
            json_path
        );

        assert!(
            Path::new(&expected_path).exists(),
            "Missing expected output: {}",
            expected_path
        );
    }
}

/// Test that doccer handles non-existent files gracefully
#[test]
fn test_invalid_input() {
    let output = Command::new("./target/release/doccer")
        .arg("nonexistent.json")
        .output()
        .expect("Failed to execute doccer");

    // Should fail with non-zero exit code
    assert!(!output.status.success());
}

/// Test that doccer requires an input argument
#[test]
fn test_missing_argument() {
    let output = Command::new("./target/release/doccer")
        .output()
        .expect("Failed to execute doccer");

    // Should fail with non-zero exit code
    assert!(!output.status.success());
}

/// Integration test to validate that JSON files are valid rustdoc output
#[test]
fn test_json_validity() {
    let fixtures = ["basic_types", "generics", "modules", "complex"];

    for fixture in &fixtures {
        let json_path = format!("tests/{}.json", fixture);
        let content = fs::read_to_string(&json_path).expect("Failed to read JSON file");

        // Parse as JSON to ensure it's valid
        let _: serde_json::Value =
            serde_json::from_str(&content).expect(&format!("Invalid JSON in {}", json_path));
    }
}

/// Test performance - all fixtures should process quickly
#[test]
fn test_performance() {
    use std::time::Instant;

    let fixtures = ["basic_types", "generics", "modules", "complex"];
    let start = Instant::now();

    for fixture in &fixtures {
        let json_path = format!("tests/{}.json", fixture);

        let output = Command::new("./target/release/doccer")
            .arg(&json_path)
            .output()
            .expect("Failed to execute doccer");

        assert!(output.status.success(), "Doccer failed on {}", fixture);
    }

    let duration = start.elapsed();

    // All fixtures should process in under 30 seconds as per success criteria
    assert!(
        duration.as_secs() < 30,
        "Test took too long: {:?} (should be < 30s)",
        duration
    );
}
