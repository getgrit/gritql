use marzano_language::target_language::TargetLanguage;
use marzano_util::{
    cache::NullCache,
    rich_path::{FileName, RichFile, TryIntoInputFile},
    runtime::ExecutionContext,
};
use serde::{Deserialize, Serialize};

use crate::api::MatchResult;

use self::{pattern_compiler::src_to_problem_libs, problem::Problem, test::TEST_EXECUTION_CONTEXT};
use anyhow::Result;

use super::*;
use std::{borrow::Cow, collections::BTreeMap, sync::mpsc};

/// SyntheticFile is used for ensuring we don't read files until their file names match
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
struct SyntheticFile {
    pub path: String,
    pub content: String,
    pub can_read: bool,
}

impl SyntheticFile {
    pub fn new(path: String, content: String, can_read: bool) -> Self {
        Self {
            path,
            content,
            can_read,
        }
    }
}

impl TryIntoInputFile for SyntheticFile {
    fn try_into_cow(&self) -> Result<Cow<RichFile>> {
        if !self.can_read {
            println!("Tried to read file that should not be read: {}", self.path);
        }

        Ok(Cow::Owned(RichFile::new(
            self.path.clone(),
            self.content.clone(),
        )))
    }
}

impl FileName for SyntheticFile {
    fn name(&self) -> String {
        self.path.to_owned()
    }
}

fn run_on_test_files(problem: &Problem, test_files: &[SyntheticFile]) -> Vec<MatchResult> {
    let mut results = vec![];
    let context = ExecutionContext::default();
    let (tx, rx) = mpsc::channel::<Vec<MatchResult>>();
    problem.execute_shared(test_files.to_vec(), &context, tx, &NullCache::new());
    for r in rx.iter() {
        results.extend(r)
    }
    results
}

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
