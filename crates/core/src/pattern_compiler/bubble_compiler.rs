use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::{
    problem::MarzanoQueryContext,
    variables::{get_variables, register_variable},
};
use anyhow::{anyhow, bail, Result};
use grit_pattern_matcher::pattern::{Bubble, Pattern, PatternDefinition};
use grit_util::{error::GritPatternError, AstNode, ByteRange};
use itertools::Itertools;
use marzano_util::node_with_source::NodeWithSource;
use std::collections::BTreeMap;

pub(crate) struct BubbleCompiler;

impl NodeCompiler for BubbleCompiler {
    type TargetPattern = Bubble<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let mut local_vars = BTreeMap::new();
        let (local_scope_index, mut local_context) = create_scope!(context, local_vars);

        // important that this occurs first, as calls assume
        // that parameters are registered first

        let parameters: Vec<_> = node
            .named_children_by_field_name("variables")
            .map(|n| Ok((n.text()?.trim().to_string(), n.byte_range())))
            .collect::<Result<Vec<(String, ByteRange)>, GritPatternError>>()?;
        if parameters.iter().unique_by(|n| &n.0).count() != parameters.len() {
            bail!("bubble parameters must be unique, but had a repeated name in its parameters.")
        }
        let params = get_variables(&parameters, &mut local_context)?;

        let body = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing body of patternDefinition"))?;
        let body = PatternCompiler::from_node(&body, &mut local_context)?;

        let args = parameters
            .iter()
            .map(|(name, range)| {
                let v = Pattern::Variable(register_variable(name, *range, context)?);
                Ok(v)
            })
            .collect::<Result<Vec<Pattern<MarzanoQueryContext>>>>()?;

        let pattern_def = PatternDefinition::new(
            "<bubble>".to_string(),
            local_scope_index,
            params,
            local_vars.values().cloned().collect(),
            body,
        );

        Ok(Bubble::new(pattern_def, args))
    }
}
