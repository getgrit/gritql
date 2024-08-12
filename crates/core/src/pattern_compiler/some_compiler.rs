use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::problem::MarzanoQueryContext;
use anyhow::{anyhow, Result};
use grit_pattern_matcher::pattern::Some;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct SomeCompiler;

impl NodeCompiler for SomeCompiler {
    type TargetPattern = Some<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let within = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern of pattern some"))?;
        let within = PatternCompiler::from_node(&within, context)?;
        Ok(Some::new(within))
    }
}
