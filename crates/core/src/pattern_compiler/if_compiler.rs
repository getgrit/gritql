use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler, predicate_compiler::PredicateCompiler,
};
use crate::pattern::r#if::{If, PrIf};
use anyhow::{anyhow, Result};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct IfCompiler;

impl NodeCompiler for IfCompiler {
    type TargetPattern = If;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let if_ = node
            .child_by_field_name("if")
            .ok_or_else(|| anyhow!("missing condition of if"))?;
        let if_ = PredicateCompiler::from_node(&if_, context)?;
        let then = node
            .child_by_field_name("then")
            .ok_or_else(|| anyhow!("missing consequence of if"))?;
        let then = PatternCompiler::from_node(&then, context)?;
        let else_ = node
            .child_by_field_name("else")
            .map(|e| PatternCompiler::from_node(&e, context))
            .map_or(Ok(None), |v| v.map(Some))?;
        Ok(If::new(if_, then, else_))
    }
}

pub(crate) struct PrIfCompiler;

impl NodeCompiler for PrIfCompiler {
    type TargetPattern = PrIf;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let if_ = node
            .child_by_field_name("if")
            .ok_or_else(|| anyhow!("missing condition of if"))?;
        let if_ = PredicateCompiler::from_node(&if_, context)?;
        let then = node
            .child_by_field_name("then")
            .ok_or_else(|| anyhow!("missing consequence of if"))?;
        let then = PredicateCompiler::from_node(&then, context)?;
        let else_ = node
            .child_by_field_name("else")
            .map(|e| PredicateCompiler::from_node(&e, context))
            .map_or(Ok(None), |v| v.map(Some))?;
        Ok(PrIf::new(if_, then, else_))
    }
}
