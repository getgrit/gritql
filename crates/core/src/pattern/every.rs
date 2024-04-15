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
pub struct Every<Q: QueryContext> {
    pub pattern: Pattern<Q>,
}

impl<Q: QueryContext> Every<Q> {
    pub fn new(pattern: Pattern<Q>) -> Self {
        Self { pattern }
    }
}

impl<Q: QueryContext> PatternName for Every<Q> {
    fn name(&self) -> &'static str {
        "EVERY"
    }
}

impl<Q: QueryContext> Matcher<Q> for Every<Q> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a, Q>,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        // might be necessary to clone init state at the top,
        // but more performant to not, so leaving out for now.
        match binding {
            ResolvedPattern::Binding(bindings) => {
                let binding = resolve!(bindings.last());
                let Some(list_items) = binding.list_items() else {
                    return Ok(false);
                };

                for item in list_items {
                    if !self.pattern.execute(
                        &ResolvedPattern::from_node(item),
                        init_state,
                        context,
                        logs,
                    )? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            ResolvedPattern::List(elements) => {
                let pattern = &self.pattern;
                for element in elements {
                    if !pattern.execute(element, init_state, context, logs)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            ResolvedPattern::Map(map) => {
                let pattern = &self.pattern;
                for (key, value) in map {
                    let key =
                        ResolvedPattern::Constant(crate::constant::Constant::String(key.clone()));
                    let resolved = ResolvedPattern::List(vector![key, value.clone()]);
                    if !pattern.execute(&resolved, init_state, context, logs)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            ResolvedPattern::Snippets(_)
            | ResolvedPattern::File(_)
            | ResolvedPattern::Files(_)
            | ResolvedPattern::Constant(_) => Ok(false),
        }
    }
}
