use super::{compiler::NodeCompilationContext, node_compiler::NodeCompiler};
use crate::pattern::list::List;
use anyhow::Result;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct ListCompiler;

impl NodeCompiler for ListCompiler {
    type TargetPattern = List;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        // TODO: Move the List compiler logic here instead of forwarding.
        List::from_node(
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
