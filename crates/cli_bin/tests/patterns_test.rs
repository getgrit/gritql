use std::{
    env, fs,
    io::{BufRead, BufReader},
    process::Stdio,
    sync::mpsc,
    thread,
    time::Duration,
};

use anyhow::Result;
use insta::assert_snapshot;

use crate::common::{get_fixture, get_test_cmd, get_test_process_cmd};

mod common;

#[test]
fn updates_nothing_valid_patterns() -> Result<()> {
    let (_temp_dir, _) = get_fixture("patterns_list", true)?;
    let before_files = fs_err::read_dir(_temp_dir.path().join("patterns_list"))?
        .filter_map(|res| {
            let path = res.ok()?.path();
            fs_err::read_to_string(path).ok()
        })
        .collect::<Vec<_>>();

    let mut cmd = get_test_cmd()?;
    cmd.arg("patterns")
        .arg("test")
        .arg("--update")
        .current_dir(_temp_dir.path().join("patterns_list"));

    println!("{:?}", cmd.output());

    let after_files = fs_err::read_dir(_temp_dir.path().join("patterns_list"))?
        .filter_map(|res| {
            let path = res.ok()?.path();
            fs_err::read_to_string(path).ok()
        })
        .collect::<Vec<_>>();

    assert_eq!(before_files, after_files, "File content has changed");

    Ok(())
}

#[test]
fn updates_invalid_pattern() -> Result<()> {
    let (_temp_dir, _) = get_fixture("patterns_list", true)?;
    let mut cmd = get_test_cmd()?;

    cmd.arg("patterns")
        .arg("test")
        .arg("--update")
        .current_dir(_temp_dir.path().join("patterns_list"));

    println!("{:?}", cmd.output());

    let after = fs_err::read_to_string(
        _temp_dir
            .path()
            .join("patterns_list/.grit/patterns/broken_pattern.md"),
    )?;
    assert_snapshot!(after);

    Ok(())
}

#[test]
fn test_excludes_patterns() -> Result<()> {
    let (_temp_dir, _) = get_fixture("patterns_list", true)?;
    let mut cmd = get_test_cmd()?;

    cmd.arg("patterns")
        .arg("test")
        .arg("--exclude")
        .arg("exclude1")
        .arg("--exclude")
        .arg("exclude2")
        .current_dir(_temp_dir.path().join("patterns_list"));

    println!("{:?}", cmd.output());
    let stdout = String::from_utf8(cmd.output()?.stdout)?;
    println!("{}", stdout);

    assert_snapshot!(stdout);

    assert!(!stdout.contains("multiple_broken"));

    Ok(())
}

#[test]
fn does_not_exclude_substring_name_match() -> Result<()> {
    let (_temp_dir, _) = get_fixture("patterns_list", true)?;
    let mut cmd = get_test_cmd()?;

    cmd.arg("patterns")
        .arg("test")
        .arg("--exclude")
        .arg("pattern")
        .current_dir(_temp_dir.path().join("patterns_list"));

    let stdout = String::from_utf8(cmd.output()?.stdout)?;

    assert!(stdout.contains("multiple_broken_patterns"));
    assert!(stdout.contains("broken_pattern"));

    Ok(())
}

#[test]
fn updates_multiple_invalid_patterns() -> Result<()> {
    let (_temp_dir, _) = get_fixture("patterns_list", true)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("patterns")
        .arg("test")
        .arg("--update")
        .current_dir(_temp_dir.path().join("patterns_list"));

    let stdout = String::from_utf8(cmd.output()?.stdout)?;
    println!("stdout: {}", stdout);
    let after = fs_err::read_to_string(
        _temp_dir
            .path()
            .join("patterns_list/.grit/patterns/multiple_broken_patterns.md"),
    )?;

    assert_snapshot!(after);

    Ok(())
}

#[test]
fn formats_hcl_files() -> Result<()> {
    let (_temp_dir, _) = get_fixture("hcl", true)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("patterns")
        .arg("test")
        .current_dir(_temp_dir.path().join("hcl"));

    println!("{:?}", cmd.output());

    let output = cmd.output()?;
    assert!(output.status.success());

    Ok(())
}

#[test]
fn test_multifile_passes() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("test_multifile", true)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("patterns").arg("test").current_dir(fixture_dir);

    let output = cmd.output()?;
    assert!(output.status.success());

    Ok(())
}

#[test]
fn fails_recursion() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("markdown_args", true)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("patterns").arg("test").current_dir(fixture_dir);

    let output = cmd.output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    println!("stdout: {}", stdout);
    println!("stderr: {}", stderr);
    assert_snapshot!(stderr);

    Ok(())
}

#[test]
fn test_multifile_fails_if_unchanged_file_has_incorrect_expected() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("test_multifile_fail", true)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("patterns").arg("test").current_dir(fixture_dir);

    let output = cmd.output()?;
    assert_eq!(output.status.code(), Some(1));

    Ok(())
}

#[test]
fn test_multifile_handles_unspecified_expected_files() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("test_multifile_missing", true)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("patterns").arg("test").current_dir(fixture_dir);

    let output = cmd.output()?;
    assert!(output.status.success());

    Ok(())
}

#[test]
fn test_multifile_fails_on_misnamed_expected_files() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("test_multifile_misnamed", true)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("patterns").arg("test").current_dir(fixture_dir);

    let output = cmd.output()?;
    assert_eq!(output.status.code(), Some(1));

    Ok(())
}

