use std::path::Path;
use std::process::Command;

mod snapshots;

/// Test doccer against a fixture by running it and returning the output
fn run_doccer_on_fixture(fixture_name: &str) -> String {
    // Path to the fixture crate
    let fixture_crate_path = format!("tests/fixtures/{}", fixture_name);

    // Ensure the fixture crate exists
    assert!(
        Path::new(&fixture_crate_path).exists(),
        "Fixture crate not found: {}",
        fixture_crate_path
    );

    // Build the doccer binary first
    let build_output = Command::new("cargo")
        .args(&["build", "--bin", "doccer"])
        .output()
        .expect("Failed to build doccer");

    if !build_output.status.success() {
        panic!(
            "Failed to build doccer:\n{}",
            String::from_utf8_lossy(&build_output.stderr)
        );
    }

    // Run doccer on the fixture
    let output = Command::new("cargo")
        .args(&[
            "run",
            "--bin",
            "doccer",
            "--",
            "--crate-path",
            &fixture_crate_path,
        ])
        .output()
        .expect("Failed to run doccer");

    if !output.status.success() {
        panic!(
            "Doccer failed on fixture '{}':\n{}",
            fixture_name,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    String::from_utf8(output.stdout).expect("Invalid UTF-8 in doccer output")
}

#[test]
fn test_basic_types_fixture() {
    let _settings = snapshots::configure_insta();
    let output = run_doccer_on_fixture("basic_types");
    insta::assert_snapshot!(output);
}

#[test]
fn test_generics_fixture() {
    let _settings = snapshots::configure_insta();
    let output = run_doccer_on_fixture("generics");
    insta::assert_snapshot!(output);
}

#[test]
fn test_modules_fixture() {
    let _settings = snapshots::configure_insta();
    let output = run_doccer_on_fixture("modules");
    insta::assert_snapshot!(output);
}

#[test]
fn test_complex_fixture() {
    let _settings = snapshots::configure_insta();
    let output = run_doccer_on_fixture("complex");
    insta::assert_snapshot!(output);
}

#[test]
fn test_deprecation_fixture() {
    let _settings = snapshots::configure_insta();
    let output = run_doccer_on_fixture("deprecation");
    insta::assert_snapshot!(output);
}