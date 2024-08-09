use crate::{
    constant::Constant,
    context::QueryContext,
    effects::Effect,
    pattern::{FileRegistry, Pattern, ResolvedPattern, State},
};
use grit_util::{
    error::{GritPatternError, GritResult},
    AnalysisLogs, ByteRange, CodeRange, Range,
};
use std::path::Path;
use std::{borrow::Cow, collections::HashMap};

pub trait Binding<'a, Q: QueryContext>: Clone + std::fmt::Debug + PartialEq + Sized {
    fn from_constant(constant: &'a Constant) -> Self;

    fn from_node(node: Q::Node<'a>) -> Self;

    fn from_path(path: &'a Path) -> Self;

    fn from_pattern(
        pattern: &'a Pattern<Q>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<Q::Binding<'a>> {
        let resolved = Q::ResolvedPattern::from_pattern(pattern, state, context, logs)?;
        if let Some(binding) = resolved.get_last_binding() {
            Ok(binding.clone())
        } else {
            Err(GritPatternError::new(
                "cannot create binding from pattern without binding",
            ))
        }
    }

    fn from_range(range: ByteRange, source: &'a str) -> Self;

    fn singleton(&self) -> Option<Q::Node<'a>>;

    fn get_sexp(&self) -> Option<String>;

    fn position(&self, language: &Q::Language<'a>) -> Option<Range>;

    fn range(&self, language: &Q::Language<'a>) -> Option<ByteRange>;

    fn code_range(&self, language: &Q::Language<'a>) -> Option<CodeRange>;

    /// Checks whether two bindings are equivalent.
    ///
    /// Bindings are considered equivalent if they refer to the same thing.
    fn is_equivalent_to(&self, other: &Self, language: &Q::Language<'a>) -> bool;

    fn is_suppressed(&self, language: &Q::Language<'a>, current_name: Option<&str>) -> bool;

    /// Returns the padding to use for inserting the given text.
    fn get_insertion_padding(
        &self,
        text: &str,
        is_first: bool,
        language: &Q::Language<'a>,
    ) -> Option<String>;

    fn linearized_text(
        &self,
        language: &Q::Language<'a>,
        effects: &[Effect<'a, Q>],
        files: &FileRegistry<'a, Q>,
        memo: &mut HashMap<CodeRange, Option<String>>,
        distributed_indent: Option<usize>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<Cow<'a, str>>;

    fn text(&self, language: &Q::Language<'a>) -> GritResult<Cow<str>>;

    fn source(&self) -> Option<&'a str>;

    /// Returns the constant this binding binds to, if and only if it is a
    /// constant binding.
    fn as_constant(&self) -> Option<&Constant>;

    /// Returns the path of this binding, if and only if it is a filename
    /// binding.
    fn as_filename(&self) -> Option<&Path>;

    /// Returns the node of this binding, if and only if it is a node binding.
    fn as_node(&self) -> Option<Q::Node<'a>>;

    /// Returns `true` if and only if this binding is bound to a list.
    fn is_list(&self) -> bool;

    /// Returns an iterator over the items in a list.
    ///
    /// Returns `None` if the binding is not bound to a list.
    fn list_items(&self) -> Option<impl Iterator<Item = Q::Node<'a>> + Clone>;

    /// Returns the parent node of this binding.
    ///
    /// Returns `None` if the binding has no relation to a node.
    fn parent_node(&self) -> Option<Q::Node<'a>>;

    fn is_truthy(&self) -> bool;

    fn log_empty_field_rewrite_error(
        &self,
        language: &Q::Language<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<()>;
}
