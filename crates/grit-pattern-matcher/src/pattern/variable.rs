use super::{
    container::{PatternOrResolved, PatternOrResolvedMut},
    patterns::{Matcher, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{
    binding::Binding,
    constants::{ABSOLUTE_PATH_INDEX, DEFAULT_FILE_NAME, FILENAME_INDEX, GLOBAL_VARS_SCOPE_INDEX},
    context::{ExecContext, QueryContext},
};
use core::fmt::Debug;
use grit_util::{
    constants::GRIT_METAVARIABLE_PREFIX,
    error::{GritPatternError, GritResult},
    AnalysisLogs, ByteRange, Language,
};
use std::{
    borrow::Cow,
    collections::BTreeSet,
    sync::{Arc, RwLock},
};

#[derive(Debug, Clone, Copy)]
pub(crate) struct VariableScope {
    pub(crate) scope: u16,
    pub(crate) index: u16,
}

impl VariableScope {
    pub fn new(scope: usize, index: usize) -> Self {
        Self {
            scope: scope as u16,
            index: index as u16,
        }
    }
}

#[derive(Debug, Clone)]
struct DynamicVariableInternal {
    name: String,
    /// Track the last scope we registed this variable in
    /// This is mainly just used for cases where we don't have state available
    last_scope: Arc<RwLock<Option<VariableScope>>>,
}

#[derive(Debug, Clone)]
enum VariableInternal {
    /// Static variable, which is bound at compile time (ex. global variables).
    /// These are slightly more efficient, and follow the traditional approach in Grit.
    /// However, they require more direct control over scopes and indexes.
    Static(VariableScope),
    /// Dynamic variables are lazy, so we just need to register them by name.
    /// They will then automatically be bound to the first scope that attempts to use them.
    /// This should be avoided where possible, since it means names will likely overwrite each other across scopes.
    Dynamic(DynamicVariableInternal),
}

#[derive(Clone, Debug)]
pub struct Variable {
    internal: VariableInternal,
}

/// VariableSource is used to track the origin of a variable
/// It can come from
#[derive(Debug, Clone)]
pub enum VariableSource {
    /// Compiled from a pattern
    Compiled {
        name: String,
        file: String,
        locations: BTreeSet<ByteRange>,
    },
    /// Global variable, which is not defined anywhere
    Global { name: String },
}

impl VariableSource {
    pub fn new(name: String, file: String) -> Self {
        Self::Compiled {
            name,
            file,
            locations: BTreeSet::new(),
        }
    }

    pub fn new_global(name: String) -> Self {
        Self::Global { name }
    }

    /// Register a location in a GritQL file where a variable is referenced
    pub fn register_location(&mut self, location: ByteRange) -> GritResult<()> {
        match self {
            VariableSource::Compiled { locations, .. } => {
                locations.insert(location);
                Ok(())
            }
            VariableSource::Global { .. } => Ok(()),
        }
    }

    /// Get locations where the variable is referenced from the main pattern file
    pub fn get_main_locations(&self) -> Vec<ByteRange> {
        if let VariableSource::Compiled {
            locations, file, ..
        } = self
        {
            if file != DEFAULT_FILE_NAME {
                return vec![];
            }
            locations.iter().cloned().collect()
        } else {
            vec![]
        }
    }

    /// Get the registered variable name
    pub fn name(&self) -> &str {
        match self {
            VariableSource::Compiled { name, .. } => name,
            VariableSource::Global { name } => name,
        }
    }
}

struct VariableMirror<'a, Q: QueryContext> {
    scope: u16,
    index: u16,
    binding: Q::Binding<'a>,
}

impl Variable {
    /// Create a variable, where we already know the scope and index it will be bound to
    pub fn new(scope: usize, index: usize) -> Self {
        Self {
            internal: VariableInternal::Static(VariableScope {
                scope: scope as u16,
                index: index as u16,
            }),
        }
    }

    /// Create a dynamic variable, which will be bound to the first scope that uses it
    ///
    /// Warning: this is not stable or tested yet. This implementation is still incomplete.
    pub fn new_dynamic(name: &str) -> Self {
        Self {
            internal: VariableInternal::Dynamic(DynamicVariableInternal {
                name: name.to_string(),
                last_scope: Arc::new(RwLock::new(None)),
            }),
        }
    }

    fn try_internal(&self) -> GritResult<VariableScope> {
        match &self.internal {
            VariableInternal::Static(scope) => Ok(*scope),
            VariableInternal::Dynamic(lock) => {
                if let Ok(reader) = lock.last_scope.try_read() {
                    if let Some(scope) = *reader {
                        return Ok(scope);
                    }
                }
                Err(GritPatternError::new_matcher(format!(
                    "variable {} not initialized",
                    lock.name
                )))
            }
        }
    }

    fn get_internal<Q: QueryContext>(&self, state: &mut State<'_, Q>) -> GritResult<VariableScope> {
        match &self.internal {
            VariableInternal::Static(internal) => Ok(*internal),
            VariableInternal::Dynamic(lock) => {
                let scope = state.register_var(&lock.name);
                if let Ok(mut writer) = lock.last_scope.write() {
                    *writer = Some(scope);
                }
                Ok(scope)
            }
        }
    }

    /// Try to get the scope of the variable, if it has been bound to a scope.
    /// If the variable has not been bound to a scope, return an error.
    /// When possible, prefer to use `get_scope()` instead, which will initialize the variable's scope if it is not already bound.
    pub fn try_scope(&self) -> GritResult<u16> {
        Ok(self.try_internal()?.scope)
    }

    /// Try to get the index of the variable, if it has been bound to an index.
    /// If the variable has not been bound to an index, return an error.
    /// When possible, prefer to use `get_index()` instead, which will initialize the variable's index if it is not already bound.
    pub fn try_index(&self) -> GritResult<u16> {
        Ok(self.try_internal()?.index)
    }

    /// Get the scope of the variable, initializing it if it is not already bound.
    pub fn get_scope<Q: QueryContext>(&self, state: &mut State<'_, Q>) -> GritResult<u16> {
        Ok(self.get_internal(state)?.scope)
    }

    /// Get the index of the variable, initializing it if it is not already bound.
    pub fn get_index<Q: QueryContext>(&self, state: &mut State<'_, Q>) -> GritResult<u16> {
        Ok(self.get_internal(state)?.index)
    }

    pub fn get_pattern_or_resolved<'a, 'b, Q: QueryContext>(
        &self,
        state: &'b State<'a, Q>,
    ) -> GritResult<Option<PatternOrResolved<'a, 'b, Q>>> {
        let v = state.trace_var(self);
        let content = &state.bindings[v.try_scope().unwrap().into()]
            .last()
            .unwrap()[v.try_index().unwrap().into()];
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
    ) -> GritResult<Option<PatternOrResolvedMut<'a, 'b, Q>>> {
        let v = state.trace_var_mut(self);
        let content = &mut state.bindings[v.try_scope().unwrap().into()]
            .back_mut()
            .unwrap()[v.try_index().unwrap().into()];
        if let Some(pattern) = content.pattern {
            Ok(Some(PatternOrResolvedMut::Pattern(pattern)))
        } else if let Some(resolved) = &mut content.value {
            Ok(Some(PatternOrResolvedMut::Resolved(resolved)))
        } else {
            Ok(None)
        }
    }

    pub fn file_name() -> Self {
        Self::new(GLOBAL_VARS_SCOPE_INDEX.into(), FILENAME_INDEX)
    }

    pub fn is_file_name(&self) -> bool {
        let VariableInternal::Static(scope) = &self.internal else {
            return false;
        };
        scope.scope == GLOBAL_VARS_SCOPE_INDEX && scope.index as usize == FILENAME_INDEX
    }

    pub fn text<'a, Q: QueryContext>(
        &self,
        state: &State<'a, Q>,
        lang: &Q::Language<'a>,
    ) -> GritResult<Cow<'a, str>> {
        state.bindings[self.try_scope().unwrap().into()]
            .last()
            .unwrap()[self.try_index().unwrap().into()]
        .text(state, lang)
    }

    fn execute_resolved<'a, Q: QueryContext>(
        &self,
        resolved_pattern: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        language: &Q::Language<'a>,
    ) -> GritResult<Option<bool>> {
        let mut variable_mirrors: Vec<VariableMirror<Q>> = Vec::new();
        {
            let scope = self.get_scope(state)?;
            let index = self.get_index(state)?;
            let variable_content = state
                .bindings
                .get_mut(scope.into())
                .unwrap()
                .back_mut()
                .unwrap()
                .get_mut(index.into());
            let Some(variable_content) = variable_content else {
                return Ok(None);
            };
            let variable_content = &mut **(variable_content);
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
                            scope: mirror.try_scope().unwrap(),
                            index: mirror.try_index().unwrap(),
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
                .get_mut(mirror.scope.into())
                .unwrap()
                .back_mut()
                .unwrap()
                .get_mut(mirror.index.into())
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
    ) -> GritResult<bool> {
        if let Some(res) = self.execute_resolved(resolved_pattern, state, context.language())? {
            return Ok(res);
        }
        // we only check the assignment if the variable is not bound already
        // otherwise, we assume that the assignment is correct

        // we do this convoluted check to avoid double-borrowing of state
        // via the variable_content variable
        let scope = self.get_scope(state)?;
        let index = self.get_index(state)?;

        let variable_content = state
            .bindings
            .get_mut(scope.into())
            .unwrap()
            .back_mut()
            .unwrap()
            .get_mut(index.into());
        let Some(variable_content) = variable_content else {
            logs.add_warning(
                None,
                format!("Variable unexpectedly not found in scope {:?}", scope),
            );
            return Ok(false);
        };

        let variable_content = &mut **(variable_content);
        if let Some(pattern) = variable_content.pattern {
            if !pattern.execute(resolved_pattern, state, context, logs)? {
                return Ok(false);
            }
        }
        let variable_content = &mut **(state
            .bindings
            .get_mut(scope.into())
            .unwrap()
            .back_mut()
            .unwrap()
            .get_mut(index.into())
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
) -> GritResult<String> {
    let file = state.bindings[GLOBAL_VARS_SCOPE_INDEX.into()]
        .last()
        .unwrap()[ABSOLUTE_PATH_INDEX]
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
) -> GritResult<String> {
    let file = state.bindings[GLOBAL_VARS_SCOPE_INDEX.into()]
        .last()
        .unwrap()[FILENAME_INDEX]
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
