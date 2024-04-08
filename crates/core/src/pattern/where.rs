use super::{
    functions::Evaluator,
    patterns::{Matcher, Name, Pattern},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::Context;
use anyhow::Result;
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Where {
    pub(crate) pattern: Pattern,
    pub(crate) side_condition: Predicate,
}

impl Where {
    pub fn new(pattern: Pattern, side_condition: Predicate) -> Self {
        Self {
            pattern,
            side_condition,
        }
    }
}

impl Name for Where {
    fn name(&self) -> &'static str {
        "WHERE"
    }
}

impl Matcher for Where {
    // order here is pattern then side condition, do we prefer side condition then pattern?
    // should the state be reset on failure?
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a>,
        context: &'a impl Context,
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
