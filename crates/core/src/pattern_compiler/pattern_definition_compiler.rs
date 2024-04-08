use super::{and_compiler::AndCompiler, compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{
    pattern_definition::PatternDefinition,
    variable::{get_variables, VariableSourceLocations},
};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct PatternDefinitionCompiler;

impl NodeCompiler for PatternDefinitionCompiler {
    type TargetPattern = PatternDefinition;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        _vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        _scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
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
        let body = AndCompiler::from_node(
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
        Ok(pattern_def)
    }
}
