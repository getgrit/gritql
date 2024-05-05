use marzano_language::target_language::TargetLanguage;
use marzano_util::{rich_path::RichFile, runtime::ExecutionContext};

use self::{pattern_compiler::src_to_problem_libs, test::TEST_EXECUTION_CONTEXT};

use super::*;
use std::collections::BTreeMap;

#[test]
fn test_lazy_file_parsing() {
    let pattern_src = r#"
        file(name=includes "target.js", pattern=contains `console.log`)
        "#;
    let libs = BTreeMap::new();
    let default_language = None;

    let matching_src = r#"
        console.log("Hello, world!");
        "#;
    let non_matching_src = r#"
        console.error("Hello, world!");
        "#;

    let pattern = src_to_problem_libs(
        pattern_src.to_string(),
        &libs,
        TargetLanguage::default(),
        None,
        None,
        None,
        None,
    )?
    .problem;

    let context = ExecutionContext::default();

    // Basic match works
    let results = pattern.execute_file(
        &RichFile::new("target.js".to_owned(), matching_src.to_owned()),
        context,
    );
}
