use std::collections::BTreeMap;

use super::{
    accessor::Accessor,
    compiler::CompilationContext,
    list_index::ListIndex,
    patterns::Pattern,
    resolved_pattern::ResolvedPattern,
    state::State,
    variable::{Variable, VariableSourceLocations},
};
use anyhow::{bail, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub(crate) enum Container {
    Variable(Variable),
    Accessor(Box<Accessor>),
    ListIndex(Box<ListIndex>),
}

#[derive(Debug)]
pub(crate) enum PatternOrResolved<'a, 'b> {
    Pattern(&'a Pattern),
    Resolved(&'b ResolvedPattern<'a>),
    ResolvedBinding(ResolvedPattern<'a>),
}

#[derive(Debug)]
pub(crate) enum PatternOrResolvedMut<'a, 'b> {
    Pattern(&'a Pattern),
    Resolved(&'b mut ResolvedPattern<'a>),
    _ResolvedBinding,
}

// A Container represents anything which "contains" a reference to a Pattern
// We have three types of containers:
// - Variable: a variable reference (ex. $foo)
// - Accessor: a map accessor (ex. $foo.bar)
// - ListIndex: a list index (ex. $foo[0])
impl Container {
    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        match node.kind().as_ref() {
            "variable" => Ok(Self::Variable(Variable::from_node(
                node,
                context.file,
                context.src,
                vars,
                global_vars,
                vars_array,
                scope_index,
            )?)),
            "mapAccessor" => Ok(Self::Accessor(Box::new(Accessor::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "listIndex" => Ok(Self::ListIndex(Box::new(ListIndex::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            s => bail!("Invalid kind for container: {}", s),
        }
    }

    pub(crate) fn set_resolved<'a>(
        &'a self,
        state: &mut State<'a>,
        value: ResolvedPattern<'a>,
    ) -> Result<Option<ResolvedPattern<'a>>> {
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
        state: &'b State<'a>,
    ) -> Result<Option<PatternOrResolved<'a, 'b>>> {
        match self {
            Container::Variable(v) => v.get_pattern_or_resolved(state),
            Container::Accessor(a) => a.get(state),
            Container::ListIndex(a) => a.get(state),
        }
    }

    pub(crate) fn get_pattern_or_resolved_mut<'a, 'b>(
        &'a self,
        state: &'b mut State<'a>,
    ) -> Result<Option<PatternOrResolvedMut<'a, 'b>>> {
        match self {
            Container::Variable(v) => v.get_pattern_or_resolved_mut(state),
            Container::Accessor(a) => a.get_mut(state),
            Container::ListIndex(l) => l.get_mut(state),
        }
    }
}
