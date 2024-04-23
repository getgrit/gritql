use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::problem::MarzanoQueryContext;
use anyhow::{anyhow, Result};
use grit_core_patterns::pattern::{float_constant::FloatConstant, like::Like, patterns::Pattern};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct LikeCompiler;

impl NodeCompiler for LikeCompiler {
    type TargetPattern = Like<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let threshold = node
            .child_by_field_name("threshold")
            .map(|n| PatternCompiler::from_node_with_rhs(&n, context, true))
            .unwrap_or(Result::Ok(Pattern::FloatConstant(FloatConstant::new(0.9))))?;
        let like = node
            .child_by_field_name("example")
            .ok_or_else(|| anyhow!("missing field example of patternLike"))?;
        let like = PatternCompiler::from_node_with_rhs(&like, context, true)?;
        Ok(Like::new(like, threshold))
    }
}
