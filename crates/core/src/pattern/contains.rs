use super::{
    compiler::CompilationContext,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::{LazyBuiltIn, ResolvedPattern, ResolvedSnippet},
    variable::VariableSourceLocations,
    Node, State,
};
use crate::{context::Context, resolve};

use anyhow::{anyhow, Result};
use core::fmt::Debug;
use im::vector;
use marzano_util::{analysis_logs::AnalysisLogs, node_with_source::NodeWithSource};

use std::collections::BTreeMap;

#[derive(Debug, Clone)]
pub struct Contains {
    pub(crate) contains: Pattern,
    pub(crate) until: Option<Pattern>,
}

impl Contains {
    pub fn new(contains: Pattern, until: Option<Pattern>) -> Self {
        Self { contains, until }
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
        let contains = node
            .child_by_field_name("contains")
            .ok_or_else(|| anyhow!("missing contains of patternContains"))?;
        let contains = Pattern::from_node(
            &contains,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        let until = node.child_by_field_name("until").map(|n| {
            Pattern::from_node(
                &n,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                false,
                logs,
            )
        });
        let until = until.map_or(Ok(None), |v| v.map(Some))?;
        Ok(Self::new(contains, until))
    }
}

impl Name for Contains {
    fn name(&self) -> &'static str {
        "CONTAINS"
    }
}

fn execute_until<'a>(
    init_state: &mut State<'a>,
    node: &Node<'a>,
    src: &'a str,
    context: &'a impl Context,
    logs: &mut AnalysisLogs,
    the_contained: &'a Pattern,
    until: &'a Option<Pattern>,
) -> Result<bool, anyhow::Error> {
    let mut did_match = false;
    let mut cur_state = init_state.clone();
    let mut cursor = node.walk();
    let mut still_computing = true;
    while still_computing {
        let node = cursor.node();
        let node_lhs = ResolvedPattern::from_node(NodeWithSource::new(node, src));

        let state = cur_state.clone();
        if the_contained.execute(&node_lhs, &mut cur_state, context, logs)? {
            did_match = true;
        } else {
            cur_state = state;
        }

        let mut state = cur_state.clone();
        let skip_children = if let Some(until) = until {
            until.execute(&node_lhs, &mut state, context, logs)?
        } else {
            false
        };

        if (!skip_children && cursor.goto_first_child()) || cursor.goto_next_sibling() {
            // all good
            continue;
        }
        // go up the parent chain until we find a sibling
        loop {
            if !cursor.goto_parent() {
                still_computing = false;
                break;
            }

            if cursor.goto_next_sibling() {
                break;
            }
        }
    }
    if did_match {
        *init_state = cur_state;
    }
    Ok(did_match)
}

// Contains and within should call the same function taking an iterator as an argument
// even better two arguments an accumulator and an iterator.
impl Matcher for Contains {
    fn execute<'a>(
        &'a self,
        resolved_pattern: &ResolvedPattern<'a>,
        init_state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        match resolved_pattern {
            ResolvedPattern::Binding(bindings) => {
                let binding = resolve!(bindings.last());
                if let Some(node) = binding.as_node() {
                    execute_until(
                        init_state,
                        &node.node,
                        node.source,
                        context,
                        logs,
                        &self.contains,
                        &self.until,
                    )
                } else if let Some(list_items) = binding.list_items() {
                    let mut did_match = false;
                    let mut cur_state = init_state.clone();
                    for item in list_items {
                        let state = cur_state.clone();
                        if self.execute(
                            &ResolvedPattern::from_node(item),
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
                    }
                    Ok(did_match)
                } else if let Some(_c) = binding.as_constant() {
                    // this seems like an infinite loop, todo return false?
                    self.contains
                        .execute(resolved_pattern, init_state, context, logs)
                } else {
                    Ok(false)
                }
            }
            ResolvedPattern::List(elements) => {
                let mut cur_state = init_state.clone();
                let mut did_match = false;
                for element in elements {
                    let state = cur_state.clone();
                    if self.execute(element, &mut cur_state, context, logs)? {
                        did_match = true;
                    } else {
                        cur_state = state;
                    }
                }
                if !did_match {
                    return Ok(false);
                }
                *init_state = cur_state;
                Ok(true)
            }
            ResolvedPattern::File(file) => {
                let mut cur_state = init_state.clone();
                let mut did_match = false;
                let prev_state = cur_state.clone();
                if self
                    .contains
                    .execute(resolved_pattern, &mut cur_state, context, logs)?
                {
                    did_match = true;
                } else {
                    cur_state = prev_state;
                }
                let prev_state = cur_state.clone();
                if self.contains.execute(
                    &file.name(&cur_state.files),
                    &mut cur_state,
                    context,
                    logs,
                )? {
                    did_match = true;
                } else {
                    cur_state = prev_state;
                }
                let prev_state = cur_state.clone();
                if self.execute(
                    &file.binding(&cur_state.files),
                    &mut cur_state,
                    context,
                    logs,
                )? {
                    did_match = true;
                } else {
                    cur_state = prev_state;
                }
                if !did_match {
                    return Ok(false);
                }
                *init_state = cur_state;
                Ok(true)
            }
            ResolvedPattern::Files(files) => {
                let mut cur_state = init_state.clone();
                let mut did_match = false;
                let prev_state = cur_state.clone();
                if self
                    .contains
                    .execute(resolved_pattern, &mut cur_state, context, logs)?
                {
                    did_match = true;
                } else {
                    cur_state = prev_state;
                }
                let prev_state = cur_state.clone();
                if self.execute(files, &mut cur_state, context, logs)? {
                    did_match = true;
                } else {
                    cur_state = prev_state;
                }
                if !did_match {
                    return Ok(false);
                }
                *init_state = cur_state;
                Ok(true)
            }
            ResolvedPattern::Snippets(snippets) => {
                let mut cur_state = init_state.clone();
                let mut did_match = false;
                for snippet in snippets {
                    let state = cur_state.clone();
                    let resolved = match snippet {
                        ResolvedSnippet::Text(_) => {
                            ResolvedPattern::Snippets(vector![snippet.to_owned()])
                        }
                        ResolvedSnippet::Binding(b) => {
                            ResolvedPattern::Binding(vector![b.to_owned()])
                        }
                        ResolvedSnippet::LazyFn(l) => match &**l {
                            LazyBuiltIn::Join(j) => ResolvedPattern::List(j.list.clone()),
                        },
                    };
                    if self.execute(&resolved, &mut cur_state, context, logs)? {
                        did_match = true;
                    } else {
                        cur_state = state;
                    }
                }
                if !did_match {
                    return Ok(false);
                }
                *init_state = cur_state;
                Ok(true)
            }
            ResolvedPattern::Map(_) | ResolvedPattern::Constant(_) => Ok(false),
        }
    }
}
