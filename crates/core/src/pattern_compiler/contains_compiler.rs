use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::pattern::contains::Contains;
use anyhow::{anyhow, Result};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct ContainsCompiler;

impl NodeCompiler for ContainsCompiler {
    type TargetPattern = Contains;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let contains = node
            .child_by_field_name("contains")
            .ok_or_else(|| anyhow!("missing contains of patternContains"))?;
        let contains = PatternCompiler::from_node(&contains, context)?;
        let until = node
            .child_by_field_name("until")
            .map(|n| PatternCompiler::from_node(&n, context));
        let until = until.map_or(Ok(None), |v| v.map(Some))?;
        Ok(Contains::new(contains, until))
    }
}
