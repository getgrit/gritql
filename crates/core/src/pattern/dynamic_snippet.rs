use super::{
    accessor::Accessor,
    built_in_functions::CallBuiltIn,
    functions::{CallForeignFunction, CallFunction},
    list_index::ListIndex,
    patterns::{Matcher, PatternName},
    resolved_pattern::ResolvedPattern,
    variable::Variable,
    State,
};
use crate::context::ProblemContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub enum DynamicSnippetPart {
    String(String),
    Variable(Variable),
}

#[derive(Debug, Clone)]
pub struct DynamicSnippet {
    pub parts: Vec<DynamicSnippetPart>,
}

#[derive(Debug, Clone)]
pub struct DynamicList<P: ProblemContext> {
    pub elements: Vec<DynamicPattern<P>>,
}

#[derive(Debug, Clone)]
pub enum DynamicPattern<P: ProblemContext> {
    Variable(Variable),
    Accessor(Box<Accessor<P>>),
    ListIndex(Box<ListIndex<P>>),
    Snippet(DynamicSnippet),
    List(DynamicList<P>),
    CallBuiltIn(CallBuiltIn<P>),
    CallFunction(CallFunction<P>),
    CallForeignFunction(CallForeignFunction<P>),
}

impl<P: ProblemContext> DynamicPattern<P> {
    pub fn text<'a>(
        &'a self,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<String> {
        let resolved = ResolvedPattern::from_dynamic_pattern(self, state, context, logs)?;
        Ok(resolved.text(&state.files)?.to_string())
    }
}

impl<P: ProblemContext> PatternName for DynamicPattern<P> {
    fn name(&self) -> &'static str {
        "DYNAMIC_PATTERN"
    }
}

impl<P: ProblemContext> Matcher<P> for DynamicPattern<P> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        if binding.text(&state.files)? == self.text(state, context, logs)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl PatternName for DynamicSnippet {
    fn name(&self) -> &'static str {
        "DYNAMIC_SNIPPET"
    }
}
