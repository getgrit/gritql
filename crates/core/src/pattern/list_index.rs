use super::{
    accessor::execute_resolved_with_binding,
    container::{Container, PatternOrResolved, PatternOrResolvedMut},
    list::List,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::{
    binding::{Binding, Constant},
    context::Context,
    resolve_opt,
};
use anyhow::{anyhow, bail, Result};
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub enum ListOrContainer {
    Container(Container),
    List(List),
}

#[derive(Debug, Clone)]
pub enum ContainerOrIndex {
    Container(Container),
    Index(isize),
}

#[derive(Debug, Clone)]
pub struct ListIndex {
    pub list: ListOrContainer,
    pub index: ContainerOrIndex,
}

impl ListIndex {
    fn get_index<'a>(&'a self, state: &State<'a>) -> Result<isize> {
        match &self.index {
            ContainerOrIndex::Container(c) => {
                let raw_index = c
                    .get_pattern_or_resolved(state)?
                    .ok_or_else(|| anyhow!("list index must be resolvable: {:?}", self))?;
                let index = match raw_index {
                    PatternOrResolved::Resolved(r) => r.text(&state.files)?,
                    PatternOrResolved::ResolvedBinding(r) => r.text(&state.files)?,
                    PatternOrResolved::Pattern(_) => bail!("list index must be resolved"),
                };
                let int_index = index
                    .parse::<isize>()
                    .map_err(|_| anyhow!("list index must be an integer but got {:?}", index))?;
                Ok(int_index)
            }
            ContainerOrIndex::Index(i) => Ok(*i),
        }
    }

    pub(crate) fn get<'a, 'b>(
        &'a self,
        state: &'b State<'a>,
    ) -> Result<Option<PatternOrResolved<'a, 'b>>> {
        let index = self.get_index(state)?;
        match &self.list {
            ListOrContainer::Container(c) => match c.get_pattern_or_resolved(state)? {
                None => Ok(None),
                Some(PatternOrResolved::Pattern(Pattern::List(l))) => {
                    Ok(l.get(index).map(PatternOrResolved::Pattern))
                }
                Some(PatternOrResolved::Resolved(ResolvedPattern::Binding(b))) => {
                    let mut list_items = b
                        .last()
                        .and_then(Binding::list_items)
                        .ok_or_else(|| anyhow!("left side of a listIndex must be a list"))?;

                    let len = list_items.clone().count();
                    let index = resolve_opt!(to_unsigned(index, len));
                    return Ok(list_items.nth(index).map(|n| {
                        PatternOrResolved::ResolvedBinding(ResolvedPattern::from_node(n))
                    }));
                }
                Some(PatternOrResolved::Resolved(ResolvedPattern::List(l))) => {
                    let index = resolve_opt!(to_unsigned(index, l.len()));
                    Ok(l.get(index).map(PatternOrResolved::Resolved))
                }
                Some(s) => bail!("left side of a listIndex must be a list but got {:?}", s),
            },
            ListOrContainer::List(l) => Ok(l.get(index).map(PatternOrResolved::Pattern)),
        }
    }

    pub(crate) fn get_mut<'a, 'b>(
        &'a self,
        state: &'b mut State<'a>,
    ) -> Result<Option<PatternOrResolvedMut<'a, 'b>>> {
        let index = self.get_index(state)?;
        match &self.list {
            ListOrContainer::Container(c) => match c.get_pattern_or_resolved_mut(state)? {
                None => Ok(None),
                Some(PatternOrResolvedMut::Pattern(Pattern::List(l))) => {
                    Ok(l.get(index).map(PatternOrResolvedMut::Pattern))
                }
                Some(PatternOrResolvedMut::Resolved(ResolvedPattern::Binding(b))) => {
                    let mut list_items = b
                        .last()
                        .and_then(Binding::list_items)
                        .ok_or_else(|| anyhow!("left side of a listIndex must be a list"))?;

                    let len = list_items.clone().count();
                    let index = resolve_opt!(to_unsigned(index, len));
                    Ok(list_items
                        .nth(index)
                        .map(|_| PatternOrResolvedMut::_ResolvedBinding))
                }
                Some(PatternOrResolvedMut::Resolved(ResolvedPattern::List(l))) => {
                    let index = resolve_opt!(to_unsigned(index, l.len()));
                    Ok(l.get_mut(index).map(PatternOrResolvedMut::Resolved))
                }
                Some(s) => bail!("left side of a listIndex must be a list but got {:?}", s),
            },
            ListOrContainer::List(l) => Ok(l.get(index).map(PatternOrResolvedMut::Pattern)),
        }
    }

    pub(crate) fn set_resolved<'a>(
        &'a self,
        state: &mut State<'a>,
        value: ResolvedPattern<'a>,
    ) -> Result<Option<ResolvedPattern<'a>>> {
        let index = self.get_index(state)?;
        match &self.list {
            ListOrContainer::Container(c) => match c.get_pattern_or_resolved_mut(state)? {
                None => Ok(None),
                Some(PatternOrResolvedMut::Resolved(ResolvedPattern::List(l))) => {
                    let index = resolve_opt!(to_unsigned(index, l.len()));
                    Ok(Some(l.set(index, value)))
                }
                Some(_) => bail!("accessor can only mutate a resolved list"),
            },
            ListOrContainer::List(_) => bail!("cannot mutate a list literal"),
        }
    }
}

pub(crate) fn to_unsigned(index: isize, len: usize) -> Option<usize> {
    if index >= 0 {
        Some(index as usize)
    } else if len as isize + index < 0 {
        None
    } else {
        Some((len as isize + index) as usize)
    }
}

impl Name for ListIndex {
    fn name(&self) -> &'static str {
        "LIST_INDEX"
    }
}

impl Matcher for ListIndex {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        match self.get(state)? {
            Some(PatternOrResolved::Resolved(r)) => {
                execute_resolved_with_binding(r, binding, state)
            }
            Some(PatternOrResolved::ResolvedBinding(r)) => {
                execute_resolved_with_binding(&r, binding, state)
            }
            Some(PatternOrResolved::Pattern(p)) => p.execute(binding, state, context, logs),
            None => Ok(
                matches!(binding, ResolvedPattern::Constant(Constant::Boolean(false)))
                    || binding.matches_undefined(),
            ),
        }
    }
}
