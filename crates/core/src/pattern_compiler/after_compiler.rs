use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::pattern::after::After;
use anyhow::{anyhow, Result};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct AfterCompiler;

impl NodeCompiler for AfterCompiler {
    type TargetPattern = After;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let pattern = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern of patternAfter"))?;
        let pattern = PatternCompiler::from_node(&pattern, context)?;
        Ok(After::new(pattern))
    }
}
