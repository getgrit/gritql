use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler, predicate_compiler::PredicateCompiler,
};
use crate::{pattern::r#where::Where, problem::MarzanoProblemContext};
use anyhow::{anyhow, Result};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct WhereCompiler;

impl NodeCompiler for WhereCompiler {
    type TargetPattern = Where<MarzanoProblemContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let pattern = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern of patternWhere"))?;
        let pattern = PatternCompiler::from_node(&pattern, context)?;
        let side_condition = node
            .child_by_field_name("side_condition")
            .ok_or_else(|| anyhow!("missing side condition of patternWhere"))?;
        let side_condition = PredicateCompiler::from_node(&side_condition, context)?;
        Ok(Where::new(pattern, side_condition))
    }
}
