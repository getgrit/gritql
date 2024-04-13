use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::pattern::before::Before;
use crate::problem::MarzanoProblemContext;
use anyhow::{anyhow, Result};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct BeforeCompiler;

impl NodeCompiler for BeforeCompiler {
    type TargetPattern = Before<MarzanoProblemContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let pattern = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern of patternBefore"))?;
        let pattern = PatternCompiler::from_node(&pattern, context)?;
        Ok(Before::new(pattern))
    }
}
