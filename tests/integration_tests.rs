use similar::{ChangeTag, TextDiff};
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Custom assertion that produces a readable diff when strings don't match
fn assert_strings_eq(actual: &str, expected: &str, message: &str) {
    if actual.trim() != expected.trim() {
        // Create a text diff
        let diff = TextDiff::from_lines(expected.trim(), actual.trim());

        // Format the diff as a string
        let mut diff_output = String::new();

        // Count the number of lines and differences
        let expected_lines = expected.trim().lines().count();
        let actual_lines = actual.trim().lines().count();
        let mut deletions = 0;
        let mut insertions = 0;

        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => deletions += 1,
                ChangeTag::Insert => insertions += 1,
                _ => {}
            }
        }

        // Add a summary header
        diff_output.push_str(&format!(
            "{}:\n\nSummary: {} line(s) changed - {} insertion(s), {} deletion(s)\n",
            message,
            deletions + insertions,
            insertions,
            deletions
        ));
        diff_output.push_str(&format!(
            "Expected: {} lines, Actual: {} lines\n\n",
            expected_lines, actual_lines
        ));
        diff_output.push_str("Legend: \x1b[31m- EXPECTED\x1b[0m | \x1b[32m+ ACTUAL\x1b[0m\n\n");

        // Line numbers to help with context
        let mut line_number_expected = 1;
        let mut line_number_actual = 1;

        // Track changes to show context lines only around differences
        let mut show_line = false;
        let mut change_lines = Vec::new();
        let context_lines = 3; // Number of context lines to show before and after changes

        // First pass: identify which lines have changes
        for (i, change) in diff.iter_all_changes().enumerate() {
            if change.tag() != ChangeTag::Equal {
                for j in i.saturating_sub(context_lines)..=i + context_lines {
                    change_lines.push(j);
                }
            }
        }

        // Second pass: build the diff output
        for (i, change) in diff.iter_all_changes().enumerate() {
            // Decide if we should show this line
            show_line = change_lines.contains(&i);

            // Add separator for non-consecutive changes
            if i > 0 && show_line && !change_lines.contains(&(i - 1)) {
                diff_output.push_str("...\n");
            }

            if show_line {
                let (prefix, content) = match change.tag() {
                    ChangeTag::Delete => {
                        let prefix = format!("{:4} \x1b[31m- EXPECTED\x1b[0m | ", line_number_expected);
                        line_number_expected += 1;
                        (prefix, format!("\x1b[31m{}\x1b[0m", change))
                    }
                    ChangeTag::Insert => {
                        let prefix = format!("{:4} \x1b[32m+ ACTUAL  \x1b[0m | ", line_number_actual);
                        line_number_actual += 1;
                        (prefix, format!("\x1b[32m{}\x1b[0m", change))
                    }
                    ChangeTag::Equal => {
                        let prefix = format!("{:4}   CONTEXT | ", line_number_expected);
                        line_number_expected += 1;
                        line_number_actual += 1;
                        (prefix, format!("{}", change))
                    }
                };

                let formatted_line = format!("{}{}", prefix, content);
                diff_output.push_str(&formatted_line);
            } else {
                // Update line numbers for hidden lines
                if change.tag() == ChangeTag::Delete || change.tag() == ChangeTag::Equal {
                    line_number_expected += 1;
                }
                if change.tag() == ChangeTag::Insert || change.tag() == ChangeTag::Equal {
                    line_number_actual += 1;
                }
            }
        }

        // If the diff is too large, suggest writing to a file for easier comparison
        if diff_output.lines().count() > 50 {
            diff_output.push_str("\nNote: For easier comparison, consider writing expected and actual outputs to files and using a diff tool.\n");
        }

        // Save actual and expected output to temporary files for manual comparison
        if diff_output.lines().count() > 30 {
            let tmp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

            let expected_path = tmp_dir.path().join("expected.txt");
            let actual_path = tmp_dir.path().join("actual.txt");

            let _ = fs::write(&expected_path, expected.trim());
            let _ = fs::write(&actual_path, actual.trim());

            diff_output.push_str(&format!(
                "\nFiles saved for comparison:\n  Expected: {}\n  Actual: {}\n",
                expected_path.display(),
                actual_path.display()
            ));

            // Don't drop the tempdir so files remain accessible
            std::mem::forget(tmp_dir);
        }

        // Finally, panic with the formatted diff
        panic!("\n{}", diff_output);
    }
}

