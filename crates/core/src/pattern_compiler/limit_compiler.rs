use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::pattern::{limit::Limit, patterns::Pattern};
use anyhow::{anyhow, Result};
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct LimitCompiler;

impl NodeCompiler for LimitCompiler {
    type TargetPattern = Pattern;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let body = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern in limit"))?;
        let body = PatternCompiler::from_node(&body, context)?;
        let limit = node
            .child_by_field_name("limit")
            .ok_or_else(|| anyhow!("missing limit in limit"))?;
        let limit = limit.text().trim().parse::<usize>()?;
        Ok(Pattern::Limit(Box::new(Limit::new(body, limit))))
    }
}
