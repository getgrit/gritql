use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::problem::MarzanoQueryContext;
use anyhow::{anyhow, Result};
use grit_pattern_matcher::pattern::Within;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct WithinCompiler;

impl NodeCompiler for WithinCompiler {
    type TargetPattern = Within<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let within = node
            .child_by_field_name("pattern")
            .ok_or_else(|| GritPatternError::new("missing pattern of pattern within"))?;
        let within = PatternCompiler::from_node(&within, context)?;
        Ok(Within::new(within))
    }
}
