use std::path::PathBuf;
use std::time::Instant;
use std::{fs, path};

use anyhow::bail;
use anyhow::{anyhow, Result};
use assert_cmd::Command;
use common::get_test_cmd;
use insta::assert_snapshot;
use marzano_gritmodule::config::{
    CONFIG_FILE_NAMES, GRIT_GLOBAL_DIR_ENV, GRIT_MODULE_DIR, REPO_CONFIG_DIR_NAME,
    REPO_CONFIG_PATTERNS_DIR,
};
#[cfg(feature = "ai_builtins")]
use ntest::timeout;

use predicates::prelude::*;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use regex::Regex;
use std::sync::Once;

use crate::common::{get_fixture, get_fixtures_root, run_init_cmd};

mod common;

static INIT: Once = Once::new();

fn run_init(cwd: &dyn AsRef<path::Path>) -> Result<()> {
    INIT.call_once(|| {
        run_init_cmd(cwd);
    });
    Ok(())
}

#[test]
fn pattern_file_does_not_exist() -> Result<()> {
    let mut cmd = get_test_cmd()?;

    cmd.arg("apply")
        .arg("--force")
        .arg("non-existent-file.grit");
    cmd.assert().failure().stderr(predicate::str::contains(
        "Could not read pattern file: non-existent-file",
    ));

    Ok(())
}

#[test]
fn malformed_stdin_input() -> Result<()> {
    let mut cmd: Command = get_test_cmd()?;

    let input = r#"{ "pattern_body" : "empty paths" }"#;

    cmd.arg("plumbing").arg("apply");
    cmd.write_stdin(input);
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to parse input JSON"));

    Ok(())
}

#[test]
fn empty_paths_array() -> Result<()> {
    let mut cmd = get_test_cmd()?;

    let input = r#"{ "pattern_body" : "empty paths", "paths" : [] }"#.to_string();

    cmd.arg("plumbing").arg("apply").arg("--jsonl");
    cmd.write_stdin(input);

    let output = cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    let line = stdout.lines().next().ok_or_else(|| anyhow!("No output"))?;
    let v: serde_json::Value = serde_json::from_str(line)?;

    let all_done_found = v
        .get("__typename")
        .map_or(false, |x| x.as_str() == Some("AllDone"))
        && v.get("reason")
            .map_or(false, |x| x.as_str() == Some("noInputPaths"));

    assert!(
        all_done_found,
        "Did not find JSON object with __typename AllDone"
    );

    Ok(())
}

#[test]
fn empty_or_returns_error() -> Result<()> {
    let mut cmd = get_test_cmd()?;

    let fixtures_root = get_fixtures_root()?;
    let fixture_path: path::PathBuf = fixtures_root.join("simple_test").join("sample.js");
    let input = format!(
        r#"{{ "pattern_body" : "or {{}}", "paths" : [ {:?} ] }}"#,
        fixture_path.to_str().unwrap()
    );

    cmd.arg("plumbing").arg("apply").arg("--jsonl");
    cmd.write_stdin(input);

    let output = cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully\n{:?}",
        output
    );

    let stdout = String::from_utf8(output.stdout)?;
    let line = stdout.lines().next().ok_or_else(|| anyhow!("No output"))?;
    let v: serde_json::Value = serde_json::from_str(line)?;

    let analysis_log_found = v
        .get("__typename")
        .map_or(false, |x| x.as_str() == Some("AnalysisLog"));

    assert!(analysis_log_found, "Did not find AnalysisLog");

    Ok(())
}

#[test]
fn error_returns_gritfile_path() -> Result<()> {
    let mut cmd = get_test_cmd()?;

    let fixtures_root = get_fixtures_root()?;
    let fixture_path = fixtures_root.join("bad_libs").join("sample.js");

    let input = format!(
        r#"{{ "pattern_body" : "no_console_log()", "paths" : [ {:?} ] }}"#,
        fixture_path.to_str().unwrap()
    );
    cmd.arg("plumbing").arg("apply").arg("--jsonl");
    cmd.write_stdin(input);

    let output = cmd.output()?;

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    let line = stdout.lines().next().ok_or_else(|| anyhow!("No output"))?;
    let v: serde_json::Value = serde_json::from_str(line)?;

    let analysis_log_found = v
        .get("__typename")
        .map_or(false, |x| x.as_str() == Some("AnalysisLog"))
        && v.get("message")
            .map_or(false, |x| x.as_str().unwrap().contains("bad.grit"));

    assert!(
        analysis_log_found,
        "Did not find AnalysisLog with message containing file path 'bad.grit'"
    );

    Ok(())
}

#[test]
fn serializes_correctly() -> Result<()> {
    let mut cmd = get_test_cmd()?;

    let fixtures_root = get_fixtures_root()?;

    cmd.arg("apply")
        .arg("--jsonl")
        .arg("--force")
        .arg(
            fixtures_root
                .join("simple_patterns")
                .join("console_log.grit"),
        )
        .arg(fixtures_root);
    let output = cmd.output()?;

    // Assert that the command executed successfully
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    // Convert stdout to string
    let stdout = String::from_utf8(output.stdout)?;
    let lines: Vec<&str> = stdout.split('\n').collect();

    // Variable to track if the AllDone object was found
    let mut all_done_found = false;

    // Iterate over each line, parsing it as JSON and verifying it contains valid JSON
    for line in lines {
        if line.is_empty() {
            continue;
        }
        // Parse the line as a JSON Value
        let v: serde_json::Value = serde_json::from_str(line)?;
        // Check if the "__typename" field of the JSON object is "AllDone"
        if v.get("__typename")
            .map_or(false, |x| x.as_str() == Some("AllDone"))
        {
            all_done_found = true;
        }
    }

    assert!(
        all_done_found,
        "Did not find JSON object with __typename AllDone"
    );

    Ok(())
}

