use super::{
    patterns::{Matcher, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::ProblemContext;
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

impl<P: ProblemContext> Matcher<P> for FloatConstant {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a, P>,
        _context: &'a P::ExecContext<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let text = binding.text(&state.files)?;
        let parsed_double = text.parse::<f64>()?;
        Ok(parsed_double == self.value)
    }
}
