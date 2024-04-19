use super::{resolved_pattern::ResolvedPattern, state::State};
use crate::context::QueryContext;
use anyhow::Result;
use grit_util::AnalysisLogs;

// Undefined is a pattern that matches when a *Grit variable* is undefined.
// It is *not* meant to match against a *JavaScript* `undefined` value.

pub(crate) struct Undefined {}

impl Undefined {
    pub(crate) fn execute<'a, Q: QueryContext>(
        binding: &Q::ResolvedPattern<'a>,
        _init_state: &mut State<'a, Q>,
        _context: &'a Q::ExecContext<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        Ok(binding.matches_undefined())
    }
}
