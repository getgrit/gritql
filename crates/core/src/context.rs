use crate::{
    pattern::{
        ast_node_pattern::AstNodePattern,
        built_in_functions::CallBuiltIn,
        function_definition::{ForeignFunctionDefinition, GritFunctionDefinition},
        pattern_definition::PatternDefinition,
        predicate_definition::PredicateDefinition,
        resolved_pattern::ResolvedPattern,
        state::State,
    },
    problem::FileOwners,
};
use anyhow::Result;
use grit_util::AstNode;
use marzano_language::target_language::TargetLanguage;
use marzano_util::analysis_logs::AnalysisLogs;

/// Contains various kinds of context about the problem being executed.
pub trait ProblemContext: Clone + std::fmt::Debug + Sized {
    type Node<'a>: AstNode;
    type NodePattern: AstNodePattern<Self>;
    type ExecContext<'a>: ExecContext<Self>;
}

/// Contains context necessary for problem execution.
pub trait ExecContext<P: ProblemContext> {
    fn pattern_definitions(&self) -> &[PatternDefinition<P>];

    fn predicate_definitions(&self) -> &[PredicateDefinition<P>];

    fn function_definitions(&self) -> &[GritFunctionDefinition<P>];

    fn foreign_function_definitions(&self) -> &[ForeignFunctionDefinition];

    fn ignore_limit_pattern(&self) -> bool;

    fn call_built_in<'a>(
        &self,
        call: &'a CallBuiltIn<P>,
        context: &'a Self,
        state: &mut State<'a, P>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>>;

    #[cfg(all(
        feature = "network_requests_external",
        feature = "external_functions_ffi",
        not(feature = "network_requests"),
        target_arch = "wasm32"
    ))]
    fn exec_external(
        &self,
        code: &[u8],
        param_names: Vec<String>,
        input_bindings: &[&str],
    ) -> Result<Vec<u8>>;

    // FIXME: Don't depend on Grit's file handling in Context.
    fn files(&self) -> &FileOwners;

    // FIXME: This introduces a dependency on TreeSitter.
    fn language(&self) -> &TargetLanguage;

    fn name(&self) -> Option<&str>;
}
