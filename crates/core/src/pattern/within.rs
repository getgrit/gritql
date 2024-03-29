use super::{
    compiler::CompilationContext,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    variable::VariableSourceLocations,
    State,
};
use crate::{context::Context, resolve};
use anyhow::{anyhow, Result};
use core::fmt::Debug;
use marzano_language::parent_traverse::{ParentTraverse, TreeSitterParentCursor};
use marzano_util::{analysis_logs::AnalysisLogs, node_with_source::NodeWithSource};
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct Within {
    pub(crate) pattern: Pattern,
}

impl Within {
    pub fn new(pattern: Pattern) -> Self {
        Self { pattern }
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
        let within = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern of pattern within"))?;
        let within = Pattern::from_node(
            &within,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        Ok(Self::new(within))
    }
}

impl Name for Within {
    fn name(&self) -> &'static str {
        "WITHIN"
    }
}

impl Matcher for Within {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let mut did_match = false;
        let mut cur_state = init_state.clone();

        let state = cur_state.clone();
        if self
            .pattern
            .execute(binding, &mut cur_state, context, logs)?
        {
            did_match = true;
        } else {
            cur_state = state;
        }

        let binding = if let ResolvedPattern::Binding(binding) = binding {
            resolve!(binding.last())
        } else {
            return Ok(did_match);
        };

        let Some(node) = binding.parent_node() else {
            return Ok(did_match);
        };
        for n in ParentTraverse::new(TreeSitterParentCursor::new(node.node)) {
            let state = cur_state.clone();
            if self.pattern.execute(
                &ResolvedPattern::from_node(NodeWithSource::new(n, node.source)),
                &mut cur_state,
                context,
                logs,
            )? {
                did_match = true;
            } else {
                cur_state = state;
            }
        }
        if did_match {
            *init_state = cur_state;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
