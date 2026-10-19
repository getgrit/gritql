use std::{env, path, process, sync::OnceLock};

use anyhow::Result;
use assert_cmd::cargo::CommandCargoExt;
use assert_cmd::Command;

use marzano_gritmodule::config::GRIT_GLOBAL_DIR_ENV;
use tempfile::tempdir;

pub const BIN_NAME: &str = "grit";

pub struct TestGlobalDir {
    _temp_dir: tempfile::TempDir,
    grit_dir: path::PathBuf,
}

impl TestGlobalDir {
    pub fn path(&self) -> &path::Path {
        &self.grit_dir
    }
}

static TEST_GLOBAL_DIR: OnceLock<std::result::Result<TestGlobalDir, String>> = OnceLock::new();

fn test_global_dir() -> Result<&'static TestGlobalDir> {
    let result = TEST_GLOBAL_DIR.get_or_init(|| {
        let temp_dir = tempfile::tempdir().map_err(|err| err.to_string())?;
        let grit_dir = temp_dir.path().join(".grit");
        let mut init_cmd = Command::cargo_bin(BIN_NAME).map_err(|err| err.to_string())?;
        init_cmd.env("GRIT_TELEMETRY_DISABLED", "true");
        init_cmd.env(GRIT_GLOBAL_DIR_ENV, &grit_dir);
        init_cmd.args(["init", "--global"]);

        let output = init_cmd.output().map_err(|err| err.to_string())?;
        if output.status.success() {
            Ok(TestGlobalDir {
                _temp_dir: temp_dir,
                grit_dir,
            })
        } else {
            Err(format!(
                "Failed to initialize the test Grit global directory:\nstdout:\n{}\nstderr:\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            ))
        }
    });

    result.as_ref().map_err(|err| anyhow::anyhow!("{err}"))
}

#[allow(dead_code)]
pub const INSTA_FILTERS: &[(&str, &str)] = &[(
    r"\b[[:xdigit:]]{8}-[[:xdigit:]]{4}-[[:xdigit:]]{4}-[[:xdigit:]]{4}-[[:xdigit:]]{12}\b",
    "[UUID]",
)];

#[allow(dead_code)]
pub fn get_test_cmd() -> Result<Command> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.env("GRIT_TELEMETRY_DISABLED", "true");
    cmd.env(GRIT_GLOBAL_DIR_ENV, test_global_dir()?.path());
    Ok(cmd)
}

#[allow(dead_code)]
pub fn get_test_process_cmd() -> Result<process::Command> {
    let mut cmd = process::Command::cargo_bin(BIN_NAME)?;
    cmd.env("GRIT_TELEMETRY_DISABLED", "true");
    cmd.env(GRIT_GLOBAL_DIR_ENV, test_global_dir()?.path());
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
pub fn run_init_cmd(cwd: &dyn AsRef<path::Path>) -> &'static TestGlobalDir {
    let cwd = cwd.as_ref();
    let grit_global_dir = test_global_dir().unwrap();

    let has_local_config = cwd.ancestors().any(|dir| dir.join(".grit").exists());
    let has_git_dir = cwd.ancestors().any(|dir| dir.join(".git").exists());
    if !has_local_config && !has_git_dir {
        // A non-repository fixture can only initialize the global config, which is ready already.
        return grit_global_dir;
    }

    let mut init_cmd = match Command::cargo_bin(BIN_NAME) {
        Ok(cmd) => cmd,
        Err(err) => {
            panic!("Failed to find binary {}: {}", BIN_NAME, err);
        }
    };
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
        "Init command didn't finish successfully:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );

    grit_global_dir
}
