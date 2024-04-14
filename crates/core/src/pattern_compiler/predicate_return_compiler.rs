use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::{pattern::predicate_return::PrReturn, problem::MarzanoQueryContext};
use anyhow::{anyhow, Result};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct PredicateReturnCompiler;

impl NodeCompiler for PredicateReturnCompiler {
    type TargetPattern = PrReturn<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let pattern = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern of return"))?;
        let pattern = PatternCompiler::from_node_with_rhs(&pattern, context, true)?;
        Ok(PrReturn::new(pattern))
    }
}
