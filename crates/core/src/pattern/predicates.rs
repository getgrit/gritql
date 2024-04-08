use super::{
    accumulate::Accumulate,
    and::PrAnd,
    any::PrAny,
    assignment::Assignment,
    call::PrCall,
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
    State,
};
use crate::context::Context;
use anyhow::Result;
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

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
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        match self {
            Predicate::Call(call) => call.execute_func(state, context, logs),
            Predicate::Or(or) => or.execute_func(state, context, logs),
            Predicate::And(and) => and.execute_func(state, context, logs),
            Predicate::Maybe(maybe) => maybe.execute_func(state, context, logs),
            Predicate::Any(any) => any.execute_func(state, context, logs),
            Predicate::Rewrite(rewrite) => rewrite.execute_func(state, context, logs),
            Predicate::Log(log) => log.execute_func(state, context, logs),
            Predicate::Match(match_) => match_.execute_func(state, context, logs),
            Predicate::Equal(equal) => equal.execute_func(state, context, logs),
            Predicate::True => Ok(FuncEvaluation {
                predicator: true,
                ret_val: None,
            }),
            Predicate::False => Ok(FuncEvaluation {
                predicator: false,
                ret_val: None,
            }),
            Predicate::Not(not) => not.execute_func(state, context, logs),
            Predicate::If(if_) => if_.execute_func(state, context, logs),
            Predicate::Assignment(assignment) => assignment.execute_func(state, context, logs),
            Predicate::Accumulate(accumulate) => accumulate.execute_func(state, context, logs),
            Predicate::Return(return_) => return_.execute_func(state, context, logs),
        }
    }
}
