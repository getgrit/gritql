use super::{compiler::NodeCompilationContext, node_compiler::NodeCompiler};
use crate::pattern::patterns::Pattern;
use anyhow::Result;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct PatternCompiler;

impl NodeCompiler for PatternCompiler {
    type TargetPattern = Pattern;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        // TODO: Move the Pattern compiler logic here instead of forwarding.
        Pattern::from_node(
            &node.node,
            context.compilation,
            context.vars,
            context.vars_array,
            context.scope_index,
            context.global_vars,
            is_rhs,
            context.logs,
        )
    }
}
