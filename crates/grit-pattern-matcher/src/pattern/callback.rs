use super::{
    patterns::{Matcher, PatternName}, State,
};

use crate::context::QueryContext;
use anyhow::Result;
use grit_util::AnalysisLogs;

/// Callback holds a flexible function callback which will be used for evaluation
/// This is meant to allow for arbitrary functions to be called during pattern matching
#[derive(Clone, Debug)]
pub struct Callback {
    index: usize,
}

impl Callback {
    pub fn new(index: usize) -> Self {
        Self { index }
    }
}

impl PatternName for Callback {
    fn name(&self) -> &'static str {
        "CALLBACK"
    }
}

impl<Q: QueryContext> Matcher<Q> for Callback {
    fn execute<'a>(
        &'a self,
        _binding: &Q::ResolvedPattern<'a>,
        _state: &mut State<'a, Q>,
        _context: &'a Q::ExecContext<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        // context.call_built_in(self, context, state, logs)?;
        // let result = (self.callback)(binding, state, context, logs)?;
        Ok(true)
    }
}
