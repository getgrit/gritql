use super::{
    and_compiler::PrAndCompiler, compiler::CompilationContext, node_compiler::NodeCompiler,
};
use crate::pattern::{
    predicate_definition::PredicateDefinition,
    variable::{get_variables, VariableSourceLocations},
};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct PredicateDefinitionCompiler;

impl NodeCompiler for PredicateDefinitionCompiler {
    type TargetPattern = PredicateDefinition;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        _vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        _scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
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
        let body = PrAndCompiler::from_node(
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
        Ok(predicate_def)
    }
}