/// Test doccer against a fixture by comparing output with expected results
fn test_fixture(fixture_name: &str) {
    // Path to the fixture crate
    let fixture_crate_path = format!("tests/fixtures/{}", fixture_name);

    // Ensure the fixture crate exists
    assert!(
        Path::new(&fixture_crate_path).exists(),
        "Fixture crate not found: {}",
        fixture_crate_path
    );

    // Get path for expected output
    let expected_path = format!("tests/expected/{}.txt", fixture_name);

    // Ensure expected output file exists
    assert!(
        Path::new(&expected_path).exists(),
        "Expected output file not found: {}",
        expected_path
    );

    // Read expected output
    let expected = fs::read_to_string(&expected_path).expect("Failed to read expected output file");

    // Build the doccer binary first
    let build_output = Command::new("cargo")
        .args(["build"])
        .output()
        .expect("Failed to build doccer");

    assert!(
        build_output.status.success(),
        "Failed to build doccer: {}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    // Run doccer with --crate-path to generate fresh documentation from the fixture crate
    let output = Command::new("./target/debug/doccer")
        .args(["--crate-path", &fixture_crate_path])
        .output()
        .expect("Failed to execute doccer");

    // Check that the command succeeded
    assert!(
        output.status.success(),
        "doccer failed to run on {}: {}",
        fixture_crate_path,
        String::from_utf8_lossy(&output.stderr)
    );

    let actual = String::from_utf8_lossy(&output.stdout).to_string();

    // Compare actual output with expected output using our custom diff function
    assert_strings_eq(
        &actual,
        &expected,
        &format!(
            "Output for '{}' doesn't match expected output",
            fixture_name
        ),
    );
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
fn test_deprecation_fixture() {
    test_fixture("deprecation");
}

/// Test that all fixture crates and expected files exist
#[test]
fn test_all_fixtures_exist() {
    let fixtures = [
        "basic_types",
        "generics",
        "modules",
        "complex",
        "deprecation",
    ];

    for fixture in &fixtures {
        let fixture_crate_path = format!("tests/fixtures/{}", fixture);
        let expected_path = format!("tests/expected/{}.txt", fixture);

        assert!(
            Path::new(&fixture_crate_path).exists(),
            "Missing fixture crate: {}",
            fixture_crate_path
        );

        assert!(
            Path::new(&expected_path).exists(),
            "Missing expected output: {}",
            expected_path
        );
    }
}

/// Test that doccer requires an input argument
#[test]
fn test_missing_argument() {
    // Ensure debug build exists
    let build_output = Command::new("cargo")
        .args(["build"])
        .output()
        .expect("Failed to build doccer");

    assert!(
        build_output.status.success(),
        "Failed to build doccer: {}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    let output = Command::new("./target/debug/doccer")
        .output()
        .expect("Failed to execute doccer");

    // Should fail with non-zero exit code
    assert!(!output.status.success());
}

/// Test to validate that we can handle sample JSON
/// This uses a small sample we know is valid, not the fixture files
#[test]
fn test_json_validity() {
    // Create a sample valid JSON
    let sample_json = r#"
    {
        "root": 1,
        "index": {
            "1": {
                "id": 1,
                "crate_id": 0,
                "name": "example",
                "visibility": "public",
                "docs": "Example crate",
                "links": {},
                "attrs": [],
                "deprecation": null,
                "inner": {"module": {"is_crate": true, "items": [], "is_stripped": false}}
            }
        },
        "external_crates": {},
        "paths": {},
        "crate_version": null,
        "includes_private": false,
        "format_version": 40
    }"#;

    // Parse as JSON to ensure we can handle this format
    let _: serde_json::Value =
        serde_json::from_str(sample_json).expect("Sample JSON should be valid");
}

/// Test the command-line parsing for local crate mode and direct file reading
/// This test doesn't require the nightly compiler to be installed
/// TODO this test is testing deprecated functionality (reading json file directly via CLI)
/// This test _content_ is valid, but it needs to be reframed as a unit test rather than an integration test.
#[test]
fn test_cli_command_parsing() {
    // Create a mock sample.json with minimal valid content
    let json_content = r#"{"root":1,"index":{"1":{"id":1,"crate_id":0,"name":"sample","visibility":"public","docs":"Sample crate","links":{},"attrs":[],"deprecation":null,"inner":{"module":{"is_crate":true,"items":[],"is_stripped":false}}}},"external_crates":{},"paths":{},"crate_version":null,"includes_private":false,"format_version":40}"#;

    // Write to a temporary file
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let temp_file = temp_dir.path().join("sample.json");
    fs::write(&temp_file, json_content).expect("Failed to write sample JSON file");

    // Ensure debug build exists
    let build_output = Command::new("cargo")
        .args(["build"])
        .output()
        .expect("Failed to build doccer");

    assert!(
        build_output.status.success(),
        "Failed to build doccer: {}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    // Verify we can read a file directly
    let output = Command::new("./target/debug/doccer")
        .arg(&temp_file)
        .output()
        .expect("Failed to run doccer with sample JSON file");

    assert!(
        output.status.success(),
        "Command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify output contains expected content
    let output_str = String::from_utf8_lossy(&output.stdout);
    assert!(
        output_str.contains("# Crate: sample"),
        "Output should contain crate name"
    );
    assert!(
        output_str.contains("Sample crate"),
        "Output should contain crate documentation"
    );

    // Test for expected command-line help output
    let help_output = Command::new("./target/debug/doccer")
        .args(["--help"])
        .output()
        .expect("Failed to run doccer --help");

    let help_text = String::from_utf8_lossy(&help_output.stdout);

    // Verify help output contains expected options
    assert!(
        help_text.contains("--crate-path"),
        "Help should mention --crate-path option"
    );
    assert!(
        help_text.contains("-p, --package"),
        "Help should mention --package option"
    );
    assert!(
        help_text.contains("-V, --crate-version"),
        "Help should mention --crate-version option"
    );
    assert!(
        help_text.contains("-t, --target"),
        "Help should mention --target option"
    );

    // Clean up
    temp_dir
        .close()
        .expect("Failed to clean up temporary directory");
}
