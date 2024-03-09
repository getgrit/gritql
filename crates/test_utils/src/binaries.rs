use anyhow::Result;

use assert_cmd::Command;
use marzano_auth::testing::TEST_AUTH_INFO;

/// Construct a cmd for the binary under test
pub fn get_test_cmd() -> Result<Command> {
    let mut cmd = Command::cargo_bin("gouda")?;
    let auth = TEST_AUTH_INFO.as_ref().unwrap();
    cmd.env("GRIT_AUTH_TOKEN", &auth.access_token);
    cmd.env("GRIT_TELEMETRY_DISABLED", "true");
    Ok(cmd)
}
