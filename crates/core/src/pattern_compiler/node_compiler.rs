use super::compiler::NodeCompilationContext;
use anyhow::Result;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) trait NodeCompiler {
    type TargetPattern;

    fn from_node(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
    ) -> Result<Self::TargetPattern> {
        Self::from_node_with_rhs(node, context, false)
    }

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Self::TargetPattern>;
}
