use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler, predicate_compiler::PredicateCompiler,
};
use crate::problem::MarzanoQueryContext;
use grit_pattern_matcher::pattern::{Any, Pattern, PrAny, Predicate};
use grit_util::error::GritResult;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct AnyCompiler;

impl NodeCompiler for AnyCompiler {
    type TargetPattern = Pattern<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> GritResult<Self::TargetPattern> {
        let mut patterns = node
            .named_children_by_field_name("patterns")
            .map(|pattern| PatternCompiler::from_node(&pattern, context))
            .collect::<GritResult<Vec<_>>>()?;
        if patterns.len() == 1 {
            Ok(patterns.remove(0))
        } else {
            Ok(Pattern::Any(Box::new(Any::new(patterns))))
        }
    }
}

pub(crate) struct PrAnyCompiler;

impl NodeCompiler for PrAnyCompiler {
    type TargetPattern = Predicate<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> GritResult<Self::TargetPattern> {
        let mut predicates = node
            .named_children_by_field_name("predicates")
            .map(|predicate| PredicateCompiler::from_node(&predicate, context))
            .collect::<GritResult<Vec<_>>>()?;
        if predicates.len() == 1 {
            Ok(predicates.remove(0))
        } else {
            Ok(Predicate::Any(Box::new(PrAny::new(predicates))))
        }
    }
}
