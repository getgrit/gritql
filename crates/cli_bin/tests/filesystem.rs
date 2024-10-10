use std::fs;

use crate::common::get_fixture;
use anyhow::Result;
use assert_cmd::{cargo::cargo_bin, Command};
use insta::assert_snapshot;

mod common;

fn prepare_read_only_install() -> Result<(tempfile::TempDir, std::path::PathBuf)> {
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

    Ok((temp_dir, dest_path))
}

#[test]
fn runs_doctor_from_read_only_dir() -> Result<()> {
    let (_temp_dir, dest_path) = prepare_read_only_install()?;

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

#[test]
fn fails_stdlib_pattern_without_grit_config() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("ro_file", false)?;

    let (_install_dir, bin_path) = prepare_read_only_install()?;

    let mut cmd = Command::new(bin_path);
    cmd.current_dir(fixture_dir.clone());
    cmd.arg("apply")
        .arg("no_console_log")
        .arg("simple.js")
        .arg("--force");

    let output = cmd.output()?;

    println!("output: {}END###", String::from_utf8_lossy(&output.stdout));
    println!("error: {}END###", String::from_utf8_lossy(&output.stderr));

    assert!(!output.status.success(), "Command was expected to fail");

    // Make sure output includes the dir, since if we don't have that and we can't initialize it we should fail
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains(&_install_dir.path().display().to_string()),);

    Ok(())
}

#[test]
fn run_stdlib_pattern_with_local_grit_config() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("ro_file", false)?;

    let (_install_dir, bin_path) = prepare_read_only_install()?;

    // Run git init, to make it a repo
    let mut cmd = Command::new("git");
    cmd.current_dir(fixture_dir.clone());
    cmd.arg("init");

    let output = cmd.output()?;

    println!(
        "git init output: {}END###",
        String::from_utf8_lossy(&output.stdout)
    );
    println!(
        "git init error: {}END###",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    // Run init first
    let mut cmd = Command::new(bin_path.clone());
    cmd.current_dir(fixture_dir.clone());
    cmd.arg("init");

    let output = cmd.output()?;

    println!(
        "init output: {}END###",
        String::from_utf8_lossy(&output.stdout)
    );
    println!(
        "init error: {}END###",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let mut cmd = Command::new(bin_path);
    cmd.current_dir(fixture_dir.clone());
    cmd.arg("apply")
        .arg("no_console_log")
        .arg("simple.js")
        .arg("--force");

    let output = cmd.output()?;

    println!("output: {}END###", String::from_utf8_lossy(&output.stdout));
    println!("error: {}END###", String::from_utf8_lossy(&output.stderr));

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    // Read back the require.js file
    let content: String = fs_err::read_to_string(fixture_dir.join("simple.js"))?;

    // assert that it matches snapshot
    assert_snapshot!(content);

    Ok(())
}
