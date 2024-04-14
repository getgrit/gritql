use super::{
    and_compiler::PrAndCompiler, compiler::NodeCompilationContext, node_compiler::NodeCompiler,
};
use crate::{
    pattern::{predicate_definition::PredicateDefinition, variable::get_variables},
    problem::MarzanoQueryContext,
};
use anyhow::{anyhow, Result};
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;
use std::collections::BTreeMap;

pub(crate) struct PredicateDefinitionCompiler;

impl NodeCompiler for PredicateDefinitionCompiler {
    type TargetPattern = PredicateDefinition<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let name = node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of pattern definition"))?;
        let name = name.text().trim();
        let mut local_vars = BTreeMap::new();
        let (scope_index, mut local_context) = create_scope!(context, local_vars);
        // important that this occurs first, as calls assume
        // that parameters are registered first
        let params = get_variables(
            &context
                .compilation
                .predicate_definition_info
                .get(name)
                .ok_or_else(|| anyhow!("cannot get info for pattern {}", name))?
                .parameters,
            &mut local_context,
        )?;

        let body = node
            .child_by_field_name("body")
            .ok_or_else(|| anyhow!("missing body of pattern definition"))?;
        let body = PrAndCompiler::from_node(&body, &mut local_context)?;
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
