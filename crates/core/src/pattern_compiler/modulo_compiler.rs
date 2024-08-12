use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::problem::MarzanoQueryContext;
use grit_pattern_matcher::pattern::Modulo;
use grit_util::error::{GritPatternError, GritResult};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct ModuloCompiler;

impl NodeCompiler for ModuloCompiler {
    type TargetPattern = Modulo<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> GritResult<Self::TargetPattern> {
        let left = node
            .child_by_field_name("left")
            .ok_or_else(|| GritPatternError::new("missing left of modulo"))?;
        let left = PatternCompiler::from_node(&left, context)?;

        let right = node
            .child_by_field_name("right")
            .ok_or_else(|| GritPatternError::new("missing right of modulo"))?;
        let right = PatternCompiler::from_node(&right, context)?;

        Ok(Modulo::new(left, right))
    }
}
