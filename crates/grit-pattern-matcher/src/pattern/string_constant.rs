use super::{
    patterns::{Matcher, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::{ExecContext, QueryContext};
use core::fmt::Debug;
use grit_util::{error::GritResult, AnalysisLogs};

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
    ) -> GritResult<bool> {
        let text = binding.text(&state.files, context.language())?;
        if text == self.text {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
