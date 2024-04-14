use super::{
    patterns::{Matcher, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::QueryContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct FilePattern<Q: QueryContext> {
    pub name: Pattern<Q>,
    pub body: Pattern<Q>,
}

impl<Q: QueryContext> FilePattern<Q> {
    pub fn new(name: Pattern<Q>, body: Pattern<Q>) -> Self {
        Self { name, body }
    }
}

impl<Q: QueryContext> Matcher<Q> for FilePattern<Q> {
    fn execute<'a>(
        &'a self,
        resolved_pattern: &ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
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