#[test]
fn run_stdlib_pattern_without_grit_config() -> Result<()> {
    let tempdir = tempfile::tempdir()?;
    let grit_global_dir = tempfile::tempdir()?;

    // copy fixtures/require.js to the tempdir
    let fixtures_root = get_fixtures_root()?;
    let require_js = fixtures_root.join("test_es6imports.js");
    let require_js_dest = tempdir.path().join("test_es6imports.js");
    std::fs::copy(require_js, &require_js_dest)?;

    // from the tempdir as cwd, run init
    run_init(&tempdir.path())?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(tempdir.path());
    apply_cmd
        .arg("apply")
        .arg("es6_imports")
        .arg("test_es6imports.js")
        .env(GRIT_GLOBAL_DIR_ENV, grit_global_dir.path());
    let output = apply_cmd.output()?;

    // Assert that the command executed successfully
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    // Read back the require.js file
    let content: String = std::fs::read_to_string(&require_js_dest)?;

    // assert that it matches snapshot
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn run_pattern_with_sequential() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("test_patterns", true)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());

    apply_cmd
        .arg("apply")
        .arg("react_to_hooks")
        .arg("react_to_hooks/input/lifecycle.tsx");

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
    let content: String = std::fs::read_to_string(dir.join("react_to_hooks/input/lifecycle.tsx"))?;

    // assert that it matches snapshot
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn run_pattern_file_referencing_stdlib() -> Result<()> {
    let tempdir = tempfile::tempdir()?;
    let grit_global_dir = tempfile::tempdir()?;

    // copy fixtures/stdlib/no_console_log.grit and fixtures/stdlib/happy.js to tempdir
    let fixtures_root = get_fixtures_root()?;
    let pattern_grit = fixtures_root.join("stdlib").join("no_console_log.grit");
    let pattern_dest = tempdir.path().join("no_console_log.grit");
    std::fs::copy(pattern_grit, pattern_dest)?;
    let extensions = ["js", "cjs", "mjs", "cts", "mts"];
    let mut destinations = vec![];
    for extension in extensions {
        let input = fixtures_root
            .join("stdlib")
            .join(format!("simple.{}", extension));
        let input_dest = tempdir.path().join(format!("simple.{}", extension));
        std::fs::copy(input, &input_dest)?;
        destinations.push(input_dest);
    }

    // from the tempdir as cwd, run init
    run_init(&tempdir.path())?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(tempdir.path());
    apply_cmd
        .arg("apply")
        .arg("no_console_log.grit")
        .arg("simple.js")
        .arg("simple.cjs")
        .arg("simple.mjs")
        .arg("simple.cts")
        .arg("simple.mts")
        .env(GRIT_GLOBAL_DIR_ENV, grit_global_dir.path());

    let output = apply_cmd.output()?;

    // Assert that the command executed successfully
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );
    for destination in destinations {
        let content = std::fs::read_to_string(&destination)?;
        assert_eq!(content, "\n".to_owned());
    }

    Ok(())
}

#[test]
fn run_pattern_file_referencing_stdlib_function() -> Result<()> {
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("stdlib", false)?;

    let modules_dir = run_init_cmd(&dir.as_path());

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());

    apply_cmd
        .arg("apply")
        .arg("todo_function.grit")
        .arg("simple.js")
        .env(GRIT_GLOBAL_DIR_ENV, modules_dir.path());

    let output = apply_cmd.output()?;

    // Print stderr
    println!("stderr: {:?}", String::from_utf8(output.stderr)?);
    println!("stdout: {:?}", String::from_utf8(output.stdout)?);

    // Assert that the command executed successfully
    assert!(output.status.success(), "Command didn't finish successfull");

    // Keep the dir
    println!("dir: {:?}", _temp_dir.into_path());

    // Check contents
    let target_file = dir.join("simple.js");
    let content: String = std::fs::read_to_string(target_file)?;
    assert_eq!(
        content,
        "// TODO: Fix this\n// console.log('sanity');\n".to_owned()
    );

    Ok(())
}

#[test]
fn run_pattern_file_referencing_python_stdlib() -> Result<()> {
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("stdlib-python", true)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());
    apply_cmd
        .arg("apply")
        .arg("print_to_log.grit")
        .arg("log.py");

    let output = apply_cmd.output()?;

    // Assert that the command executed successfully
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let input_dest = dir.as_path().join("log.py");
    let content: String = std::fs::read_to_string(input_dest)?;
    assert_eq!(content, "log(\"hello world!\")\n".to_owned());

    Ok(())
}

#[test]
fn run_python_stdlib_pattern_name() -> Result<()> {
    let tempdir = tempfile::tempdir()?;
    let grit_global_dir = tempfile::tempdir()?;

    let fixtures_root = get_fixtures_root()?;
    let input = fixtures_root.join("stdlib-python").join("log.py");
    let input_dest = tempdir.path().join("log.py");
    std::fs::copy(input, &input_dest)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(tempdir.path());
    apply_cmd
        .arg("apply")
        .arg("print_to_log")
        .arg("log.py")
        .env(GRIT_GLOBAL_DIR_ENV, grit_global_dir.path());

    let output = apply_cmd.output()?;

    // Assert that the command executed successfully
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let content: String = std::fs::read_to_string(&input_dest)?;
    assert_eq!(content, "log(\"hello world!\")\n".to_owned());

    Ok(())
}

#[test]
fn run_named_pattern_referencing_python_stdlib() -> Result<()> {
    let tempdir = tempfile::tempdir()?;
    let grit_global_dir = tempfile::tempdir()?;

    let fixtures_root = get_fixtures_root()?;
    let pattern_grit = fixtures_root.join("stdlib-python").join("fun_logger.md");
    let pattern_dest = tempdir
        .path()
        .join(REPO_CONFIG_DIR_NAME)
        .join(REPO_CONFIG_PATTERNS_DIR)
        .join("fun_logger.md");
    fs::create_dir_all(pattern_dest.parent().unwrap())?;
    std::fs::copy(pattern_grit, pattern_dest)?;
    let input = fixtures_root.join("stdlib-python").join("log.py");
    let input_dest = tempdir.path().join("log.py");
    std::fs::copy(input, &input_dest)?;

    run_init(&tempdir.path())?;

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(tempdir.path());
    apply_cmd
        .arg("apply")
        .arg("fun_logger")
        .arg("log.py")
        .env(GRIT_GLOBAL_DIR_ENV, grit_global_dir.path());

    let output = apply_cmd.output()?;

    // Assert that the command executed successfully
    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    let content: String = std::fs::read_to_string(&input_dest)?;
    assert_eq!(content, "log(\"hello world!\")\n".to_owned());

    Ok(())
}

