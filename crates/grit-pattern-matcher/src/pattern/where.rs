use super::{
    functions::Evaluator,
    patterns::{Matcher, Pattern, PatternName},
    predicates::Predicate,
    State,
};
use crate::context::QueryContext;
use core::fmt::Debug;
use grit_util::{error::GritResult, AnalysisLogs};

#[derive(Debug, Clone)]
pub struct Where<Q: QueryContext> {
    pub pattern: Pattern<Q>,
    pub side_condition: Predicate<Q>,
}

impl<Q: QueryContext> Where<Q> {
    pub fn new(pattern: Pattern<Q>, side_condition: Predicate<Q>) -> Self {
        Self {
            pattern,
            side_condition,
        }
    }
}

impl<Q: QueryContext> PatternName for Where<Q> {
    fn name(&self) -> &'static str {
        "WHERE"
    }
}

impl<Q: QueryContext> Matcher<Q> for Where<Q> {
    // order here is pattern then side condition, do we prefer side condition then pattern?
    // should the state be reset on failure?
    fn execute<'a, 'b>(
        &'b self,
        binding: &Q::ResolvedPattern<'a>,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs
    ) -> GritResult<bool> {
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
        } }
}
