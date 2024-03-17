use anyhow::Result;
use ntest::timeout;
use std::{fs, io::Read};

use crate::common::{get_fixture, get_test_cmd};
mod common;

#[test]
#[ignore = "requires a running Temporal server"]
#[timeout(15000)]
fn basic_migration() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("es6", false)?;
    let mut cmd = get_test_cmd()?;
    let output = cmd
        .current_dir(dir.clone())
        .arg("apply")
        .arg("remove_console_log")
        .output()?;

    assert!(output.status.success());

    // Read the content of dir/export.js
    let export_js_path = dir.join("export.js");
    let mut file_content = String::new();
    fs::File::open(export_js_path)?.read_to_string(&mut file_content)?;

    // Assert that the file contains logger.log and does not contain console.log
    assert!(
        file_content.contains("logger.log"),
        "export.js does not contain logger.log"
    );
    assert!(
        !file_content.contains("console.log"),
        "export.js contains console.log, which should have been removed"
    );

    Ok(())
}