#[test]
fn grit_dir_with_only_empty_gritmodules() -> Result<()> {
    let tempdir = tempfile::tempdir()?;

    // copy fixtures/require.js to the tempdir
    let fixtures_root = get_fixtures_root()?;
    let require_js = fixtures_root.join("short-story.ts");
    let require_js_dest = tempdir.path().join("short-story.ts");
    std::fs::copy(require_js, &require_js_dest)?;

    // make an empty .grit/.gritmodules in the tempdir
    let grit_modules_dir = tempdir
        .path()
        .join(REPO_CONFIG_DIR_NAME)
        .join(GRIT_MODULE_DIR);
    fs::create_dir_all(grit_modules_dir)?;

    // from the tempdir as cwd, run init
    run_init_cmd(&tempdir.path());

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(tempdir.path());
    apply_cmd
        .arg("apply")
        .arg("openai_v4")
        .arg("short-story.ts");
    let output = apply_cmd.output()?;

    // Assert that the command executed successfully
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    println!("stderr: {:?}", String::from_utf8(output.stderr)?);
    println!("stdout: {:?}", String::from_utf8(output.stdout)?);

    // Read back the require.js file
    let content: String = std::fs::read_to_string(&require_js_dest)?;

    // assert that it matches snapshot
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn grit_dir_with_no_gritmodules_and_empty_config() -> Result<()> {
    let tempdir = tempfile::tempdir()?;

    // copy fixtures/require.js to the tempdir
    let fixtures_root = get_fixtures_root()?;
    let require_js = fixtures_root.join("short-story.ts");
    let require_js_dest = tempdir.path().join("short-story.ts");
    std::fs::copy(require_js, &require_js_dest)?;

    let empty_config = r#"version: 0.0.1
patterns: []"#;

    let grit_dir = tempdir.path().join(REPO_CONFIG_DIR_NAME);
    let grit_yml = grit_dir.join(CONFIG_FILE_NAMES[0]);
    fs::create_dir_all(&grit_dir)?;
    fs::write(grit_yml, empty_config)?;

    // from the tempdir as cwd, run init
    run_init(&tempdir.path())?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(tempdir.path());
    apply_cmd
        .arg("apply")
        .arg("openai_v4")
        .arg("short-story.ts");
    let output = apply_cmd.output()?;

    // Assert that the command executed successfully
    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    println!("stderr: {:?}", String::from_utf8(output.stderr)?);
    println!("stdout: {:?}", String::from_utf8(output.stdout)?);

    // Read back the require.js file
    let content: String = std::fs::read_to_string(&require_js_dest)?;

    // assert that it matches snapshot
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn basic_terraform_apply() -> Result<()> {
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("terraform", true)?;

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());

    apply_cmd.arg("apply").arg("--force").arg("pattern.grit");
    let output = apply_cmd.output()?;

    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    assert_snapshot!(String::from_utf8(output.stdout)?);

    Ok(())
}

#[test]
fn multifile_terraform_apply() -> Result<()> {
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("terraform", true)?;

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());

    apply_cmd.arg("apply").arg("--force").arg("files.grit");
    let output = apply_cmd.output()?;

    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    assert_snapshot!(String::from_utf8(output.stdout)?);

    Ok(())
}

#[test]
fn terraform_complex() -> Result<()> {
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("terraform_complex", true)?;

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());

    apply_cmd.arg("apply").arg("--force").arg("pattern.grit");
    let output = apply_cmd.output()?;

    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    assert_snapshot!(String::from_utf8(output.stdout)?);

    Ok(())
}

#[test]
fn random_int() -> Result<()> {
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("random", true)?;

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());

    apply_cmd.arg("apply").arg("--force").arg("pattern.grit");
    let output = apply_cmd.output()?;

    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    let file = dir.join("input.ts");
    let content = std::fs::read_to_string(file)?;

    println!("content: {:?}", content);

    let ones = content.matches('1').count();
    let zeros = content.matches('0').count();

    // We should have at least 2 of each
    assert!(zeros >= 2, "Not enough zeros: {}", zeros);
    assert!(ones >= 2, "Not enough ones: {}", ones);

    // We use a fixed seed, so this is safe
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn shuffle_list() -> Result<()> {
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("random", true)?;

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());

    apply_cmd
        .arg("apply")
        .arg("--force")
        .arg("shuffle_list.grit");
    let output = apply_cmd.output()?;

    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    let file = dir.join("input.ts");
    let content = std::fs::read_to_string(file)?;
    println!("content: {:?}", content);

    // Split it by line
    let lines: Vec<&str> = content.split('\n').collect();

    // The first two lines should not be the same
    assert_ne!(lines[0], lines[1]);

    // We should have a zebra
    assert!(content.contains("zebra"));

    Ok(())
}

#[test]
fn test_absolute_path() -> Result<()> {
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("resolve_paths", true)?;

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());

    apply_cmd
        .arg("apply")
        .arg("pattern.grit")
        .arg("dir2/nested/normal.js")
        .arg("dir2/unique.js")
        .arg("dir2/nice.js")
        .arg("dir2/nested/deep.js")
        .arg("target_dir/other.js");

    let output = apply_cmd.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    println!("stdout: {:?}", stdout);
    println!("stderr: {:?}", stderr);

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let paths = vec![
        "dir2/nice.js",
        "dir2/nested/deep.js",
        "dir2/nested/normal.js",
        "target_dir/other.js",
    ];

    // Assert they all have the same content as each other
    let mut content = None;
    for path in paths {
        let file = dir.join(path);
        let file_content = std::fs::read_to_string(file)?;
        if let Some(ref content) = content {
            assert_eq!(content, &file_content);
        } else {
            content = Some(file_content);
        }
    }

    // Verify it includes foo.js
    assert!(content.unwrap().contains("foo.js"));

    // Now we read unique.js
    let file = dir.join("dir2/unique.js");
    let content = std::fs::read_to_string(file)?;

    // Verify it contains dir2/unique.js
    assert!(content.contains("dir2/unique.js"));

    // Verify it does not include dir2 twice
    assert_eq!(content.matches("dir2").count(), 1);

    Ok(())
}

