use super::{
    and::And,
    compiler::CompilationContext,
    patterns::{Matcher, Pattern},
    resolved_pattern::ResolvedPattern,
    variable::{get_variables, Variable, VariableSourceLocations},
    State,
};
use crate::context::Context;
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Clone, Debug)]
pub struct PatternDefinition {
    pub name: String,
    pub(crate) scope: usize,
    pub params: Vec<(String, Variable)>,
    // this could just be a usize representing the len
    pub local_vars: Vec<usize>,
    pub pattern: Pattern,
}

impl PatternDefinition {
    pub fn new(
        name: String,
        scope: usize,
        params: Vec<(String, Variable)>,
        local_vars: Vec<usize>,
        pattern: Pattern,
    ) -> Self {
        Self {
            name,
            scope,
            params,
            local_vars,
            pattern,
        }
    }

    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        pattern_definitions: &mut Vec<PatternDefinition>,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<()> {
        // TODO: make sure pattern definitions are only allowed at the top level
        let name = node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of patternDefinition"))?;
        let name = name.utf8_text(context.src.as_bytes())?;
        let name = name.trim();
        let scope_index = vars_array.len();
        vars_array.push(vec![]);
        let mut local_vars = BTreeMap::new();
        // important that this occurs first, as calls assume
        // that parameters are registered first
        let params = get_variables(
            &context
                .pattern_definition_info
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
            .ok_or_else(|| anyhow!("missing body of patternDefinition"))?;
        let body = And::from_node(
            &body,
            context,
            &mut local_vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        let pattern_def = PatternDefinition::new(
            name.to_owned(),
            scope_index,
            params,
            local_vars.values().cloned().collect(),
            body,
        );
        // todo check for duplicate names
        pattern_definitions.push(pattern_def);
        Ok(())
    }

    pub(crate) fn call<'a>(
        &'a self,
        state: &mut State<'a>,
        binding: &ResolvedPattern<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
        args: &'a [Option<Pattern>],
    ) -> Result<bool> {
        state.reset_vars(self.scope, args);
        let res = self.pattern.execute(binding, state, context, logs);

        let fn_state = state.bindings[self.scope].pop_back().unwrap();
        let cur_fn_state = state.bindings[self.scope].back_mut().unwrap();
        for (cur, last) in cur_fn_state.iter_mut().zip(fn_state) {
            cur.value_history.extend(last.value_history)
        }
        res
    }
}
