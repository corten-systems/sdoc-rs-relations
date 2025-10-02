use assert_cmd::prelude::*;
use serde_json::Value;

use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn project_root() -> PathBuf {
    // Tests run with CWD at the project root by default
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn inputs_dir() -> PathBuf {
    project_root().join("tests/in")
}

fn outputs_dir() -> PathBuf {
    project_root().join("tests/out")
}

#[test]
fn cli_outputs_match_expected_json_for_all_inputs() {
    let in_dir = inputs_dir();
    let out_dir = outputs_dir();

    // Collect all .rs files in tests/in
    let mut inputs: Vec<PathBuf> = fs::read_dir(&in_dir)
        .expect("tests/in dir should exist")
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|e| e == "rs").unwrap_or(false))
        .collect();

    // Ensure deterministic order
    inputs.sort();

    assert!(
        !inputs.is_empty(),
        "No input .rs files found in {}",
        in_dir.display()
    );

    for input in inputs {
        // The expected output file path
        let expected_path = out_dir.join(
            input
                .file_name()
                .expect("input file should have a name")
                .to_string_lossy()
                .to_string()
                + ".json",
        );

        // Ensure the expected file exists
        assert!(
            expected_path.exists(),
            "Missing expected JSON for input {} at {}",
            input.display(),
            expected_path.display()
        );

        // Run the binary with the prefix set to tests/in so only the filename remains
        let mut cmd = Command::cargo_bin("sdoc-rs-relations").expect("binary builds");
        let assert = cmd.arg("--prefix").arg(&in_dir).arg(&input).assert();

        // Successful exit
        let output = assert.get_output();
        if !output.status.success() {
            panic!(
                "Binary failed for {}: status: {:?}\nstdout:\n{}\nstderr:\n{}",
                input.display(),
                output.status.code(),
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        // Parse actual JSON from stdout
        let actual_json: Value = serde_json::from_slice(&output.stdout)
            .unwrap_or_else(|e| panic!("Invalid JSON from stdout for {}: {}", input.display(), e));

        // Load and parse expected JSON
        let expected_bytes = fs::read(&expected_path)
            .unwrap_or_else(|e| panic!("Failed reading {}: {}", expected_path.display(), e));
        let expected_json: Value = serde_json::from_slice(&expected_bytes).unwrap_or_else(|e| {
            panic!(
                "Invalid JSON in expected file {}: {}",
                expected_path.display(),
                e
            )
        });

        // Compare canonicalized JSON structures
        pretty_assertions::assert_eq!(
            expected_json,
            actual_json,
            "JSON mismatch for input {} (expected: {}, actual: stdout)",
            input.display(),
            expected_path.display()
        );
    }
}
