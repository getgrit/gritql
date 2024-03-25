use super::{
    patterns::{Matcher, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::Context;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct FilePattern {
    pub name: Pattern,
    pub body: Pattern,
}

impl FilePattern {
    pub fn new(name: Pattern, body: Pattern) -> Self {
        Self { name, body }
    }
}

impl Matcher for FilePattern {
    fn execute<'a>(
        &'a self,
        resolved_pattern: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
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
