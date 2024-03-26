use super::{
    compiler::CompilationContext,
    container::Container,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    variable::VariableSourceLocations,
    State,
};
use crate::{context::Context, errors::debug};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct Match {
    val: Container,
    pub(crate) pattern: Option<Pattern>,
}
impl Match {
    pub(crate) fn new(val: Container, pattern: Option<Pattern>) -> Self {
        Self { val, pattern }
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
        let value = node
            .child_by_field_name("left")
            .ok_or_else(|| anyhow!("missing lhs of predicateMatch"))?;
        let value = Container::from_node(
            &value,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        let pattern = node
            .child_by_field_name("right")
            .ok_or_else(|| anyhow!("missing rhs of predicateMatch"))?;
        let pattern = Some(Pattern::from_node(
            &pattern,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?);
        Ok(Match::new(value, pattern))
    }
}

impl Name for Match {
    fn name(&self) -> &'static str {
        "MATCH"
    }
}

impl Evaluator for Match {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
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
                                debug(logs, state, message.as_str())?;
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
                            debug(logs, state, message.as_str())?;
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
        }
    }
}
