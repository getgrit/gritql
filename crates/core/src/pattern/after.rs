use super::{
    compiler::CompilationContext,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::{pattern_to_binding, ResolvedPattern},
    variable::VariableSourceLocations,
    Node, State,
};
use crate::{binding::Constant, errors::debug};
use crate::{context::Context, resolve};
use anyhow::{anyhow, bail, Result};
use core::fmt::Debug;
use grit_util::AstNode;
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct After {
    pub(crate) after: Pattern,
}

impl After {
    pub fn new(after: Pattern) -> Self {
        Self { after }
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
        let pattern = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern of patternAfter"))?;
        let pattern = Pattern::from_node(
            &pattern,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        Ok(Self::new(pattern))
    }

    pub(crate) fn next_pattern<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>> {
        let binding = pattern_to_binding(&self.after, state, context, logs)?;
        let Some(node) = binding.as_node() else {
            bail!("cannot get the node after this binding")
        };

        if let Some(next) = node.next_non_trivia_node() {
            Ok(ResolvedPattern::from_node(next))
        } else {
            debug(
                logs,
                state,
                "no node after current node, treating as undefined",
            )?;
            Ok(ResolvedPattern::Constant(Constant::Undefined))
        }
    }
}

impl Name for After {
    fn name(&self) -> &'static str {
        "AFTER"
    }
}

impl Matcher for After {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let binding = match binding {
            ResolvedPattern::Binding(binding) => resolve!(binding.last()),
            ResolvedPattern::Snippets(_)
            | ResolvedPattern::List(_)
            | ResolvedPattern::Map(_)
            | ResolvedPattern::File(_)
            | ResolvedPattern::Files(_)
            | ResolvedPattern::Constant(_) => return Ok(true),
        };
        let mut cur_state = init_state.clone();
        // todo implement for empty and empty list
        let Some(node) = binding.as_node() else {
            return Ok(true);
        };
        let prev_node = resolve!(node.previous_non_trivia_node());
        if !self.after.execute(
            &ResolvedPattern::from_node(prev_node),
            &mut cur_state,
            context,
            logs,
        )? {
            return Ok(false);
        }
        *init_state = cur_state;
        Ok(true)
    }
}
