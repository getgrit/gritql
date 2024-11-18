use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_grit_format() {
    let temp_dir = tempdir().unwrap();
    let temp_path = temp_dir.path();

    // Create a sample .grit file
    let grit_file_path = temp_path.join("sample.grit");
    fs::write(&grit_file_path, "pattern sample_pattern() { }").unwrap();

    // Create a sample .yaml file
    let yaml_file_path = temp_path.join("sample.yaml");
    fs::write(&yaml_file_path, "patterns:\n  - pattern: sample_pattern\n").unwrap();

    // Create a sample markdown file
    let md_file_path = temp_path.join("sample.md");
    fs::write(&md_file_path, "```grit\npattern sample_pattern() { }\n```\n").unwrap();

    // Run the `grit format` command
    let mut cmd = Command::cargo_bin("grit").unwrap();
    cmd.arg("format")
        .current_dir(temp_path)
        .assert()
        .success()
        .stdout(predicate::str::contains("Formatted"));

    // Verify the .grit file is formatted
    let formatted_grit = fs::read_to_string(&grit_file_path).unwrap();
    assert_eq!(formatted_grit, "pattern sample_pattern() {}\n");

    // Verify the .yaml file is formatted
    let formatted_yaml = fs::read_to_string(&yaml_file_path).unwrap();
    assert_eq!(formatted_yaml, "patterns:\n  - pattern: sample_pattern\n");

    // Verify the markdown file is formatted
    let formatted_md = fs::read_to_string(&md_file_path).unwrap();
    assert_eq!(formatted_md, "```grit\npattern sample_pattern() {}\n```\n");
}
