use super::{
    compiler::CompilationContext,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Name, Pattern},
    variable::{Variable, VariableSourceLocations},
    State,
};
use crate::context::Context;
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct Equal {
    var: Variable,
    pub(crate) pattern: Pattern,
}
impl Equal {
    pub fn new(var: Variable, pattern: Pattern) -> Self {
        Self { var, pattern }
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
        let variable = node
            .child_by_field_name("left")
            .ok_or_else(|| anyhow!("missing lhs of predicateEqual"))?;
        let variable = Pattern::from_node(
            &variable,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            true,
            logs,
        )?;
        let pattern = node
            .child_by_field_name("right")
            .ok_or_else(|| anyhow!("missing rhs of predicateEqual"))?;
        let pattern = Pattern::from_node(
            &pattern,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            true,
            logs,
        )?;
        if let Pattern::Variable(var) = variable {
            Ok(Equal::new(var, pattern))
        } else {
            Err(anyhow!(
                "predicateEqual must have a variable as first argument",
            ))
        }
    }
}

impl Name for Equal {
    fn name(&self) -> &'static str {
        "EQUAL"
    }
}

impl Evaluator for Equal {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let lhs_text = self.var.text(state)?;
        let rhs_text = self.pattern.text(state, context, logs)?;
        Ok(FuncEvaluation {
            predicator: lhs_text == rhs_text,
            ret_val: None,
        })
    }
}
