use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler, variable_compiler::VariableCompiler,
};
use crate::problem::MarzanoQueryContext;
use anyhow::Result;
use grit_pattern_matcher::pattern::{Log, VariableInfo};
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct LogCompiler;

impl NodeCompiler for LogCompiler {
    type TargetPattern = Log<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let message = node.child_by_field_name("message");
        let message = if let Some(message) = message {
            Some(PatternCompiler::from_node(&message, context)?)
        } else {
            None
        };
        let variable_node = node.child_by_field_name("variable");
        let variable = variable_node
            .map(|n| {
                let name = n.text()?;
                let variable = VariableCompiler::from_node(&n, context)?;
                Ok::<_, anyhow::Error>(VariableInfo::new(name.to_string(), variable))
            })
            .transpose()?;

        Ok(Log::new(variable, message))
    }
}
