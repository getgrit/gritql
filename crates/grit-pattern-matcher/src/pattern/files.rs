use super::{
    patterns::{Matcher, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::QueryContext;
use grit_util::{error::GritResult, AnalysisLogs};

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
        resolved_pattern: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        if let Some(files) = resolved_pattern.get_files() {
            self.pattern.execute(files, state, context, logs)
        } else if resolved_pattern.get_file().is_some() {
            let files = ResolvedPattern::from_list_parts([resolved_pattern.to_owned()].into_iter());
            self.pattern.execute(&files, state, context, logs)
        } else {
            Ok(false)
        }
    }
}
