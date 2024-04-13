use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::ProblemContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct And<P: ProblemContext> {
    pub patterns: Vec<Pattern<P>>,
}

impl<P: ProblemContext> And<P> {
    pub fn new(patterns: Vec<Pattern<P>>) -> Self {
        Self { patterns }
    }
}

impl<P: ProblemContext> PatternName for And<P> {
    fn name(&self) -> &'static str {
        "AND"
    }
}

impl<P: ProblemContext> Matcher<P> for And<P> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        for p in self.patterns.iter() {
            if !p.execute(binding, state, context, logs)? {
                return Ok(false);
            };
        }
        Ok(true)
    }
}

#[derive(Debug, Clone)]
pub struct PrAnd<P: ProblemContext> {
    pub predicates: Vec<Predicate<P>>,
}

impl<P: ProblemContext> PrAnd<P> {
    pub fn new(predicates: Vec<Predicate<P>>) -> Self {
        Self { predicates }
    }
}

impl<P: ProblemContext> PatternName for PrAnd<P> {
    fn name(&self) -> &'static str {
        "PREDICATE_AND"
    }
}

impl<P: ProblemContext> Evaluator<P> for PrAnd<P> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
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
