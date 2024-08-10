use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler, predicate_compiler::PredicateCompiler,
};
use crate::problem::MarzanoQueryContext;
use grit_pattern_matcher::pattern::{If, PrIf};
use grit_util::error::{GritPatternError, GritResult};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct IfCompiler;

impl NodeCompiler for IfCompiler {
    type TargetPattern = If<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> GritResult<Self::TargetPattern> {
        let if_ = node
            .child_by_field_name("if")
            .ok_or_else(|| GritPatternError::new("missing condition of if"))?;
        let if_ = PredicateCompiler::from_node(&if_, context)?;
        let then = node
            .child_by_field_name("then")
            .ok_or_else(|| GritPatternError::new("missing consequence of if"))?;
        let then = PatternCompiler::from_node(&then, context)?;
        let else_ = node
            .child_by_field_name("else")
            .map(|e| PatternCompiler::from_node(&e, context))
            .transpose()?;
        Ok(If::new(if_, then, else_))
    }
}

pub(crate) struct PrIfCompiler;

impl NodeCompiler for PrIfCompiler {
    type TargetPattern = PrIf<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> GritResult<Self::TargetPattern> {
        let if_ = node
            .child_by_field_name("if")
            .ok_or_else(|| GritPatternError::new("missing condition of if"))?;
        let if_ = PredicateCompiler::from_node(&if_, context)?;
        let then = node
            .child_by_field_name("then")
            .ok_or_else(|| GritPatternError::new("missing consequence of if"))?;
        let then = PredicateCompiler::from_node(&then, context)?;
        let else_ = node
            .child_by_field_name("else")
            .map(|e| PredicateCompiler::from_node(&e, context))
            .transpose()?;
        Ok(PrIf::new(if_, then, else_))
    }
}
