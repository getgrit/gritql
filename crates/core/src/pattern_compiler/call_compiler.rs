use super::{compiler::NodeCompilationContext, node_compiler::NodeCompiler};
use crate::pattern::call::PrCall;
use anyhow::Result;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct PrCallCompiler;

impl NodeCompiler for PrCallCompiler {
    type TargetPattern = PrCall;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        // TODO: Move the PrCall compiler logic here instead of forwarding.
        PrCall::from_node(
            &node.node,
            context.compilation,
            context.vars,
            context.vars_array,
            context.scope_index,
            context.global_vars,
            context.logs,
        )
    }
}
