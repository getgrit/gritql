use super::{
    and_compiler::AndCompiler, compiler::NodeCompilationContext, node_compiler::NodeCompiler,
};
use crate::{problem::MarzanoQueryContext, variables::get_variables};
use anyhow::{anyhow, Result};
use grit_pattern_matcher::pattern::PatternDefinition;
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;
use std::collections::BTreeMap;

pub(crate) struct PatternDefinitionCompiler;

impl NodeCompiler for PatternDefinitionCompiler {
    type TargetPattern = PatternDefinition<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        // TODO: make sure pattern definitions are only allowed at the top level
        let name = node
            .child_by_field_name("name")
            .ok_or_else(|| anyhow!("missing name of patternDefinition"))?;
        let name = name.text()?;
        let name = name.trim();
        let mut local_vars = BTreeMap::new();
        let (scope_index, mut context) = create_scope!(context, local_vars);
        // important that this occurs first, as calls assume
        // that parameters are registered first
        let params = get_variables(
            &context
                .compilation
                .pattern_definition_info
                .get(name)
                .ok_or_else(|| anyhow!("cannot get info for pattern {name}"))?
                .parameters,
            &mut context,
        )?;

        let body = node
            .child_by_field_name("body")
            .ok_or_else(|| anyhow!("missing body of patternDefinition"))?;
        let body = AndCompiler::from_node(&body, &mut context)?;
        let pattern_def = PatternDefinition::new(name.to_owned(), scope_index, params, body);
        Ok(pattern_def)
    }
}
