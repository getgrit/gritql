use grit_pattern_matcher::pattern::Matcher;
use grit_pattern_matcher::pattern::Pattern;
use grit_pattern_matcher::pattern::State;
use grit_util::error::GritResult;
use grit_util::AnalysisLogs;

use crate::marzano_context::MarzanoContext;
use crate::{marzano_resolved_pattern::MarzanoResolvedPattern, problem::MarzanoQueryContext};

#[derive(Debug, Clone)]
pub struct LazyTraversal<'a, 'b> {
    root: &'b MarzanoResolvedPattern<'a>,
}

impl<'a, 'b> LazyTraversal<'a, 'b> {
    pub(crate) fn new(root: &'b MarzanoResolvedPattern<'a>) -> Self {
        Self { root }
    }

    #[allow(dead_code)]
    pub(crate) fn matches(
        &mut self,
        pattern: Pattern<MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
        state: &mut State<'a, MarzanoQueryContext>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        // THIS IS UNSAFE
        // TODO: make this safe / improve the lifetimes so pattern does not need to be static
        let borrowed_pattern: &'static Pattern<MarzanoQueryContext> =
            unsafe { std::mem::transmute(&pattern) };

        let matches = borrowed_pattern.execute(self.root, state, context, logs)?;
        Ok(matches)
    }
}

#[cfg(test)]
mod test {

    use grit_pattern_matcher::pattern::{Pattern, StringConstant};
    use grit_pattern_matcher::{constants::DEFAULT_FILE_NAME, pattern::Contains};
    use marzano_language::{grit_parser::MarzanoGritParser, target_language::TargetLanguage};
    use std::{
        path::Path,
        sync::{atomic::AtomicBool, Arc},
    };

    use crate::problem::MarzanoQueryContext;
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
        builder =
            builder.matches_callback(Box::new(move |_binding, context, state, logs, lazy| {
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
                assert!(lazy.matches(pattern, context, state, logs).unwrap());

                let non_matching_pattern = Pattern::Contains(Box::new(Contains::new(
                    Pattern::<MarzanoQueryContext>::StringConstant(StringConstant::new(
                        "not_found".to_owned(),
                    )),
                    None,
                )));
                assert!(!lazy
                    .matches(non_matching_pattern, context, state, logs)
                    .unwrap());

                callback_called_clone.store(true, std::sync::atomic::Ordering::SeqCst);
                Ok(true)
            }));
        let CompilationResult { problem, .. } = builder.compile(None, None, true).unwrap();

        println!("problem: {:?}", problem);

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
}
