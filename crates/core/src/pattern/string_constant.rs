use super::{
    patterns::{Matcher, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{
    binding::Binding,
    context::{ExecContext, QueryContext},
};
use anyhow::Result;
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct StringConstant {
    pub text: String,
}

impl StringConstant {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

impl PatternName for StringConstant {
    fn name(&self) -> &'static str {
        "STRING_CONSTANT"
    }
}

// this does what a raw string should do
// TODO: rename this, and implement StringConstant that checks sort.
impl<Q: QueryContext> Matcher<Q> for StringConstant {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let text = binding.text(&state.files, context.language())?;
        if text == self.text {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
