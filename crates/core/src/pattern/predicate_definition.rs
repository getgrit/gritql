use super::{
    and::PrAnd,
    compiler::CompilationContext,
    functions::Evaluator,
    patterns::Pattern,
    predicates::Predicate,
    variable::{get_variables, Variable, VariableSourceLocations},
    State,
};
use crate::context::Context;
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Clone, Debug)]
pub struct PredicateDefinition {
    pub name: String,
    scope: usize,
    pub params: Vec<(String, Variable)>,
    // this could just be a usize representing the len
    pub local_vars: Vec<usize>,
    pub predicate: Predicate,
}

impl PredicateDefinition {
    pub fn new(
        name: String,
        scope: usize,
        params: Vec<(String, Variable)>,
        local_vars: Vec<usize>,
        predicate: Predicate,
    ) -> Self {
        Self {
            name,
            scope,
            params,
            local_vars,
            predicate,
        }
    }

    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        predicate_definitions: &mut Vec<PredicateDefinition>,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<()> {
        let name = node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of pattern definition"))?;
        let name = name.utf8_text(context.src.as_bytes())?;
        let name = name.trim();
        let scope_index = vars_array.len();
        vars_array.push(vec![]);
        let mut local_vars = BTreeMap::new();
        // important that this occurs first, as calls assume
        // that parameters are registered first
        let params = get_variables(
            &context
                .predicate_definition_info
                .get(name)
                .ok_or_else(|| anyhow!("cannot get info for pattern {}", name))?
                .parameters,
            context.file,
            vars_array,
            scope_index,
            &mut local_vars,
            global_vars,
        )?;

        let body = node
            .child_by_field_name("body")
            .ok_or_else(|| anyhow!("missing body of pattern definition"))?;
        let body = PrAnd::from_node(
            &body,
            context,
            &mut local_vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        let predicate_def = PredicateDefinition::new(
            name.to_owned(),
            scope_index,
            params,
            local_vars.values().cloned().collect(),
            body,
        );
        // todo check for duplicate names
        predicate_definitions.push(predicate_def);
        Ok(())
    }

    pub fn call<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        args: &'a [Option<Pattern>],
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        state.reset_vars(self.scope, args);
        let res = self.predicate.execute_func(state, context, logs)?;
        Ok(res.predicator)
    }
}
