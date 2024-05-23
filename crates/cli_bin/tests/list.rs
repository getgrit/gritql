use crate::common::get_test_cmd;
use anyhow::Result;
use common::get_fixture;
use insta::assert_snapshot;
use insta::assert_yaml_snapshot;

mod common;

#[test]
fn list_just_workflows() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("workflows", false)?;

    let mut cmd = get_test_cmd()?;

    cmd.arg("workflows").arg("list").current_dir(dir.clone());

    let output = cmd.output()?;
    println!("{}", String::from_utf8(output.stdout.clone())?);

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    assert_snapshot!(String::from_utf8(output.stdout)?);

    Ok(())
}

#[test]
fn list_corrupted_dir() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("one_bad_markdown", false)?;

    let mut cmd = get_test_cmd()?;

    cmd.arg("patterns").arg("list").current_dir(dir.clone());

    let output = cmd.output()?;

    assert!(
        !output.status.success(),
        "Command was expected to fail but it didn't"
    );

    println!("stderr: {}", String::from_utf8(output.stderr.clone())?);
    println!("stdout: {}", String::from_utf8(output.stdout.clone())?);

    assert!(String::from_utf8(output.stderr)?.contains("ailed to parse markdown pattern"));

    Ok(())
}

#[test]
fn list_jsonl() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("patterns_list", true)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("patterns")
        .arg("list")
        .arg("--jsonl")
        .current_dir(dir);

    let output = cmd.output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let parsed_patterns: Vec<serde_json::Value> = stdout
        .lines()
        .map(|line| serde_json::from_str(line).expect("Failed to parse line as JSON"))
        .collect();

    assert!(!parsed_patterns.is_empty(), "No JSON lines were parsed.");

    assert!(
        parsed_patterns.len() > 5,
        "Total JSON lines parsed should be more than 5."
    );

    assert_yaml_snapshot!(parsed_patterns);

    Ok(())
}

/// List all patterns in a directory with a custom grit directory by specifying the `--grit-dir` flag
#[test]
fn list_custom_grit_dir() -> Result<()> {
    let (_other_fixture, other_dir) = get_fixture("patterns_list", true)?;

    let (_temp_dir, dir) = get_fixture("one_bad_markdown", false)?;

    let mut cmd = get_test_cmd()?;

    cmd.arg("patterns")
        .arg("list")
        .arg("--jsonl")
        .arg("--grit-dir")
        .arg(other_dir)
        .current_dir(dir);

    let output = cmd.output()?;

    println!("stderr: {}", String::from_utf8(output.stderr.clone())?);
    println!("stdout: {}", String::from_utf8(output.stdout.clone())?);

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    assert!(String::from_utf8(output.stdout)?.contains("remove_console_error"));

    Ok(())
}
