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
fn lists_user_workflow() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("format", false)?;
    let (_user_config, user_dir) = get_fixture("user_pattern", false)?;
    let user_grit_dir = user_dir.join(REPO_CONFIG_DIR_NAME);

    let mut lst_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    apply_cmd
        .arg("apply")
        .arg("very_special_console_log")
        .arg("whitespace.js")
        .env("GRIT_USER_CONFIG", user_grit_dir);
    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("Processed 1 files and found 1 matches"));

    let content: String = fs_err::read_to_string(dir.join("whitespace.js"))?;
    assert_snapshot!(content);

    Ok(())
}