#[test]
fn shuffle_binding() -> Result<()> {
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("random", true)?;

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());

    apply_cmd
        .arg("apply")
        .arg("--force")
        .arg("shuffle_binding.grit");
    let output = apply_cmd.output()?;

    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    let file = dir.join("input.ts");
    let content = std::fs::read_to_string(file)?;
    println!("content: {:?}", content);

    // Split it by line
    let lines: Vec<&str> = content.split('\n').collect();

    // The first two lines should not be the same
    assert_ne!(lines[0], lines[1]);

    // We should have a zebra
    assert!(content.contains('a'));

    Ok(())
}

#[test]
fn basic_python_apply() -> Result<()> {
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("simple_python", false)?;

    // from the tempdir as cwd, run init
    run_init(&dir.as_path())?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    // apply_cmd.current_dir(basic_path);
    apply_cmd.arg("apply").arg("--force").arg("pattern.grit");
    let output = apply_cmd.output()?;

    // Assert that the command executed successfully
    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    // Read back the require.js file
    let target_file = dir.join("main.py");
    let content: String = std::fs::read_to_string(target_file)?;

    // assert that it matches snapshot
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn basic_js_in_vue_apply() -> Result<()> {
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("js_in_vue", false)?;

    // from the tempdir as cwd, run init
    run_init(&dir.as_path())?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    // apply_cmd.current_dir(basic_path);
    apply_cmd.arg("apply").arg("--force").arg("pattern.grit");
    let output = apply_cmd.output()?;

    // Assert that the command executed successfully
    println!("OUTPUT: {:#?}", output);
    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    // Read back the require.js file
    let target_file = dir.join("simple.vue");
    let content: String = std::fs::read_to_string(target_file)?;

    // assert that it matches snapshot
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn basic_css_in_vue_apply() -> Result<()> {
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("css_in_vue", false)?;

    // from the tempdir as cwd, run init
    run_init(&dir.as_path())?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    // apply_cmd.current_dir(basic_path);
    apply_cmd.arg("apply").arg("--force").arg("pattern.grit");
    let output = apply_cmd.output()?;

    // Assert that the command executed successfully
    println!("OUTPUT: {:#?}", output);
    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    // Read back the require.js file
    let target_file = dir.join("simple.vue");
    let content: String = std::fs::read_to_string(target_file)?;

    // assert that it matches snapshot
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn invalid_md_file_parse_errors() -> Result<()> {
    let tempdir = tempfile::tempdir()?;

    // copy fixtures/require.js to the tempdir
    let fixtures_root = get_fixtures_root()?;
    let require_js = fixtures_root.join("short-story.ts");
    let require_js_dest = tempdir.path().join("short-story.ts");
    std::fs::copy(require_js, require_js_dest)?;

    // make an empty .grit/.gritmodules in the tempdir
    let grit_modules_dir = tempdir
        .path()
        .join(REPO_CONFIG_DIR_NAME)
        .join(GRIT_MODULE_DIR);
    fs::create_dir_all(&grit_modules_dir)?;

    // from the tempdir as cwd, run init
    run_init_cmd(&tempdir.path());

    let bad_pattern = fixtures_root.join("bad_pattern.md");
    // rm -rf openai_v4.md
    let bad_pattern_dest = grit_modules_dir
        .join("github.com")
        .join("getgrit")
        .join("stdlib")
        .join(".grit")
        .join("patterns")
        .join("js")
        .join("bad_pattern.md");
    std::fs::copy(bad_pattern, bad_pattern_dest)?;

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(tempdir.path());
    apply_cmd
        .arg("apply")
        .arg("bad_pattern")
        .arg("short-story.ts");
    let output = apply_cmd.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    assert_eq!(output.status.code(), Some(1),);
    assert!(stdout.contains("Pattern syntax error at"));
    Ok(())
}

#[test]
fn grit_dir_with_outdated_grit_modules() -> Result<()> {
    let tempdir = tempfile::tempdir()?;

    // copy fixtures/require.js to the tempdir
    let fixtures_root = get_fixtures_root()?;
    let require_js = fixtures_root.join("short-story.ts");
    let require_js_dest = tempdir.path().join("short-story.ts");
    std::fs::copy(require_js, &require_js_dest)?;

    // make an empty .grit/.gritmodules in the tempdir
    let grit_modules_dir = tempdir
        .path()
        .join(REPO_CONFIG_DIR_NAME)
        .join(GRIT_MODULE_DIR);
    fs::create_dir_all(&grit_modules_dir)?;

    // from the tempdir as cwd, run init
    run_init_cmd(&tempdir.path());

    // rm -rf openai_v4.md
    let openai_v4_md = grit_modules_dir
        .join("github.com")
        .join("getgrit")
        .join("stdlib")
        .join(".grit")
        .join("patterns")
        .join("js")
        .join("openai_v4.md");
    fs::remove_file(openai_v4_md)?;

    run_init_cmd(&tempdir.path());

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(tempdir.path());
    apply_cmd
        .arg("apply")
        .arg("openai_v4")
        .arg("short-story.ts");
    let output = apply_cmd.output()?;

    // Assert that the command executed successfully
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    // Read back the require.js file
    let content: String = std::fs::read_to_string(&require_js_dest)?;

    // assert that it matches snapshot
    assert_snapshot!(content);

    Ok(())
}

#[test]
#[ignore = "flakes in CI"]
fn removes_extraneous_whitespace() -> Result<()> {
    let tempdir = tempfile::tempdir()?;
    let mut cmd = get_test_cmd()?;

    let fixtures_root = get_fixtures_root()?;
    let fixture_path = fixtures_root.join("format").join("whitespace.js");
    let fixture_dest = tempdir.path().join("whitespace.js");
    std::fs::copy(fixture_path, &fixture_dest)?;

    cmd.arg("apply")
        .arg("no_console_log")
        .arg("--format")
        .arg(&fixture_dest);

    let output = cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let content: String = std::fs::read_to_string(&fixture_dest)?;

    assert_snapshot!(content);

    Ok(())
}

#[test]
fn warns_on_target_file_parsing_errors() -> Result<()> {
    let mut cmd = get_test_cmd()?;

    let fixtures_root = get_fixtures_root()?;
    let pattern_path = fixtures_root.join("require.grit");
    let fixture_path: path::PathBuf = fixtures_root.join("malformed.js");

    cmd.arg("apply")
        .arg(pattern_path)
        .arg(fixture_path)
        .arg("--force");

    let output = cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    let log_message = "Error parsing source code";
    assert!(stdout.contains(log_message));

    Ok(())
}

#[test]
fn gives_helpful_error_for_file() -> Result<()> {
    let mut cmd = get_test_cmd()?;

    cmd.arg("apply")
        .arg("check_actions_clean/.grit/grit.yaml")
        .arg("--force");

    let output = cmd.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;

    println!("stdout: {:?}", stdout);
    println!("stderr: {:?}", stderr);

    assert_eq!(output.status.code(), Some(1));

    assert_snapshot!(stdout);

    Ok(())
}

#[test]
fn python_with_tabs() -> Result<()> {
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("python_with_tabs", false)?;

    // from the tempdir as cwd, run init
    run_init(&dir.as_path())?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    // apply_cmd.current_dir(basic_path);
    apply_cmd
        .arg("apply")
        .arg("--force")
        .arg("print_twice.grit");
    let output = apply_cmd.output()?;

    // Assert that the command executed successfully
    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    // Read back the main.py file
    let target_file = dir.join("main.py");
    let content: String = std::fs::read_to_string(target_file)?;

    // assert that it matches snapshot
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn only_print_first_error() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("misc", true)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir);
    apply_cmd
        .arg("apply")
        .arg("no_console_log")
        .arg("many_problems.js");

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    println!("## stdout ##");
    print!("{}", stdout);
    println!("## /stdout ##");

    let log_message = "Error parsing source code";
    assert!(stdout.contains(log_message));
    let search_string = "ERROR (code: 300)";
    assert!(stdout.contains(search_string));
    let n_problems = stdout.matches(search_string).count();
    assert_eq!(
        n_problems, 1,
        "Expected parse error ({}) to appear once, but it appeared {} times",
        search_string, n_problems
    );

    Ok(())
}

#[test]
fn handles_logging_unbound_variable() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("log_unbound", false)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    apply_cmd.arg("apply").arg("pattern.grit").arg("file.js");
    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    Ok(())
}

#[test]
fn matches_respect_grit_disable() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("check_ignore", false)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    apply_cmd.arg("apply").arg("test").arg("test.js");
    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("Processed 1 files and found 0 matches"));

    Ok(())
}

