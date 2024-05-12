use std::{borrow::Cow, collections::BTreeMap, sync::mpsc};

use anyhow::Result;
use marzano_language::target_language::TargetLanguage;
use marzano_util::{
    cache::NullCache,
    rich_path::{FileName, RichFile, TryIntoInputFile},
    runtime::ExecutionContext,
};
use serde::{Deserialize, Serialize};

use crate::{api::MatchResult, pattern_compiler::src_to_problem_libs, problem::Problem};

/// SyntheticFile is used for ensuring we don't read files until their file names match
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub(crate) struct SyntheticFile {
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

enum TestCaseExpectation {
    Match,
}

pub struct TestCase {
    files: Vec<SyntheticFile>,
    pattern: String,
    expectation: TestCaseExpectation,
}

impl TestCase {
    pub fn new_match(file_contents: &str, pattern: &str) -> Self {
        Self {
            files: vec![SyntheticFile::new(
                "target.js".to_string(),
                file_contents.to_string(),
                true,
            )],
            pattern: pattern.to_string(),
            expectation: TestCaseExpectation::Match,
        }
    }
}

pub(crate) fn run_on_test_files(
    problem: &Problem,
    test_files: &[SyntheticFile],
) -> Vec<MatchResult> {
    let mut results = vec![];
    let context = ExecutionContext::default();
    let (tx, rx) = mpsc::channel::<Vec<MatchResult>>();
    problem.execute_shared(test_files.to_vec(), &context, tx, &NullCache::new());
    for r in rx.iter() {
        results.extend(r)
    }
    results
}

pub fn run_test(case: TestCase) -> Vec<MatchResult> {
    let pattern_src = case.pattern;
    let libs = BTreeMap::new();

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

    let results = run_on_test_files(&pattern, &case.files);

    match case.expectation {
        TestCaseExpectation::None => {}
        TestCaseExpectation::Match => {
            let match_case = results.iter().any(|r| r.is_match());
            assert!(match_case, "Expected a match, but got none");
        }
    }

    results
}
