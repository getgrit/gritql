use std::path::PathBuf;

use crate::common::get_fixture;
use anyhow::Result;
use common::get_test_cmd;
use insta::assert_snapshot;

mod common;

fn check_cmd_output(dir: PathBuf, args: &[&str], expected_code: Option<i32>) -> Result<String> {
    let mut check_cmd = get_test_cmd()?;
    check_cmd.current_dir(dir).arg("check").arg("--no-cache");
    for arg in args {
        check_cmd.arg(arg);
    }

    let output = check_cmd.output()?;
    println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
    println!("stdout: {}", String::from_utf8_lossy(&output.stdout));

    if let Some(expected_code) = expected_code {
        let actual_code = output.status.code().unwrap();
        assert_eq!(
            actual_code, expected_code,
            "Expected exit code {} but got {}",
            expected_code, actual_code
        );
    }

    let combined = String::from_utf8_lossy(&output.stdout).to_string()
        + "\n"
        + String::from_utf8_lossy(&output.stderr).as_ref();

    Ok(combined)
}

#[test]
fn grit_dir_with_pattern_config_py() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("check_python", true)?;
    let output = check_cmd_output(dir, &["test.py"], None)?;
    assert!(output.contains("Fix available"));
    Ok(())
}

#[test]
fn grit_dir_with_pattern_config_js() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("check_js", true)?;
    let output = check_cmd_output(dir, &["test.js"], None)?;
    assert!(output.contains("Fix available"));
    Ok(())
}

#[test]
fn grit_dir_multiple_languages_single_target_js() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("check_multiple_targeted", true)?;
    let output = check_cmd_output(dir, &["test.js"], None)?;
    let no_rewrites = output.matches("Fix available.").count();
    assert!(no_rewrites == 1);
    Ok(())
}

#[test]
fn grit_dir_multiple_languages_single_multiple_targets() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("check_multiple_targeted", true)?;
    let output = check_cmd_output(dir, &[], None)?;
    assert_snapshot!(output);
    Ok(())
}

#[test]
fn check_json_output() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("check_multiple_targeted", true)?;
    let output = check_cmd_output(dir, &["--json"], None)?;
    assert!(output.contains(r##"{"check_id":"#test_python/python","local_name":"test_python","start":{"line":2,"col":3,"offset":15},"end":{"line":2,"col":17,"offset":29},"path":"./test.py","extra":{"message":null,"severity":"error"}}"##));
    assert!(output.contains(r##"{"check_id":"#test_js/js","local_name":"test_js","start":{"line":2,"col":3,"offset":35},"end":{"line":2,"col":8,"offset":40},"path":"./test.js","extra":{"message":null,"severity":"error"}}"##));
    Ok(())
}

#[test]
fn grit_dir_without_grit_modules() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("check_js", false)?;
    let output = check_cmd_output(dir, &["test.js"], Some(1))?;
    assert!(output.contains("Fix available"));
    Ok(())
}

#[test]
fn check_respects_js_suppress() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("check_ignore", false)?;
    let output = check_cmd_output(dir, &["test.js"], None)?;
    assert!(output.contains("Fix available"));
    assert!(output.contains("third"));
    assert!(!output.contains("another_test"));
    Ok(())
}

#[test]
fn check_respects_python_suppress() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("check_ignore", false)?;
    let output = check_cmd_output(dir, &["test.py"], None)?;
    assert!(output.contains("Fix available"));
    assert!(output.contains("test_python"));
    assert!(!output.contains("another_python"));
    Ok(())
}

#[test]
fn check_github_output() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("check_actions", true)?;

    let summary_file = dir.join("summary.md");

    let mut check_cmd = get_test_cmd()?;
    check_cmd
        .current_dir(dir)
        .env("GITHUB_STEP_SUMMARY", summary_file.to_str().unwrap())
        .arg("check")
        .arg("--no-cache")
        .arg("--github-actions")
        .arg("--level")
        .arg("info");

    let output = check_cmd.output()?;

    // Verify the exit code - we need to make sure we fail if there are any error
    assert_eq!(output.status.code(), Some(1));

    let output = String::from_utf8_lossy(&output.stdout).to_string();

    // sort it by line to reduce flakes
    let mut lines: Vec<&str> = output.lines().collect();
    lines.sort();
    let output = lines.join("\n");
    assert_snapshot!(output);

    // Make sure we wrote the summary file
    let summary = std::fs::read_to_string(summary_file)?;
    assert_snapshot!(summary);

    Ok(())
}

#[test]
fn check_clean_github_output() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("check_actions_clean", true)?;

    let summary_file = dir.join("summary.md");

    let mut check_cmd = get_test_cmd()?;
    check_cmd
        .current_dir(dir)
        .env("GITHUB_STEP_SUMMARY", summary_file.to_str().unwrap())
        .arg("check")
        .arg("--no-cache")
        .arg("--github-actions")
        .arg("--level")
        .arg("info");

    let output = check_cmd.output()?;

    // Verify the exit code - this should be clean
    assert_eq!(output.status.code(), Some(0));

    let output = String::from_utf8_lossy(&output.stdout).to_string();

    // sort it by line to reduce flakes
    let mut lines: Vec<&str> = output.lines().collect();
    lines.sort();
    let output = lines.join("\n");
    assert_snapshot!(output);

    // Make sure we wrote the summary file
    let summary = std::fs::read_to_string(summary_file)?;
    assert_snapshot!(summary);

    Ok(())
}

#[test]
fn does_not_attempt_to_check_universal_pattern() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("check_universal", true)?;
    let output = check_cmd_output(dir, &["--level=info", "test.js"], None)?;
    assert!(output.contains("Fix available"));
    Ok(())
}

#[test]
fn check_only_in_diff() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("only_diff_check", true)?;

    let mut cmd = get_test_cmd()?;

    cmd.arg("check").arg("--only-in-diff").arg("test.diff").current_dir(dir.clone());

    let output = cmd.output()?;

    assert!(
        output.status.success(),
        "Command failed"
    );

    assert!(String::from_utf8(output.stdout)?.contains("Processed 1 files and found 1 match"));
    
    let content = std::fs::read_to_string(dir.join("index.js"))?;
    assert!(!content.contains("console.log('really cool')"));
    assert!(content.contains("console.log('cool')"));

    Ok(())
}
