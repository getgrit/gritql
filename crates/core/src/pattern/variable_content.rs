use anyhow::{anyhow, Result};
use std::borrow::Cow;

use crate::pattern::patterns::Pattern;

use super::{resolved_pattern::ResolvedPattern, state::State, variable::Variable};

#[derive(Debug, Clone)]
pub struct VariableContent<'a> {
    pub name: String,
    pub pattern: Option<&'a Pattern>,
    // needs to be boxed for lifetime reasons
    pub(crate) value: Option<ResolvedPattern<'a>>,
    pub(crate) value_history: Vec<ResolvedPattern<'a>>,
    // If the value is a binding, whenever it is updated the mirrors should be updated as well
    pub(crate) mirrors: Vec<&'a Variable>,
}

impl<'a> VariableContent<'a> {
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
    pub fn text(&self, state: &State<'a>) -> Result<Cow<'a, str>> {
        if let Some(value) = &self.value {
            value.text(&state.files)
        } else {
            Err(anyhow!("no value for variable {}", self.name))
        }
    }

    pub(crate) fn set_value(&mut self, value: ResolvedPattern<'a>) -> Option<ResolvedPattern<'a>> {
        std::mem::replace(&mut self.value, Some(value))
    }
}
