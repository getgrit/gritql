use super::{Matcher, PatternName, State};
use crate::context::ExecContext;
use crate::context::QueryContext;
use grit_util::{error::GritResult, AnalysisLogs};
use std::fmt::Debug;

/// A callback pattern is used to reference against a callback function.
/// The actual callback function is stored in the context, just the index is provided here.
/// This is useful for adding new user-defined functions in Rust.
pub struct CallbackPattern {
    pub callback_index: usize,
}

impl Clone for CallbackPattern {
    fn clone(&self) -> Self {
        Self {
            callback_index: self.callback_index,
        }
    }
}

impl CallbackPattern {
    pub fn new(callback_index: usize) -> Self {
        Self { callback_index }
    }
}

impl PatternName for CallbackPattern {
    fn name(&self) -> &'static str {
        "CALLBACK_PATTERN"
    }
}

impl Debug for CallbackPattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CallbackPattern")
    }
}

impl<Q: QueryContext> Matcher<Q> for CallbackPattern {
    fn execute<'a, 'b>(
        &'b self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs
    ) -> GritResult<bool> {
    context.call_callback(self, context, binding, state, logs) }
}
