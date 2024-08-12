use super::{compiler::NodeCompilationContext, node_compiler::NodeCompiler};
use crate::variables::register_variable;
use grit_pattern_matcher::pattern::Variable;
use grit_util::{error::GritResult, AstNode};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct VariableCompiler;

impl NodeCompiler for VariableCompiler {
    type TargetPattern = Variable;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> GritResult<Self::TargetPattern> {
        let name = node.text()?;
        let name = name.trim();
        let range = node.byte_range();
        register_variable(name, range, context)
    }
}
