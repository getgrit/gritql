use crate::common::{get_fixtures_root, get_test_cmd};
use anyhow::Result;

use insta::assert_snapshot;

mod common;

#[test]
fn returns_expected_help() -> Result<()> {
    let mut cmd = get_test_cmd()?;

    let _fixtures_root = get_fixtures_root()?;

    cmd.arg("help");

    let output = cmd.output()?;

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    assert_snapshot!(String::from_utf8(output.stdout)?);

    Ok(())
}
