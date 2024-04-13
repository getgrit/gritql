use super::{
    patterns::{Matcher, Name},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::Context;
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

impl Name for IntConstant {
    fn name(&self) -> &'static str {
        "INT_CONSTANT"
    }
}

impl Matcher for IntConstant {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        _context: &'a impl Context,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let text = binding.text(&state.files)?;
        let parsed_int = text.parse::<i64>()?;
        Ok(parsed_int == self.value)
    }
}