#[test]
fn analyzes_unsuppressed_patterns() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("check_ignore", false)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    apply_cmd.arg("apply").arg("third").arg("test.js");
    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("Processed 1 files and found 1 matches"));

    Ok(())
}

#[test]
#[timeout(30000)]
#[cfg(feature = "ai_builtins")]
fn applies_openai() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("openai", false)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    apply_cmd.arg("apply").arg("pattern.grit").arg("input.py");
    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    Ok(())
}

#[test]
#[cfg(feature = "ai_builtins")]
fn applies_openai_js() -> Result<()> {
    let tempdir = tempfile::tempdir()?;
    let grit_global_dir = tempfile::tempdir()?;
    let fixtures_root = get_fixtures_root()?;
    let pattern_grit = fixtures_root.join("openai").join("basic_llm_call.grit");
    let pattern_dest = tempdir.path().join("basic_llm_call.grit");
    std::fs::copy(pattern_grit, pattern_dest.clone())?;
    let input = fixtures_root.join("openai").join("foo.js");
    let input_dest = tempdir.path().join("foo.js");
    std::fs::copy(input, &input_dest)?;
    run_init(&tempdir.path())?;
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(tempdir.path());
    apply_cmd
        .arg("apply")
        .arg("basic_llm_call.grit")
        .arg("foo.js")
        .env(GRIT_GLOBAL_DIR_ENV, grit_global_dir.path());

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );
    let content = std::fs::read_to_string(&input_dest)?;
    assert!(content.contains("How can I assist you today?"));
    Ok(())
}

#[ignore = "emabeling the embedding feature currently breaks CI"]
#[test]
fn embedding_like() -> Result<()> {
    let (_tempdir, dir) = get_fixture("embeddings", true)?;
    let mut apply_cmd = get_test_cmd()?;

    apply_cmd.current_dir(dir.clone());
    apply_cmd
        .arg("apply")
        .arg("embedding_like.grit")
        .arg("simple_assign.js");

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );
    let content = std::fs::read_to_string(dir.join("simple_assign.js"))?;
    assert_eq!(content, "console.log('hello')");
    Ok(())
}

#[test]
fn filtered_apply() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("filtered_apply", true)?;

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());
    apply_cmd
        .arg("apply")
        .arg("fix.grit")
        .arg("--only-in-json")
        .arg("eslint.json");

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    println!("stdout: {:?}", stdout);
    assert!(stdout.contains("2 matches"));

    let content = std::fs::read_to_string(dir.join("file.js"))?;
    assert_snapshot!(content);

    let content2 = std::fs::read_to_string(dir.join("file2.js"))?;
    assert_snapshot!(content2);

    Ok(())
}

#[test]
#[cfg(feature = "ai_builtins")]
fn uses_llm_choice() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("openai", true)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());
    apply_cmd
        .arg("apply")
        .arg("llm_choice.grit")
        .arg("color.js");

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let content = std::fs::read_to_string(dir.join("color.js"))?;
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn yaml_padding() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("yaml_padding", true)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());
    apply_cmd.arg("apply").arg("pattern.grit").arg("file.yaml");

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let content = std::fs::read_to_string(dir.join("file.yaml"))?;
    assert_snapshot!(content);

    Ok(())
}

