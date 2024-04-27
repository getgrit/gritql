use super::{
    container::Container,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::pattern::functions::GritCall;
use crate::{
    context::{ExecContext, QueryContext},
    errors::debug,
};
use anyhow::Result;
use grit_util::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Match<Q: QueryContext> {
    pub val: Container<Q>,
    pub pattern: Option<Pattern<Q>>,
}

impl<Q: QueryContext> Match<Q> {
    pub fn new(val: Container<Q>, pattern: Option<Pattern<Q>>) -> Self {
        Self { val, pattern }
    }
}

impl<Q: QueryContext> PatternName for Match<Q> {
    fn name(&self) -> &'static str {
        "MATCH"
    }
}

impl<Q: QueryContext> Evaluator<Q> for Match<Q> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation<Q>> {
        match &self.val {
            Container::Variable(var) => {
                let var = state.trace_var(var);
                let var_content = &state.bindings[var.scope].last().unwrap()[var.index];
                let predicator = if let Some(pattern) = &self.pattern {
                    if let Some(important_binding) = &var_content.value {
                        pattern.execute(&important_binding.clone(), state, context, logs)?
                    } else if let Some(var_pattern) = var_content.pattern {
                        let resolved_pattern =
                            ResolvedPattern::from_pattern(var_pattern, state, context, logs)?;
                        pattern.execute(&resolved_pattern, state, context, logs)?
                    } else if let Some(Pattern::BooleanConstant(b)) = &self.pattern {
                        if !b.value {
                            true
                        } else {
                            let resolved_pattern = ResolvedPattern::undefined();
                            let res = pattern.execute(&resolved_pattern, state, context, logs)?;
                            if !res {
                                let message = format!(
                                    "Attempted to match against undefined variable {}",
                                    state.get_name(&var)
                                );
                                debug(logs, state, context.language(), message.as_str())?;
                            }
                            res
                        }
                    } else {
                        let resolved_pattern = ResolvedPattern::undefined();
                        let res = pattern.execute(&resolved_pattern, state, context, logs)?;
                        if !res {
                            let message = format!(
                                "Attempted to match against undefined variable {}",
                                state.get_name(&var)
                            );
                            debug(logs, state, context.language(), message.as_str())?;
                        }
                        res
                    }
                } else {
                    var_content.value.is_none() && var_content.pattern.is_none()
                        || var_content
                            .value
                            .as_ref()
                            .is_some_and(|v| v.matches_undefined())
                };
                Ok(FuncEvaluation {
                    predicator,
                    ret_val: None,
                })
            }
            Container::Accessor(accessor) => {
                let resolved_accessor =
                    ResolvedPattern::from_accessor(accessor, state, context, logs)?;
                let predicator = if let Some(pattern) = &self.pattern {
                    pattern.execute(&resolved_accessor, state, context, logs)?
                } else {
                    resolved_accessor.matches_undefined()
                };
                Ok(FuncEvaluation {
                    predicator,
                    ret_val: None,
                })
            }
            Container::ListIndex(index) => {
                let resolved_accessor =
                    ResolvedPattern::from_list_index(index, state, context, logs)?;
                let predicator = if let Some(pattern) = &self.pattern {
                    pattern.execute(&resolved_accessor, state, context, logs)?
                } else {
                    resolved_accessor.matches_undefined()
                };
                Ok(FuncEvaluation {
                    predicator,
                    ret_val: None,
                })
            }
            Container::FunctionCall(f) => {
                let resolved_accessor = f.call(state, context, logs)?;
                Ok(FuncEvaluation {
                    predicator: if let Some(pattern) = &self.pattern {
                        pattern.execute(&resolved_accessor, state, context, logs)?
                    } else {
                        resolved_accessor.matches_undefined()
                    },
                    ret_val: None,
                })
            }
        }
    }
}
