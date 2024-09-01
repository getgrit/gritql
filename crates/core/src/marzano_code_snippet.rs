use crate::{
    marzano_context::MarzanoContext, marzano_resolved_pattern::MarzanoResolvedPattern,
    problem::MarzanoQueryContext,
};
use grit_pattern_matcher::{
    binding::Binding,
    context::ExecContext,
    pattern::{CodeSnippet, DynamicPattern, Matcher, Pattern, PatternName, ResolvedPattern, State},
};
use grit_util::{error::GritResult, AnalysisLogs};
use marzano_language::language::SortId;

#[derive(Debug, Clone)]
pub struct MarzanoCodeSnippet {
    pub(crate) patterns: Vec<(SortId, Pattern<MarzanoQueryContext>)>,
    pub(crate) source: String,
    pub(crate) dynamic_snippet: Option<DynamicPattern<MarzanoQueryContext>>,
}

impl MarzanoCodeSnippet {
    pub fn new(
        patterns: Vec<(SortId, Pattern<MarzanoQueryContext>)>,
        dynamic_snippet: Option<DynamicPattern<MarzanoQueryContext>>,
        source: &str,
    ) -> Self {
        Self {
            patterns,
            source: source.to_string(),
            dynamic_snippet,
        }
    }
}

impl CodeSnippet<MarzanoQueryContext> for MarzanoCodeSnippet {
    fn patterns(&self) -> impl Iterator<Item = &Pattern<MarzanoQueryContext>> {
        self.patterns.iter().map(|p| &p.1)
    }

    fn dynamic_snippet(&self) -> Option<&DynamicPattern<MarzanoQueryContext>> {
        self.dynamic_snippet.as_ref()
    }
}

impl PatternName for MarzanoCodeSnippet {
    fn name(&self) -> &'static str {
        "CODESNIPPET"
    }
}

impl Matcher<MarzanoQueryContext> for MarzanoCodeSnippet {
    // wrong, but whatever for now
    fn execute<'a, 'b>(
        &'b self,
        resolved: &MarzanoResolvedPattern<'a>,
        state: &mut State<'a, MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
        logs: &mut AnalysisLogs
    ) -> GritResult<bool> {
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
        } }
}
