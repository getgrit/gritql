use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::pattern::{list::List, patterns::Pattern};
use anyhow::{bail, Result};
use marzano_language::language::Field;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct ListCompiler;

impl ListCompiler {
    pub(crate) fn from_node_in_context(
        node: NodeWithSource,
        context_field: &Field,
        context: &mut NodeCompilationContext,
    ) -> Result<Pattern> {
        let kind = node.node.kind();
        match kind.as_ref() {
            "assocNode" => {
                if !context_field.multiple() {
                    bail!(
                        "Field {} does not accept list patterns",
                        context_field.name()
                    )
                }
                Ok(Pattern::List(Box::new(Self::from_node(node, context)?)))
            }
            _ => PatternCompiler::from_node(node, context),
        }
    }
}

impl NodeCompiler for ListCompiler {
    type TargetPattern = List;

    fn from_node_with_rhs(
        node: NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let patterns = node
            .named_children_by_field_name("patterns")
            .map(|pattern| PatternCompiler::from_node(pattern, context))
            .collect::<Result<Vec<_>>>()?;
        Ok(List::new(patterns))
    }
}
