use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler, predicate_compiler::PredicateCompiler,
};
use crate::pattern::{
    or::{Or, PrOr},
    patterns::Pattern,
    predicates::Predicate,
};
use anyhow::Result;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct OrCompiler;

impl NodeCompiler for OrCompiler {
    type TargetPattern = Pattern;

    fn from_node_with_rhs(
        node: NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let mut patterns = node
            .named_children_by_field_name("patterns")
            .map(|pattern| PatternCompiler::from_node(pattern, context))
            .collect::<Result<Vec<_>>>()?;
        if patterns.len() == 1 {
            Ok(patterns.remove(0))
        } else {
            Ok(Pattern::Or(Box::new(Or::new(patterns))))
        }
    }
}

pub(crate) struct PrOrCompiler;

impl NodeCompiler for PrOrCompiler {
    type TargetPattern = Predicate;

    fn from_node_with_rhs(
        node: NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let mut predicates = node
            .named_children_by_field_name("predicates")
            .map(|predicate| PredicateCompiler::from_node(predicate, context))
            .collect::<Result<Vec<_>>>()?;
        if predicates.len() == 1 {
            Ok(predicates.remove(0))
        } else {
            Ok(Predicate::Or(Box::new(PrOr::new(predicates))))
        }
    }
}
