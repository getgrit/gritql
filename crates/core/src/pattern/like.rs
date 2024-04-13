use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::ProblemContext;
use anyhow::{anyhow, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Like<P: ProblemContext> {
    pub like: Pattern<P>,
    pub threshold: Pattern<P>,
}

impl<P: ProblemContext> Like<P> {
    pub fn new(like: Pattern<P>, threshold: Pattern<P>) -> Self {
        Self { like, threshold }
    }
}

impl<P: ProblemContext> PatternName for Like<P> {
    fn name(&self) -> &'static str {
        "LIKE"
    }
}

impl<P: ProblemContext> Matcher<P> for Like<P> {
    #[cfg(feature = "embeddings")]
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        use crate::errors::debug;

        let snippet = self.like.text(state, context, logs)?;
        let code = binding.text(&state.files)?.to_string();
        let model = embeddings::embed::EmbeddingModel::VoyageCode2;
        let similarity =
            model.similarity(snippet, code, context.runtime, |s| debug(logs, state, s))? as f64;
        Ok(similarity > self.threshold.float(state, context, logs)?)
    }

    #[cfg(not(feature = "embeddings"))]
    fn execute<'a>(
        &'a self,
        _binding: &ResolvedPattern<'a>,
        _state: &mut State<'a, P>,
        _context: &'a P::ExecContext<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        Err(anyhow!("Like only available under the embeddings feature"))
    }
}
