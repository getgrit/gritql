use super::{
    accumulate_compiler::AccumulateCompiler, and_compiler::PrAndCompiler,
    any_compiler::PrAnyCompiler, assignment_compiler::AssignmentCompiler,
    call_compiler::PrCallCompiler, compiler::NodeCompilationContext, equal_compiler::EqualCompiler,
    if_compiler::PrIfCompiler, log_compiler::LogCompiler, match_compiler::MatchCompiler,
    maybe_compiler::PrMaybeCompiler, node_compiler::NodeCompiler, not_compiler::PrNotCompiler,
    or_compiler::PrOrCompiler, predicate_return_compiler::PredicateReturnCompiler,
    rewrite_compiler::RewriteCompiler,
};
use crate::pattern::predicates::Predicate;
use anyhow::{anyhow, bail, Result};
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct PredicateCompiler;

impl NodeCompiler for PredicateCompiler {
    type TargetPattern = Predicate;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let kind = node.node.kind();
        match kind.as_ref() {
            "predicateNot" => Ok(Predicate::Not(Box::new(PrNotCompiler::from_node(
                node, context,
            )?))),
            "predicateAnd" => PrAndCompiler::from_node(node, context),
            "predicateOr" => PrOrCompiler::from_node(node, context),
            "predicateMaybe" => Ok(Predicate::Maybe(Box::new(PrMaybeCompiler::from_node(
                node, context,
            )?))),
            "predicateAny" => PrAnyCompiler::from_node(node, context),
            "predicateIfElse" => Ok(Predicate::If(Box::new(PrIfCompiler::from_node(
                node, context,
            )?))),
            "predicateRewrite" => Ok(Predicate::Rewrite(Box::new(RewriteCompiler::from_node(
                node, context,
            )?))),
            "log" => Ok(Predicate::Log(LogCompiler::from_node(node, context)?)),
            "predicateMatch" => Ok(Predicate::Match(Box::new(MatchCompiler::from_node(
                node, context,
            )?))),
            "predicateEqual" => Ok(Predicate::Equal(Box::new(EqualCompiler::from_node(
                node, context,
            )?))),
            "predicateCall" => Ok(Predicate::Call(Box::new(PrCallCompiler::from_node(
                node, context,
            )?))),
            "booleanConstant" => match node.text().trim() {
                "true" => Ok(Predicate::True),
                "false" => Ok(Predicate::False),
                _ => Err(anyhow!("invalid booleanConstant")),
            },
            "predicateAssignment" => Ok(Predicate::Assignment(Box::new(
                AssignmentCompiler::from_node(node, context)?,
            ))),
            "predicateAccumulate" => Ok(Predicate::Accumulate(Box::new(
                AccumulateCompiler::from_node(node, context)?,
            ))),
            "predicateReturn" => Ok(Predicate::Return(Box::new(
                PredicateReturnCompiler::from_node(node, context)?,
            ))),
            _ => bail!("unknown predicate kind: {}", kind),
        }
    }
}
