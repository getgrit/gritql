use super::{
    patterns::{Matcher, Pattern, PatternName},
    PatternDefinition, State,
};
use crate::context::QueryContext;
use grit_util::{error::GritResult, AnalysisLogs};

#[derive(Debug, Clone)]
pub struct Bubble<Q: QueryContext> {
    pub pattern_def: PatternDefinition<Q>,
    pub args: Vec<Option<Pattern<Q>>>,
}

impl<Q: QueryContext> Bubble<Q> {
    pub fn new(pattern_def: PatternDefinition<Q>, args: Vec<Pattern<Q>>) -> Self {
        Self {
            pattern_def,
            args: args.into_iter().map(Some).collect(),
        }
    }
}

impl<Q: QueryContext> PatternName for Bubble<Q> {
    fn name(&self) -> &'static str {
        "BUBBLE"
    }
}

impl<Q: QueryContext> Matcher<Q> for Bubble<Q> {
    fn execute<'a, 'b>(
        &'b self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs
    ) -> GritResult<bool> {
    self.pattern_def
            .call(state, binding, context, logs, &self.args) }
}
