use anyhow::{anyhow, Result};
use insta::assert_yaml_snapshot;
use marzano_gritmodule::config::REPO_CONFIG_DIR_NAME;
use serde_json::Value;

use crate::common::{get_fixture, get_test_cmd};

mod common;

#[test]
fn gets_json_output() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("patterns_list", false)?;

    let mut cmd = get_test_cmd()?;
    cmd.arg("patterns")
        .arg("list")
        .arg("--json")
        .current_dir(dir);

    let output = cmd.output()?;

    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    let line = stdout.lines().next().ok_or_else(|| GritPatternError::new("No output"))?;
    let v: serde_json::Value = serde_json::from_str(line)?;

    assert_yaml_snapshot!(v);

    Ok(())
}

#[test]
fn lists_patterns_with_top_heading_as_title() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("heading_title", false)?;

    let mut list = get_test_cmd()?;
    list.arg("plumbing")
        .arg("list")
        .arg("--json")
        .current_dir(dir.clone());
    let input = format!(r#"{{ "grit_dir" : {:?} }}"#, dir.join(".grit"));
    list.write_stdin(input);

    let output = list.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    let patterns: Vec<Value> = serde_json::from_str(&stdout)?;

    let hathora = patterns
        .iter()
        .find(|p| p.get("localName").unwrap() == "hathora_ts");
    assert!(hathora.is_some());
    let hathora = hathora.unwrap();
    let title = hathora.get("config").unwrap().get("title").unwrap();
    assert_eq!(title, "Upgrade Hathora to Dedicated TS SDK");

    let description = hathora.get("config").unwrap().get("description").unwrap();
    assert_eq!(description, "Migrate from the [legacy Hathora Cloud SDK](https://github.com/hathora/hathora-cloud-sdks/tree/main/typescript) to the [TypeScript SDK](https://github.com/hathora/cloud-sdk-typescript).");

    Ok(())
}

#[test]
fn does_not_list_functions_or_predicates() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("list_definitions", false)?;

    let mut list = get_test_cmd()?;
    list.arg("patterns").arg("list").current_dir(dir.clone());

    let output = list.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("literal_value"));
    assert!(stdout.contains("function_like"));
    assert!(!stdout.contains("lines"));
    assert!(!stdout.contains("todo"));
    assert!(!stdout.contains("logger"));

    Ok(())
}

#[test]
fn lists_user_patterns() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("patterns_list", false)?;
    let (_user_config, user_dir) = get_fixture("list_definitions", false)?;

    let mut list = get_test_cmd()?;
    list.arg("patterns")
        .arg("list")
        .current_dir(dir.clone())
        .env("GRIT_USER_CONFIG", user_dir.join(REPO_CONFIG_DIR_NAME));

    let output = list.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    println!("{}", stdout);
    assert!(stdout.contains("NamedPattern"));
    assert!(stdout.contains("OtherPattern"));
    assert!(stdout.contains("literal_value"));
    assert!(stdout.contains("function_like"));

    Ok(())
}

#[test]
fn filters_for_user_patterns() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("patterns_list", false)?;
    let (_user_config, user_dir) = get_fixture("list_definitions", false)?;

    let mut list = get_test_cmd()?;
    list.arg("patterns")
        .arg("list")
        .arg("--source=user")
        .current_dir(dir.clone())
        .env("GRIT_USER_CONFIG", user_dir.join(REPO_CONFIG_DIR_NAME));

    let output = list.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    println!("{}", stdout);
    assert!(!stdout.contains("NamedPattern"));
    assert!(!stdout.contains("OtherPattern"));
    assert!(stdout.contains("literal_value"));
    assert!(stdout.contains("function_like"));

    Ok(())
}

#[test]
fn deduplicates_double_imported_patterns() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("duplicate_gritmodule", true)?;

    let mut list = get_test_cmd()?;
    list.arg("patterns").arg("list").current_dir(dir.clone());
    let output = list.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    assert_eq!(stdout.matches("EtherTransfer").count(), 1);
    assert_eq!(stdout.matches("junit_ignored_tests").count(), 1);
    assert_eq!(stdout.matches("explicit_type_conversion").count(), 1);

    Ok(())
}

#[test]
fn correctly_lists_same_named_patterns_from_different_languages() -> Result<()> {
    let (_temp_dir, dir) = get_fixture("duplicate_gritmodule", true)?;

    let mut list = get_test_cmd()?;
    list.arg("patterns").arg("list").current_dir(dir.clone());
    let output = list.output()?;
    assert!(
        output.status.success(),
        "Command didn't finish successfully"
    );

    let stdout = String::from_utf8(output.stdout)?;
    println!("{}", stdout);
    assert!(!stdout.contains(
        "  ✔ after_each_file_handle_imports
  ✔ after_each_file_handle_imports"
    ));
    assert!(!stdout.contains(
        "  ✔ ensure_import_from
  ✔ ensure_import_from"
    ));

    Ok(())
}
