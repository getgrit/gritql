use super::{resolved_pattern::ResolvedPattern, state::State};
use crate::context::QueryContext;
use grit_util::{error::GritResult, AnalysisLogs};

// Undefined is a pattern that matches when a *Grit variable* is undefined.
// It is *not* meant to match against a *JavaScript* `undefined` value.

pub struct Undefined {}

impl Undefined {
    pub fn execute<'a, Q: QueryContext>(
        binding: &Q::ResolvedPattern<'a>,
        _init_state: &mut State<'a, Q>,
        _context: &'a Q::ExecContext<'a>,
        _logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        Ok(binding.matches_undefined())
    }
}
