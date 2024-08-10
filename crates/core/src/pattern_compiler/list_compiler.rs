use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::problem::MarzanoQueryContext;
use grit_pattern_matcher::pattern::{List, Pattern};
use grit_util::error::{GritPatternError, GritResult};
use marzano_language::language::Field;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct ListCompiler;

impl ListCompiler {
    pub(crate) fn from_node_in_context(
        node: &NodeWithSource,
        context_field: &Field,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> GritResult<Pattern<MarzanoQueryContext>> {
        let kind = node.node.kind();
        match kind.as_ref() {
            "assocNode" => {
                if !context_field.multiple() {
                    return Err(GritPatternError::new(format!(
                        "Field {} does not accept list patterns",
                        context_field.name(),
                    )));
                }
                Ok(Pattern::List(Box::new(Self::from_node_with_rhs(
                    node, context, is_rhs,
                )?)))
            }
            _ => PatternCompiler::from_node(node, context),
        }
    }
}

impl NodeCompiler for ListCompiler {
    type TargetPattern = List<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> GritResult<Self::TargetPattern> {
        let patterns = node
            .named_children_by_field_name("patterns")
            .map(|pattern| PatternCompiler::from_node_with_rhs(&pattern, context, is_rhs))
            .collect::<GritResult<Vec<_>>>()?;
        Ok(List::new(patterns))
    }
}
