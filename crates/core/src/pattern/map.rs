use super::{
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
};
use crate::context::Context;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct GritMap {
    pub elements: BTreeMap<String, Pattern>,
}

impl GritMap {
    pub(crate) fn new(elements: BTreeMap<String, Pattern>) -> Self {
        Self { elements }
    }

    pub(crate) fn get(&self, key: &str) -> Option<&Pattern> {
        self.elements.get(key)
    }
}

impl Name for GritMap {
    fn name(&self) -> &'static str {
        "MAP"
    }
}

impl Matcher for GritMap {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut super::state::State<'a>,
        context: &'a impl Context,
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
