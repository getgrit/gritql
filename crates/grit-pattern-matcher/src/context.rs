use crate::{
    binding::Binding,
    constants::{GLOBAL_VARS_SCOPE_INDEX, PROGRAM_INDEX},
    file_owners::FileOwners,
    pattern::{
        AstLeafNodePattern, AstNodePattern, CallBuiltIn, CodeSnippet, File, GritFunctionDefinition,
        Pattern, PatternDefinition, PredicateDefinition, ResolvedPattern, State,
    },
};
use anyhow::Result;
use grit_util::{AnalysisLogs, Ast, AstNode, Language};

/// Contains various kinds of context about the query being executed.
pub trait QueryContext: Clone + std::fmt::Debug + Sized + 'static {
    type Node<'a>: AstNode + Clone;
    type NodePattern: AstNodePattern<Self>;
    type LeafNodePattern: AstLeafNodePattern<Self>;
    type ExecContext<'a>: ExecContext<'a, Self>;
    type Binding<'a>: Binding<'a, Self>;
    type CodeSnippet: CodeSnippet<Self>;
    type ResolvedPattern<'a>: ResolvedPattern<'a, Self>;
    type Language<'a>: Language<Node<'a> = Self::Node<'a>>;
    type File<'a>: File<'a, Self>;
    type Tree: Ast + Clone;
}

/// Contains context necessary for query execution.
pub trait ExecContext<'a, Q: QueryContext> {
    fn pattern_definitions(&self) -> &[PatternDefinition<Q>];

    fn predicate_definitions(&self) -> &[PredicateDefinition<Q>];

    fn function_definitions(&self) -> &[GritFunctionDefinition<Q>];

    fn ignore_limit_pattern(&self) -> bool;

    fn call_built_in(
        &self,
        call: &'a CallBuiltIn<Q>,
        context: &'a Self,
        state: &mut State<'a, Q>,
        logs: &mut AnalysisLogs,
    ) -> Result<Q::ResolvedPattern<'a>>;

    /// Call this when "entering" a file to lazily load it.
    /// This MUST be implemented correctly, or the query engine will not work.
    /// Once the file is loaded,
    ///
    /// TODO: ideally this should be async, but that requires engine-wide async support.
    fn load_file(
        &self,
        file: &Q::File<'a>,
        state: &mut State<'a, Q>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool>;

    // FIXME: Don't depend on Grit's file handling in Context.
    fn files(&self) -> &FileOwners<Q::Tree>;

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
