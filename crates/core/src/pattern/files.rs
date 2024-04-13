use super::{
    patterns::{Matcher, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::ProblemContext;
use anyhow::Result;
use im::vector;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Files<P: ProblemContext> {
    pub pattern: Pattern<P>,
}

impl<P: ProblemContext> Files<P> {
    pub fn new(pattern: Pattern<P>) -> Self {
        Self { pattern }
    }
}

impl<P: ProblemContext> Matcher<P> for Files<P> {
    fn execute<'a>(
        &'a self,
        resolved_pattern: &ResolvedPattern<'a>,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
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
