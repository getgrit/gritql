use super::{
    compiler::NodeCompilationContext, container_compiler::ContainerCompiler,
    node_compiler::NodeCompiler, pattern_compiler::PatternCompiler,
};
use crate::problem::MarzanoQueryContext;
use anyhow::{anyhow, bail, Result};
use grit_pattern_matcher::pattern::{is_reserved_metavariable, Assignment};
use grit_util::{constants::GRIT_METAVARIABLE_PREFIX, AstNode};
use marzano_language::target_language::TargetLanguage;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct AssignmentCompiler;

impl NodeCompiler for AssignmentCompiler {
    type TargetPattern = Assignment<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let pattern = node
            .child_by_field_name("pattern")
            .ok_or_else(|| GritPatternError::new("missing pattern of assignment"))?;
        let pattern = PatternCompiler::from_node_with_rhs(&pattern, context, true)?;

        let container = node
            .child_by_field_name("container")
            .ok_or_else(|| GritPatternError::new("missing container of assignment"))?;
        let var_text = container.text()?;
        if is_reserved_metavariable(&var_text, None::<&TargetLanguage>) {
            return Err(GritPatternError::new("{} is a reserved metavariable name. For more information, check out the docs at https://docs.grit.io/language/patterns#metavariables.", var_text.trim_start_matches(GRIT_METAVARIABLE_PREFIX)));
        }
        let variable = ContainerCompiler::from_node(&container, context)?;
        Ok(Assignment::new(variable, pattern))
    }
}
