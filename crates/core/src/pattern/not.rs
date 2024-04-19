use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    predicates::Predicate,
    State,
};
use crate::context::QueryContext;
use anyhow::{bail, Ok, Result};
use core::fmt::Debug;
use grit_util::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Not<Q: QueryContext> {
    pub pattern: Pattern<Q>,
}

impl<Q: QueryContext> Not<Q> {
    pub fn new(pattern: Pattern<Q>) -> Self {
        Self { pattern }
    }
}

impl<Q: QueryContext> PatternName for Not<Q> {
    fn name(&self) -> &'static str {
        "NOT"
    }
}

impl<Q: QueryContext> Matcher<Q> for Not<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        Ok(!self
            .pattern
            .execute(binding, &mut state.clone(), context, logs)?)
    }
}

#[derive(Debug, Clone)]
pub struct PrNot<Q: QueryContext> {
    pub(crate) predicate: Predicate<Q>,
}

impl<Q: QueryContext> PrNot<Q> {
    pub fn new(predicate: Predicate<Q>) -> Self {
        Self { predicate }
    }
}

impl<Q: QueryContext> PatternName for PrNot<Q> {
    fn name(&self) -> &'static str {
        "PREDICATE_NOT"
    }
}

impl<Q: QueryContext> Evaluator<Q> for PrNot<Q> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation<Q>> {
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
