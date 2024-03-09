use crate::common::{get_fixture, get_test_cmd};
use anyhow::Result;

mod common;

#[test]
fn runs_doctor_in_fixture() -> Result<()> {
    let (_temp_dir, temp_fixtures_root) = get_fixture("grit_modules", true)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("doctor").current_dir(temp_fixtures_root);

    let output = cmd.output()?;
    println!("output: {:?}", String::from_utf8(output.stdout.clone())?);

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    Ok(())
}
