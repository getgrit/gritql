use super::{
    patterns::{Matcher, Name},
    resolved_pattern::ResolvedPattern,
    state::State,
    step::Step,
};
use crate::context::Context;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;
use std::ops;

#[derive(Debug, Clone)]
pub struct Sequential(pub Vec<Step>);

impl Matcher for Sequential {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
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

impl From<Vec<Step>> for Sequential {
    fn from(logs: Vec<Step>) -> Self {
        Self(logs)
    }
}

impl ops::Deref for Sequential {
    type Target = Vec<Step>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Name for Sequential {
    fn name(&self) -> &'static str {
        "SEQUENTIAL"
    }
}
