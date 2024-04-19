use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    predicates::Predicate,
    state::State,
};
use crate::context::QueryContext;
use anyhow::Result;
use grit_util::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Maybe<Q: QueryContext> {
    pub pattern: Pattern<Q>,
}

impl<Q: QueryContext> Maybe<Q> {
    pub fn new(pattern: Pattern<Q>) -> Self {
        Self { pattern }
    }
}

impl<Q: QueryContext> Matcher<Q> for Maybe<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let mut state = init_state.clone();
        if self.pattern.execute(binding, &mut state, context, logs)? {
            *init_state = state;
        }
        Ok(true)
    }
}

impl<Q: QueryContext> PatternName for Maybe<Q> {
    fn name(&self) -> &'static str {
        "MAYBE"
    }
}

#[derive(Debug, Clone)]
pub struct PrMaybe<Q: QueryContext> {
    pub(crate) predicate: Predicate<Q>,
}

impl<Q: QueryContext> PrMaybe<Q> {
    pub fn new(predicate: Predicate<Q>) -> Self {
        Self { predicate }
    }
}

impl<Q: QueryContext> Evaluator<Q> for PrMaybe<Q> {
    fn execute_func<'a>(
        &'a self,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation<Q>> {
        let mut state = init_state.clone();
        if self
            .predicate
            .execute_func(&mut state, context, logs)?
            .predicator
        {
            *init_state = state;
        }
        Ok(FuncEvaluation {
            predicator: true,
            ret_val: None,
        })
    }
}

impl<Q: QueryContext> PatternName for PrMaybe<Q> {
    fn name(&self) -> &'static str {
        "MAYBE"
    }
}
