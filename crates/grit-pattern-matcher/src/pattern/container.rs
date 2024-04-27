use super::{
    accessor::Accessor, list_index::ListIndex, patterns::Pattern, state::State, variable::Variable,
    CallBuiltIn,
};
use crate::context::QueryContext;
use anyhow::{bail, Result};

/// A `Container` represents anything which "contains" a reference to a Pattern.
///
/// We have three types of containers:
/// - Variable: a variable reference (ex. `$foo`)
/// - Accessor: a map accessor (ex. `$foo.bar`)
/// - ListIndex: a list index (ex. `$foo[0]`)
#[derive(Debug, Clone)]
pub enum Container<Q: QueryContext> {
    Variable(Variable),
    Accessor(Box<Accessor<Q>>),
    ListIndex(Box<ListIndex<Q>>),
    FunctionCall(Box<CallBuiltIn<Q>>),
}

#[derive(Debug)]
pub enum PatternOrResolved<'a, 'b, Q: QueryContext> {
    Pattern(&'a Pattern<Q>),
    Resolved(&'b Q::ResolvedPattern<'a>),
    ResolvedBinding(Q::ResolvedPattern<'a>),
}

#[derive(Debug)]
pub enum PatternOrResolvedMut<'a, 'b, Q: QueryContext> {
    Pattern(&'a Pattern<Q>),
    Resolved(&'b mut Q::ResolvedPattern<'a>),
    _ResolvedBinding,
}

impl<Q: QueryContext> Container<Q> {
    pub fn set_resolved<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        lang: &Q::Language<'a>,
        value: Q::ResolvedPattern<'a>,
    ) -> Result<bool> {
        match self {
            Container::Variable(v) => {
                let var = state.trace_var(v);
                let content = &mut state.bindings[var.scope].back_mut().unwrap()[var.index];
                match content.pattern {
                    Some(Pattern::Accessor(a)) => a.set_resolved(state, lang, value),
                    Some(Pattern::ListIndex(l)) => l.set_resolved(state, lang, value),
                    None | Some(_) => {
                        content.set_value(value);
                        Ok(true)
                    }
                }
            }
            Container::Accessor(a) => a.set_resolved(state, lang, value),
            Container::ListIndex(l) => l.set_resolved(state, lang, value),
            Container::FunctionCall(f) => bail!(
                "You cannot assign to the result of a function call: {:?}",
                f
            ),
        }
    }

    pub fn get_pattern_or_resolved<'a, 'b>(
        &'a self,
        state: &'b State<'a, Q>,
        lang: &Q::Language<'a>,
    ) -> Result<Option<PatternOrResolved<'a, 'b, Q>>> {
        match self {
            Container::Variable(v) => v.get_pattern_or_resolved(state),
            Container::Accessor(a) => a.get(state, lang),
            Container::ListIndex(a) => a.get(state, lang),
            Container::FunctionCall(f) => {
                bail!("You cannot get the value of a function call: {:?}", f)
            }
        }
    }

    pub fn get_pattern_or_resolved_mut<'a, 'b>(
        &'a self,
        state: &'b mut State<'a, Q>,
        lang: &Q::Language<'a>,
    ) -> Result<Option<PatternOrResolvedMut<'a, 'b, Q>>> {
        match self {
            Container::Variable(v) => v.get_pattern_or_resolved_mut(state),
            Container::Accessor(a) => a.get_mut(state, lang),
            Container::ListIndex(l) => l.get_mut(state, lang),
            Container::FunctionCall(f) => {
                bail!("You cannot get the value of a function call: {:?}", f)
            }
        }
    }
}
