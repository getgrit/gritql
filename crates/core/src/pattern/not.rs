use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::ProblemContext;
use anyhow::{bail, Ok, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Not<P: ProblemContext> {
    pub pattern: Pattern<P>,
}

impl<P: ProblemContext> Not<P> {
    pub fn new(pattern: Pattern<P>) -> Self {
        Self { pattern }
    }
}

impl<P: ProblemContext> PatternName for Not<P> {
    fn name(&self) -> &'static str {
        "NOT"
    }
}

impl<P: ProblemContext> Matcher<P> for Not<P> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        Ok(!self
            .pattern
            .execute(binding, &mut state.clone(), context, logs)?)
    }
}

#[derive(Debug, Clone)]
pub struct PrNot<P: ProblemContext> {
    pub(crate) predicate: Predicate<P>,
}

impl<P: ProblemContext> PrNot<P> {
    pub fn new(predicate: Predicate<P>) -> Self {
        Self { predicate }
    }
}

impl<P: ProblemContext> PatternName for PrNot<P> {
    fn name(&self) -> &'static str {
        "PREDICATE_NOT"
    }
}

impl<P: ProblemContext> Evaluator<P> for PrNot<P> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let res = self
            .predicate
            .execute_func(&mut state.clone(), context, logs)?;
        if res.ret_val.is_some() {
            bail!("Cannot return from within not clause");
        }
        Ok(FuncEvaluation {
            predicator: !res.predicator,
            ret_val: None,
        })
    }
}
