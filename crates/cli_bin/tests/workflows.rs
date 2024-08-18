use crate::common::{get_fixture, get_test_cmd};
use anyhow::Result;

mod common;

#[test]
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
