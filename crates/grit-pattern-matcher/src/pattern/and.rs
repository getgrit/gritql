use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    predicates::Predicate,
    State,
};
use crate::context::QueryContext;
use grit_util::{error::GritResult, AnalysisLogs};

#[derive(Debug, Clone)]
pub struct And<Q: QueryContext> {
    pub patterns: Vec<Pattern<Q>>,
}

impl<Q: QueryContext> And<Q> {
    pub fn new(patterns: Vec<Pattern<Q>>) -> Self {
        Self { patterns }
    }
}

impl<Q: QueryContext> PatternName for And<Q> {
    fn name(&self) -> &'static str {
        "AND"
    }
}

impl<Q: QueryContext> Matcher<Q> for And<Q> {
    fn execute<'a, 'b>(
        &'b self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs
    ) -> GritResult<bool> {
    for p in self.patterns.iter() {
            if !p.execute(binding, state, context, logs)? {
                return Ok(false);
            };
        }
        Ok(true) }
}

#[derive(Debug, Clone)]
pub struct PrAnd<Q: QueryContext> {
    pub predicates: Vec<Predicate<Q>>,
}

impl<Q: QueryContext> PrAnd<Q> {
    pub fn new(predicates: Vec<Predicate<Q>>) -> Self {
        Self { predicates }
    }
}

impl<Q: QueryContext> PatternName for PrAnd<Q> {
    fn name(&self) -> &'static str {
        "PREDICATE_AND"
    }
}

impl<Q: QueryContext> Evaluator<Q> for PrAnd<Q> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<FuncEvaluation<Q>> {
        for p in self.predicates.iter() {
            let res = p.execute_func(state, context, logs)?;
            match res.predicator {
                true => {}
                false => return Ok(res),
            };
        }
        Ok(FuncEvaluation {
            predicator: true,
            ret_val: None,
        })
    }
}
