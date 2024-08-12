use super::{
    compiler::NodeCompilationContext, container_compiler::ContainerCompiler,
    node_compiler::NodeCompiler, pattern_compiler::PatternCompiler,
};
use crate::problem::MarzanoQueryContext;
use grit_pattern_matcher::pattern::Match;
use grit_util::error::{GritPatternError, GritResult};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct MatchCompiler;

impl NodeCompiler for MatchCompiler {
    type TargetPattern = Match<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> GritResult<Self::TargetPattern> {
        let value = node
            .child_by_field_name("left")
            .ok_or_else(|| GritPatternError::new("missing lhs of predicateMatch"))?;
        let value = ContainerCompiler::from_node(&value, context)?;
        let pattern = node
            .child_by_field_name("right")
            .ok_or_else(|| GritPatternError::new("missing rhs of predicateMatch"))?;
        let pattern = Some(PatternCompiler::from_node(&pattern, context)?);
        Ok(Match::new(value, pattern))
    }
}
