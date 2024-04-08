use super::{
    accessor::Accessor,
    built_in_functions::CallBuiltIn,
    functions::{CallForeignFunction, CallFunction},
    list_index::ListIndex,
    patterns::{Matcher, Name},
    resolved_pattern::ResolvedPattern,
    variable::Variable,
    State,
};
use crate::context::Context;
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
pub struct DynamicList {
    pub elements: Vec<DynamicPattern>,
}

#[derive(Debug, Clone)]
pub enum DynamicPattern {
    Variable(Variable),
    Accessor(Box<Accessor>),
    ListIndex(Box<ListIndex>),
    Snippet(DynamicSnippet),
    List(DynamicList),
    CallBuiltIn(CallBuiltIn),
    CallFunction(CallFunction),
    CallForeignFunction(CallForeignFunction),
}

impl DynamicPattern {
    pub fn text<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<String> {
        let resolved = ResolvedPattern::from_dynamic_pattern(self, state, context, logs)?;
        Ok(resolved.text(&state.files)?.to_string())
    }
}

impl Name for DynamicPattern {
    fn name(&self) -> &'static str {
        "DYNAMIC_PATTERN"
    }
}

impl Matcher for DynamicPattern {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        if binding.text(&state.files)? == self.text(state, context, logs)? {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Name for DynamicSnippet {
    fn name(&self) -> &'static str {
        "DYNAMIC_SNIPPET"
    }
}
