use super::{
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{context::Context, resolve};
use anyhow::Result;
use im::vector;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Some {
    pub pattern: Pattern,
}

impl Some {
    pub fn new(pattern: Pattern) -> Self {
        Self { pattern }
    }
}

impl Name for Some {
    fn name(&self) -> &'static str {
        "SOME"
    }
}

impl Matcher for Some {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        match binding {
            ResolvedPattern::Binding(bindings) => {
                let binding = resolve!(bindings.last());
                let Some(list_items) = binding.list_items() else {
                    return Ok(false);
                };

                let mut did_match = false;
                let mut cur_state = init_state.clone();
                for item in list_items {
                    let state = cur_state.clone();
                    if self.pattern.execute(
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
                *init_state = cur_state;
                Ok(did_match)
            }
            ResolvedPattern::List(elements) => {
                let pattern = &self.pattern;
                let mut cur_state = init_state.clone();
                let mut did_match = false;
                for element in elements {
                    let state = cur_state.clone();
                    if pattern.execute(element, &mut cur_state, context, logs)? {
                        did_match = true;
                    } else {
                        cur_state = state;
                    }
                }
                *init_state = cur_state;
                Ok(did_match)
            }
            ResolvedPattern::Map(map) => {
                let pattern = &self.pattern;
                let mut cur_state = init_state.clone();
                let mut did_match = false;
                for (key, value) in map {
                    let state = cur_state.clone();
                    let key =
                        ResolvedPattern::Constant(crate::binding::Constant::String(key.clone()));
                    let resolved = ResolvedPattern::List(vector![key, value.clone()]);
                    if pattern.execute(&resolved, &mut cur_state, context, logs)? {
                        did_match = true;
                    } else {
                        cur_state = state;
                    }
                }
                *init_state = cur_state;
                Ok(did_match)
            }
            ResolvedPattern::Snippets(_)
            | ResolvedPattern::File(_)
            | ResolvedPattern::Files(_)
            | ResolvedPattern::Constant(_) => Ok(false),
        }
    }
}
