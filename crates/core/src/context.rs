use crate::pattern::{
    built_in_functions::CallBuiltIn,
    function_definition::{ForeignFunctionDefinition, GritFunctionDefinition},
    pattern_definition::PatternDefinition,
    predicate_definition::PredicateDefinition,
    resolved_pattern::ResolvedPattern,
    state::State,
    FileOwners,
};
use anyhow::Result;
use marzano_language::target_language::TargetLanguage;
use marzano_util::analysis_logs::AnalysisLogs;

pub trait Context {
    fn pattern_definitions(&self) -> &[PatternDefinition];

    fn predicate_definitions(&self) -> &[PredicateDefinition];

    fn function_definitions(&self) -> &[GritFunctionDefinition];

    fn foreign_function_definitions(&self) -> &[ForeignFunctionDefinition];

    fn ignore_limit_pattern(&self) -> bool;

    fn call_built_in<'a>(
        &self,
        call: &'a CallBuiltIn,
        context: &'a Self,
        state: &mut State<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>>;

    // FIXME: Don't depend on Grit's file handling in Context.
    fn files(&self) -> &FileOwners;

    // FIXME: This introduces a dependency on TreeSitter.
    fn language(&self) -> &TargetLanguage;

    fn name(&self) -> Option<&str>;
}
