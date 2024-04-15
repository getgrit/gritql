use super::{
    container::{Container, PatternOrResolved, PatternOrResolvedMut},
    map::GritMap,
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
    variable::Variable,
};
use crate::{
    binding::Constant,
    context::{ExecContext, QueryContext},
};
use anyhow::{bail, Result};
use marzano_language::language::Language;
use marzano_util::analysis_logs::AnalysisLogs;
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

    fn get_key<'a>(&'a self, state: &State<'a, Q>, lang: &impl Language) -> Result<Cow<'a, str>> {
        match &self.key {
            AccessorKey::String(s) => Ok(Cow::Borrowed(s)),
            AccessorKey::Variable(v) => v.text(state, lang),
        }
    }

    pub(crate) fn get<'a, 'b>(
        &'a self,
        state: &'b State<'a, Q>,
        lang: &impl Language,
    ) -> Result<Option<PatternOrResolved<'a, 'b, Q>>> {
        let key = self.get_key(state, lang)?;
        match &self.map {
            AccessorMap::Container(c) => match c.get_pattern_or_resolved(state, lang)? {
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
        state: &'b mut State<'a, Q>,
        lang: &impl Language,
    ) -> Result<Option<PatternOrResolvedMut<'a, 'b, Q>>> {
        let key = self.get_key(state, lang)?;
        match &self.map {
            AccessorMap::Container(c) => match c.get_pattern_or_resolved_mut(state, lang)? {
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
        state: &mut State<'a, Q>,
        lang: &impl Language,
        value: ResolvedPattern<'a>,
    ) -> Result<Option<ResolvedPattern<'a>>> {
        match &self.map {
            AccessorMap::Container(c) => {
                let key = self.get_key(state, lang)?;
                match c.get_pattern_or_resolved_mut(state, lang)? {
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

impl<Q: QueryContext> PatternName for Accessor<Q> {
    fn name(&self) -> &'static str {
        "ACCESSOR"
    }
}

impl<Q: QueryContext> Matcher<Q> for Accessor<Q> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
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
            None => Ok(
                matches!(binding, ResolvedPattern::Constant(Constant::Boolean(false)))
                    || binding.matches_undefined(),
            ),
        }
    }
}

pub(crate) fn execute_resolved_with_binding<'a, Q: QueryContext>(
    r: &ResolvedPattern<'a>,
    binding: &ResolvedPattern<'a>,
    state: &State<'a, Q>,
    lang: &impl Language,
) -> Result<bool> {
    if let ResolvedPattern::Binding(r) = r {
        if let ResolvedPattern::Binding(b) = binding {
            if let (Some(r), Some(b)) = (r.last(), b.last()) {
                return Ok(r.is_equivalent_to(b, lang));
            } else {
                bail!("Resolved pattern missing binding")
            }
        }
    }
    Ok(r.text(&state.files, lang)? == binding.text(&state.files, lang)?)
}
