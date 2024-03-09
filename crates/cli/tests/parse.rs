use anyhow::{anyhow, Result};
use insta::assert_yaml_snapshot;

use crate::common::{get_fixture, get_fixtures_root, get_test_cmd};

mod common;

#[test]
fn returns_input_file() -> Result<()> {
    let mut cmd = get_test_cmd()?;

    let fixtures_root = get_fixtures_root()?;
    let fixture_path = fixtures_root.join("simple_test").join("sample.js");

    let input = format!(
        r#"{{ "pattern_body" : "language js `const $x = require($y)`", "paths" : [{:?}] }}"#,
        fixture_path.to_str().unwrap()
    );

    cmd.arg("plumbing").arg("parse").arg("--jsonl");
    cmd.write_stdin(input);

    let output = cmd.output()?;

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    let line = stdout
        .lines()
        .next_back()
        .ok_or_else(|| anyhow!("No output"))?;
    let v: serde_json::Value = serde_json::from_str(line)?;

    let found_input_file = v
        .get("__typename")
        .map_or(false, |x| x.as_str() == Some("InputFile"));

    assert!(
        found_input_file,
        "Did not find JSON object with __typename InputFile"
    );

    let mut found_input_file = v.as_object().unwrap().to_owned();
    found_input_file.remove("sourceFile");

    assert_yaml_snapshot!(found_input_file);

    Ok(())
}

#[test]
fn returns_input_file_when_pattern_has_syntax_error() -> Result<()> {
    let mut cmd = get_test_cmd()?;

    let fixtures_root = get_fixtures_root()?;
    let fixture_path = fixtures_root.join("simple_test").join("sample.js");

    let input = format!(
        r#"{{ "pattern_body" : "language js `const $x = require($y)", "paths" : [{:?}] }}"#,
        fixture_path.to_str().unwrap()
    );

    cmd.arg("plumbing").arg("parse").arg("--jsonl");
    cmd.write_stdin(input);

    let output = cmd.output()?;

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    let line = stdout
        .lines()
        .next_back()
        .ok_or_else(|| anyhow!("No output"))?;
    let v: serde_json::Value = serde_json::from_str(line)?;

    let found_input_file = v
        .get("__typename")
        .map_or(false, |x| x.as_str() == Some("InputFile"));

    assert!(
        found_input_file,
        "Did not find JSON object with __typename InputFile"
    );

    Ok(())
}

#[test]
fn parses_grit_file() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("simple_patterns", false)?;
    let fixture_path = dir.clone().join("console_log.grit");

    let mut cmd = get_test_cmd()?;

    cmd.current_dir(dir);
    cmd.arg("parse");
    cmd.arg("--jsonl");
    cmd.arg(fixture_path.to_str().unwrap());

    let output = cmd.output()?;

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    let v: serde_json::Value = serde_json::from_str(stdout.as_str())?;

    assert_yaml_snapshot!(v, {
        ".parsedPattern" => "[..]"
    });

    Ok(())
}

#[test]
fn parses_sql_file() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("sql", false)?;
    let files = vec![
        "create_function.sql",
        "create_table.sql",
        "create_procedure_declare.sql",
    ];
    for file in files {
        let fixture_path = dir.clone().join(file);
        let mut cmd = get_test_cmd()?;

        cmd.current_dir(dir.clone());
        cmd.arg("parse");
        cmd.arg("--jsonl");
        cmd.arg(fixture_path.to_str().unwrap());

        let output = cmd.output()?;

        assert!(
            output.status.success(),
            "Command didn't finish successfully for file {file}"
        );
    }

    Ok(())
}

#[test]
fn correct_variable_ranges_in_snippet_with_multiple_contexts() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("variable_scopes", false)?;
    let config = r#"{
        "pattern_body": "`function () { $body }`",
        "paths": []
    }"#;

    let mut cmd = get_test_cmd()?;
    cmd.current_dir(dir);
    cmd.arg("plumbing").arg("parse").arg("--jsonl");

    cmd.write_stdin(String::from_utf8(config.into())?);

    let output = cmd.output()?;

    assert!(
        output.status.success(),
        "Command failed: {}",
        String::from_utf8(output.stderr)?
    );

    let stdout = String::from_utf8(output.stdout)?;
    let lines = stdout.lines();
    let mut parsed_lines = Vec::new();
    for line in lines {
        let v: serde_json::Value = serde_json::from_str(line)?;
        parsed_lines.push(v);
    }
    assert_eq!(parsed_lines.len(), 1);
    assert_yaml_snapshot!(parsed_lines[0], {
        ".parsedPattern" => "[..]"
    });

    Ok(())
}

#[test]
fn correct_variable_ranges_multiple_snippets_multiple_contexts() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("variable_scopes", false)?;
    let config = r#"{
        "pattern_body": "or { bubble `function ($args) { $body }`, bubble `($args) => { $body }` }",
        "paths": []
    }"#;

    let mut cmd = get_test_cmd()?;
    cmd.current_dir(dir);
    cmd.arg("plumbing").arg("parse").arg("--jsonl");

    cmd.write_stdin(String::from_utf8(config.into())?);

    let output = cmd.output()?;

    assert!(
        output.status.success(),
        "Command failed: {}",
        String::from_utf8(output.stderr)?
    );

    let stdout = String::from_utf8(output.stdout)?;
    // Split stdout into lines
    let lines = stdout.lines();
    // Parse each line as JSON
    let mut parsed_lines = Vec::new();
    for line in lines {
        let v: serde_json::Value = serde_json::from_str(line)?;
        parsed_lines.push(v);
    }
    // Length must be 2
    assert_eq!(parsed_lines.len(), 1);

    // Snapshots for each line
    assert_yaml_snapshot!(parsed_lines[0], {
        ".parsedPattern" => "[..]"
    });

    Ok(())
}

