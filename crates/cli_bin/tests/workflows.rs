use crate::common::{get_fixture, get_test_cmd};
use anyhow::Result;
use marzano_gritmodule::config::REPO_CONFIG_DIR_NAME;

mod common;

#[test]
#[ignore = "No auth token is available in CI"]
fn run_test_workflow() -> Result<()> {
    let (_temp_dir, temp_fixtures_root) = get_fixture("grit_modules", true)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("apply")
        .arg("https://storage.googleapis.com/grit-workflows-dev-workflow_definitions/test/hello.js")
        .current_dir(temp_fixtures_root);

    let output = cmd.output()?;
    println!("stdout: {:?}", String::from_utf8(output.stdout.clone())?);
    println!("stderr: {:?}", String::from_utf8(output.stderr.clone())?);

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("Running hello workflow"),);

    Ok(())
}

#[test]
fn lists_user_workflow_only_once() -> Result<()> {
    let (_user_config, user_dir) = get_fixture("user_pattern", false)?;
    let user_grit_dir = user_dir.join(REPO_CONFIG_DIR_NAME);

    let mut cmd = get_test_cmd()?;
    cmd.current_dir(user_dir.as_path());
    cmd.arg("workflows")
        .arg("list")
        .env("TEST_ONLY_GRIT_USER_CONFIG", user_grit_dir);
    let output = cmd.output()?;

    println!("stdout: {:?}", String::from_utf8(output.stdout.clone())?);
    println!("stderr: {:?}", String::from_utf8(output.stderr.clone())?);

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;

    let occurrences = stdout.matches("hello").count();
    assert_eq!(occurrences, 1, "hello does not appear exactly once");

    Ok(())
}

// Ensure we can list workflows from ~/.grit/workflows
#[test]
fn lists_user_workflow() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("other_dir", false)?;
    let (_user_config, user_dir) = get_fixture("user_pattern", false)?;
    let user_grit_dir = user_dir.join(REPO_CONFIG_DIR_NAME);

    let mut cmd = get_test_cmd()?;
    cmd.current_dir(dir.as_path());
    cmd.arg("workflows")
        .arg("list")
        .env("TEST_ONLY_GRIT_USER_CONFIG", user_grit_dir);
    let output = cmd.output()?;

    println!("stdout: {:?}", String::from_utf8(output.stdout.clone())?);
    println!("stderr: {:?}", String::from_utf8(output.stderr.clone())?);

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("goodbye"));
    assert!(stdout.contains("hello"));

    Ok(())
}

// Ensure we can apply user workflows from ~/.grit/workflows
#[test]
#[ignore = "No auth token is available in CI"]
fn applies_user_workflows() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("other_dir", false)?;
    let (_user_config, user_dir) = get_fixture("user_pattern", false)?;
    let user_grit_dir = user_dir.join(REPO_CONFIG_DIR_NAME);

    let mut cmd = get_test_cmd()?;
    cmd.current_dir(dir.as_path());
    cmd.arg("apply")
        .arg("hello")
        .env("TEST_ONLY_GRIT_USER_CONFIG", user_grit_dir);
    let output = cmd.output()?;
    println!("stdout: {:?}", String::from_utf8(output.stdout.clone())?);
    println!("stderr: {:?}", String::from_utf8(output.stderr.clone())?);

    Ok(())
}
