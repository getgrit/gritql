use crate::common::{get_fixture, get_test_cmd};
use anyhow::Result;

mod common;

#[test]
fn confirm_telemetry_flush() -> Result<()> {
    let (_temp_dir, temp_fixtures_root) = get_fixture("grit_modules", true)?;

    let mut cmd = get_test_cmd()?;
    cmd.env("GRIT_TELEMETRY_DISABLED", "false");
    cmd.env("GRIT_TELEMETRY_FOREGROUND", "true");
    cmd.arg("doctor").current_dir(temp_fixtures_root);

    let output = cmd.output()?;
    println!("output: {:?}", String::from_utf8(output.stdout.clone())?);

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    // Confirm output flushed
    let output_str = String::from_utf8(output.stdout.clone())?;
    assert!(output_str.contains("Successfully sent event command-completed"));

    Ok(())
}
