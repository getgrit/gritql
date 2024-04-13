use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::ProblemContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct GritMap<P: ProblemContext> {
    pub elements: BTreeMap<String, Pattern<P>>,
}

impl<P: ProblemContext> GritMap<P> {
    pub(crate) fn new(elements: BTreeMap<String, Pattern<P>>) -> Self {
        Self { elements }
    }

    pub(crate) fn get(&self, key: &str) -> Option<&Pattern<P>> {
        self.elements.get(key)
    }
}

impl<P: ProblemContext> PatternName for GritMap<P> {
    fn name(&self) -> &'static str {
        "MAP"
    }
}

impl<P: ProblemContext> Matcher<P> for GritMap<P> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        if let ResolvedPattern::Map(map) = binding {
            for element in map.iter() {
                if let Some(pattern) = self.elements.get(element.0) {
                    if !pattern.execute(element.1, state, context, logs)? {
                        return Ok(false);
                    }
                } else {
                    return Ok(false);
                }
            }
            for element in self.elements.iter() {
                if !map.contains_key(element.0) {
                    return Ok(false);
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
