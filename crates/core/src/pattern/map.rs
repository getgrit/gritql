use super::{
    compiler::CompilationContext,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    variable::VariableSourceLocations,
};
use crate::context::Context;
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

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
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        is_rhs: bool,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        let mut cursor = node.walk();
        let children = node
            .children_by_field_name("elements", &mut cursor)
            .filter(|n| n.is_named());
        let mut elements = BTreeMap::new();
        for element in children {
            let key = element
                .child_by_field_name("key")
                .ok_or_else(|| anyhow!("key not found in map element"))?
                .utf8_text(context.src.as_bytes())?
                .to_string();
            let value = element
                .child_by_field_name("value")
                .ok_or_else(|| anyhow!("value not found in map element"))?;
            let pattern = Pattern::from_node(
                &value,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                is_rhs,
                logs,
            )?;
            elements.insert(key, pattern);
        }
        Ok(Self::new(elements))
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
