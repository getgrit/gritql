use super::{
    patterns::{Matcher, Pattern},
    state::State,
};
use crate::context::{ExecContext, QueryContext};
use grit_util::{error::GritResult, AnalysisLogs};

#[derive(Debug, Clone)]
pub struct Step<Q: QueryContext> {
    pub pattern: Pattern<Q>,
}

impl<Q: QueryContext> Step<Q> {
    pub fn new(pattern: Pattern<Q>) -> Self {
        Self { pattern }
    }
}

impl<Q: QueryContext> Matcher<Q> for Step<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        context.exec_step(&self.pattern, binding, state, logs)
    }
}
