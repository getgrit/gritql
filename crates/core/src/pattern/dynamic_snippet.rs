use std::collections::BTreeMap;

use anyhow::{bail, Result};
use marzano_language::target_language::TargetLanguage;
use marzano_util::{
    analysis_logs::AnalysisLogs,
    position::{Position, Range},
};

use crate::{context::Context, split_snippet::split_snippet};

use super::{
    accessor::Accessor,
    built_in_functions::CallBuiltIn,
    compiler::DEFAULT_FILE_NAME,
    functions::{CallForeignFunction, CallFunction},
    list_index::ListIndex,
    patterns::{Matcher, Name},
    resolved_pattern::ResolvedPattern,
    variable::{register_variable, Variable, VariableSourceLocations, GLOBAL_VARS_SCOPE_INDEX},
    State,
};
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

impl DynamicSnippet {
    /**
     * Uses split_snippet to split the snippet into parts.
     */
    #[allow(clippy::too_many_arguments)]
    pub fn from(
        raw_source: &str,
        file: &str,
        source_range: Range,
        vars: &mut BTreeMap<String, usize>,
        global_vars: &mut BTreeMap<String, usize>,
        vars_array: &mut [Vec<VariableSourceLocations>],
        scope_index: usize,
        lang: &TargetLanguage,
    ) -> Result<Self> {
        let source_string = raw_source
            .replace("\\n", "\n")
            .replace("\\$", "$")
            .replace("\\^", "^")
            .replace("\\`", "`")
            .replace("\\\"", "\"")
            .replace("\\\\", "\\");
        let source = source_string.as_str();
        let mut metavariables = split_snippet(source, lang);
        metavariables.reverse();
        let mut parts = Vec::new();
        let mut last = 0;
        let mut last_pos = source_range.start;
        for (byte_range, var) in metavariables {
            parts.push(DynamicSnippetPart::String(
                source[last as usize..byte_range.start as usize].to_string(),
            ));
            let start_pos =
                Position::from_byte_index(source, Some((last_pos, last)), byte_range.start);
            // todo: does this handle utf8 correctly?
            last_pos = Position::new(start_pos.line, start_pos.column + var.len() as u32);
            let range = Range::new(
                start_pos,
                last_pos,
                source_range.start_byte + byte_range.start,
                source_range.start_byte + byte_range.start + var.len() as u32,
            );
            if let Some(var) = vars.get(&var.to_string()) {
                vars_array[scope_index][*var].locations.insert(range);
                parts.push(DynamicSnippetPart::Variable(Variable::new(
                    scope_index,
                    *var,
                )));
            } else if let Some(var) = global_vars.get(&var.to_string()) {
                if file == DEFAULT_FILE_NAME {
                    vars_array[GLOBAL_VARS_SCOPE_INDEX][*var]
                        .locations
                        .insert(range);
                }
                parts.push(DynamicSnippetPart::Variable(Variable::new(
                    GLOBAL_VARS_SCOPE_INDEX,
                    *var,
                )));
            } else if var.starts_with("$GLOBAL_") {
                let variable = register_variable(
                    &var,
                    file,
                    range,
                    vars,
                    global_vars,
                    vars_array,
                    scope_index,
                )?;
                parts.push(DynamicSnippetPart::Variable(variable));
            } else {
                bail!(
                    "Could not find variable {} in this context, for snippet {}",
                    var,
                    source
                );
            }
            last = byte_range.end;
        }
        parts.push(DynamicSnippetPart::String(
            source[last as usize..].to_string(),
        ));
        Ok(Self { parts })
    }
}

impl Name for DynamicSnippet {
    fn name(&self) -> &'static str {
        "DYNAMIC_SNIPPET"
    }
}
