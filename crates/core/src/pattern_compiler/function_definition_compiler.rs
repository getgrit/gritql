use super::{
    and_compiler::PrAndCompiler, compiler::NodeCompilationContext,
    foreign_language_compiler::ForeignLanguageCompiler, node_compiler::NodeCompiler,
};
use crate::pattern::{
    function_definition::{ForeignFunctionDefinition, GritFunctionDefinition},
    variable::get_variables,
};
use anyhow::{anyhow, Result};
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;
use std::collections::BTreeMap;

pub(crate) struct GritFunctionDefinitionCompiler;

impl NodeCompiler for GritFunctionDefinitionCompiler {
    type TargetPattern = GritFunctionDefinition;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let name = node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of function definition"))?;
        let name = name.text().trim();
        let mut local_vars = BTreeMap::new();
        let (scope_index, mut local_context) = create_scope!(context, local_vars);

        let params = get_variables(
            &local_context
                .compilation
                .function_definition_info
                .get(name)
                .ok_or_else(|| anyhow!("cannot get info for function {}", name))?
                .parameters,
            local_context.compilation.file,
            local_context.vars_array,
            scope_index,
            local_context.vars,
            local_context.global_vars,
        )?;

        let body = node
            .child_by_field_name("body")
            .ok_or_else(|| anyhow!("missing body of grit function definition"))?;
        let body = PrAndCompiler::from_node(&body, &mut local_context)?;
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

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let name = node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of function definition"))?;
        let name = name.text().trim();
        let mut local_vars = BTreeMap::new();
        let (scope_index, local_context) = create_scope!(context, local_vars);
        let params = get_variables(
            &context
                .compilation
                .foreign_function_definition_info
                .get(name)
                .ok_or_else(|| anyhow!("cannot get info for function {}", name))?
                .parameters,
            local_context.compilation.file,
            local_context.vars_array,
            scope_index,
            local_context.vars,
            local_context.global_vars,
        )?;
        let body = node
            .child_by_field_name("body")
            .ok_or_else(|| anyhow!("missing body of foreign function definition"))?
            .child_by_field_name("code")
            .ok_or_else(|| anyhow!("missing code of foreign function body"))?;
        let foreign_language = ForeignLanguageCompiler::from_node(
            &node
                .child_by_field_name("language")
                .ok_or_else(|| anyhow!("missing language of foreign function definition"))?,
            context,
        )?;
        let function_definition = ForeignFunctionDefinition::new(
            name.to_owned(),
            scope_index,
            params,
            foreign_language,
            body.text().as_bytes(),
        );
        Ok(function_definition)
    }
}
