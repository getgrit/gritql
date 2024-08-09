use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler, predicate_compiler::PredicateCompiler,
};
use crate::problem::MarzanoQueryContext;
use anyhow::{anyhow, Result};
use grit_pattern_matcher::pattern::{Maybe, PrMaybe};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct MaybeCompiler;

impl NodeCompiler for MaybeCompiler {
    type TargetPattern = Maybe<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let pattern = node
            .child_by_field_name("pattern")
            .ok_or_else(|| GritPatternError::new("missing pattern of patternMaybe"))?;
        let pattern = PatternCompiler::from_node(&pattern, context)?;
        Ok(Maybe::new(pattern))
    }
}

pub(crate) struct PrMaybeCompiler;

impl NodeCompiler for PrMaybeCompiler {
    type TargetPattern = PrMaybe<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let predicate = node
            .child_by_field_name("predicate")
            .ok_or_else(|| GritPatternError::new("missing predicate of predicateMaybe"))?;
        let predicate = PredicateCompiler::from_node(&predicate, context)?;
        Ok(PrMaybe::new(predicate))
    }
}
