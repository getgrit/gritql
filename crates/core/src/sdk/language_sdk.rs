use anyhow::Result;
use grit_pattern_matcher::pattern::Pattern;
use grit_util::AnalysisLogs;
use marzano_language::target_language::TargetLanguage;

use crate::{
    built_in_functions::BuiltIns,
    pattern_compiler::{
        auto_wrap::{auto_wrap_pattern, should_autowrap, should_wrap_in_file},
        compiler::VariableLocations,
        PatternBuilder,
    },
    problem::{MarzanoQueryContext, Problem},
};

use super::StatelessCompilerContext;

/// GlobalBuilder provides a higher level interface for building and composing patterns
///
/// The primary goal is to provide simple hooks for other languages (JS, Python, etc) to
/// hook into, while sharing common logic and utilities.
pub struct LanguageSdk {
    language: TargetLanguage,
    compiler: StatelessCompilerContext,
}

impl LanguageSdk {
    pub fn default() -> Self {
        let language = TargetLanguage::from_string("js", None).unwrap();
        Self {
            compiler: StatelessCompilerContext::new(language),
            language,
        }
    }

    pub fn snippet(&self, snippet: &str) -> Result<Pattern<MarzanoQueryContext>> {
        self.compiler.clone().parse_snippet(snippet)
    }

    pub fn build(&self, pattern: Pattern<MarzanoQueryContext>) -> Result<Problem> {
        let mut built_ins = BuiltIns::get_built_in_functions();
        let mut logs: AnalysisLogs = vec![].into();
        let mut global_vars = PatternBuilder::build_standard_global_vars();
        let mut pattern_definitions = vec![];

        let is_multifile = false;

        let problem = Problem::new_from_pattern(
            pattern,
            self.language,
            built_ins,
            is_multifile,
            false,
            None,
            VariableLocations::new(vec![]),
            pattern_definitions,
            vec![],
            vec![],
            vec![],
        );
        Ok(problem)
    }
}