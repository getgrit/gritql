use super::{
    patterns::{Matcher, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::QueryContext;
use anyhow::Result;
use im::vector;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Files<Q: QueryContext> {
    pub pattern: Pattern<Q>,
}

impl<Q: QueryContext> Files<Q> {
    pub fn new(pattern: Pattern<Q>) -> Self {
        Self { pattern }
    }
}

impl<Q: QueryContext> Matcher<Q> for Files<Q> {
    fn execute<'a>(
        &'a self,
        resolved_pattern: &ResolvedPattern<'a, Q>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        match resolved_pattern {
            ResolvedPattern::Files(files) => self.pattern.execute(files, state, context, logs),
            ResolvedPattern::File(_) => {
                let files = ResolvedPattern::List(vector![resolved_pattern.to_owned()]);
                self.pattern.execute(&files, state, context, logs)
            }
            ResolvedPattern::Binding(_)
            | ResolvedPattern::Snippets(_)
            | ResolvedPattern::List(_)
            | ResolvedPattern::Map(_)
            | ResolvedPattern::Constant(_) => Ok(false),
        }
    }
}
