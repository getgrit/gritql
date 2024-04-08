use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler, predicate_compiler::PredicateCompiler,
};
use crate::pattern::{
    iter_pattern::PatternOrPredicate,
    not::{Not, PrNot},
    patterns::Pattern,
    predicates::Predicate,
};
use anyhow::{anyhow, Result};
use marzano_util::{
    analysis_logs::AnalysisLogBuilder, node_with_source::NodeWithSource, position::Range,
};

pub(crate) struct NotCompiler;

impl NodeCompiler for NotCompiler {
    type TargetPattern = Not;

    fn from_node_with_rhs(
        node: NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let pattern = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern of patternNot"))?;
        let range: Range = pattern.range().into();
        let pattern = PatternCompiler::from_node(pattern, context)?;
        if pattern.iter().any(|p| {
            matches!(
                p,
                PatternOrPredicate::Pattern(Pattern::Rewrite(_))
                    | PatternOrPredicate::Predicate(Predicate::Rewrite(_))
            )
        }) {
            let log = AnalysisLogBuilder::default()
                .level(441_u16)
                .file(context.compilation.file)
                .source(node.source)
                .position(range.start)
                .range(range)
                .message("Warning: rewrites inside of a not will never be applied")
                .build()?;
            context.logs.push(log);
        }
        Ok(Not::new(pattern))
    }
}

pub(crate) struct PrNotCompiler;

impl NodeCompiler for PrNotCompiler {
    type TargetPattern = PrNot;

    fn from_node_with_rhs(
        node: NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let not = node
            .child_by_field_name("predicate")
            .ok_or_else(|| anyhow!("predicateNot missing predicate"))?;
        let range: Range = not.range().into();
        let not = PredicateCompiler::from_node(not, context)?;
        if not.iter().any(|p| {
            matches!(
                p,
                PatternOrPredicate::Pattern(Pattern::Rewrite(_))
                    | PatternOrPredicate::Predicate(Predicate::Rewrite(_))
            )
        }) {
            let log = AnalysisLogBuilder::default()
                .level(441_u16)
                .file(context.compilation.file)
                .source(node.source)
                .position(range.start)
                .range(range)
                .message("Warning: rewrites inside of a not will never be applied")
                .build()?;
            context.logs.push(log);
        }
        Ok(PrNot::new(not))
    }
}
