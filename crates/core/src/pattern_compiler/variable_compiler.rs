use super::{compiler::NodeCompilationContext, node_compiler::NodeCompiler};
use crate::variables::register_variable;
use anyhow::Result;
use grit_core_patterns::pattern::variable::Variable;
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct VariableCompiler;

impl NodeCompiler for VariableCompiler {
    type TargetPattern = Variable;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let name = node.text()?;
        let name = name.trim();
        let range = node.range();
        register_variable(name, range, context)
    }
}
