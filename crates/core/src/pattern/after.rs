use super::{
    before::Before,
    compiler::CompilationContext,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::{pattern_to_binding, ResolvedPattern},
    variable::VariableSourceLocations,
    Node, State,
};
use crate::{binding::Binding, context::Context, resolve};
use crate::{binding::Constant, errors::debug};
use anyhow::{anyhow, bail, Result};
use core::fmt::Debug;
use im::vector;
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
        let (source, node) = match binding {
            Binding::String(_, _) => bail!("cannot get the node before a string"),
            Binding::FileName(_) => bail!("cannot get the node before a filename"),
            Binding::Node(s, n) => (s, n),
            Binding::List(s, n, _) => (s, n),
            Binding::Empty(s, n, _) => (s, n),
            Binding::ConstantRef(_) => bail!("cannot get the node before a constant"),
        };
        if let Some(prev) = Before::next_node(node) {
            Ok(ResolvedPattern::Binding(vector![Binding::Node(
                source, prev
            )]))
        } else {
            debug(
                logs,
                state,
                "no node after current node, treating as undefined",
            )?;
            Ok(ResolvedPattern::Constant(Constant::Undefined))
        }
    }

    pub(crate) fn prev_node(node: Node) -> Option<Node> {
        let mut current_node = node;
        loop {
            if let Some(sibling) = current_node.prev_named_sibling() {
                return Some(sibling);
            }
            current_node = current_node.parent()?;
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
        let (src, node) = match binding {
            Binding::Empty(_, _, _) => return Ok(true),
            Binding::Node(src, node) => (src, node.to_owned()),
            Binding::String(_, _) => return Ok(true),
            Binding::List(src, node, field) => (src, resolve!(node.child_by_field_id(*field))),
            Binding::ConstantRef(_) => return Ok(true),
            Binding::FileName(_) => return Ok(true),
        };
        let after_node = resolve!(Self::prev_node(node.clone()));
        if !self.after.execute(
            &ResolvedPattern::from_node(src, after_node),
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
