use crate::{binding::Binding, context::Context, resolve};

use super::{
    compiler::CompilationContext,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::{LazyBuiltIn, ListBinding, Lists, ResolvedPattern, ResolvedSnippet},
    variable::VariableSourceLocations,
    Node, State,
};

use anyhow::{anyhow, Result};
use core::fmt::Debug;
use im::vector;
use marzano_util::analysis_logs::AnalysisLogs;

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
    context: &'a impl Context<'a>,
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
        let node_lhs = ResolvedPattern::from_node(src, node);

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
        context: &'a impl Context<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        match resolved_pattern {
            ResolvedPattern::Binding(bindings) => {
                let binding = resolve!(bindings.last());
                let mut did_match = false;
                let mut cur_state = init_state.clone();
                let mut cursor; // needed for scope in case of list.
                match binding {
                    Binding::Empty(_, _, _) => Ok(false),
                    Binding::String(_, _) => Ok(false),
                    Binding::Node(src, node) => execute_until(
                        init_state,
                        node,
                        src,
                        context,
                        logs,
                        &self.contains,
                        &self.until,
                    ),
                    Binding::List(src, node, field_id) => {
                        cursor = node.walk();
                        let children = node.children_by_field_id(*field_id, &mut cursor);

                        for child in children {
                            let state = cur_state.clone();
                            if self.execute(
                                &ResolvedPattern::from_node(src, child),
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
                    }
                    Binding::FileName(_) => Ok(false),
                    // this seems like an infinite loop, todo return false?
                    Binding::ConstantRef(_c) => {
                        self.contains
                            .execute(resolved_pattern, init_state, context, logs)
                    }
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
                            LazyBuiltIn::Join(j) => match &j.list {
                                Lists::Binding(ListBinding {
                                    src,
                                    parent_node,
                                    field,
                                }) => ResolvedPattern::Binding(vector![Binding::List(
                                    src,
                                    parent_node.to_owned(),
                                    *field
                                )]),
                                Lists::Resolved(l) => ResolvedPattern::List(l.to_owned()),
                            },
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
