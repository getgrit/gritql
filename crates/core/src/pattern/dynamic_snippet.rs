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
use crate::context::QueryContext;
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
pub struct DynamicList<Q: QueryContext> {
    pub elements: Vec<DynamicPattern<Q>>,
}

#[derive(Debug, Clone)]
pub enum DynamicPattern<Q: QueryContext> {
    Variable(Variable),
    Accessor(Box<Accessor<Q>>),
    ListIndex(Box<ListIndex<Q>>),
    Snippet(DynamicSnippet),
    List(DynamicList<Q>),
    CallBuiltIn(CallBuiltIn<Q>),
    CallFunction(CallFunction<Q>),
    CallForeignFunction(CallForeignFunction<Q>),
}

impl<Q: QueryContext> DynamicPattern<Q> {
    pub fn text<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<String> {
        let resolved = Q::ResolvedPattern::from_dynamic_pattern(self, state, context, logs)?;
        Ok(resolved.text(&state.files)?.to_string())
    }
}

impl<Q: QueryContext> PatternName for DynamicPattern<Q> {
    fn name(&self) -> &'static str {
        "DYNAMIC_PATTERN"
    }
}

impl<Q: QueryContext> Matcher<Q> for DynamicPattern<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
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