#[test]
fn parses_foreign_function() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("foreign_js", false)?;

    let mut cmd = get_test_cmd()?;

    cmd.current_dir(dir);
    cmd.arg("parse");
    cmd.arg("--jsonl");
    cmd.arg("simple.grit");

    let output = cmd.output()?;

    assert!(
        output.status.success(),
        "{}",
        format!("Command failed: {}", String::from_utf8(output.stderr)?)
    );

    let stdout = String::from_utf8(output.stdout)?;
    let v: serde_json::Value = serde_json::from_str(stdout.as_str())?;

    println!("{:?}", v);

    assert!(
        stdout.as_str().contains("foreignFunctionBody"),
        "Did not find foreignBody in output"
    );

    assert_yaml_snapshot!(v, {
        ".parsedPattern" => "[..]"
    });

    Ok(())
}

#[test]
fn no_extraneous_ranges_for_multiple_metavariables_in_snippet() -> Result<()> {
    let config = r#"{
        "pattern_body": "`console.log($foo, $bar, $baz)`",
        "paths": []
    }"#;

    let mut cmd = get_test_cmd()?;
    cmd.arg("plumbing").arg("parse").arg("--jsonl");

    cmd.write_stdin(String::from_utf8(config.into())?);

    let output = cmd.output()?;

    assert!(
        output.status.success(),
        "Command failed: {}",
        String::from_utf8(output.stderr)?
    );

    let stdout = String::from_utf8(output.stdout)?;
    let deserialized: serde_json::Value = serde_json::from_str(&stdout)?;
    let variables = deserialized
        .get("variables")
        .ok_or_else(|| anyhow!("No variables"))?;
    let variables = variables.as_array().unwrap();
    let bar = variables
        .iter()
        .find(|v| v.get("name").unwrap().as_str().unwrap() == "$bar")
        .unwrap();

    let ranges = bar.get("ranges").unwrap().as_array().unwrap();
    assert_eq!(ranges.len(), 1);
    let range = &ranges[0];
    let start_byte = range.get("startByte").unwrap().as_u64().unwrap();
    let end_byte = range.get("endByte").unwrap().as_u64().unwrap();
    assert_eq!(start_byte, 19);
    assert_eq!(end_byte, 23);

    let baz_var = variables
        .iter()
        .find(|v| v.get("name").unwrap().as_str().unwrap() == "$baz")
        .unwrap();
    let ranges = baz_var.get("ranges").unwrap().as_array().unwrap();
    assert_eq!(ranges.len(), 1);
    let range = &ranges[0];
    let start_byte = range.get("startByte").unwrap().as_u64().unwrap();
    let end_byte = range.get("endByte").unwrap().as_u64().unwrap();
    assert_eq!(start_byte, 25);
    assert_eq!(end_byte, 29);

    Ok(())
}

#[test]
fn no_extraneous_ranges_for_multiple_metavariables_in_snippet_with_rewrite() -> Result<()> {
    let config = r#"{
        "pattern_body": "`Data($params).Scenario('$description', $func)` => `($params), $description -> $func`",
        "paths": []
    }"#;

    let mut cmd = get_test_cmd()?;
    cmd.arg("plumbing").arg("parse").arg("--jsonl");

    cmd.write_stdin(String::from_utf8(config.into())?);

    let output = cmd.output()?;

    assert!(
        output.status.success(),
        "Command failed: {}",
        String::from_utf8(output.stderr)?
    );

    let stdout = String::from_utf8(output.stdout)?;
    let deserialized: serde_json::Value = serde_json::from_str(&stdout)?;
    let variables = deserialized
        .get("variables")
        .ok_or_else(|| anyhow!("No variables"))?;
    let variables = variables.as_array().unwrap();

    let description = variables
        .iter()
        .find(|v| v.get("name").unwrap().as_str().unwrap() == "$description")
        .unwrap();

    let ranges = description.get("ranges").unwrap().as_array().unwrap();
    assert_eq!(ranges.len(), 2);
    let lhs = &ranges[0];
    let lhs_start = lhs.get("start").unwrap();
    let lhs_start_line = lhs_start.get("line").unwrap().as_u64().unwrap();
    let lhs_start_column = lhs_start.get("column").unwrap().as_u64().unwrap();
    let lhs_end = lhs.get("end").unwrap();
    let lhs_end_line = lhs_end.get("line").unwrap().as_u64().unwrap();
    let lhs_end_column = lhs_end.get("column").unwrap().as_u64().unwrap();
    assert_eq!(lhs_start_line, 1);
    assert_eq!(lhs_start_column, 26);
    assert_eq!(lhs_end_line, 1);
    assert_eq!(lhs_end_column, 38);
    let rhs = &ranges[1];
    let rhs_start = rhs.get("start").unwrap();
    let rhs_start_line = rhs_start.get("line").unwrap().as_u64().unwrap();
    let rhs_start_column = rhs_start.get("column").unwrap().as_u64().unwrap();
    let rhs_end = rhs.get("end").unwrap();
    let rhs_end_line = rhs_end.get("line").unwrap().as_u64().unwrap();
    let rhs_end_column = rhs_end.get("column").unwrap().as_u64().unwrap();
    assert_eq!(rhs_start_line, 1);
    assert_eq!(rhs_start_column, 64);
    assert_eq!(rhs_end_line, 1);
    assert_eq!(rhs_end_column, 76);

    Ok(())
}
