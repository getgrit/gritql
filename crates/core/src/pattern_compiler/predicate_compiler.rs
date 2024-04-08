use super::{
    accumulate_compiler::AccumulateCompiler, and_compiler::PrAndCompiler,
    any_compiler::PrAnyCompiler, assignment_compiler::AssignmentCompiler,
    compiler::CompilationContext, equal_compiler::EqualCompiler, log_compiler::LogCompiler,
    match_compiler::MatchCompiler, maybe_compiler::PrMaybeCompiler, node_compiler::NodeCompiler,
    or_compiler::PrOrCompiler, predicate_return_compiler::PredicateReturnCompiler,
    rewrite_compiler::RewriteCompiler,
};
use crate::pattern::{
    call::PrCall, not::PrNot, predicates::Predicate, r#if::PrIf, variable::VariableSourceLocations,
};
use anyhow::{anyhow, bail, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct PredicateCompiler;

impl NodeCompiler for PredicateCompiler {
    type TargetPattern = Predicate;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let kind = node.kind();
        let kind = kind.as_ref();
        match kind {
            "predicateNot" => Ok(Predicate::Not(Box::new(PrNot::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "predicateAnd" => PrAndCompiler::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            ),
            "predicateOr" => PrOrCompiler::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            ),
            "predicateMaybe" => Ok(Predicate::Maybe(Box::new(PrMaybeCompiler::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "predicateAny" => PrAnyCompiler::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            ),
            "predicateIfElse" => Ok(Predicate::If(Box::new(PrIf::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "predicateRewrite" => Ok(Predicate::Rewrite(Box::new(RewriteCompiler::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "log" => Ok(Predicate::Log(LogCompiler::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?)),
            "predicateMatch" => Ok(Predicate::Match(Box::new(MatchCompiler::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "predicateEqual" => Ok(Predicate::Equal(Box::new(EqualCompiler::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "predicateCall" => Ok(Predicate::Call(Box::new(PrCall::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "booleanConstant" => {
                let value = node.utf8_text(context.src.as_bytes())?;
                let value = value.trim();
                if value == "true" {
                    Ok(Predicate::True)
                } else if value == "false" {
                    Ok(Predicate::False)
                } else {
                    Err(anyhow!("invalid booleanConstant"))
                }
            }
            "predicateAssignment" => Ok(Predicate::Assignment(Box::new(
                AssignmentCompiler::from_node(
                    node,
                    context,
                    vars,
                    vars_array,
                    scope_index,
                    global_vars,
                    logs,
                )?,
            ))),
            "predicateAccumulate" => Ok(Predicate::Accumulate(Box::new(
                AccumulateCompiler::from_node(
                    node,
                    context,
                    vars,
                    vars_array,
                    scope_index,
                    global_vars,
                    logs,
                )?,
            ))),
            "predicateReturn" => Ok(Predicate::Return(Box::new(
                PredicateReturnCompiler::from_node(
                    node,
                    context,
                    vars,
                    vars_array,
                    scope_index,
                    global_vars,
                    logs,
                )?,
            ))),
            _ => bail!("unknown predicate kind: {}", kind),
        }
    }
}
