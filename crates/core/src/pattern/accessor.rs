use super::{
    container::{Container, PatternOrResolved, PatternOrResolvedMut},
    map::GritMap,
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
    variable::Variable,
};
use crate::{
    binding::Binding,
    context::{ExecContext, QueryContext},
};
use anyhow::{bail, Result};
use grit_util::AnalysisLogs;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum AccessorMap<Q: QueryContext> {
    Container(Container<Q>),
    Map(GritMap<Q>),
}

#[derive(Debug, Clone)]
pub struct Accessor<Q: QueryContext> {
    pub map: AccessorMap<Q>,
    pub key: AccessorKey,
}

#[derive(Debug, Clone)]
pub enum AccessorKey {
    String(String),
    Variable(Variable),
}

impl<Q: QueryContext> Accessor<Q> {
    pub fn new(map: AccessorMap<Q>, key: AccessorKey) -> Self {
        Self { map, key }
    }

    fn get_key<'a>(&'a self, state: &State<'a, Q>, lang: &Q::Language<'a>) -> Result<Cow<'a, str>> {
        match &self.key {
            AccessorKey::String(s) => Ok(Cow::Borrowed(s)),
            AccessorKey::Variable(v) => v.text(state, lang),
        }
    }

    pub(crate) fn get<'a, 'b>(
        &'a self,
        state: &'b State<'a, Q>,
        lang: &Q::Language<'a>,
    ) -> Result<Option<PatternOrResolved<'a, 'b, Q>>> {
        let key = self.get_key(state, lang)?;
        match &self.map {
            AccessorMap::Container(c) => match c.get_pattern_or_resolved(state, lang)? {
                None => Ok(None),
                Some(PatternOrResolved::Pattern(Pattern::Map(m))) => {
                    Ok(m.get(&key).map(PatternOrResolved::Pattern))
                }
                Some(PatternOrResolved::Resolved(resolved)) => match resolved.get_map() {
                    Some(m) => Ok(m.get(key.as_ref()).map(PatternOrResolved::Resolved)),
                    None => bail!("left side of an accessor must be a map"),
                },
                Some(_) => bail!("left side of an accessor must be a map"),
            },
            AccessorMap::Map(m) => Ok(m.get(&key).map(PatternOrResolved::Pattern)),
        }
    }

    pub(crate) fn get_mut<'a, 'b>(
        &'a self,
        state: &'b mut State<'a, Q>,
        lang: &Q::Language<'a>,
    ) -> Result<Option<PatternOrResolvedMut<'a, 'b, Q>>> {
        let key = self.get_key(state, lang)?;
        match &self.map {
            AccessorMap::Container(c) => match c.get_pattern_or_resolved_mut(state, lang)? {
                None => Ok(None),
                Some(PatternOrResolvedMut::Pattern(Pattern::Map(m))) => {
                    Ok(m.get(&key).map(PatternOrResolvedMut::Pattern))
                }
                Some(PatternOrResolvedMut::Resolved(resolved)) => match resolved.get_map_mut() {
                    Some(m) => Ok(m.get_mut(key.as_ref()).map(PatternOrResolvedMut::Resolved)),
                    None => bail!("left side of an accessor must be a map"),
                },
                Some(_) => bail!("left side of an accessor must be a map"),
            },
            AccessorMap::Map(m) => Ok(m.get(&key).map(PatternOrResolvedMut::Pattern)),
        }
    }

    pub(crate) fn set_resolved<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        lang: &Q::Language<'a>,
        value: Q::ResolvedPattern<'a>,
    ) -> Result<bool> {
        match &self.map {
            AccessorMap::Container(c) => {
                let key = self.get_key(state, lang)?;
                match c.get_pattern_or_resolved_mut(state, lang)? {
                    None => Ok(false),
                    Some(PatternOrResolvedMut::Resolved(resolved)) => {
                        if let Some(m) = resolved.get_map_mut() {
                            m.insert(key.to_string(), value);
                            Ok(true)
                        } else {
                            bail!("accessor can only mutate a resolved map");
                        }
                    }
                    Some(_) => bail!("accessor can only mutate a resolved map"),
                }
            }
            AccessorMap::Map(_) => bail!("cannot mutate a map literal"),
        }
    }
}

impl<Q: QueryContext> PatternName for Accessor<Q> {
    fn name(&self) -> &'static str {
        "ACCESSOR"
    }
}

impl<Q: QueryContext> Matcher<Q> for Accessor<Q> {
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

pub(crate) fn execute_resolved_with_binding<'a, Q: QueryContext>(
    r: &Q::ResolvedPattern<'a>,
    binding: &Q::ResolvedPattern<'a>,
    state: &State<'a, Q>,
    language: &Q::Language<'a>,
) -> Result<bool> {
    if let (Some(r), Some(b)) = (r.get_last_binding(), binding.get_last_binding()) {
        Ok(r.is_equivalent_to(b, language))
    } else {
        Ok(r.text(&state.files, language)? == binding.text(&state.files, language)?)
    }
}
