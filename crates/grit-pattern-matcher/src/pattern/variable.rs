use super::{
    container::{PatternOrResolved, PatternOrResolvedMut},
    patterns::{Matcher, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{
    binding::Binding,
    constants::{ABSOLUTE_PATH_INDEX, FILENAME_INDEX, GLOBAL_VARS_SCOPE_INDEX},
    context::{ExecContext, QueryContext},
};
use anyhow::{Result};
use core::fmt::Debug;
use grit_util::{constants::GRIT_METAVARIABLE_PREFIX, AnalysisLogs, ByteRange, Language};
use std::{borrow::Cow, collections::BTreeSet};

#[derive(Clone, Debug, Copy)]
pub struct Variable {
    pub scope: usize,
    pub index: usize,
}

#[derive(Debug, Clone)]
pub struct VariableSourceLocations {
    pub name: String,
    pub file: String,
    pub locations: BTreeSet<ByteRange>,
}

struct VariableMirror<'a, Q: QueryContext> {
    scope: usize,
    index: usize,
    binding: Q::Binding<'a>,
}

impl Variable {
    pub fn new(scope: usize, index: usize) -> Self {
        Self { scope, index }
    }

    pub fn get_pattern_or_resolved<'a, 'b, Q: QueryContext>(
        &self,
        state: &'b State<'a, Q>,
    ) -> Result<Option<PatternOrResolved<'a, 'b, Q>>> {
        let v = state.trace_var(self);
        let content = &state.bindings[v.scope].last().unwrap()[v.index];
        if let Some(pattern) = content.pattern {
            Ok(Some(PatternOrResolved::Pattern(pattern)))
        } else if let Some(resolved) = &content.value {
            Ok(Some(PatternOrResolved::Resolved(resolved)))
        } else {
            Ok(None)
        }
    }
    pub fn get_pattern_or_resolved_mut<'a, 'b, Q: QueryContext>(
        &self,
        state: &'b mut State<'a, Q>,
    ) -> Result<Option<PatternOrResolvedMut<'a, 'b, Q>>> {
        let v = state.trace_var(self);
        let content = &mut state.bindings[v.scope].back_mut().unwrap()[v.index];
        if let Some(pattern) = content.pattern {
            Ok(Some(PatternOrResolvedMut::Pattern(pattern)))
        } else if let Some(resolved) = &mut content.value {
            Ok(Some(PatternOrResolvedMut::Resolved(resolved)))
        } else {
            Ok(None)
        }
    }

    pub fn file_name() -> Self {
        Self::new(GLOBAL_VARS_SCOPE_INDEX, FILENAME_INDEX)
    }

    pub fn is_file_name(&self) -> bool {
        self.scope == GLOBAL_VARS_SCOPE_INDEX && self.index == FILENAME_INDEX
    }

    pub fn text<'a, Q: QueryContext>(
        &self,
        state: &State<'a, Q>,
        lang: &Q::Language<'a>,
    ) -> Result<Cow<'a, str>> {
        state.bindings[self.scope].last().unwrap()[self.index].text(state, lang)
    }

    fn execute_resolved<'a, Q: QueryContext>(
        &self,
        resolved_pattern: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        language: &Q::Language<'a>,
    ) -> Result<Option<bool>> {
        let mut variable_mirrors: Vec<VariableMirror<Q>> = Vec::new();
        {
            let variable_content = &mut **(state
                .bindings
                .get_mut(self.scope)
                .unwrap()
                .back_mut()
                .unwrap()
                .get_mut(self.index)
                .unwrap());
            let value = &mut variable_content.value;

            if let Some(var_side_resolve_pattern) = value {
                if let (Some(var_binding), Some(binding)) = (
                    var_side_resolve_pattern.get_last_binding(),
                    resolved_pattern.get_last_binding(),
                ) {
                    if !var_binding.is_equivalent_to(binding, language) {
                        return Ok(Some(false));
                    }
                    let value_history = &mut variable_content.value_history;
                    var_side_resolve_pattern.push_binding(binding.clone())?;

                    // feels wrong maybe we should push ResolvedPattern::Binding(bindings)?
                    value_history.push(ResolvedPattern::from_binding(binding.clone()));
                    variable_mirrors.extend(variable_content.mirrors.iter().map(|mirror| {
                        VariableMirror {
                            scope: mirror.scope,
                            index: mirror.index,
                            binding: binding.clone(),
                        }
                    }));
                } else {
                    return Ok(Some(
                        resolved_pattern.text(&state.files, language)?
                            == var_side_resolve_pattern.text(&state.files, language)?,
                    ));
                }
            } else {
                return Ok(None);
            };
        }
        for mirror in variable_mirrors {
            let mirror_content = &mut **(state
                .bindings
                .get_mut(mirror.scope)
                .unwrap()
                .back_mut()
                .unwrap()
                .get_mut(mirror.index)
                .unwrap());
            if let Some(value) = &mut mirror_content.value {
                if value.is_binding() {
                    value.push_binding(mirror.binding.clone())?;
                    let value_history = &mut mirror_content.value_history;
                    value_history.push(ResolvedPattern::from_binding(mirror.binding));
                }
            }
        }
        Ok(Some(true))
    }
}

