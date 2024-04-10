use super::{
    container::{Container, PatternOrResolved, PatternOrResolvedMut},
    map::GritMap,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
    variable::Variable,
};
use crate::{binding::Constant, context::Context};
use anyhow::{bail, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub enum AccessorMap {
    Container(Container),
    Map(GritMap),
}

#[derive(Debug, Clone)]
pub struct Accessor {
    pub map: AccessorMap,
    pub key: AccessorKey,
}

#[derive(Debug, Clone)]
pub enum AccessorKey {
    String(String),
    Variable(Variable),
}

impl Accessor {
    pub fn new(map: AccessorMap, key: AccessorKey) -> Self {
        Self { map, key }
    }

    fn get_key<'a>(&'a self, state: &State<'a>) -> Result<Cow<'a, str>> {
        match &self.key {
            AccessorKey::String(s) => Ok(Cow::Borrowed(s)),
            AccessorKey::Variable(v) => v.text(state),
        }
    }

    pub(crate) fn get<'a, 'b>(
        &'a self,
        state: &'b State<'a>,
    ) -> Result<Option<PatternOrResolved<'a, 'b>>> {
        let key = self.get_key(state)?;
        match &self.map {
            AccessorMap::Container(c) => match c.get_pattern_or_resolved(state)? {
                None => Ok(None),
                Some(PatternOrResolved::Pattern(Pattern::Map(m))) => {
                    Ok(m.get(&key).map(PatternOrResolved::Pattern))
                }
                Some(PatternOrResolved::Resolved(ResolvedPattern::Map(m))) => {
                    Ok(m.get(key.as_ref()).map(PatternOrResolved::Resolved))
                }
                Some(_) => bail!("left side of an accessor must be a map"),
            },
            AccessorMap::Map(m) => Ok(m.get(&key).map(PatternOrResolved::Pattern)),
        }
    }

    pub(crate) fn get_mut<'a, 'b>(
        &'a self,
        state: &'b mut State<'a>,
    ) -> Result<Option<PatternOrResolvedMut<'a, 'b>>> {
        let key = self.get_key(state)?;
        match &self.map {
            AccessorMap::Container(c) => match c.get_pattern_or_resolved_mut(state)? {
                None => Ok(None),
                Some(PatternOrResolvedMut::Pattern(Pattern::Map(m))) => {
                    Ok(m.get(&key).map(PatternOrResolvedMut::Pattern))
                }
                Some(PatternOrResolvedMut::Resolved(ResolvedPattern::Map(m))) => {
                    Ok(m.get_mut(key.as_ref()).map(PatternOrResolvedMut::Resolved))
                }
                Some(_) => bail!("left side of an accessor must be a map"),
            },
            AccessorMap::Map(m) => Ok(m.get(&key).map(PatternOrResolvedMut::Pattern)),
        }
    }

    pub(crate) fn set_resolved<'a>(
        &'a self,
        state: &mut State<'a>,
        value: ResolvedPattern<'a>,
    ) -> Result<Option<ResolvedPattern<'a>>> {
        match &self.map {
            AccessorMap::Container(c) => {
                let key = self.get_key(state)?;
                match c.get_pattern_or_resolved_mut(state)? {
                    None => Ok(None),
                    Some(PatternOrResolvedMut::Resolved(ResolvedPattern::Map(m))) => {
                        Ok(m.insert(key.to_string(), value))
                    }
                    Some(_) => bail!("accessor can only mutate a resolved map"),
                }
            }
            AccessorMap::Map(_) => bail!("cannot mutate a map literal"),
        }
    }
}

impl Name for Accessor {
    fn name(&self) -> &'static str {
        "ACCESSOR"
    }
}

impl Matcher for Accessor {
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

pub(crate) fn execute_resolved_with_binding<'a>(
    r: &ResolvedPattern<'a>,
    binding: &ResolvedPattern<'a>,
    state: &State<'a>,
) -> Result<bool> {
    if let ResolvedPattern::Binding(r) = r {
        if let ResolvedPattern::Binding(b) = binding {
            if let (Some(r), Some(b)) = (r.last(), b.last()) {
                return Ok(r.is_equivalent_to(b));
            } else {
                bail!("Resolved pattern missing binding")
            }
        }
    }
    Ok(r.text(&state.files)? == binding.text(&state.files)?)
}
