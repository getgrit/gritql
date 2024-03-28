use super::{
    compiler::CompilationContext,
    list_index,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
    variable::VariableSourceLocations,
};
use crate::context::Context;
use anyhow::{anyhow, bail, Result};
use core::fmt::Debug;
use marzano_language::language::Field;
use marzano_util::analysis_logs::AnalysisLogs;
use std::{borrow::Cow, collections::BTreeMap};
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct List {
    pub patterns: Vec<Pattern>,
}

impl List {
    pub(crate) fn new(patterns: Vec<Pattern>) -> Self {
        Self { patterns }
    }

    pub(crate) fn get(&self, index: isize) -> Option<&Pattern> {
        self.patterns
            .get(list_index::to_unsigned(index, self.patterns.len())?)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_node_in_context(
        node: Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        context_field: &Field,
        global_vars: &mut BTreeMap<String, usize>,
        is_rhs: bool,
        logs: &mut AnalysisLogs,
    ) -> Result<Pattern> {
        let kind = node.kind();
        match kind.as_ref() {
            "assocNode" => {
                if !context_field.multiple() {
                    bail!(
                        "Field {} does not accept list patterns",
                        context_field.name()
                    )
                }
                Ok(Pattern::List(Box::new(Self::from_node(
                    &node,
                    context,
                    vars,
                    vars_array,
                    scope_index,
                    global_vars,
                    is_rhs,
                    logs,
                )?)))
            }
            _ => Pattern::from_node(
                &node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                false,
                logs,
            ),
        }
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
            .children_by_field_name("patterns", &mut cursor)
            .filter(|n| n.is_named());
        let mut patterns = Vec::new();
        for pattern in children {
            patterns.push(Pattern::from_node(
                &pattern,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                is_rhs,
                logs,
            )?);
        }
        Ok(Self::new(patterns))
    }
}

impl Name for List {
    fn name(&self) -> &'static str {
        "LIST"
    }
}

impl Matcher for List {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut super::state::State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        match binding {
            ResolvedPattern::Binding(v) => {
                let Some(list_items) = v.last().and_then(|b| b.list_items()) else {
                    return Ok(false);
                };

                let children: Vec<Cow<ResolvedPattern>> = list_items
                    .map(ResolvedPattern::from_node)
                    .map(Cow::Owned)
                    .collect();

                execute_assoc(&self.patterns, &children, state, context, logs)
            }
            ResolvedPattern::List(patterns) => {
                let patterns: Vec<Cow<ResolvedPattern<'_>>> =
                    patterns.into_iter().map(Cow::Borrowed).collect();
                execute_assoc(&self.patterns, &patterns, state, context, logs)
            }
            ResolvedPattern::Snippets(_)
            | ResolvedPattern::File(_)
            | ResolvedPattern::Files(_)
            | ResolvedPattern::Map(_)
            | ResolvedPattern::Constant(_) => Ok(false),
        }
    }
}

fn execute_assoc<'a>(
    patterns: &'a [Pattern],
    children: &[Cow<ResolvedPattern<'a>>],
    current_state: &mut State<'a>,
    context: &'a impl Context,
    logs: &mut AnalysisLogs,
) -> Result<bool> {
    let mut working_state = current_state.clone();
    match patterns {
        // short circuit for common case
        [pattern_for_first_node, Pattern::Dots] => {
            if children.is_empty() {
                return Ok(false);
            }
            let first_node = children[0].clone();
            if pattern_for_first_node.execute(&first_node, &mut working_state, context, logs)? {
                *current_state = working_state;
                Ok(true)
            } else {
                Ok(false)
            }
        }
        // short circuit for common case
        [Pattern::Dots, pattern_for_last_node] => {
            if let Some(last_node) = children.last() {
                if pattern_for_last_node.execute(last_node, &mut working_state, context, logs)? {
                    *current_state = working_state;
                    Ok(true)
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        }
        [Pattern::Dots, head_pattern, tail_patterns @ ..] => {
            if let Pattern::Dots = head_pattern {
                return Err(anyhow!("Multiple subsequent dots are not allowed."));
            }
            for index in 0..children.len() {
                if head_pattern.execute(&children[index], &mut working_state, context, logs)?
                    && execute_assoc(
                        tail_patterns,
                        &children[index + 1..],
                        &mut working_state,
                        context,
                        logs,
                    )?
                {
                    *current_state = working_state;
                    return Ok(true);
                }
            }
            Ok(false)
        }
        [Pattern::Dots] => Ok(true),
        [only_pattern] => {
            if children.len() == 1 {
                if only_pattern.execute(&children[0], &mut working_state, context, logs)? {
                    *current_state = working_state;
                    Ok(true)
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        }
        [head_pattern, tail_patterns @ ..] => match children {
            [head_node, tail_nodes @ ..] => {
                if head_pattern.execute(head_node, &mut working_state, context, logs)? {
                    if let Ok(true) =
                        execute_assoc(tail_patterns, tail_nodes, &mut working_state, context, logs)
                    {
                        *current_state = working_state;
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            [] => Ok(false),
        },
        [] => Ok(children.is_empty()),
    }
}
