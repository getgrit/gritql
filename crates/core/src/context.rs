use crate::{
    binding::Binding,
    pattern::{
        ast_node_pattern::{AstLeafNodePattern, AstNodePattern},
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

/// Contains various kinds of context about the query being executed.
pub trait QueryContext: Clone + std::fmt::Debug + Sized {
    type Node<'a>: AstNode;
    type NodePattern: AstNodePattern<Self>;
    type LeafNodePattern: AstLeafNodePattern<Self>;
    type ExecContext<'a>: ExecContext<Self>;
    type Binding<'a>: Binding<'a, Self>;
}

/// Contains context necessary for query execution.
pub trait ExecContext<Q: QueryContext> {
    fn pattern_definitions(&self) -> &[PatternDefinition<Q>];

    fn predicate_definitions(&self) -> &[PredicateDefinition<Q>];

    fn function_definitions(&self) -> &[GritFunctionDefinition<Q>];

    fn foreign_function_definitions(&self) -> &[ForeignFunctionDefinition];

    fn ignore_limit_pattern(&self) -> bool;

    fn call_built_in<'a>(
        &self,
        call: &'a CallBuiltIn<Q>,
        context: &'a Self,
        state: &mut State<'a, Q>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a, Q>>;

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
