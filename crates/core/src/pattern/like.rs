use super::{
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::Context;
use anyhow::{anyhow, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Like {
    pub like: Pattern,
    pub threshold: Pattern,
}

impl Like {
    pub fn new(like: Pattern, threshold: Pattern) -> Self {
        Self { like, threshold }
    }
}

impl Name for Like {
    fn name(&self) -> &'static str {
        "LIKE"
    }
}

impl Matcher for Like {
    #[cfg(feature = "embeddings")]
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
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
        _state: &mut State<'a>,
        _context: &'a impl Context,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        Err(anyhow!("Like only available under the embeddings feature"))
    }
}
