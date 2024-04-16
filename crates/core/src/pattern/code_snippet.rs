use super::{
    dynamic_snippet::DynamicPattern,
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::{ExecContext, QueryContext};
use anyhow::Result;
use core::fmt::Debug;
use marzano_language::language::SortId;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct CodeSnippet<Q: QueryContext> {
    pub(crate) patterns: Vec<(SortId, Pattern<Q>)>,
    pub(crate) source: String,
    pub(crate) dynamic_snippet: Option<DynamicPattern<Q>>,
}

impl<Q: QueryContext> CodeSnippet<Q> {
    pub fn new(
        patterns: Vec<(SortId, Pattern<Q>)>,
        dynamic_snippet: Option<DynamicPattern<Q>>,
        source: &str,
    ) -> Self {
        Self {
            patterns,
            source: source.to_string(),
            dynamic_snippet,
        }
    }
}

impl<Q: QueryContext> PatternName for CodeSnippet<Q> {
    fn name(&self) -> &'static str {
        "CODESNIPPET"
    }
}

impl<Q: QueryContext> Matcher<Q> for CodeSnippet<Q> {
    // wrong, but whatever for now
    fn execute<'a>(
        &'a self,
        resolved: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let Some(binding) = resolved.get_last_binding() else {
            return Ok(resolved.text(&state.files, context.language())?.trim() == self.source);
        };

        let Some(node) = binding.singleton() else {
            return Ok(false);
        };

        if let Some((_, pattern)) = self
            .patterns
            .iter()
            .find(|(id, _)| *id == node.node.kind_id())
        {
            pattern.execute(resolved, state, context, logs)
        } else {
            Ok(false)
        }
    }
}
