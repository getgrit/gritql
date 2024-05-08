use crate::common::{get_fixtures_root, get_test_cmd};
use anyhow::Result;
use common::get_fixture;
use insta::assert_snapshot;

mod common;

fn collect_rewrites(stdout: &str) -> Vec<serde_json::Value> {
    let mut results = Vec::new();
    for line in stdout.lines() {
        let v: serde_json::Value = serde_json::from_str(line).unwrap();
        if v.get("__typename").unwrap().as_str() == Some("Rewrite") {
            results.push(v);
        }
    }
    results
}

#[test]
fn returns_check_results_consistently() -> Result<()> {
    let mut cmd = get_test_cmd()?;

    let (_temp_dir, fixtures_root) = get_fixture("check_plumbing", true)?;
    let fixture_path = fixtures_root.join("check.ts");

    let input = format!(r#"{{ "paths" : [{:?}] }}"#, fixture_path.to_str().unwrap());

    cmd.arg("plumbing").arg("check");
    cmd.write_stdin(input.clone());

    let output = cmd.output()?;

    println!("stdout: {}", String::from_utf8(output.stdout.clone())?);
    println!("stderr: {}", String::from_utf8(output.stderr.clone())?);

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    let results = collect_rewrites(&stdout);

    assert_eq!(results.len(), 2);

    let mut rerun_cmd = get_test_cmd()?;

    rerun_cmd.arg("plumbing").arg("check");
    rerun_cmd.write_stdin(input);
    let output = rerun_cmd.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );
    let stdout = String::from_utf8(output.stdout)?;
    let results = collect_rewrites(&stdout);
    assert_eq!(results.len(), 2);

    Ok(())
}

#[test]
fn compact_json_output() -> Result<()> {
    let (_temp_dir, fixture_path) = get_fixture("quick_scan", true)?;

    let mut cmd = get_test_cmd()?;
    let input = r#"{ "pattern_body" : "file($body) where {$body <: contains `x` => `y` }", "paths": ["file1.js", "file2.js"] }"#;

    println!("input: {}", input);

    cmd.arg("plumbing")
        .arg("apply")
        .arg("--output=compact")
        .current_dir(fixture_path)
        .arg("--jsonl");
    cmd.write_stdin(input);

    let output = cmd.output()?;

    println!("stdout: {}", String::from_utf8(output.stdout.clone())?);
    println!("stderr: {}", String::from_utf8(output.stderr.clone())?);

    assert!(
        output.status.success(),
        "Command didn't finish successfully: {}",
        String::from_utf8(output.stderr)?
    );

    let content = String::from_utf8(output.stdout)?;
    // Has 2 rewrites
    assert_eq!(content.lines().count(), 3);
    assert!(content.contains(r#""__typename":"Rewrite""#));
    assert!(content.contains(r#""__typename":"AllDone""#));

    Ok(())
}

#[test]
fn returns_check_results_for_level() -> Result<()> {
    let mut cmd = get_test_cmd()?;

    let (_temp_dir, fixtures_root) = get_fixture("check_level", false)?;

    let input = format!(r#"{{ "paths" : [{:?}] }}"#, "check.ts");

    cmd.arg("plumbing")
        .arg("check")
        .arg("--level")
        .arg("error")
        .current_dir(fixtures_root);
    cmd.write_stdin(input);

    let output = cmd.output()?;

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    let results = collect_rewrites(&stdout);

    assert_eq!(results.len(), 1);

    Ok(())
}

#[test]
fn checks_patterns_round_trip() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("roundtrip", false)?;

    // init first
    let mut init = get_test_cmd()?;
    init.current_dir(dir.clone()).arg("init").output()?;

    let mut list = get_test_cmd()?;
    list.arg("patterns")
        .arg("list")
        .arg("--json")
        .current_dir(dir.clone());

    let output = list.output()?;
    println!("{}", String::from_utf8(output.stdout.clone())?);

    // Feed this stdout back in
    let mut cmd = get_test_cmd()?;
    cmd.arg("plumbing").arg("test");

    cmd.write_stdin(String::from_utf8(output.stdout)?);

    let result = cmd.output()?;

    // Result must be successful
    assert!(result.status.success());

    Ok(())
}

#[test]
fn checks_patterns_without_samples() -> Result<()> {
    let fixtures_root = get_fixtures_root()?;

    let config = r#"[
        {"body":"engine marzano(0.1)\nlanguage js\n`console.log`","config":{"samples":[]}}
    ]"#;

    let mut cmd = get_test_cmd()?;
    cmd.arg("plumbing").arg("test").current_dir(fixtures_root);

    cmd.write_stdin(String::from_utf8(config.into())?);

    let result = cmd.output()?;

    // Result must be successful
    assert!(result.status.success());

    Ok(())
}

#[test]
fn checks_invalid_patterns() -> Result<()> {
    // Feed this stdout back in
    let config = r#"[
        {"body":"`console.lo","config":{"samples":[]}}
    ]"#;

    let mut cmd = get_test_cmd()?;
    cmd.arg("plumbing").arg("test");

    cmd.write_stdin(String::from_utf8(config.into())?);

    let result = cmd.output()?;

    // snapshot stderr
    assert_snapshot!(String::from_utf8(result.stderr)?);

    // It must fail
    assert!(!result.status.success());

    Ok(())
}

#[test]
fn checks_multifile_patterns() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("test_multifile", false)?;

    let mut list = get_test_cmd()?;
    list.arg("patterns")
        .arg("list")
        .arg("--json")
        .arg("--source=local")
        .current_dir(dir.clone());

    let output = list.output()?;

    // Feed this stdout back in
    let mut cmd = get_test_cmd()?;
    cmd.arg("plumbing").arg("test");

    cmd.write_stdin(String::from_utf8(output.stdout)?);
    let result = cmd.output()?;

    assert!(result.status.success());

    Ok(())
}

#[test]
fn lists_imported_patterns() -> Result<()> {
    let fixtures_root = get_fixtures_root()?;
    let fixture_path = fixtures_root.join("import_list/.grit");

    // Delete fixtures_path/.gritmodules, if it exists
    if fixture_path.join(".gritmodules").exists() {
        std::fs::remove_dir_all(fixture_path.join(".gritmodules"))?;
    }

    let input = format!(r#"{{ "grit_dir" : {:?} }}"#, fixture_path.to_str().unwrap());

    println!("input: {}", input);

    let mut cmd = get_test_cmd()?;
    cmd.arg("plumbing").arg("list").arg("--json");

    cmd.write_stdin(String::from_utf8(input.into())?);

    let result = cmd.output()?;

    // ensure correct stderr
    assert_snapshot!(String::from_utf8(result.stderr)?);

    Ok(())
}
