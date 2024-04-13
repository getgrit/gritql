use super::{resolved_pattern::ResolvedPattern, state::State};
use crate::context::ProblemContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

// Undefined is a pattern that matches when a *Grit variable* is undefined.
// It is *not* meant to match against a *JavaScript* `undefined` value.

pub(crate) struct Undefined {}

impl Undefined {
    pub(crate) fn execute<'a, P: ProblemContext>(
        binding: &ResolvedPattern<'a>,
        _init_state: &mut State<'a, P>,
        _context: &'a P::ExecContext<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        Ok(binding.matches_undefined())
    }
}
