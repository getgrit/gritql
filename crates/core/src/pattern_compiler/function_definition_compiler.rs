use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{
    and::PrAnd,
    function_definition::{ForeignFunctionDefinition, GritFunctionDefinition},
    variable::{get_variables, VariableSourceLocations},
};
use anyhow::{anyhow, Result};
use marzano_language::foreign_language::ForeignLanguage;
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct GritFunctionDefinitionCompiler;

impl NodeCompiler for GritFunctionDefinitionCompiler {
    type TargetPattern = GritFunctionDefinition;

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
            .ok_or_else(|| anyhow!("missing name of function definition"))?;
        let name = name.utf8_text(context.src.as_bytes())?;
        let name = name.trim();
        let scope_index = vars_array.len();
        vars_array.push(vec![]);
        let mut local_vars = BTreeMap::new();

        let params = get_variables(
            &context
                .function_definition_info
                .get(name)
                .ok_or_else(|| anyhow!("cannot get info for function {}", name))?
                .parameters,
            context.file,
            vars_array,
            scope_index,
            &mut local_vars,
            global_vars,
        )?;

        let body = node
            .child_by_field_name("body")
            .ok_or_else(|| anyhow!("missing body of grit function definition"))?;
        let body = PrAnd::from_node(
            &body,
            context,
            &mut local_vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        let function_definition = GritFunctionDefinition::new(
            name.to_owned(),
            scope_index,
            params,
            local_vars.values().cloned().collect(),
            body,
        );
        Ok(function_definition)
    }
}

pub(crate) struct ForeignFunctionDefinitionCompiler;

impl NodeCompiler for ForeignFunctionDefinitionCompiler {
    type TargetPattern = ForeignFunctionDefinition;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        _vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        _scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        _logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let name = node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of function definition"))?;
        let name = name.utf8_text(context.src.as_bytes())?;
        let name = name.trim();
        let scope_index = vars_array.len();
        vars_array.push(vec![]);
        let mut local_vars = BTreeMap::new();
        let params = get_variables(
            &context
                .foreign_function_definition_info
                .get(name)
                .ok_or_else(|| anyhow!("cannot get info for function {}", name))?
                .parameters,
            context.file,
            vars_array,
            scope_index,
            &mut local_vars,
            global_vars,
        )?;
        let body = node
            .child_by_field_name("body")
            .ok_or_else(|| anyhow!("missing body of foreign function definition"))?
            .child_by_field_name("code")
            .ok_or_else(|| anyhow!("missing code of foreign function body"))?;
        let byte_range = body.byte_range();
        let byte_range = byte_range.start as usize..byte_range.end as usize;
        let body = &context.src.as_bytes()[byte_range];
        let foreign_language = ForeignLanguage::from_node(
            node.child_by_field_name("language")
                .ok_or_else(|| anyhow!("missing language of foreign function definition"))?,
            context.src,
        )?;
        let function_definition = ForeignFunctionDefinition::new(
            name.to_owned(),
            scope_index,
            params,
            foreign_language,
            body,
        );
        Ok(function_definition)
    }
}