// Ensure we don't highlight non-diff lines in the colored output
#[test]
fn yaml_color() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("yaml_color", true)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());
    apply_cmd.env("CLICOLOR_FORCE", "1");
    apply_cmd
        .arg("apply")
        .arg("pattern.grit")
        .arg("file.yaml")
        .arg("--dry-run");

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let content = String::from_utf8(output.stdout)?;
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn match_only_format() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("matching", true)?;

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());
    apply_cmd.env("CLICOLOR_FORCE", "1");
    apply_cmd
        .arg("apply")
        .arg("match.grit")
        .arg("big.ts")
        .arg("--dry-run");

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    let content = String::from_utf8(output.stdout)?;
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn compact_output() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("matching", true)?;

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());
    apply_cmd
        .arg("apply")
        .arg("match.grit")
        .arg("--dry-run")
        .arg("--output")
        .arg("compact");

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    let content = String::from_utf8(output.stdout)?;
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn output_jsonl() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("matching", true)?;

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());
    apply_cmd
        .arg("apply")
        .arg("match.grit")
        .arg("big.ts")
        .arg("small.ts")
        .arg("--jsonl")
        .arg("--output-file")
        .arg("output.jsonl");

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    let content = std::fs::read_to_string(dir.join("output.jsonl"))?;
    assert_snapshot!(content);

    let line_count = content.lines().count();
    assert_eq!(line_count, 3);

    Ok(())
}

#[test]
fn nested_dir() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("nested", true)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());
    apply_cmd.arg("apply").arg("fix_red").arg("main.hcl");

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let content = std::fs::read_to_string(dir.join("main.hcl"))?;
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn handles_invalid_ffi() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("foreign_js", true)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());
    apply_cmd.arg("apply").arg("invalid.grit").arg("input.js");

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    println!("stdout: {:?}", stdout);
    assert!(stdout.contains("We are in JavaScript now!"));

    Ok(())
}

#[test]
fn fizzbuzz_ffi() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("foreign_js", true)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());
    apply_cmd.arg("apply").arg("fizzbuzz.grit").arg("input.js");

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let content = std::fs::read_to_string(dir.join("input.js"))?;
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn ffi_assignment() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("foreign_js", true)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());
    apply_cmd.arg("apply").arg("assign.grit").arg("input.js");

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let content = std::fs::read_to_string(dir.join("input.js"))?;
    assert_snapshot!(content);

    Ok(())
}

#[test]
#[ignore = "flakes in CI"]
fn apply_multifile_sample() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("multifiles", true)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());
    apply_cmd.arg("apply").arg("pattern.grit");

    // do it twice
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());
    apply_cmd.arg("apply").arg("pattern.grit");

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    println!("stdout: {:?}", stdout);

    // Read user2.ts
    let content = std::fs::read_to_string(dir.join("user2.ts"))?;
    assert_snapshot!(content);

    Ok(())
}

#[test]
#[cfg(feature = "ai_builtins")]
fn ai_constraint() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("ai_constraint", true)?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.clone());
    apply_cmd.arg("apply").arg("pattern.grit").arg("input.js");

    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    println!("stdout: {:?}", stdout);

    let content = std::fs::read_to_string(dir.join("input.js"))?;
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn fails_when_binding_to_reserved_name() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("reserved_binding", false)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("apply")
        .arg("reserved.grit")
        .current_dir(fixture_dir);

    let output = cmd.output()?;

    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("reserved metavariable name"));

    Ok(())
}

#[test]
fn fails_when_assigning_to_reserved_name() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("reserved_assignment", false)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("apply")
        .arg("reserved.grit")
        .current_dir(fixture_dir);

    let output = cmd.output()?;

    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("reserved metavariable name"));

    Ok(())
}

#[test]
fn emits_done_file_for_skipped_extension() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("png", true)?;

    let png_path = fixture_dir.join("logo1.png");
    let py_path = fixture_dir.join("main.py");
    let input = format!(
        r#"{{ "pattern_body" : "print_to_log()", "paths" :  [ {:?}, {:?} ]  }}"#,
        png_path.to_str().unwrap(),
        py_path.to_str().unwrap()
    );

    let mut cmd = get_test_cmd()?;
    cmd.arg("plumbing")
        .arg("apply")
        .arg("--jsonl")
        .arg("--min-visibility")
        .arg("hidden")
        .current_dir(fixture_dir);
    cmd.write_stdin(input);

    let output = cmd.output()?;

    let stdout = String::from_utf8(output.stdout)?;
    let lines: Vec<_> = stdout.lines().collect();
    let values = lines
        .iter()
        .map(|x| serde_json::from_str::<serde_json::Value>(x));

    let done_files: Vec<_> = values
        .filter(|x| {
            x.as_ref().map_or(false, |x| {
                x.get("__typename").map_or(false, |x| x == "DoneFile")
            })
        })
        .collect();
    assert_eq!(done_files.len(), 2);

    Ok(())
}

#[test]
fn applies_multifile_pattern_from_resolved_md() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("multifile_md", false)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("apply")
        .arg("multi")
        .arg("file1.js")
        .arg("file2.js")
        .current_dir(&fixture_dir);

    let output = cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let test1 = std::fs::read_to_string(fixture_dir.join("file1.js"))?;
    assert_eq!(test1, "foo(1)");
    let test2 = std::fs::read_to_string(fixture_dir.join("file2.js"))?;
    assert_eq!(test2, "baz(1)\nbar(3)");

    Ok(())
}

