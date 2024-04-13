use super::{
    dynamic_snippet::DynamicPattern,
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{context::ProblemContext, resolve};
use anyhow::Result;
use core::fmt::Debug;
use marzano_language::language::SortId;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct CodeSnippet<P: ProblemContext> {
    pub(crate) patterns: Vec<(SortId, Pattern<P>)>,
    pub(crate) source: String,
    pub(crate) dynamic_snippet: Option<DynamicPattern<P>>,
}

impl<P: ProblemContext> CodeSnippet<P> {
    pub fn new(
        patterns: Vec<(SortId, Pattern<P>)>,
        dynamic_snippet: Option<DynamicPattern<P>>,
        source: &str,
    ) -> Self {
        Self {
            patterns,
            source: source.to_string(),
            dynamic_snippet,
        }
    }
}

impl<P: ProblemContext> PatternName for CodeSnippet<P> {
    fn name(&self) -> &'static str {
        "CODESNIPPET"
    }
}

impl<P: ProblemContext> Matcher<P> for CodeSnippet<P> {
    // wrong, but whatever for now
    fn execute<'a>(
        &'a self,
        resolved_pattern: &ResolvedPattern<'a>,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let binding = match resolved_pattern {
            ResolvedPattern::Binding(binding) => resolve!(binding.last()),
            resolved @ ResolvedPattern::Snippets(_)
            | resolved @ ResolvedPattern::List(_)
            | resolved @ ResolvedPattern::Map(_)
            | resolved @ ResolvedPattern::File(_)
            | resolved @ ResolvedPattern::Files(_)
            | resolved @ ResolvedPattern::Constant(_) => {
                return Ok(resolved.text(&state.files)?.trim() == self.source)
            }
        };

        let Some(node) = binding.singleton() else {
            return Ok(false);
        };

        if let Some((_, pattern)) = self
            .patterns
            .iter()
            .find(|(id, _)| *id == node.node.kind_id())
        {
            pattern.execute(resolved_pattern, state, context, logs)
        } else {
            Ok(false)
        }
    }
}
