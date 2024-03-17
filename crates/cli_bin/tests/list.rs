use crate::common::get_test_cmd;
use anyhow::Result;
use common::get_fixture;
use insta::assert_snapshot;

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
