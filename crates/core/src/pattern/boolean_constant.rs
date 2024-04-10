use super::{
    patterns::{Matcher, Name},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::Context;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct BooleanConstant {
    pub value: bool,
}

impl BooleanConstant {
    pub fn new(value: bool) -> Self {
        Self { value }
    }
}

impl Name for BooleanConstant {
    fn name(&self) -> &'static str {
        "BOOLEAN_CONSTANT"
    }
}

impl Matcher for BooleanConstant {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        _context: &'a impl Context,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        binding
            .is_truthy(state)
            .map(|truthiness| truthiness == self.value)
    }
}
