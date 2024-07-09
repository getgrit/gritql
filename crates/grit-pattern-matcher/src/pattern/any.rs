use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    predicates::Predicate,
    State,
};
use crate::context::QueryContext;
use crate::errors::GritResult;
use core::fmt::Debug;
use grit_util::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Any<Q: QueryContext> {
    pub patterns: Vec<Pattern<Q>>,
}

impl<Q: QueryContext> Any<Q> {
    pub fn new(patterns: Vec<Pattern<Q>>) -> Self {
        Self { patterns }
    }
}

impl<Q: QueryContext> PatternName for Any<Q> {
    fn name(&self) -> &'static str {
        "ANY"
    }
}

impl<Q: QueryContext> Matcher<Q> for Any<Q> {
    // apply all successful updates to the state
    // must have at least one successful match
    // return soft and failed on failure
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        let mut matched = false;
        let mut cur_state = init_state.clone();
        for pattern in &self.patterns {
            let state = cur_state.clone();
            if pattern.execute(binding, &mut cur_state, context, logs)? {
                matched = true;
            } else {
                cur_state = state;
            }
        }
        if matched {
            *init_state = cur_state;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
#[derive(Debug, Clone)]
pub struct PrAny<Q: QueryContext> {
    pub predicates: Vec<Predicate<Q>>,
}

impl<Q: QueryContext> PrAny<Q> {
    pub fn new(predicates: Vec<Predicate<Q>>) -> Self {
        Self { predicates }
    }
}

impl<Q: QueryContext> PatternName for PrAny<Q> {
    fn name(&self) -> &'static str {
        "PREDICATE_ANY"
    }
}

impl<Q: QueryContext> Evaluator<Q> for PrAny<Q> {
    fn execute_func<'a>(
        &'a self,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<FuncEvaluation<Q>> {
        let mut matched = false;
        let mut cur_state = init_state.clone();
        for predicate in &self.predicates {
            let state = cur_state.clone();
            if predicate
                .execute_func(&mut cur_state, context, logs)?
                .predicator
            {
                matched = true;
            } else {
                cur_state = state;
            }
        }
        if matched {
            *init_state = cur_state;
            Ok(FuncEvaluation {
                predicator: true,
                ret_val: None,
            })
        } else {
            Ok(FuncEvaluation {
                predicator: false,
                ret_val: None,
            })
        }
    }
}
