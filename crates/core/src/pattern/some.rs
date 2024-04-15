use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{binding::Binding, context::QueryContext, resolve};
use anyhow::Result;
use im::vector;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Some<Q: QueryContext> {
    pub pattern: Pattern<Q>,
}

impl<Q: QueryContext> Some<Q> {
    pub fn new(pattern: Pattern<Q>) -> Self {
        Self { pattern }
    }
}

impl<Q: QueryContext> PatternName for Some<Q> {
    fn name(&self) -> &'static str {
        "SOME"
    }
}

impl<Q: QueryContext> Matcher<Q> for Some<Q> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a, Q>,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
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
                        ResolvedPattern::Constant(crate::constant::Constant::String(key.clone()));
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
