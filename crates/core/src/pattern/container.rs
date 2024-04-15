use crate::context::QueryContext;

use super::{
    accessor::Accessor, list_index::ListIndex, patterns::Pattern,
    resolved_pattern::ResolvedPattern, state::State, variable::Variable,
};
use anyhow::Result;

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
}

#[derive(Debug)]
pub(crate) enum PatternOrResolved<'a, 'b, Q: QueryContext> {
    Pattern(&'a Pattern<Q>),
    Resolved(&'b ResolvedPattern<'a, Q>),
    ResolvedBinding(ResolvedPattern<'a, Q>),
}

#[derive(Debug)]
pub(crate) enum PatternOrResolvedMut<'a, 'b, Q: QueryContext> {
    Pattern(&'a Pattern<Q>),
    Resolved(&'b mut ResolvedPattern<'a, Q>),
    _ResolvedBinding,
}

impl<Q: QueryContext> Container<Q> {
    pub(crate) fn set_resolved<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        value: ResolvedPattern<'a, Q>,
    ) -> Result<Option<ResolvedPattern<'a, Q>>> {
        match self {
            Container::Variable(v) => {
                let var = state.trace_var(v);
                let content = &mut state.bindings[var.scope].back_mut().unwrap()[var.index];
                match content.pattern {
                    Some(Pattern::Accessor(a)) => a.set_resolved(state, value),
                    Some(Pattern::ListIndex(l)) => l.set_resolved(state, value),
                    None | Some(_) => Ok(content.set_value(value)),
                }
            }
            Container::Accessor(a) => a.set_resolved(state, value),
            Container::ListIndex(l) => l.set_resolved(state, value),
        }
    }

    pub(crate) fn get_pattern_or_resolved<'a, 'b>(
        &'a self,
        state: &'b State<'a, Q>,
    ) -> Result<Option<PatternOrResolved<'a, 'b, Q>>> {
        match self {
            Container::Variable(v) => v.get_pattern_or_resolved(state),
            Container::Accessor(a) => a.get(state),
            Container::ListIndex(a) => a.get(state),
        }
    }

    pub(crate) fn get_pattern_or_resolved_mut<'a, 'b>(
        &'a self,
        state: &'b mut State<'a, Q>,
    ) -> Result<Option<PatternOrResolvedMut<'a, 'b, Q>>> {
        match self {
            Container::Variable(v) => v.get_pattern_or_resolved_mut(state),
            Container::Accessor(a) => a.get_mut(state),
            Container::ListIndex(l) => l.get_mut(state),
        }
    }
}
