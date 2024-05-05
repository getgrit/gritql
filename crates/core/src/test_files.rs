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
    problem.execute_shared(&test_files, &context, tx, &NullCache::new());
    for r in rx.iter() {
        results.extend(r)
    }
    results
}

#[test]
fn test_lazy_file_parsing() {
    let pattern_src = r#"
        file(name=includes "target.js", body=contains `console.log`)
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

    let context = ExecutionContext::default();

    // Basic match works
    let test_files = vec![SyntheticFile::new(
        "target.js".to_owned(),
        matching_src.to_owned(),
        true,
    )];
    let results = run_on_test_files(&pattern, &test_files);
    assert_eq!(results.len(), 2);

    // Non-match match works
    let test_files = vec![SyntheticFile::new(
        "worng.js".to_owned(),
        matching_src.to_owned(),
        true,
    )];
    let results = run_on_test_files(&pattern, &test_files);
    assert_eq!(results.len(), 1);

    // Unreadable file should not be read
    let test_files = vec![SyntheticFile::new(
        "do_not_read.js".to_owned(),
        String::new(),
        false,
    )];
    let results = run_on_test_files(&pattern, &test_files);
    assert_eq!(results.len(), 1);

    // Panic for now
    panic!("TODO: Implement lazy file parsing");
}
