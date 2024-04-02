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
pub struct Before {
    pub(crate) before: Pattern,
}

impl Before {
    pub fn new(before: Pattern) -> Self {
        Self { before }
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
            .ok_or_else(|| anyhow!("missing pattern of patternBefore"))?;
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

    pub(crate) fn prev_pattern<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>> {
        let binding = pattern_to_binding(&self.before, state, context, logs)?;
        let Some(node) = binding.as_node() else {
            bail!("cannot get the node before this binding")
        };

        if let Some(prev) = node.previous_non_trivia_node() {
            Ok(ResolvedPattern::from_node(prev))
        } else {
            debug(
                logs,
                state,
                "no node before current node, treating as undefined",
            )?;
            Ok(ResolvedPattern::Constant(Constant::Undefined))
        }
    }
}

impl Name for Before {
    fn name(&self) -> &'static str {
        "BEFORE"
    }
}

impl Matcher for Before {
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
        let next_node = resolve!(node.next_non_trivia_node());
        if !self.before.execute(
            &ResolvedPattern::from_node(next_node),
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
