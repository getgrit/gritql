use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::pattern::{float_constant::FloatConstant, like::Like, patterns::Pattern};
use crate::problem::MarzanoProblemContext;
use anyhow::{anyhow, Result};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct LikeCompiler;

impl NodeCompiler for LikeCompiler {
    type TargetPattern = Like<MarzanoProblemContext>;

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
