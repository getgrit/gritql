use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

use super::{resolved_pattern::ResolvedPattern, state::State, Context};

// Undefined is a pattern that matches when a *Grit variable* is undefined.
// It is *not* meant to match against a *JavaScript* `undefined` value.

pub(crate) struct Undefined {}

impl Undefined {
    pub(crate) fn execute<'a>(
        binding: &ResolvedPattern<'a>,
        _init_state: &mut State<'a>,
        _context: &Context<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        Ok(binding.matches_undefined())
    }
}
