use super::{
    compiler::CompilationContext,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    variable::VariableSourceLocations,
    Node, State,
};
use crate::context::Context;
use anyhow::{anyhow, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Includes {
    pub(crate) includes: Pattern,
}

impl Includes {
    pub fn new(includes: Pattern) -> Self {
        Self { includes }
    }

    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        let includes = node
            .child_by_field_name("includes")
            .ok_or_else(|| anyhow!("missing includes of patternIncludes"))?;
        let includes = Pattern::from_node(
            &includes,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        Ok(Self::new(includes))
    }
}

impl Name for Includes {
    fn name(&self) -> &'static str {
        "INCLUDES"
    }
}

// Includes and within should call the same function taking an iterator as an argument
// even better two arguments an accumulator and an iterator.
impl Matcher for Includes {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let resolved = ResolvedPattern::from_pattern(&self.includes, state, context, logs)?;
        let substring = resolved.text(&state.files)?;
        let string = binding.text(&state.files)?;
        if string.contains(&*substring) {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
