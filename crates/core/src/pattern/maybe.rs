use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::ProblemContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Maybe<P: ProblemContext> {
    pub pattern: Pattern<P>,
}

impl<P: ProblemContext> Maybe<P> {
    pub fn new(pattern: Pattern<P>) -> Self {
        Self { pattern }
    }
}

impl<P: ProblemContext> Matcher<P> for Maybe<P> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let mut state = init_state.clone();
        if self.pattern.execute(binding, &mut state, context, logs)? {
            *init_state = state;
        }
        Ok(true)
    }
}

impl<P: ProblemContext> PatternName for Maybe<P> {
    fn name(&self) -> &'static str {
        "MAYBE"
    }
}

#[derive(Debug, Clone)]
pub struct PrMaybe<P: ProblemContext> {
    pub(crate) predicate: Predicate<P>,
}

impl<P: ProblemContext> PrMaybe<P> {
    pub fn new(predicate: Predicate<P>) -> Self {
        Self { predicate }
    }
}

impl<P: ProblemContext> Evaluator<P> for PrMaybe<P> {
    fn execute_func<'a>(
        &'a self,
        init_state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
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

impl<P: ProblemContext> PatternName for PrMaybe<P> {
    fn name(&self) -> &'static str {
        "MAYBE"
    }
}
