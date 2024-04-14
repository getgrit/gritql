use super::{
    patterns::{Matcher, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::QueryContext;
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

impl PatternName for BooleanConstant {
    fn name(&self) -> &'static str {
        "BOOLEAN_CONSTANT"
    }
}

impl<Q: QueryContext> Matcher<Q> for BooleanConstant {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        _context: &'a Q::ExecContext<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        binding
            .is_truthy(state)
            .map(|truthiness| truthiness == self.value)
    }
}
