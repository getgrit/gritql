use super::{
    compiler::CompilationContext,
    float_constant::FloatConstant,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    variable::VariableSourceLocations,
    Node, State,
};
use crate::context::Context;
use anyhow::{anyhow, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Like {
    pub(crate) like: Pattern,
    pub(crate) threshold: Pattern,
}

impl Like {
    pub fn new(like: Pattern, threshold: Pattern) -> Self {
        Self { like, threshold }
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        let threshold = node
            .child_by_field_name("threshold")
            .map(|n| {
                Pattern::from_node(
                    &n,
                    context,
                    vars,
                    vars_array,
                    scope_index,
                    global_vars,
                    true,
                    logs,
                )
            })
            .unwrap_or(Result::Ok(Pattern::FloatConstant(FloatConstant::new(0.9))))?;
        let like = node
            .child_by_field_name("example")
            .ok_or_else(|| anyhow!("missing field example of patternLike"))?;
        let like = Pattern::from_node(
            &like,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            true,
            logs,
        )?;
        Ok(Self::new(like, threshold))
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
