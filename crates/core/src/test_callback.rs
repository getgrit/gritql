use grit_pattern_matcher::constants::DEFAULT_FILE_NAME;
use grit_pattern_matcher::pattern::ResolvedPattern;
use marzano_language::{grit_parser::MarzanoGritParser, target_language::TargetLanguage};
use std::{
    path::Path,
    sync::{atomic::AtomicBool, Arc},
};

use crate::{
    pattern_compiler::{CompilationResult, PatternBuilder},
    test_utils::{run_on_test_files, SyntheticFile},
};

#[test]
fn test_callback() {
    let src = r#"language js `console.log($_)`"#;
    let mut parser = MarzanoGritParser::new().unwrap();
    let src_tree = parser
        .parse_file(src, Some(Path::new(DEFAULT_FILE_NAME)))
        .unwrap();
    let lang = TargetLanguage::from_tree(&src_tree).unwrap();

    let callback_called = Arc::new(AtomicBool::new(false));
    let callback_called_clone = Arc::clone(&callback_called);

    assert!(!callback_called.load(std::sync::atomic::Ordering::SeqCst));

    let mut builder = PatternBuilder::start_empty(src, lang).unwrap();
    builder = builder.matches_callback(Box::new(move |binding, context, state, _| {
        let text = binding
            .text(&state.files, context.language)
            .unwrap()
            .to_string();
        assert_eq!(text, "console.log(\"hello\")");
        callback_called_clone.store(true, std::sync::atomic::Ordering::SeqCst);
        Ok(true)
    }));
    let CompilationResult { problem, .. } = builder.compile(None, None, true).unwrap();

    let test_files = vec![SyntheticFile::new(
        "file.js".to_owned(),
        r#"function myLogger() {
            console.log("hello");
        }"#
        .to_owned(),
        true,
    )];
    let _results = run_on_test_files(&problem, &test_files);
    assert!(callback_called.load(std::sync::atomic::Ordering::SeqCst));
}
