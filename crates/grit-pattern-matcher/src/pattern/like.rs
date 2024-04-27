use super::{
    patterns::{Matcher, Pattern, PatternName},
    State,
};
use crate::context::QueryContext;
use anyhow::{anyhow, Result};
use core::fmt::Debug;
use grit_util::AnalysisLogs;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Like<Q: QueryContext> {
    pub like: Pattern<Q>,
    pub threshold: Pattern<Q>,
}

impl<Q: QueryContext> Like<Q> {
    pub fn new(like: Pattern<Q>, threshold: Pattern<Q>) -> Self {
        Self { like, threshold }
    }
}

impl<Q: QueryContext> PatternName for Like<Q> {
    fn name(&self) -> &'static str {
        "LIKE"
    }
}

impl<Q: QueryContext> Matcher<Q> for Like<Q> {
    #[cfg(feature = "embeddings")]
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
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
        _binding: &Q::ResolvedPattern<'a>,
        _state: &mut State<'a, Q>,
        _context: &'a Q::ExecContext<'a>,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        Err(anyhow!("Like only available under the embeddings feature"))
    }
}
