use std::{collections::BTreeMap, ops};

use marzano_util::analysis_logs::AnalysisLogs;
use tree_sitter::Node;

use crate::{context::Context, pattern::Step};

use super::{
    compiler::CompilationContext,
    files::Files,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
    variable::VariableSourceLocations,
};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct Sequential(pub(crate) Vec<Step>);

impl Sequential {
    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        let mut sequential = vec![];
        let mut cursor = node.walk();
        for n in node
            .children_by_field_name("sequential", &mut cursor)
            .filter(|n| n.is_named())
        {
            let step = Step::from_node(
                &n,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?;
            sequential.push(step);
        }
        Ok(sequential.into())
    }

    pub(crate) fn from_files_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        let mut sequential = vec![];
        let mut cursor = node.walk();
        for n in node
            .children_by_field_name("files", &mut cursor)
            .filter(|n| n.is_named())
        {
            let step = Step::from_node(
                &n,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?;
            let some = Pattern::Some(Box::new(super::some::Some::new(step.pattern)));
            let files = Pattern::Files(Box::new(Files::new(some)));
            let step = Step { pattern: files };
            sequential.push(step);
        }
        Ok(sequential.into())
    }
}

impl Matcher for Sequential {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        for step in &self.0 {
            if !step.execute(binding, state, context, logs)? {
                return Ok(false);
            }
        }
        Ok(true)
    }
}

impl From<Vec<Step>> for Sequential {
    fn from(logs: Vec<Step>) -> Self {
        Self(logs)
    }
}

impl ops::Deref for Sequential {
    type Target = Vec<Step>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Name for Sequential {
    fn name(&self) -> &'static str {
        "SEQUENTIAL"
    }
}
