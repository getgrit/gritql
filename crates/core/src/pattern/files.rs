use super::{
    patterns::{Matcher, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::Context;
use anyhow::Result;
use im::vector;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Files {
    pub pattern: Pattern,
}

impl Files {
    pub fn new(pattern: Pattern) -> Self {
        Self { pattern }
    }
}

impl Matcher for Files {
    fn execute<'a>(
        &'a self,
        resolved_pattern: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
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