impl PatternName for Variable {
    fn name(&self) -> &'static str {
        "VARIABLE"
    }
}

impl<Q: QueryContext> Matcher<Q> for Variable {
    fn execute<'a>(
        &'a self,
        resolved_pattern: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        if let Some(res) = self.execute_resolved(resolved_pattern, state, context.language())? {
            return Ok(res);
        }
        // we only check the assignment if the variable is not bound already
        // otherwise, we assume that the assignment is correct

        // we do this convoluted check to avoid double-borrowing of state
        // via the variable_content variable
        let variable_content = &mut **(state
            .bindings
            .get_mut(self.scope)
            .unwrap()
            .back_mut()
            .unwrap()
            .get_mut(self.index)
            .unwrap());
        if let Some(pattern) = variable_content.pattern {
            if !pattern.execute(resolved_pattern, state, context, logs)? {
                return Ok(false);
            }
        }
        let variable_content = &mut **(state
            .bindings
            .get_mut(self.scope)
            .unwrap()
            .back_mut()
            .unwrap()
            .get_mut(self.index)
            .unwrap());
        variable_content.value = Some(resolved_pattern.clone());
        variable_content
            .value_history
            .push(resolved_pattern.clone());
        Ok(true)
    }
}

pub fn get_absolute_file_name<'a, Q: QueryContext>(
    state: &State<'a, Q>,
    lang: &Q::Language<'a>,
) -> Result<String, anyhow::Error> {
    let file = state.bindings[GLOBAL_VARS_SCOPE_INDEX].last().unwrap()[ABSOLUTE_PATH_INDEX]
        .value
        .as_ref();
    let file = file
        .map(|f| f.text(&state.files, lang).map(|s| s.to_string()))
        .unwrap_or(Ok("No File Found".to_string()))?;
    Ok(file)
}

pub fn get_file_name<'a, Q: QueryContext>(
    state: &State<'a, Q>,
    lang: &Q::Language<'a>,
) -> Result<String, anyhow::Error> {
    let file = state.bindings[GLOBAL_VARS_SCOPE_INDEX].last().unwrap()[FILENAME_INDEX]
        .value
        .as_ref();
    let file = file
        .map(|f| f.text(&state.files, lang).map(|s| s.to_string()))
        .unwrap_or(Ok("No File Found".to_string()))?;
    Ok(file)
}

pub fn is_reserved_metavariable(var: &str, lang: Option<&impl Language>) -> bool {
    let name = var.trim_start_matches(GRIT_METAVARIABLE_PREFIX);
    let name = if let Some(lang) = lang {
        name.trim_start_matches(lang.metavariable_prefix_substitute())
    } else {
        name
    };
    name == "match"
        || name == "filename"
        || name == "absolute_filename"
        || name == "new_files"
        || name == "program"
        || name.starts_with("grit_")
}
