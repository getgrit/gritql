use crate::{
    binding::Binding,
    pattern::{
        ast_node_pattern::{AstLeafNodePattern, AstNodePattern},
        built_in_functions::CallBuiltIn,
        function_definition::{ForeignFunctionDefinition, GritFunctionDefinition},
        pattern_definition::PatternDefinition,
        patterns::{CodeSnippet, Pattern},
        predicate_definition::PredicateDefinition,
        resolved_pattern::{File, ResolvedPattern},
        state::State,
    },
    problem::FileOwners,
};
use anyhow::Result;
use grit_util::{AnalysisLogs, AstNode, Language};

/// Contains various kinds of context about the query being executed.
pub trait QueryContext: Clone + std::fmt::Debug + Sized + 'static {
    type Node<'a>: AstNode;
    type NodePattern: AstNodePattern<Self>;
    type LeafNodePattern: AstLeafNodePattern<Self>;
    type ExecContext<'a>: ExecContext<'a, Self>;
    type Binding<'a>: Binding<'a, Self>;
    type CodeSnippet: CodeSnippet<Self>;
    type ResolvedPattern<'a>: ResolvedPattern<'a, Self>;
    type Language<'a>: Language<Node<'a> = Self::Node<'a>>;
    type File<'a>: File<'a, Self>;
}

/// Contains context necessary for query execution.
pub trait ExecContext<'a, Q: QueryContext> {
    fn pattern_definitions(&self) -> &[PatternDefinition<Q>];

    fn predicate_definitions(&self) -> &[PredicateDefinition<Q>];

    fn function_definitions(&self) -> &[GritFunctionDefinition<Q>];

    fn foreign_function_definitions(&self) -> &[ForeignFunctionDefinition];

    fn ignore_limit_pattern(&self) -> bool;

    fn call_built_in(
        &self,
        call: &'a CallBuiltIn<Q>,
        context: &'a Self,
        state: &mut State<'a, Q>,
        logs: &mut AnalysisLogs,
    ) -> Result<Q::ResolvedPattern<'a>>;

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

    fn language(&self) -> &Q::Language<'a>;

    fn exec_step(
        &'a self,
        step: &'a Pattern<Q>,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool>;

    fn name(&self) -> Option<&str>;
}
