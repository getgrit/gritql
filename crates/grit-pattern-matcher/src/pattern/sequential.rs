use super::{
    patterns::{Matcher, PatternName},
    state::State,
    step::Step,
};
use crate::context::QueryContext;
use anyhow::Result;
use grit_util::AnalysisLogs;
use std::ops;

#[derive(Debug, Clone)]
pub struct Sequential<Q: QueryContext>(pub Vec<Step<Q>>);

impl<Q: QueryContext> Matcher<Q> for Sequential<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        for step in &self.0 {
            if !step.execute(binding, state, context, logs)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

impl<Q: QueryContext> From<Vec<Step<Q>>> for Sequential<Q> {
    fn from(logs: Vec<Step<Q>>) -> Self {
        Self(logs)
    }
}

impl<Q: QueryContext> ops::Deref for Sequential<Q> {
    type Target = Vec<Step<Q>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<Q: QueryContext> PatternName for Sequential<Q> {
    fn name(&self) -> &'static str {
        "SEQUENTIAL"
    }
}
