use super::{
    accessor::execute_resolved_with_binding,
    container::{Container, PatternOrResolved, PatternOrResolvedMut},
    list::List,
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::{
    binding::Binding,
    context::{ExecContext, QueryContext},
};
use anyhow::{anyhow, bail, Result};
use grit_util::AnalysisLogs;
use marzano_language::language::Language;

#[derive(Debug, Clone)]
pub enum ListOrContainer<Q: QueryContext> {
    Container(Container<Q>),
    List(List<Q>),
}

#[derive(Debug, Clone)]
pub enum ContainerOrIndex<Q: QueryContext> {
    Container(Container<Q>),
    Index(isize),
}

#[derive(Debug, Clone)]
pub struct ListIndex<Q: QueryContext> {
    pub list: ListOrContainer<Q>,
    pub index: ContainerOrIndex<Q>,
}

impl<Q: QueryContext> ListIndex<Q> {
    fn get_index<'a>(&'a self, state: &State<'a, Q>, lang: &impl Language) -> Result<isize> {
        match &self.index {
            ContainerOrIndex::Container(c) => {
                let raw_index = c
                    .get_pattern_or_resolved(state, lang)?
                    .ok_or_else(|| anyhow!("list index must be resolvable: {:?}", self))?;
                let index = match raw_index {
                    PatternOrResolved::Resolved(r) => r.text(&state.files, lang)?,
                    PatternOrResolved::ResolvedBinding(r) => r.text(&state.files, lang)?,
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
        state: &'b State<'a, Q>,
        lang: &impl Language,
    ) -> Result<Option<PatternOrResolved<'a, 'b, Q>>> {
        let index = self.get_index(state, lang)?;
        match &self.list {
            ListOrContainer::Container(c) => match c.get_pattern_or_resolved(state, lang)? {
                None => Ok(None),
                Some(PatternOrResolved::Pattern(Pattern::List(l))) => {
                    Ok(l.get(index).map(PatternOrResolved::Pattern))
                }
                Some(PatternOrResolved::Resolved(resolved)) => {
                    if resolved.is_list() {
                        Ok(resolved
                            .get_list_item_at(index)
                            .map(PatternOrResolved::Resolved))
                    } else if let Some(mut items) =
                        resolved.get_last_binding().and_then(Binding::list_items)
                    {
                        let len = items.clone().count();
                        return Ok(to_unsigned(index, len)
                            .and_then(|index| items.nth(index))
                            .map(|n| {
                                PatternOrResolved::ResolvedBinding(
                                    ResolvedPattern::from_node_binding(n),
                                )
                            }));
                    } else {
                        bail!("left side of a listIndex must be a list")
                    }
                }
                Some(s) => bail!("left side of a listIndex must be a list but got {:?}", s),
            },
            ListOrContainer::List(l) => Ok(l.get(index).map(PatternOrResolved::Pattern)),
        }
    }

    pub(crate) fn get_mut<'a, 'b>(
        &'a self,
        state: &'b mut State<'a, Q>,
        lang: &impl Language,
    ) -> Result<Option<PatternOrResolvedMut<'a, 'b, Q>>> {
        let index = self.get_index(state, lang)?;
        match &self.list {
            ListOrContainer::Container(c) => match c.get_pattern_or_resolved_mut(state, lang)? {
                None => Ok(None),
                Some(PatternOrResolvedMut::Pattern(Pattern::List(l))) => {
                    Ok(l.get(index).map(PatternOrResolvedMut::Pattern))
                }
                Some(PatternOrResolvedMut::Resolved(resolved)) => {
                    if let Some(mut items) = resolved.get_list_binding_items() {
                        let len = items.clone().count();
                        return Ok(to_unsigned(index, len)
                            .and_then(|index| items.nth(index))
                            .map(|_| PatternOrResolvedMut::_ResolvedBinding));
                    }

                    if resolved.is_list() {
                        Ok(resolved
                            .get_list_item_at_mut(index)
                            .map(PatternOrResolvedMut::Resolved))
                    } else {
                        bail!("left side of a listIndex must be a list")
                    }
                }
                Some(s) => bail!("left side of a listIndex must be a list but got {:?}", s),
            },
            ListOrContainer::List(l) => Ok(l.get(index).map(PatternOrResolvedMut::Pattern)),
        }
    }

    pub(crate) fn set_resolved<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        lang: &impl Language,
        value: Q::ResolvedPattern<'a>,
    ) -> Result<bool> {
        let index = self.get_index(state, lang)?;
        match &self.list {
            ListOrContainer::Container(c) => match c.get_pattern_or_resolved_mut(state, lang)? {
                None => Ok(false),
                Some(PatternOrResolvedMut::Resolved(resolved)) => {
                    resolved.set_list_item_at_mut(index, value)
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

impl<Q: QueryContext> PatternName for ListIndex<Q> {
    fn name(&self) -> &'static str {
        "LIST_INDEX"
    }
}

impl<Q: QueryContext> Matcher<Q> for ListIndex<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        match self.get(state, context.language())? {
            Some(PatternOrResolved::Resolved(r)) => {
                execute_resolved_with_binding(r, binding, state, context.language())
            }
            Some(PatternOrResolved::ResolvedBinding(r)) => {
                execute_resolved_with_binding(&r, binding, state, context.language())
            }
            Some(PatternOrResolved::Pattern(p)) => p.execute(binding, state, context, logs),
            None => Ok(binding.matches_false_or_undefined()),
        }
    }
}
