use super::{
    patterns::{Matcher, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::ProblemContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct FilePattern<P: ProblemContext> {
    pub name: Pattern<P>,
    pub body: Pattern<P>,
}

impl<P: ProblemContext> FilePattern<P> {
    pub fn new(name: Pattern<P>, body: Pattern<P>) -> Self {
        Self { name, body }
    }
}

impl<P: ProblemContext> Matcher<P> for FilePattern<P> {
    fn execute<'a>(
        &'a self,
        resolved_pattern: &ResolvedPattern<'a>,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        match resolved_pattern {
            ResolvedPattern::File(file) => {
                if !self
                    .name
                    .execute(&file.name(&state.files), state, context, logs)?
                {
                    return Ok(false);
                }
                if !self
                    .body
                    .execute(&file.binding(&state.files), state, context, logs)?
                {
                    return Ok(false);
                }
                Ok(true)
            }
            ResolvedPattern::Binding(_)
            | ResolvedPattern::Snippets(_)
            | ResolvedPattern::List(_)
            | ResolvedPattern::Map(_)
            | ResolvedPattern::Files(_)
            | ResolvedPattern::Constant(_) => Ok(false),
        }
    }
}