#[test]
fn applies_recursive_multifile_pattern_from_resolved_md() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("multifile_md", false)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("apply")
        .arg("recursive_multi")
        .arg("file1.js")
        .arg("file2.js")
        .current_dir(&fixture_dir);

    let output = cmd.output()?;
    println!("Stderr is {:?}", String::from_utf8(output.stderr)?);
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let test1 = std::fs::read_to_string(fixture_dir.join("file1.js"))?;
    assert_eq!(test1, "foo(1)");
    let test2 = std::fs::read_to_string(fixture_dir.join("file2.js"))?;
    assert_eq!(test2, "baz(1)\nbar(3)");

    Ok(())
}

#[test]
fn applies_indirect_multi() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("multifile_md", false)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("apply")
        .arg("outer_multi")
        .arg("file1.js")
        .arg("file2.js")
        .current_dir(&fixture_dir);

    let output = cmd.output()?;
    println!("Stderr is {:?}", String::from_utf8(output.stderr)?);
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let test1 = std::fs::read_to_string(fixture_dir.join("file1.js"))?;
    assert_eq!(test1, "foo(1)");
    let test2 = std::fs::read_to_string(fixture_dir.join("file2.js"))?;
    assert_eq!(test2, "baz(1)\nbar(3)");

    Ok(())
}

#[test]
fn applies_limit_on_multifile() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("limit_files", false)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("apply")
        .arg("pattern.grit")
        .arg("file1.js")
        .arg("file2.js")
        .current_dir(&fixture_dir);

    let output = cmd.output()?;
    println!("Stderr is {:?}", String::from_utf8(output.stderr)?);
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let test1 = std::fs::read_to_string(fixture_dir.join("file1.js"))?;
    let test2 = std::fs::read_to_string(fixture_dir.join("file2.js"))?;

    assert!(test1 == "const x = 6;" || test2 == "const x = 6;");
    assert!(test1 == "const y = 6;" || test2 == "const y = 6;");

    let mut cmd = get_test_cmd()?;
    cmd.arg("apply")
        .arg("pattern.grit")
        .arg("file1.js")
        .arg("file2.js")
        .current_dir(&fixture_dir);

    let output = cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let test1 = std::fs::read_to_string(fixture_dir.join("file1.js"))?;
    let test2 = std::fs::read_to_string(fixture_dir.join("file2.js"))?;
    assert!(test1 == "const y = 6;" && test2 == "const y = 6;");

    Ok(())
}

#[test]
fn test_ignores_limit_on_scans() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("limit_files", false)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("apply")
        .arg("pattern.grit")
        .arg("file1.js")
        .arg("file2.js")
        .arg("--ignore-limit")
        .current_dir(&fixture_dir);

    let output = cmd.output()?;
    let stdout = String::from_utf8(output.stdout)?;
    let stderr = String::from_utf8(output.stderr)?;
    println!("stdout: {:?}", stdout);
    println!("stderr: {:?}", stderr);

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    assert!(stdout.contains("found 2 matches"));

    Ok(())
}

/// If we are not careful, Grit operations end up causing race conditions when done simultaneously
#[test]
#[ignore = "need to fix perf problems"]
fn run_simultaneous_apply_ops() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("parallel_apply", false)?;

    let total_cases = 10;

    // For each case, copy test.js to testN.js
    for i in 0..total_cases {
        let src = fixture_dir.join("test.js");
        let dest = fixture_dir.join(format!("test{}.js", i));
        std::fs::copy(src, dest)?;
    }

    println!("Copied files, starting actual test");

    let r = (0..total_cases)
        .into_par_iter()
        .try_for_each(|i| -> Result<()> {
            let file = format!("test{}.js", i);
            let mut cmd = get_test_cmd()?;
            cmd.arg("apply")
                .arg("var_mutate")
                .arg(&file)
                .current_dir(&fixture_dir);

            let output = cmd.output()?;
            let stdout = String::from_utf8(output.stdout)?;
            println!("stdout({}): {:?}", file, stdout);
            let stderr = String::from_utf8(output.stderr)?;
            println!("stderr({}): {:?}", file, stderr);

            if !output.status.success() {
                bail!("Command didn't finish successfully");
            }

            let test = std::fs::read_to_string(fixture_dir.join(&file))?;
            if !test.contains("const did_it_get_touched = true;") {
                bail!("File {} was not mutated", file);
            }

            Ok(())
        });
    r?;

    Ok(())
}

#[test]
fn informs_if_attempting_to_use_reserved_keyword_as_identifier() -> Result<()> {
    let (_temp_dir, fixture_dir) = get_fixture("reserved_identifier", false)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("apply").arg("bad.grit").current_dir(fixture_dir);

    let output = cmd.output()?;

    let stdout = String::from_utf8(output.stdout)?;
    println!("stdout: {}", stdout);
    assert!(stdout.contains("any is a reserved keyword"));

    Ok(())
}

#[test]
fn applies_user_pattern() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("format", false)?;
    let (_user_config, user_dir) = get_fixture("user_pattern", false)?;
    let user_grit_dir = user_dir.join(REPO_CONFIG_DIR_NAME);

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    apply_cmd
        .arg("apply")
        .arg("very_special_console_log")
        .arg("whitespace.js")
        .env("GRIT_USER_CONFIG", user_grit_dir);
    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("Processed 1 files and found 1 matches"));

    let content: String = std::fs::read_to_string(dir.join("whitespace.js"))?;
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn apply_works_on_our_repo() -> Result<()> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).canonicalize()?;
    let mut cmd = get_test_cmd()?;
    cmd.arg("apply")
        .arg("no_console_log")
        .arg("--dry-run")
        .current_dir(&dir);
    let output = cmd.output()?;
    let regex = Regex::new(r"Processed \d+ files and found \d+ matches")?;
    let stdout = String::from_utf8(output.stdout)?;
    assert!(regex.is_match(&stdout));
    Ok(())
}

