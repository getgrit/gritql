use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::ProblemContext;
use anyhow::Result;
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Any<P: ProblemContext> {
    pub(crate) patterns: Vec<Pattern<P>>,
}

impl<P: ProblemContext> Any<P> {
    pub fn new(patterns: Vec<Pattern<P>>) -> Self {
        Self { patterns }
    }
}

impl<P: ProblemContext> PatternName for Any<P> {
    fn name(&self) -> &'static str {
        "ANY"
    }
}

impl<P: ProblemContext> Matcher<P> for Any<P> {
    // apply all successful updates to the state
    // must have at least one successful match
    // return soft and failed on failure
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
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
pub struct PrAny<P: ProblemContext> {
    pub predicates: Vec<Predicate<P>>,
}

impl<P: ProblemContext> PrAny<P> {
    pub fn new(predicates: Vec<Predicate<P>>) -> Self {
        Self { predicates }
    }
}

impl<P: ProblemContext> PatternName for PrAny<P> {
    fn name(&self) -> &'static str {
        "PREDICATE_ANY"
    }
}

impl<P: ProblemContext> Evaluator<P> for PrAny<P> {
    fn execute_func<'a>(
        &'a self,
        init_state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
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
