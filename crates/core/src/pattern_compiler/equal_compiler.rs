use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::problem::MarzanoQueryContext;
use anyhow::{anyhow, Result};
use grit_core_patterns::pattern::{equal::Equal, patterns::Pattern};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct EqualCompiler;

impl NodeCompiler for EqualCompiler {
    type TargetPattern = Equal<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let variable = node
            .child_by_field_name("left")
            .ok_or_else(|| anyhow!("missing lhs of predicateEqual"))?;
        let variable = PatternCompiler::from_node_with_rhs(&variable, context, true)?;
        let pattern = node
            .child_by_field_name("right")
            .ok_or_else(|| anyhow!("missing rhs of predicateEqual"))?;
        let pattern = PatternCompiler::from_node_with_rhs(&pattern, context, true)?;
        if let Pattern::Variable(var) = variable {
            Ok(Equal::new(var, pattern))
        } else {
            Err(anyhow!(
                "predicateEqual must have a variable as first argument",
            ))
        }
    }
}
