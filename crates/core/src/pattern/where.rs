use super::{
    functions::Evaluator,
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
pub struct Where<P: ProblemContext> {
    pub(crate) pattern: Pattern<P>,
    pub(crate) side_condition: Predicate<P>,
}

impl<P: ProblemContext> Where<P> {
    pub fn new(pattern: Pattern<P>, side_condition: Predicate<P>) -> Self {
        Self {
            pattern,
            side_condition,
        }
    }
}

impl<P: ProblemContext> PatternName for Where<P> {
    fn name(&self) -> &'static str {
        "WHERE"
    }
}

impl<P: ProblemContext> Matcher<P> for Where<P> {
    // order here is pattern then side condition, do we prefer side condition then pattern?
    // should the state be reset on failure?
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let mut cur_state = init_state.clone();
        if !self
            .pattern
            .execute(binding, &mut cur_state, context, logs)?
        {
            return Ok(false);
        }
        if self
            .side_condition
            .execute_func(&mut cur_state, context, logs)?
            .predicator
        {
            *init_state = cur_state;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
