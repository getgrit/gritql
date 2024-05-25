use insta::assert_snapshot;
use marzano_language::target_language::TargetLanguage;

use crate::{
    api::MatchResult,
    test_utils::{run_on_test_files, SyntheticFile},
};

use self::pattern_compiler::src_to_problem_libs;

use super::*;
use std::collections::BTreeMap;

#[test]
fn test_base_case() {
    let pattern_src = r#"
        language python

        `print($x)` => `flink($x)`
        "#;
    let libs = BTreeMap::new();

    let matching_src = include_str!("../../../crates/cli_bin/fixtures/notebooks/tiny_nb.ipynb");

    let pattern = src_to_problem_libs(
        pattern_src.to_string(),
        &libs,
        TargetLanguage::from_extension("ipynb").unwrap(),
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .problem;

    // Basic match works
    let test_files = vec![SyntheticFile::new(
        "target.ipynb".to_owned(),
        matching_src.to_owned(),
        true,
    )];
    let results = run_on_test_files(&pattern, &test_files);
    println!("{:?}", results);
    assert!(!results.iter().any(|r| r.is_error()));

    let rewrite = results
        .iter()
        .find(|r| matches!(r, MatchResult::Rewrite(_)))
        .unwrap();

    if let MatchResult::Rewrite(rewrite) = rewrite {
        assert_snapshot!(rewrite.rewritten.content);
    } else {
        panic!("Expected a rewrite");
    }
}

#[test]
fn test_old_notebooks() {
    let pattern_src = r#"
        language python

        `print($x)` => `flink($x)`
        "#;
    let libs = BTreeMap::new();

    let matching_src = include_str!("../../../crates/cli_bin/fixtures/notebooks/old_nb.ipynb");

    let pattern = src_to_problem_libs(
        pattern_src.to_string(),
        &libs,
        TargetLanguage::from_extension("ipynb").unwrap(),
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .problem;

    // Basic match works
    let test_files = vec![SyntheticFile::new(
        "target.ipynb".to_owned(),
        matching_src.to_owned(),
        true,
    )];
    let results = run_on_test_files(&pattern, &test_files);
    // We *do* expect an error on old notebooks
    assert!(results.iter().any(|r| r.is_error()));
}

#[test]
fn test_changing_size() {
    // The rewrite has a different length, so the source map needs to be used

    let pattern_src = r#"
        language python

        `print($x)` => `THIS_IS_MUCH_MUCH_MUCH_MUCH_MUCH_MUCH_LONGER($x)`
        "#;
    let libs = BTreeMap::new();

    let matching_src = include_str!("../../../crates/cli_bin/fixtures/notebooks/tiny_nb.ipynb");

    let pattern = src_to_problem_libs(
        pattern_src.to_string(),
        &libs,
        TargetLanguage::from_extension("ipynb").unwrap(),
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .problem;

    // Basic match works
    let test_files = vec![SyntheticFile::new(
        "target.ipynb".to_owned(),
        matching_src.to_owned(),
        true,
    )];
    let results = run_on_test_files(&pattern, &test_files);

    println!("{:?}", results);
    assert!(!results.iter().any(|r| r.is_error()));

    let rewrite = results
        .iter()
        .find(|r| matches!(r, MatchResult::Rewrite(_)))
        .unwrap();

    if let MatchResult::Rewrite(rewrite) = rewrite {
        assert_snapshot!(rewrite.rewritten.content);
    } else {
        panic!("Expected a rewrite");
    }
}
