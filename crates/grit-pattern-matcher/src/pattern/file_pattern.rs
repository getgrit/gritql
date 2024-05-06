use std::error::Error;

use super::{
    patterns::{Matcher, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::{
    constants::{GLOBAL_VARS_SCOPE_INDEX, PROGRAM_INDEX},
    context::ExecContext,
};
use crate::{context::QueryContext, pattern::resolved_pattern::File};
use anyhow::Result;
use grit_util::AnalysisLogs;

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
        resolved_pattern: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let Some(file) = resolved_pattern.get_file() else {
            return Ok(false);
        };

        let name = file.name(&state.files);

        if !self.name.execute(&name, state, context, logs)? {
            return Ok(false);
        }

        // If the file isn't loaded yet, we must load it now
        if !context.load_file(file, state, logs)? {
            // The file wasn't loaded, so we can't match the body
            return Ok(false);
        }

        // Fill in the program variable now
        state.bindings[GLOBAL_VARS_SCOPE_INDEX].back_mut().unwrap()[PROGRAM_INDEX].value =
            Some(file.binding(&state.files));

        if !self
            .body
            .execute(&file.binding(&state.files), state, context, logs)?
        {
            return Ok(false);
        }

        Ok(true)
    }
}
