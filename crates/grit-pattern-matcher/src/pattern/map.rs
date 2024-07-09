use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::QueryContext;
use crate::errors::GritResult;
use grit_util::AnalysisLogs;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct GritMap<Q: QueryContext> {
    pub elements: BTreeMap<String, Pattern<Q>>,
}

impl<Q: QueryContext> GritMap<Q> {
    pub fn new(elements: BTreeMap<String, Pattern<Q>>) -> Self {
        Self { elements }
    }

    pub fn get(&self, key: &str) -> Option<&Pattern<Q>> {
        self.elements.get(key)
    }
}

impl<Q: QueryContext> PatternName for GritMap<Q> {
    fn name(&self) -> &'static str {
        "MAP"
    }
}

impl<Q: QueryContext> Matcher<Q> for GritMap<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        let Some(map) = binding.get_map() else {
            return Ok(false);
        };

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
    }
}
