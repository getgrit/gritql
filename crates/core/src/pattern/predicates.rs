use super::{
    accumulate::Accumulate,
    and::PrAnd,
    any::PrAny,
    assignment::Assignment,
    call::PrCall,
    compiler::CompilationContext,
    equal::Equal,
    functions::{Evaluator, FuncEvaluation},
    log::Log,
    maybe::PrMaybe,
    not::PrNot,
    or::PrOr,
    patterns::Name,
    predicate_return::PrReturn,
    r#if::PrIf,
    r#match::Match,
    rewrite::Rewrite,
    variable::VariableSourceLocations,
    State,
};
use crate::context::Context;
use anyhow::{anyhow, bail, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub enum Predicate {
    Call(Box<PrCall>),
    Not(Box<PrNot>),
    If(Box<PrIf>),
    True,
    False,
    Or(Box<PrOr>),
    And(Box<PrAnd>),
    Maybe(Box<PrMaybe>),
    Any(Box<PrAny>),
    Rewrite(Box<Rewrite>),
    Log(Log),
    Match(Box<Match>),
    Equal(Box<Equal>),
    Assignment(Box<Assignment>),
    Accumulate(Box<Accumulate>),
    Return(Box<PrReturn>),
}

impl Predicate {
    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
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
            "predicateAnd" => PrAnd::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            ),
            "predicateOr" => PrOr::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            ),
            "predicateMaybe" => Ok(Predicate::Maybe(Box::new(PrMaybe::maybe_from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "predicateAny" => PrAny::from_node(
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
            "predicateRewrite" => Ok(Predicate::Rewrite(Box::new(Rewrite::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "log" => Ok(Predicate::Log(Log::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?)),
            "predicateMatch" => Ok(Predicate::Match(Box::new(Match::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "predicateEqual" => Ok(Predicate::Equal(Box::new(Equal::from_node(
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
            "predicateAssignment" => Ok(Predicate::Assignment(Box::new(Assignment::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "predicateAccumulate" => Ok(Predicate::Accumulate(Box::new(Accumulate::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "predicateReturn" => Ok(Predicate::Return(Box::new(PrReturn::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            _ => bail!("unknown predicate kind: {}", kind),
        }
    }
}

impl Name for Predicate {
    fn name(&self) -> &'static str {
        match self {
            Predicate::Call(call) => call.name(),
            Predicate::Not(not) => not.name(),
            Predicate::If(if_) => if_.name(),
            Predicate::True => "TRUE",
            Predicate::False => "FALSE",
            Predicate::Or(or) => or.name(),
            Predicate::And(and) => and.name(),
            Predicate::Maybe(maybe) => maybe.name(),
            Predicate::Any(any) => any.name(),
            Predicate::Rewrite(rewrite) => rewrite.name(),
            Predicate::Log(log) => log.name(),
            Predicate::Match(match_) => match_.name(),
            Predicate::Equal(equal) => equal.name(),
            Predicate::Assignment(assignment) => assignment.name(),
            Predicate::Accumulate(accumulate) => accumulate.name(),
            Predicate::Return(return_) => return_.name(),
        }
    }
}

impl Evaluator for Predicate {
    async fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        match self {
            Predicate::Call(call) => Box::pin(call.execute_func(state, context, logs)).await,
            Predicate::Or(or) => Box::pin(or.execute_func(state, context, logs)).await,
            Predicate::And(and) => Box::pin(and.execute_func(state, context, logs)).await,
            Predicate::Maybe(maybe) => Box::pin(maybe.execute_func(state, context, logs)).await,
            Predicate::Any(any) => Box::pin(any.execute_func(state, context, logs)).await,
            Predicate::Rewrite(rewrite) => {
                Box::pin(rewrite.execute_func(state, context, logs)).await
            }
            Predicate::Log(log) => Box::pin(log.execute_func(state, context, logs)).await,
            Predicate::Match(match_) => Box::pin(match_.execute_func(state, context, logs)).await,
            Predicate::Equal(equal) => Box::pin(equal.execute_func(state, context, logs)).await,
            Predicate::True => Ok(FuncEvaluation {
                predicator: true,
                ret_val: None,
            }),
            Predicate::False => Ok(FuncEvaluation {
                predicator: false,
                ret_val: None,
            }),
            Predicate::Not(not) => Box::pin(not.execute_func(state, context, logs)).await,
            Predicate::If(if_) => Box::pin(if_.execute_func(state, context, logs)).await,
            Predicate::Assignment(assignment) => {
                Box::pin(assignment.execute_func(state, context, logs)).await
            }
            Predicate::Accumulate(accumulate) => {
                Box::pin(accumulate.execute_func(state, context, logs)).await
            }
            Predicate::Return(return_) => {
                Box::pin(return_.execute_func(state, context, logs)).await
            }
        }
    }
}
