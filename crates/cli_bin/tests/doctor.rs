use std::fs;

use crate::common::{get_fixture, get_test_cmd};
use anyhow::Result;
use assert_cmd::{cargo::cargo_bin, Command};

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

#[test]
fn test_runs_from_read_only_dir() -> Result<()> {
    let bin_path = cargo_bin(env!("CARGO_PKG_NAME"));
    let temp_dir = tempfile::tempdir()?;

    let install_dir = temp_dir.path().join("install").join("bin");
    fs::create_dir_all(&install_dir)?;

    let dest_path = install_dir.join("grit");

    fs::copy(bin_path, &dest_path)?;

    // Make the temp dir read-only
    let mut stack = vec![temp_dir.path().to_path_buf()];

    while let Some(current_dir) = stack.pop() {
        for entry in fs::read_dir(&current_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                stack.push(entry.path());
            }
        }
        let mut perms = fs::metadata(&current_dir)?.permissions();
        perms.set_readonly(true);
        fs::set_permissions(&current_dir, perms)?;
    }

    let mut cmd = Command::new(dest_path);
    cmd.arg("doctor");

    let output = cmd.output()?;

    let stderr = String::from_utf8_lossy(&output.stderr);
    println!("stderr: {}", stderr);

    let stdout = String::from_utf8_lossy(&output.stdout);
    println!("stdout: {}", stdout);

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    Ok(())
}
