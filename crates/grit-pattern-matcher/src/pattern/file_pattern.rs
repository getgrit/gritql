use super::{
    patterns::{Matcher, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::{
    constants::{ABSOLUTE_PATH_INDEX, FILENAME_INDEX, GLOBAL_VARS_SCOPE_INDEX, PROGRAM_INDEX},
    context::ExecContext,
};
use crate::{context::QueryContext, pattern::resolved_pattern::File};
use grit_util::{error::GritResult, AnalysisLogs};

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
    ) -> GritResult<bool> {
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

        // Re-execute the name pattern to bind the name variable
        self.name
            .execute(&file.name(&state.files), state, context, logs)?;

        // Fill in the variables now - this is a bit of a hack
        state.bindings[GLOBAL_VARS_SCOPE_INDEX.into()]
            .back_mut()
            .unwrap()[PROGRAM_INDEX]
        .value = Some(file.binding(&state.files));
        state.bindings[GLOBAL_VARS_SCOPE_INDEX.into()]
            .back_mut()
            .unwrap()[FILENAME_INDEX]
        .value = Some(file.name(&state.files));
        state.bindings[GLOBAL_VARS_SCOPE_INDEX.into()]
            .back_mut()
            .unwrap()[ABSOLUTE_PATH_INDEX]
            .value = Some(file.name(&state.files));
        state.bindings[GLOBAL_VARS_SCOPE_INDEX.into()]
            .back_mut()
            .unwrap()[ABSOLUTE_PATH_INDEX]
            .value = Some(file.absolute_path(&state.files, context.language())?);

        if !self
            .body
            .execute(&file.binding(&state.files), state, context, logs)?
        {
            return Ok(false);
        }

        Ok(true)
    }
}
