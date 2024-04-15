use super::{
    patterns::{Matcher, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::QueryContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

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
        binding: &ResolvedPattern<'a, Q>,
        state: &mut State<'a, Q>,
        _context: &'a Q::ExecContext<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let text = binding.text(&state.files)?;
        let parsed_double = text.parse::<f64>()?;
        Ok(parsed_double == self.value)
    }
}
