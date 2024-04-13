use super::{
    compiler::NodeCompilationContext, container_compiler::ContainerCompiler,
    node_compiler::NodeCompiler, pattern_compiler::PatternCompiler,
};
use crate::pattern::r#match::Match;
use crate::problem::MarzanoProblemContext;
use anyhow::{anyhow, Result};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct MatchCompiler;

impl NodeCompiler for MatchCompiler {
    type TargetPattern = Match<MarzanoProblemContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let value = node
            .child_by_field_name("left")
            .ok_or_else(|| anyhow!("missing lhs of predicateMatch"))?;
        let value = ContainerCompiler::from_node(&value, context)?;
        let pattern = node
            .child_by_field_name("right")
            .ok_or_else(|| anyhow!("missing rhs of predicateMatch"))?;
        let pattern = Some(PatternCompiler::from_node(&pattern, context)?);
        Ok(Match::new(value, pattern))
    }
}
