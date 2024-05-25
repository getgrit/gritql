use insta::assert_snapshot;
use marzano_language::target_language::TargetLanguage;

use crate::{
    api::{MatchResult, Rewrite},
    test_utils::{run_on_test_files, SyntheticFile},
};

use self::pattern_compiler::src_to_problem_libs;

use super::*;
use std::collections::BTreeMap;

#[test]
fn test_changing_lengths() {
    let pattern_src = r#"
        language python

        `print($x)` => `p($x)`
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
    assert!(!results.iter().any(|r| r.is_error()));

    let rewrite = results
        .iter()
        .find(|r| matches!(r, MatchResult::Rewrite(_)))
        .unwrap();

    match rewrite {
        MatchResult::Rewrite(rewrite) => {
            assert_snapshot!(rewrite.rewritten.content);
        }
        _ => panic!("Expected a rewrite"),
    };
}
