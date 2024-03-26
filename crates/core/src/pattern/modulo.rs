use super::{
    compiler::CompilationContext,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
    variable::VariableSourceLocations,
};
use crate::{binding::Constant, context::Context};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct Modulo {
    pub(crate) lhs: Pattern,
    pub(crate) rhs: Pattern,
}

impl Modulo {
    pub fn new(lhs: Pattern, rhs: Pattern) -> Self {
        Self { lhs, rhs }
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
        let left = node
            .child_by_field_name("left")
            .ok_or_else(|| anyhow!("missing left of modulo"))?;
        let left = Pattern::from_node(
            &left,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;

        let right = node
            .child_by_field_name("right")
            .ok_or_else(|| anyhow!("missing right of modulo"))?;
        let right = Pattern::from_node(
            &right,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;

        Ok(Self::new(left, right))
    }

    pub(crate) fn call<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>> {
        let res = self.evaluate(state, context, logs)?;
        Ok(ResolvedPattern::Constant(Constant::Integer(res)))
    }

    fn evaluate<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<i64> {
        let lhs = self.lhs.text(state, context, logs)?;
        let rhs = self.rhs.text(state, context, logs)?;
        let lhs_int = lhs.parse::<i64>()?;
        let rhs_int = rhs.parse::<i64>()?;
        let res = lhs_int % rhs_int;
        Ok(res)
    }
}

impl Name for Modulo {
    fn name(&self) -> &'static str {
        "MODULO"
    }
}

impl Matcher for Modulo {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let binding_text = binding.text(&state.files)?;
        let binding_int = binding_text.parse::<i64>()?;
        let target = self.evaluate(state, context, logs)?;
        Ok(binding_int == target)
    }
}
