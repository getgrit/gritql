use super::{
    patterns::{Matcher, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::QueryContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct IntConstant {
    pub value: i64,
}

impl IntConstant {
    pub fn new(value: i64) -> Self {
        Self { value }
    }
}

impl PatternName for IntConstant {
    fn name(&self) -> &'static str {
        "INT_CONSTANT"
    }
}

impl<Q: QueryContext> Matcher<Q> for IntConstant {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        _context: &'a Q::ExecContext<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let text = binding.text(&state.files)?;
        let parsed_int = text.parse::<i64>()?;
        Ok(parsed_int == self.value)
    }
}
