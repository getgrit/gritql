use super::{
    patterns::{Matcher, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
    step::Step,
};
use crate::context::ProblemContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;
use std::ops;

#[derive(Debug, Clone)]
pub struct Sequential<P: ProblemContext>(pub Vec<Step<P>>);

impl<P: ProblemContext> Matcher<P> for Sequential<P> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
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

impl<P: ProblemContext> From<Vec<Step<P>>> for Sequential<P> {
    fn from(logs: Vec<Step<P>>) -> Self {
        Self(logs)
    }
}

impl<P: ProblemContext> ops::Deref for Sequential<P> {
    type Target = Vec<Step<P>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P: ProblemContext> PatternName for Sequential<P> {
    fn name(&self) -> &'static str {
        "SEQUENTIAL"
    }
}
