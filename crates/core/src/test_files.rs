use marzano_language::target_language::TargetLanguage;

use crate::{
    api::{MatchResult, Rewrite},
    test_utils::{run_on_test_files, SyntheticFile},
};

use self::pattern_compiler::src_to_problem_libs;

use super::*;
use std::collections::BTreeMap;

#[test]
fn test_lazy_file_parsing() {
    let pattern_src = r#"
        file(name=includes "target.js", body=contains bubble `$x` where {
            $x <: contains `console.log($_)`
        })
        "#;
    let libs = BTreeMap::new();

    let matching_src = r#"
        console.log("Hello, world!");
        "#;

    let pattern = src_to_problem_libs(
        pattern_src.to_string(),
        &libs,
        TargetLanguage::default(),
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .problem;

    // Basic match works
    let test_files = vec![SyntheticFile::new(
        "target.js".to_owned(),
        matching_src.to_owned(),
        true,
    )];
    let results = run_on_test_files(&pattern, &test_files);
    assert!(results.iter().any(|r| r.is_match()));

    // Non-match match works
    let test_files = vec![SyntheticFile::new(
        "worng.js".to_owned(),
        matching_src.to_owned(),
        true,
    )];
    let results = run_on_test_files(&pattern, &test_files);
    assert!(!results.iter().any(|r| r.is_match()));

    // Unreadable file should not be read
    let test_files = vec![SyntheticFile::new(
        "do_not_read.js".to_owned(),
        String::new(),
        false,
    )];
    let results = run_on_test_files(&pattern, &test_files);
    assert!(!results.iter().any(|r| r.is_match()));

    // All together now
    let test_files = vec![
        SyntheticFile::new("wrong.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("target.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("other.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("do_not_read.js".to_owned(), String::new(), false),
    ];
    let results = run_on_test_files(&pattern, &test_files);
    // Confirm we have 4 DoneFiles and 1 match
    assert_eq!(results.len(), 5);
    assert!(results.iter().any(|r| r.is_match()));
}

#[test]
fn test_lazy_filename_variable() {
    let pattern_src = r#"
        file(name=includes "target.js", body=contains bubble `$x` where {
            $x <: contains `console.log($_)`,
            $filename <: includes "target.js",
        })
        "#;
    let libs = BTreeMap::new();

    let matching_src = r#"
        console.log("Hello, world!");
        "#;

    let pattern = src_to_problem_libs(
        pattern_src.to_string(),
        &libs,
        TargetLanguage::default(),
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .problem;

    // All together now
    let test_files = vec![
        SyntheticFile::new("wrong.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("target.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("other.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("do_not_read.js".to_owned(), String::new(), false),
    ];
    let results = run_on_test_files(&pattern, &test_files);
    // Confirm we have 4 DoneFiles and 1 match
    assert_eq!(results.len(), 5);
    assert!(results.iter().any(|r| r.is_match()));
}

#[test]
fn test_absolute_path_resolution() {
    let pattern_src = r#"
        file(body=contains bubble `console.log($msg)` where {
            $resolved = resolve($absolute_filename),
            $msg => `Hello, from $resolved`
        })
        "#;
    let libs = BTreeMap::new();

    let matching_src = r#"
        console.log("Hello, world!");
        "#;

    let pattern = src_to_problem_libs(
        pattern_src.to_string(),
        &libs,
        TargetLanguage::default(),
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .problem;

    // All together now
    let test_files = vec![SyntheticFile::new(
        "file/dir/target.js".to_owned(),
        matching_src.to_owned(),
        true,
    )];
    let results = run_on_test_files(&pattern, &test_files);
    assert!(!results.iter().any(|r| r.is_error()));
    let mut has_rewrite = false;
    for r in results.iter() {
        if let MatchResult::Rewrite(Rewrite { rewritten, .. }) = r {
            let content = &rewritten.content;
            assert!(content.contains("core/file/dir/target.js"));
            has_rewrite = true;
        }
    }
    assert!(has_rewrite);
}

#[test]
fn test_lazy_program_variable() {
    let pattern_src = r#"
        file(name=includes "target.js", body=contains bubble `$x` where {
            $x <: contains `console.log($_)`,
            $filename <: includes "target.js",
            $program <: includes `console`
        })
        "#;
    let libs = BTreeMap::new();

    let matching_src = r#"
        console.log("Hello, world!");
        "#;

    let pattern = src_to_problem_libs(
        pattern_src.to_string(),
        &libs,
        TargetLanguage::default(),
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .problem;

    // All together now
    let test_files = vec![
        SyntheticFile::new("wrong.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("target.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("other.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("do_not_read.js".to_owned(), String::new(), false),
    ];
    let results = run_on_test_files(&pattern, &test_files);
    // Confirm we have 4 DoneFiles and 1 match
    assert_eq!(results.len(), 5);
    assert!(results.iter().any(|r| r.is_match()));
}

#[test]
fn test_pattern_contains() {
    let pattern_src = r#"
        pattern main_thing() {
            `console.log` where { $filename <: includes "target.js" }
        }
        contains main_thing()
        "#;
    let libs = BTreeMap::new();

    let matching_src = r#"
        console.log("Hello, world!");
        "#;

    let pattern = src_to_problem_libs(
        pattern_src.to_string(),
        &libs,
        TargetLanguage::default(),
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .problem;

    // All together now
    let test_files = vec![
        SyntheticFile::new("wrong.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("target.js".to_owned(), matching_src.to_owned(), true),
    ];
    let results = run_on_test_files(&pattern, &test_files);
    assert_eq!(results.len(), 3);
    assert!(results.iter().any(|r| r.is_match()));
}

#[test]
fn test_sequential_contains() {
    let pattern_src = r#"
        pattern main_thing() {
            `console.log` where { $filename <: includes "target.js" }
        }
        sequential {
            contains main_thing()
        }
        "#;
    let libs = BTreeMap::new();

    let matching_src = r#"
        console.log("Hello, world!");
        "#;

    let pattern = src_to_problem_libs(
        pattern_src.to_string(),
        &libs,
        TargetLanguage::default(),
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .problem;

    // All together now
    let test_files = vec![
        SyntheticFile::new("wrong.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("target.js".to_owned(), matching_src.to_owned(), true),
    ];
    let results = run_on_test_files(&pattern, &test_files);
    assert_eq!(results.len(), 3);
    assert!(results.iter().any(|r| r.is_match()));
}

#[test]
fn test_sequential_contains_with_program() {
    let pattern_src = r#"
        pattern main_thing() {
            `console.log` as $lg where {
                $filename <: includes "target.js",
                $program <: contains `log`
            }
        }
        sequential {
            contains main_thing()
        }
        "#;
    let libs = BTreeMap::new();

    let matching_src = r#"
        console.log("Hello, world!");
        "#;

    let pattern = src_to_problem_libs(
        pattern_src.to_string(),
        &libs,
        TargetLanguage::default(),
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .problem;

    // All together now
    let test_files = vec![
        SyntheticFile::new("wrong.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("target.js".to_owned(), matching_src.to_owned(), true),
    ];
    let results = run_on_test_files(&pattern, &test_files);
    assert_eq!(results.len(), 3);
    assert!(results.iter().any(|r| r.is_match()));
}

#[test]
fn test_multifile_mania() {
    let pattern_src = r#"
        pattern main_thing() {
            `console.log` where { $filename <: includes "target.js" }
        }
        multifile {
            contains main_thing(),
            contains `log` where { $filename <: includes "target.js" }
        }
        "#;
    let libs = BTreeMap::new();

    let matching_src = r#"
        console.log("Hello, world!");
        "#;

    let pattern = src_to_problem_libs(
        pattern_src.to_string(),
        &libs,
        TargetLanguage::default(),
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .problem;

    // All together now
    let test_files = vec![
        SyntheticFile::new("wrong.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("target.js".to_owned(), matching_src.to_owned(), true),
    ];
    let results = run_on_test_files(&pattern, &test_files);
    assert!(results.iter().any(|r| r.is_match()));
    // Make sure no errors
    assert!(!results.iter().any(|r| r.is_error()));
}

#[test]
fn test_filename_query_optimization() {
    let pattern_src = r#"
        `console.log($_)` where {
            $filename <: "target.js",
        }
        "#;
    let libs = BTreeMap::new();

    let matching_src = r#"
        console.log("Hello, world!");
        "#;

    let pattern = src_to_problem_libs(
        pattern_src.to_string(),
        &libs,
        TargetLanguage::default(),
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .problem;

    // All together now
    let test_files = vec![
        SyntheticFile::new("wrong.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("target.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("other.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("do_not_read.js".to_owned(), String::new(), false),
    ];
    let results = run_on_test_files(&pattern, &test_files);
    // Confirm we have 4 DoneFiles and 1 match
    assert_eq!(results.len(), 5);
    assert!(results.iter().any(|r| r.is_match()));
}

#[test]
fn avoid_unsafe_hoists() {
    let pattern_src = r#"
        `console.log($msg)` where {
            /// This is not safe to hoist, since $msg is not defined in the outer scope
            $filename <: includes or { $msg, "target.js" }
        }
        "#;
    let libs = BTreeMap::new();

    let matching_src = r#"
        console.log("target.js");
        "#;

    let pattern = src_to_problem_libs(
        pattern_src.to_string(),
        &libs,
        TargetLanguage::default(),
        None,
        None,
        None,
        None,
    )
    .unwrap()
    .problem;

    // All together now
    let test_files = vec![
        SyntheticFile::new("wrong.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("target.js".to_owned(), matching_src.to_owned(), true),
        SyntheticFile::new("other.js".to_owned(), matching_src.to_owned(), true),
    ];
    let results = run_on_test_files(&pattern, &test_files);
    // Confirm we have 3 DoneFiles and 1 match
    assert_eq!(results.len(), 4);
    assert!(results.iter().any(|r| r.is_match()));
}