#[test]
#[ignore = "flakes in CI, the timing is too tight"]
fn caches_if_enabled() -> Result<()> {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("..")
        .canonicalize()?;
    let mut cmd = get_test_cmd()?;
    cmd.arg("apply")
        .arg("no_console_log")
        .arg("--refresh-cache")
        .arg("--dry-run")
        .current_dir(&dir);
    let runtime = Instant::now();
    let output = cmd.output()?;
    let uncached_runtime = runtime.elapsed();

    let uncached_output = String::from_utf8(output.stdout)?;
    let processed = Regex::new(r"Processed (\d+) files and found (\d+) matches")?;
    let captures = processed.captures(&uncached_output).unwrap();
    let uncached_processed_files = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let uncached_processed_matches = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();

    let mut cmd = get_test_cmd()?;
    cmd.arg("apply")
        .arg("no_console_log")
        .arg("--dry-run")
        .arg("--cache")
        .current_dir(&dir);
    let runtime = Instant::now();
    let output = cmd.output()?;
    let cached_runtime = runtime.elapsed();

    let cached_output = String::from_utf8(output.stdout)?;
    let captures = processed.captures(&cached_output).unwrap();
    let cached_processed_files = captures.get(1).unwrap().as_str().parse::<u32>().unwrap();
    let cached_processed_matches = captures.get(2).unwrap().as_str().parse::<u32>().unwrap();

    assert!(uncached_runtime.as_secs() - cached_runtime.as_secs() > 1);
    assert_eq!(uncached_processed_files, cached_processed_files);
    assert_eq!(uncached_processed_matches, cached_processed_matches);
    Ok(())
}

#[test]
fn applies_on_file_in_hidden_directory() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("hidden_dir", false)?;

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    apply_cmd.arg("apply").arg("pattern.grit");
    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("Processed 1 files and found 1 matches"));

    let content: String = std::fs::read_to_string(dir.join(".circleci").join("config.yml"))?;
    assert_eq!(content, "");

    Ok(())
}

#[test]
fn ignores_file_in_grit_dir() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("ignore_grit", false)?;

    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    apply_cmd.arg("apply").arg("pattern.grit");
    let output = apply_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    println!("stdout: {:?}", stdout);
    assert!(stdout.contains("Processed 0 files and found 0 matches"));

    Ok(())
}

#[test]
fn language_option_file_pattern_apply() -> Result<()> {
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("simple_python", false)?;
    let origin_content = std::fs::read_to_string(dir.join("main.py"))?;

    // from the tempdir as cwd, run init
    run_init(&dir.as_path())?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    apply_cmd
        .arg("apply")
        .arg("--force")
        .arg("pattern.grit")
        .arg("--language")
        .arg("java");
    let output = apply_cmd.output()?;

    // Assert that the command failed
    assert!(
        !output.status.success(),
        "Command with incorrect language option should fail: {}",
        String::from_utf8(output.stdout)?
    );

    // Read back the main.py file
    let target_file = dir.join("main.py");
    let content: String = std::fs::read_to_string(target_file)?;

    assert_eq!(origin_content, content);

    Ok(())
}

#[test]
fn language_option_inline_pattern_apply() -> Result<()> {
    let pattern = r"`os.getenv` => `dotenv.fetch`";
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("simple_python", false)?;

    // from the tempdir as cwd, run init
    run_init(&dir.as_path())?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    apply_cmd
        .arg("apply")
        .arg(pattern)
        .arg("--force")
        .arg("--lang")
        .arg("python");
    let output = apply_cmd.output()?;

    // Assert that the command executed successfully
    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stdout)?
    );

    // Read back the main.py file
    let target_file = dir.join("main.py");
    let content: String = std::fs::read_to_string(target_file)?;

    // assert that it matches snapshot
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn language_option_named_pattern_apply() -> Result<()> {
    let pattern = r"pattern test_pattern() {
        `os.getenv` => `dotenv.fetch`
    }
    test_pattern()
    ";
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("simple_python", false)?;

    // from the tempdir as cwd, run init
    run_init(&dir.as_path())?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    apply_cmd
        .arg("apply")
        .arg(pattern)
        .arg("--force")
        .arg("--lang")
        .arg("python");
    let output = apply_cmd.output()?;

    // Assert that the command executed successfully
    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stdout)?
    );

    // Read back the main.py file
    let target_file = dir.join("main.py");
    let content: String = std::fs::read_to_string(target_file)?;

    // assert that it matches snapshot
    assert_snapshot!(content);

    Ok(())
}

#[test]
fn language_option_conflict_apply() -> Result<()> {
    let pattern = r"language java
     `os.getenv` => `dotenv.fetch`";
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("simple_python", false)?;

    let origin_content = std::fs::read_to_string(dir.join("main.py"))?;

    // from the tempdir as cwd, run init
    run_init(&dir.as_path())?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    apply_cmd
        .arg("apply")
        .arg(pattern)
        .arg("--force")
        .arg("--language")
        .arg("python");
    let output = apply_cmd.output()?;

    // Assert that the command failed
    assert!(
        !output.status.success(),
        "Command with conflict language option and pattern should fail"
    );

    // Read back the main.py file
    let target_file = dir.join("main.py");
    let content: String = std::fs::read_to_string(target_file)?;

    // assert that it matches snapshot
    assert_eq!(origin_content, content);

    Ok(())
}

#[test]
fn invalid_language_option_apply() -> Result<()> {
    let pattern = r"`os.getenv` => `dotenv.fetch`";
    // Keep _temp_dir around so that the tempdir is not deleted
    let (_temp_dir, dir) = get_fixture("simple_python", false)?;
    let origin_content = std::fs::read_to_string(dir.join("main.py"))?;

    // from the tempdir as cwd, run init
    run_init(&dir.as_path())?;

    // from the tempdir as cwd, run marzano apply
    let mut apply_cmd = get_test_cmd()?;
    apply_cmd.current_dir(dir.as_path());
    apply_cmd
        .arg("apply")
        .arg(pattern)
        .arg("--force")
        .arg("--lang")
        .arg("__invalid");
    let output = apply_cmd.output()?;

    // Assert that the command failed
    assert!(
        !output.status.success(),
        "Command with invalid language option should fail: {}",
        String::from_utf8(output.stdout)?
    );

    // Read back the main.py file
    let target_file = dir.join("main.py");
    let content: String = std::fs::read_to_string(target_file)?;

    // assert that it matches snapshot
    assert_eq!(origin_content, content);

    Ok(())
}
