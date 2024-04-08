use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::pattern::every::Every;
use anyhow::{anyhow, Result};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct EveryCompiler;

impl NodeCompiler for EveryCompiler {
    type TargetPattern = Every;

    fn from_node_with_rhs(
        node: NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let within = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern of pattern every"))?;
        let within = PatternCompiler::from_node(within, context)?;
        Ok(Every::new(within))
    }
}
