use std::{env, path, process};

use anyhow::Result;
use assert_cmd::cargo::CommandCargoExt;
use assert_cmd::Command;

use marzano_gritmodule::config::GRIT_GLOBAL_DIR_ENV;
use tempfile::tempdir;

pub const BIN_NAME: &str = "marzano";
pub const INSTA_FILTERS: &[(&str, &str)] = &[(
    r"\b[[:xdigit:]]{8}-[[:xdigit:]]{4}-[[:xdigit:]]{4}-[[:xdigit:]]{4}-[[:xdigit:]]{12}\b",
    "[UUID]",
)];

#[allow(dead_code)]
pub fn get_test_cmd() -> Result<Command> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.env("GRIT_TELEMETRY_DISABLED", "true");
    Ok(cmd)
}

#[allow(dead_code)]
pub fn get_test_process_cmd() -> Result<process::Command> {
    let mut cmd = process::Command::cargo_bin(BIN_NAME)?;
    cmd.env("GRIT_TELEMETRY_DISABLED", "true");
    Ok(cmd)
}

// This is used in tests
#[allow(dead_code)]
pub fn get_fixtures_root() -> Result<std::path::PathBuf> {
    let mut fixtures_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fixtures_root.push("fixtures");
    Ok(fixtures_root)
}

/**
 * This function is used in tests to get a copy of a particular fixture in a tempdir.
 * Note: tempdir is automatically deleted after the test is run.
 * If you want to keep the tempdir for debugging, you can use `tempdir.into_path()`
 * Ex. `println!("dir: {:?}", temp_dir.into_path());`
 */
#[allow(dead_code)]
pub fn get_fixture(
    subdirectory: &str,
    with_init: bool,
) -> Result<(tempfile::TempDir, std::path::PathBuf)> {
    // Create a temporary directory
    let temp_dir = tempdir()?;

    // Get the path of the temporary directory
    let temp_fixtures_root = temp_dir.path().to_path_buf();

    // Construct the source path for the subdirectory inside fixtures
    let mut fixtures_root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    fixtures_root.push("fixtures");
    fixtures_root.push(subdirectory);

    // Copy the contents of the subdirectory to the temporary directory
    let mut options = fs_extra::dir::CopyOptions::new();
    options.copy_inside = true;
    fs_extra::dir::copy(&fixtures_root, &temp_fixtures_root, &options)?;

    // Run init command if requested
    if with_init {
        run_init_cmd(&temp_fixtures_root.join(subdirectory));
    }

    Ok((temp_dir, temp_fixtures_root.join(subdirectory)))
}

// Used in tests
#[allow(dead_code)]
pub fn run_init_cmd(cwd: &dyn AsRef<path::Path>) -> tempfile::TempDir {
    let mut init_cmd = match Command::cargo_bin(BIN_NAME) {
        Ok(cmd) => cmd,
        Err(err) => {
            panic!("Failed to find binary {}: {}", BIN_NAME, err);
        }
    };
    let grit_global_dir = tempfile::tempdir().unwrap();

    init_cmd.env("GRIT_TELEMETRY_DISABLED", "true");
    init_cmd.env(GRIT_GLOBAL_DIR_ENV, grit_global_dir.path());
    init_cmd.current_dir(cwd);
    init_cmd.arg("init");
    let output = match init_cmd.output() {
        Ok(output) => output,
        Err(err) => {
            panic!("Failed to execute command: {}", err);
        }
    };

    assert!(
        output.status.success(),
        "Init command didn't finish successfully"
    );

    grit_global_dir
}
