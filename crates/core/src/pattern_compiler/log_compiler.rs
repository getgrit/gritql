use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler, variable_compiler::VariableCompiler,
};
use crate::problem::MarzanoQueryContext;
use anyhow::{bail, Result};
use grit_pattern_matcher::pattern::{CallBuiltIn, Pattern, StringConstant};
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct LogCompiler;

impl NodeCompiler for LogCompiler {
    type TargetPattern = CallBuiltIn<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let variable_node = node.child_by_field_name("variable");
        let variable = variable_node
            .map(
                |n| match (n.text(), VariableCompiler::from_node(&n, context)) {
                    (Ok(name), Ok(variable)) => Ok((name.into_owned(), variable)),
                    (Err(err), _) => Err(err.into()),
                    (_, Err(err)) => Err(err),
                },
            )
            .transpose()?;

        let message = node.child_by_field_name("message");
        let mut args = if let Some(message) = message {
            vec![Some(PatternCompiler::from_node(&message, context)?)]
        } else if let Some((name, _)) = variable.as_ref() {
            vec![Some(Pattern::StringConstant(StringConstant {
                text: format!("Logging {name}"),
            }))]
        } else {
            bail!("log() requires a message or variable");
        };

        if let Some((_, variable)) = variable {
            args.push(Some(Pattern::Variable(variable)));
        }

        let fn_index = context
            .compilation
            .built_ins
            .get_built_ins()
            .iter()
            .position(|built_in| built_in.name == "log")
            .expect("built-in log function not found");

        Ok(CallBuiltIn::new(fn_index, "log", args))
    }
}
