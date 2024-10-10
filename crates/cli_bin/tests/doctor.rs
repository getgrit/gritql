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

#[test]
fn test_runs_from_read_only_dir() -> Result<()> {
    let bin_path = cargo_bin(env!("CARGO_PKG_NAME"));
    let temp_dir = tempfile::tempdir()?;

    let install_dir = temp_dir.path().join("layer1").join("layer2").join("layer3");
    fs::create_dir_all(&install_dir)?;

    let dest_path = install_dir.join("install");

    fs::copy(bin_path, &dest_path)?;

    // Make the temp dir read-only
    let mut perms = fs::metadata(&temp_dir)?.permissions();
    perms.set_readonly(true);
    fs::set_permissions(&temp_dir, perms)?;

    let mut cmd = Command::new(dest_path);
    cmd.arg("doctor");

    let stderr = cmd.output()?.stderr;
    let stderr_str = String::from_utf8_lossy(&stderr);
    println!("stderr: {}", stderr_str);

    let stdout = cmd.output()?.stdout;
    let stdout_str = String::from_utf8_lossy(&stdout);
    println!("stdout: {}", stdout_str);

    panic!("test failed");

    Ok(())
}
