#[cfg(test)]
mod test {
    use grit_pattern_matcher::pattern::ResolvedPattern;
    use grit_pattern_matcher::pattern::{Pattern, StringConstant};
    use grit_pattern_matcher::{constants::DEFAULT_FILE_NAME, pattern::Contains};
    use marzano_language::{grit_parser::MarzanoGritParser, target_language::TargetLanguage};
    use std::sync::atomic::AtomicUsize;
    use std::{
        path::Path,
        sync::{atomic::AtomicBool, Arc},
    };

    use crate::problem::MarzanoQueryContext;
    use crate::stateless::StatelessCompilerContext;
    use crate::{
        pattern_compiler::{CompilationResult, PatternBuilder},
        test_utils::{run_on_test_files, SyntheticFile},
    };

    #[test]
    fn test_callback_with_variable() {
        let src = r#"language js

    pattern this_thing() {
        $dude where {
            $dude <: contains `name` as $baz => `noice`
        }
    }

    `console.log($foo)` as $bar where $foo <: contains this_thing()"#;
        let mut parser = MarzanoGritParser::new().unwrap();
        let src_tree = parser
            .parse_file(src, Some(Path::new(DEFAULT_FILE_NAME)))
            .unwrap();
        let lang = TargetLanguage::from_tree(&src_tree).unwrap();

        let callback_called = Arc::new(AtomicBool::new(false));
        let callback_called_clone = Arc::clone(&callback_called);

        assert!(!callback_called.load(std::sync::atomic::Ordering::SeqCst));

        let mut builder = PatternBuilder::start_empty(src, lang).unwrap();
        builder = builder.matches_callback(Box::new(move |binding, context, state, logs| {
            assert!(state.find_var_in_scope("$foo").is_some());
            assert!(state.find_var_in_scope("$bar").is_some());
            assert!(state.find_var_in_scope("$dude").is_none());
            assert!(state.find_var_in_scope("$baz").is_none());
            let _registered_var = state.register_var("fuzz");
            assert!(state.find_var_in_scope("fuzz").is_some());

            let pattern = Pattern::Contains(Box::new(Contains::new(
                Pattern::<MarzanoQueryContext>::StringConstant(StringConstant::new(
                    "name".to_owned(),
                )),
                None,
            )));
            assert!(binding.matches(&pattern, state, context, logs).unwrap());

            let non_matching_pattern = Pattern::Contains(Box::new(Contains::new(
                Pattern::<MarzanoQueryContext>::StringConstant(StringConstant::new(
                    "not_found".to_owned(),
                )),
                None,
            )));
            assert!(!binding
                .matches(&non_matching_pattern, state, context, logs)
                .unwrap());

            callback_called_clone.store(true, std::sync::atomic::Ordering::SeqCst);
            Ok(true)
        }));
        let CompilationResult { problem, .. } = builder.compile(None, None, true).unwrap();

        let test_files = vec![SyntheticFile::new(
            "file.js".to_owned(),
            r#"function myLogger() {
                console.log(name);
            }"#
            .to_owned(),
            true,
        )];
        let _results = run_on_test_files(&problem, &test_files);
        assert!(callback_called.load(std::sync::atomic::Ordering::SeqCst));
    }

    #[test]
    fn test_find_pattern_in_callback() {
        let src = r#"language js

    `function $name($args) { $body }`"#;
        let mut parser = MarzanoGritParser::new().unwrap();
        let src_tree = parser
            .parse_file(src, Some(Path::new(DEFAULT_FILE_NAME)))
            .unwrap();
        let lang = TargetLanguage::from_tree(&src_tree).unwrap();

        let valid_matches = Arc::new(AtomicUsize::new(0));
        let valid_matches_clone = Arc::clone(&valid_matches);

        let mut builder = PatternBuilder::start_empty(src, lang).unwrap();
        builder = builder.matches_callback(Box::new(move |binding, context, state, logs| {
            println!(
                "binding: {:?}",
                binding.text(&state.files, &context.language)
            );

            let pattern = Pattern::Contains(Box::new(Contains::new(
                StatelessCompilerContext::new(TargetLanguage::default())
                    // console.log does not work yet
                    .parse_snippet("console")?,
                None,
            )));

            if binding.matches(&pattern, state, context, logs).unwrap() {
                valid_matches_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }

            Ok(true)
        }));
        let CompilationResult { problem, .. } = builder.compile(None, None, true).unwrap();

        let test_files = vec![SyntheticFile::new(
            "file.js".to_owned(),
            r#"
            function notHere() {
                console.error("sam");
            }

            function myLogger() {
                console.log(name);
            }"#
            .to_owned(),
            true,
        )];
        let _results = run_on_test_files(&problem, &test_files);
        assert_eq!(valid_matches.load(std::sync::atomic::Ordering::SeqCst), 1);
    }
}
