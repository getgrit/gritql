use anyhow::Result;
use common::get_test_cmd;
use insta::{assert_snapshot, assert_yaml_snapshot};

use crate::common::get_fixture;

mod common;

#[test]
fn run_pattern_with_private() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("private_patterns", true)?;

    // Get the pattern list
    let mut list_cmd = get_test_cmd()?;
    list_cmd.current_dir(dir.clone());
    list_cmd.arg("list").arg("--json");

    // Parse and snapshot it
    let output = list_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    let parsed = serde_json::from_str::<serde_json::Value>(&stdout)?;
    assert_yaml_snapshot!(parsed);

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());

    apply_cmd.arg("apply").arg("combined").arg("input.ts");

    let output = apply_cmd.output()?;

    // Assert stdout
    let stdout = String::from_utf8(output.stdout)?;

    println!("stdout: {:?}", stdout);

    // Assert that the command executed successfully
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    // Read back the lifecycle.tsx file
    let content: String = fs_err::read_to_string(dir.join("input.ts"))?;

    // assert that it matches snapshot
    assert_snapshot!(content);

    Ok(())
}
