use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler, variable_compiler::VariableCompiler,
};
use crate::pattern::log::{Log, VariableInfo};
use crate::problem::MarzanoQueryContext;
use anyhow::Result;
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
                let name = n.text().to_string();
                let variable = VariableCompiler::from_node(&n, context)?;
                Ok(VariableInfo::new(name, variable))
            })
            .map_or(Ok(None), |v: Result<VariableInfo>| v.map(Some))?;

        Ok(Log::new(variable, message))
    }
}
