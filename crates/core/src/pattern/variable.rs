use super::{
    constants::{ABSOLUTE_PATH_INDEX, DEFAULT_FILE_NAME, FILENAME_INDEX, GLOBAL_VARS_SCOPE_INDEX},
    container::{PatternOrResolved, PatternOrResolvedMut},
    patterns::{Matcher, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{
    binding::Binding, context::QueryContext, pattern_compiler::compiler::NodeCompilationContext,
};
use anyhow::{bail, Result};
use core::fmt::Debug;
use im::vector;
use marzano_language::language::{Language, GRIT_METAVARIABLE_PREFIX};
use marzano_util::analysis_logs::AnalysisLogs;
use marzano_util::position::Range;

#[derive(Clone, Debug, Copy)]
pub struct Variable {
    pub scope: usize,
    pub index: usize,
}

#[derive(Debug, Clone)]
pub struct VariableSourceLocations {
    pub(crate) name: String,
    pub(crate) file: String,
    pub(crate) locations: BTreeSet<Range>,
}
use std::{borrow::Cow, collections::BTreeSet};

struct VariableMirror<'a, Q: QueryContext> {
    scope: usize,
    index: usize,
    binding: Q::Binding<'a>,
}

impl Variable {
    pub(crate) fn new(scope: usize, index: usize) -> Self {
        Self { scope, index }
    }

    pub(crate) fn get_pattern_or_resolved<'a, 'b, Q: QueryContext>(
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
            bail!("variable has no pattern or value")
        }
    }
    pub(crate) fn get_pattern_or_resolved_mut<'a, 'b, Q: QueryContext>(
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
            bail!("variable has no pattern or value")
        }
    }

    pub(crate) fn file_name() -> Self {
        Self::new(GLOBAL_VARS_SCOPE_INDEX, FILENAME_INDEX)
    }

    pub(crate) fn from_name(name: &str, context: &mut NodeCompilationContext) -> Result<Self> {
        register_variable_optional_range(name, None, context)
    }

    pub(crate) fn text<'a, Q: QueryContext>(&self, state: &State<'a, Q>) -> Result<Cow<'a, str>> {
        state.bindings[self.scope].last().unwrap()[self.index].text(state)
    }

    fn execute_resolved<'a, Q: QueryContext>(
        &self,
        resolved_pattern: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
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
                if let Some(var_binding) = var_side_resolve_pattern.get_binding() {
                    if let Some(binding) = resolved_pattern.get_binding() {
                        if !var_binding.is_equivalent_to(binding) {
                            return Ok(Some(false));
                        }
                        let value_history = &mut variable_content.value_history;
                        bindings.push_back(binding.clone());

                        // feels wrong maybe we should push ResolvedPattern::Binding(bindings)?
                        value_history.push(Q::ResolvedPattern::from_binding(binding.clone()));
                        variable_mirrors.extend(variable_content.mirrors.iter().map(|mirror| {
                            VariableMirror {
                                scope: mirror.scope,
                                index: mirror.index,
                                binding: binding.clone(),
                            }
                        }));
                    } else {
                        return Ok(Some(
                            resolved_pattern.text(&state.files)? == var_binding.text(),
                        ));
                    }
                } else {
                    return Ok(Some(
                        resolved_pattern.text(&state.files)?
                            == var_side_resolve_pattern.text(&state.files)?,
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
            let value = &mut mirror_content.value;
            if let Some(ResolvedPattern::Binding(bindings)) = value {
                bindings.push_back(mirror.binding.clone());
                let value_history = &mut mirror_content.value_history;
                value_history.push(ResolvedPattern::Binding(vector![mirror.binding]));
            }
        }
        Ok(Some(true))
    }
}

pub(crate) fn get_variables(
    params: &[(String, Range)],
    context: &mut NodeCompilationContext,
) -> Result<Vec<(String, Variable)>> {
    params
        .iter()
        .map(|(name, range)| {
            let index = register_variable(name, *range, context)?;
            Ok((name.to_owned(), index))
        })
        .collect()
}

pub(crate) fn register_variable(
    name: &str,
    range: Range,
    context: &mut NodeCompilationContext,
) -> Result<Variable> {
    register_variable_optional_range(
        name,
        Some(FileLocation {
            range,
            file_name: context.compilation.file,
        }),
        context,
    )
}

struct FileLocation<'a> {
    file_name: &'a str,
    range: Range,
}

fn register_variable_optional_range(
    name: &str,
    location: Option<FileLocation>,
    context: &mut NodeCompilationContext,
) -> Result<Variable> {
    let NodeCompilationContext {
        vars,
        vars_array,
        global_vars,
        scope_index,
        ..
    } = context;

    if let Some(i) = vars.get(name) {
        if let Some(FileLocation { range, .. }) = location {
            vars_array[*scope_index][*i].locations.insert(range);
        }
        return Ok(Variable::new(*scope_index, *i));
    }

    if let Some(i) = global_vars.get(name) {
        if let Some(FileLocation { range, file_name }) = location {
            if file_name == DEFAULT_FILE_NAME {
                vars_array[GLOBAL_VARS_SCOPE_INDEX][*i]
                    .locations
                    .insert(range);
            }
        }
        return Ok(Variable::new(GLOBAL_VARS_SCOPE_INDEX, *i));
    }
    let (name_map, scope_index) = if name.starts_with("$GLOBAL_") || name.starts_with("^GLOBAL_") {
        (global_vars, GLOBAL_VARS_SCOPE_INDEX)
    } else {
        (vars, *scope_index)
    };
    let scope = &mut vars_array[scope_index];
    let index = scope.len();
    name_map.insert(name.to_owned(), index);

    let (locations, file) = if let Some(FileLocation { range, file_name }) = location {
        let mut set = BTreeSet::new();
        set.insert(range);
        (set, file_name.to_owned())
    } else {
        // this currently only comes up with the $match variable which we autowrap, and is not
        // usually used by the user, but feels like this could potentially be a source of bugs
        (BTreeSet::new(), DEFAULT_FILE_NAME.to_owned())
    };
    scope.push(VariableSourceLocations {
        name: name.to_owned(),
        file,
        locations,
    });
    Ok(Variable::new(scope_index, index))
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
        if let Some(res) = self.execute_resolved(resolved_pattern, state)? {
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

pub(crate) fn get_absolute_file_name<Q: QueryContext>(
    state: &State<'_, Q>,
) -> Result<String, anyhow::Error> {
    let file = state.bindings[GLOBAL_VARS_SCOPE_INDEX].last().unwrap()[ABSOLUTE_PATH_INDEX]
        .value
        .as_ref();
    let file = file
        .map(|f| f.text(&state.files).map(|s| s.to_string()))
        .unwrap_or(Ok("No File Found".to_string()))?;
    Ok(file)
}

pub(crate) fn get_file_name<Q: QueryContext>(
    state: &State<'_, Q>,
) -> Result<String, anyhow::Error> {
    let file = state.bindings[GLOBAL_VARS_SCOPE_INDEX].last().unwrap()[FILENAME_INDEX]
        .value
        .as_ref();
    let file = file
        .map(|f| f.text(&state.files).map(|s| s.to_string()))
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
