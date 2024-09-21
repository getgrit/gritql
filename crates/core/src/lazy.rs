#[cfg(test)]
mod test {
    use grit_pattern_matcher::context::ExecContext;
    use grit_pattern_matcher::pattern::{Pattern, StringConstant};
    use grit_pattern_matcher::pattern::{PatternOrResolved, ResolvedPattern};
    use grit_pattern_matcher::{constants::DEFAULT_FILE_NAME, pattern::Contains};
    use marzano_language::{grit_parser::MarzanoGritParser, target_language::TargetLanguage};
    use std::any::Any;
    use std::sync::atomic::AtomicUsize;
    use std::{
        path::Path,
        sync::{atomic::AtomicBool, Arc},
    };

    use crate::problem::MarzanoQueryContext;
    use crate::sdk::StatelessCompilerContext;
    use crate::{
        pattern_compiler::{CompilationResult, PatternBuilder},
        test_utils::{run_on_test_files, SyntheticFile},
    };

    /// Just shorthand for Contains
    fn p_contains(pattern: Pattern<MarzanoQueryContext>) -> Pattern<MarzanoQueryContext> {
        Pattern::Contains(Box::new(Contains::new(pattern, None)))
    }

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
    fn test_callback_with_contains_ast_node() {
        let src = r#"language js

    function_declaration()"#;
        let mut parser = MarzanoGritParser::new().unwrap();
        let src_tree = parser
            .parse_file(src, Some(Path::new(DEFAULT_FILE_NAME)))
            .unwrap();
        let lang = TargetLanguage::from_tree(&src_tree).unwrap();

        let this_lang = TargetLanguage::from_string("js", None).unwrap();
        assert_eq!(lang.type_id(), this_lang.type_id());

        let matches_found = Arc::new(AtomicUsize::new(0));
        let matches_found_clone = Arc::clone(&matches_found);

        let mut builder = PatternBuilder::start_empty(src, lang).unwrap();

        builder = builder.matches_callback(Box::new(move |binding, context, state, logs| {
            let this_lang = TargetLanguage::from_string("js", None).unwrap();

            let console_builder =
                PatternBuilder::start_empty("call_expression()", this_lang).unwrap();
            let console_pattern = console_builder
                .compile(None, None, false)
                .unwrap()
                .root_pattern();

            let contains_pattern = p_contains(console_pattern);

            println!("contains pattern: {:?}", contains_pattern);

            if binding
                .matches(&contains_pattern, state, context, logs)
                .unwrap()
            {
                matches_found_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            }
            Ok(true)
        }));
        let CompilationResult { problem, .. } = builder.compile(None, None, true).unwrap();

        let test_files = vec![SyntheticFile::new(
            "file.js".to_owned(),
            r#"function notMatch() {
                // call nothing
            }

            function myLogger() {
                console.log(name);
            }"#
            .to_owned(),
            true,
        )];
        let results = run_on_test_files(&problem, &test_files);
        assert_eq!(results.len(), 2);
        assert_eq!(matches_found.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[test]
    fn test_callback_with_contains_snippet() {
        let src = r#"language js

    function_declaration()"#;
        let mut parser = MarzanoGritParser::new().unwrap();
        let src_tree = parser
            .parse_file(src, Some(Path::new(DEFAULT_FILE_NAME)))
            .unwrap();
        let lang = TargetLanguage::from_tree(&src_tree).unwrap();

        let this_lang = TargetLanguage::from_string("js", None).unwrap();
        assert_eq!(lang.type_id(), this_lang.type_id());

        let matches_found = Arc::new(AtomicUsize::new(0));
        let matches_found_clone = Arc::clone(&matches_found);

        let mut builder = PatternBuilder::start_empty(src, lang).unwrap();

        builder = builder.matches_callback(Box::new(move |binding, context, state, logs| {
            let this_lang = TargetLanguage::from_string("js", None).unwrap();

            let console_builder =
                PatternBuilder::start_empty("`console.log(name)`", this_lang.clone()).unwrap();
            let console_pattern = console_builder
                .compile(None, None, false)
                .unwrap()
                .root_pattern();

            let contains_pattern = p_contains(console_pattern);

            if binding
                .matches(&contains_pattern, state, context, logs)
                .unwrap()
            {
                matches_found_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

                // Ok we are operating on the right function, now verify more things
                let mut compiler = StatelessCompilerContext::new(this_lang.clone());
                let standalone_snippet = compiler.parse_snippet("console.log(name)").unwrap();
                let standalone_contains = p_contains(standalone_snippet);

                assert_eq!(
                    format!("{:?}", standalone_contains),
                    format!("{:?}", contains_pattern)
                );

                let also_contained = binding
                    .matches(&standalone_contains, state, context, logs)
                    .unwrap();
                assert!(also_contained, "standalone snippet should match");

                // Let's also make sure we can match a variable
                let contains_var = p_contains(compiler.parse_snippet("console.log($msg)").unwrap());
                assert!(
                    binding
                        .matches(&contains_var, state, context, logs)
                        .unwrap(),
                    "variable should match"
                );

                let our_name = state
                    .find_var_in_scope("$msg")
                    .unwrap()
                    .get_pattern_or_resolved(state)
                    .unwrap();
                let resolved = our_name.unwrap();
                let PatternOrResolved::Resolved(resolved) = resolved else {
                    panic!("No resolved pattern found");
                };
                let text = resolved.text(&state.files, context.language()).unwrap();
                assert_eq!(text, "\"my name is bob\"");
            }
            Ok(true)
        }));
        let CompilationResult { problem, .. } = builder.compile(None, None, true).unwrap();

        let test_files = vec![SyntheticFile::new(
            "file.js".to_owned(),
            r#"function notMatch() {
                // call nothing
            }

            function myLogger() {
                console.log("my name is bob");
                console.log(name);
            }"#
            .to_owned(),
            true,
        )];
        let results = run_on_test_files(&problem, &test_files);
        assert_eq!(results.len(), 2);
        assert_eq!(matches_found.load(std::sync::atomic::Ordering::SeqCst), 1);
    }
}