#[test]
fn test_multifile_fails_on_extra_expected_files() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("test_multifile_extra", true)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("patterns").arg("test").current_dir(fixture_dir);

    let output = cmd.output()?;
    assert_eq!(output.status.code(), Some(1));

    Ok(())
}

#[test]
fn fails_on_error_analysis_logs() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("bad_test", false)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("patterns").arg("test").current_dir(fixture_dir);

    let output = cmd.output()?;

    assert_eq!(output.status.code(), Some(1));
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("Error parsing source code"));

    Ok(())
}

#[test]
fn passes_on_info_analysis_logs() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("bad_output", false)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("patterns").arg("test").current_dir(fixture_dir);

    let output = cmd.output()?;

    assert!(output.status.success());
    Ok(())
}

#[test]
fn tests_patterns_with_foreign_function_call_from_dot_grit_lib() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("foreign_test", false)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("patterns").arg("test").current_dir(fixture_dir);

    let output = cmd.output()?;

    let stderr = String::from_utf8(output.stderr)?;
    println!("{}", stderr);

    assert!(output.status.success());
    Ok(())
}

#[test]
fn checks_non_matching_yaml_sample() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("yaml_unmatched", false)?;

    let mut test = get_test_cmd()?;
    test.arg("patterns").arg("test").current_dir(dir);

    let output = test.output()?;

    assert!(output.status.success());

    Ok(())
}

#[test]
fn tests_match_only_with_file_name() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("match_filename", false)?;

    let mut test = get_test_cmd()?;
    test.arg("patterns").arg("test").current_dir(dir);

    let output = test.output()?;

    assert!(output.status.success());

    Ok(())
}

#[test]
fn tests_python_pattern_with_file_name() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("python_filename", false)?;

    let mut test = get_test_cmd()?;
    test.arg("patterns").arg("test").current_dir(dir);

    let output = test.output()?;

    assert!(output.status.success());

    Ok(())
}

#[test]
fn test_watch_mode_changed() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let (temp_dir, temp_fixture_path) = get_fixture("patterns_test", false)?;
    let test_yaml_path = temp_fixture_path.join(".grit/grit.yaml");

    println!("temp_fixture_path: {:?}", test_yaml_path);
    println!("temp_dir_path: {:?}", temp_dir.into_path());

    let _cmd_handle = thread::spawn(move || {
        let mut cmd = get_test_process_cmd()
            .unwrap()
            .args(["patterns", "test", "--watch"])
            .current_dir(&temp_fixture_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start command");

        let stdout = BufReader::new(cmd.stdout.take().unwrap());
        let stderr = BufReader::new(cmd.stderr.take().unwrap());
        for line in stdout.lines().chain(stderr.lines()).flatten() {
            println!("LINE: {:?}", line);
            tx.send(line).unwrap();
        }
    });
    thread::sleep(Duration::from_secs(3));

    // Update it
    let old_content = fs::read_to_string(&test_yaml_path).expect("Unable to read the file");
    let new_content = old_content.replace("console.log(bar)", "console.log(bad)");
    fs::write(&test_yaml_path, new_content)?;
    thread::sleep(Duration::from_secs(3));

    // Then reset it
    fs::write(&test_yaml_path, old_content)?;
    thread::sleep(Duration::from_secs(3));

    let mut output = Vec::new();
    while let Ok(line) = rx.try_recv() {
        output.push(line);
    }
    let expected_output = vec![
        "Found 2 testable patterns.",
        "Watching for changes",
        "✓ All 7 samples passed.",
        ".grit/grit.yaml was modified",
        "1 out of 1 samples failed.",
    ];
    for expected_line in expected_output {
        assert!(
            output.iter().any(|line| line.contains(expected_line)),
            "Expected output not found: {}",
            expected_line
        );
    }

    assert!(
        output
            .iter()
            .filter(|line| line.contains("samples passed"))
            .count()
            == 2
    );

    Ok(())
}

#[test]
fn test_watch_mode_deleted() -> Result<()> {
    let (tx, rx) = mpsc::channel();

    let (temp_dir, temp_fixture_path) = get_fixture("patterns_test", false)?;
    let test_yaml_path = temp_fixture_path.join(".grit/grit.yaml");

    println!("temp_fixture_path: {:?}", test_yaml_path);
    println!("temp_dir_path: {:?}", temp_dir.into_path());

    let _cmd_handle = thread::spawn(move || {
        let mut cmd = get_test_process_cmd()
            .unwrap()
            .args(["patterns", "test", "--watch"])
            .current_dir(&temp_fixture_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start command");

        let stdout = BufReader::new(cmd.stdout.take().unwrap());
        let stderr = BufReader::new(cmd.stderr.take().unwrap());
        for line in stdout.lines().chain(stderr.lines()).flatten() {
            println!("LINE: {:?}", line);
            tx.send(line).unwrap();
        }
    });
    thread::sleep(Duration::from_secs(3));

    // Update it to an empty config
    fs::write(&test_yaml_path, "patterns: []")?;
    thread::sleep(Duration::from_secs(3));

    let mut output = Vec::new();
    while let Ok(line) = rx.try_recv() {
        output.push(line);
    }
    let expected_output = vec![
        "Found 2 testable patterns.",
        "Watching for changes",
        "✓ All 7 samples passed.",
        ".grit/grit.yaml was modified",
        "no testable pattern changes were found",
    ];
    for expected_line in expected_output {
        assert!(
            output.iter().any(|line| line.contains(expected_line)),
            "Expected output not found: {}",
            expected_line
        );
    }

    Ok(())
}
