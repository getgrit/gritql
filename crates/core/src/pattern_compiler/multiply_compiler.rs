use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::problem::MarzanoQueryContext;
use grit_pattern_matcher::pattern::Multiply;
use grit_util::error::{GritPatternError, GritResult};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct MultiplyCompiler;

impl NodeCompiler for MultiplyCompiler {
    type TargetPattern = Multiply<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> GritResult<Self::TargetPattern> {
        let left = node
            .child_by_field_name("left")
            .ok_or_else(|| GritPatternError::new("missing left of multiply"))?;
        let left = PatternCompiler::from_node(&left, context)?;

        let right = node
            .child_by_field_name("right")
            .ok_or_else(|| GritPatternError::new("missing right of multiply"))?;
        let right = PatternCompiler::from_node(&right, context)?;

        Ok(Multiply::new(left, right))
    }
}
