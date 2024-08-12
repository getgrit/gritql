use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::problem::MarzanoQueryContext;
use anyhow::{anyhow, Result};
use grit_pattern_matcher::pattern::Contains;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct ContainsCompiler;

impl NodeCompiler for ContainsCompiler {
    type TargetPattern = Contains<MarzanoQueryContext>;

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
            .map(|n| PatternCompiler::from_node(&n, context))
            .transpose()?;
        Ok(Contains::new(contains, until))
    }
}
