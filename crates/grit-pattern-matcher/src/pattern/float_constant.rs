use super::{
    patterns::{Matcher, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::{ExecContext, QueryContext};
use anyhow::Result;
use grit_util::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct FloatConstant {
    pub value: f64,
}

impl FloatConstant {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl PatternName for FloatConstant {
    fn name(&self) -> &'static str {
        "DOUBLE_CONSTANT"
    }
}

impl<Q: QueryContext> Matcher<Q> for FloatConstant {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let text = binding.text(&state.files, context.language())?;
        let parsed_double = text.parse::<f64>()?;
        Ok(parsed_double == self.value)
    }
}
