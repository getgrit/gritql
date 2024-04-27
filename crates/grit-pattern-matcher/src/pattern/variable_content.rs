use super::{resolved_pattern::ResolvedPattern, state::State, variable::Variable};
use crate::{context::QueryContext, pattern::patterns::Pattern};
use anyhow::{anyhow, Result};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct VariableContent<'a, Q: QueryContext> {
    pub name: String,
    pub pattern: Option<&'a Pattern<Q>>,
    // needs to be boxed for lifetime reasons
    pub value: Option<Q::ResolvedPattern<'a>>,
    pub value_history: Vec<Q::ResolvedPattern<'a>>,
    // If the value is a binding, whenever it is updated the mirrors should be updated as well
    pub mirrors: Vec<&'a Variable>,
}

impl<'a, Q: QueryContext> VariableContent<'a, Q> {
    pub fn new(name: String) -> Self {
        Self {
            name,
            pattern: None,
            value: None,
            value_history: vec![],
            mirrors: vec![],
        }
    }

    // should we return an option instead of a Result?
    // should we trace pattern calls here? - currently only used by variable which already traces
    pub fn text(&self, state: &State<'a, Q>, language: &Q::Language<'a>) -> Result<Cow<'a, str>> {
        if let Some(value) = &self.value {
            value.text(&state.files, language)
        } else {
            Err(anyhow!("no value for variable {}", self.name))
        }
    }

    pub(crate) fn set_value(&mut self, value: Q::ResolvedPattern<'a>) {
        self.value = Some(value);
    }
}
