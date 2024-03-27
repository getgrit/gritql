use super::{
    compiler::CompilationContext,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Name, Pattern},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    variable::VariableSourceLocations,
    State,
};
use crate::context::Context;
use anyhow::{anyhow, bail, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct If {
    pub(crate) if_: Predicate,
    pub(crate) then: Pattern,
    pub(crate) else_: Pattern,
}
impl If {
    pub fn new(if_: Predicate, then: Pattern, else_: Option<Pattern>) -> Self {
        Self {
            if_,
            then,
            else_: else_.unwrap_or(Pattern::Top),
        }
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
        let if_ = node
            .child_by_field_name("if")
            .ok_or_else(|| anyhow!("missing condition of if"))?;
        let if_ = Predicate::from_node(
            &if_,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        let then = node
            .child_by_field_name("then")
            .ok_or_else(|| anyhow!("missing consequence of if"))?;
        let then = Pattern::from_node(
            &then,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        let else_ = node
            .child_by_field_name("else")
            .map(|e| {
                Pattern::from_node(
                    &e,
                    context,
                    vars,
                    vars_array,
                    scope_index,
                    global_vars,
                    false,
                    logs,
                )
            })
            .map_or(Ok(None), |v| v.map(Some))?;
        Ok(If::new(if_, then, else_))
    }
}

impl Name for If {
    fn name(&self) -> &'static str {
        "IF"
    }
}

impl Matcher for If {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let mut state = init_state.clone();
        if self.if_.execute_func(&mut state, context, logs)?.predicator {
            *init_state = state;
            self.then.execute(binding, init_state, context, logs)
        } else {
            self.else_.execute(binding, init_state, context, logs)
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrIf {
    pub(crate) if_: Predicate,
    pub(crate) then: Predicate,
    pub(crate) else_: Predicate,
}

impl PrIf {
    pub fn new(if_: Predicate, then: Predicate, else_: Option<Predicate>) -> Self {
        Self {
            if_,
            then,
            else_: else_.unwrap_or(Predicate::True),
        }
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
        let if_ = node
            .child_by_field_name("if")
            .ok_or_else(|| anyhow!("missing condition of if"))?;
        let if_ = Predicate::from_node(
            &if_,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        let then = node
            .child_by_field_name("then")
            .ok_or_else(|| anyhow!("missing consequence of if"))?;
        let then = Predicate::from_node(
            &then,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        let else_ = node
            .child_by_field_name("else")
            .map(|e| {
                Predicate::from_node(
                    &e,
                    context,
                    vars,
                    vars_array,
                    scope_index,
                    global_vars,
                    logs,
                )
            })
            .map_or(Ok(None), |v| v.map(Some))?;
        Ok(PrIf::new(if_, then, else_))
    }
}

impl Name for PrIf {
    fn name(&self) -> &'static str {
        "PREDICATE_IF"
    }
}

impl Evaluator for PrIf {
    fn execute_func<'a>(
        &'a self,
        init_state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let mut state = init_state.clone();
        let condition = self.if_.execute_func(&mut state, context, logs)?;
        if condition.ret_val.is_some() {
            bail!("Cannot return from within if condition");
        }
        if condition.predicator {
            *init_state = state;
            self.then.execute_func(init_state, context, logs)
        } else {
            self.else_.execute_func(init_state, context, logs)
        }
    }
}
