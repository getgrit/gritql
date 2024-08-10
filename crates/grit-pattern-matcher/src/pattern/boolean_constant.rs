use super::{
    patterns::{Matcher, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::{ExecContext, QueryContext};
use grit_util::{error::GritResult, AnalysisLogs};

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
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        _logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        binding
            .is_truthy(state, context.language())
            .map(|truthiness| truthiness == self.value)
    }
}
