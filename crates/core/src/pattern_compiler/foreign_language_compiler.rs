use super::{compiler::NodeCompilationContext, node_compiler::NodeCompiler};
use grit_util::{error::GritResult, AstNode};
use marzano_language::foreign_language::ForeignLanguage;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct ForeignLanguageCompiler;

impl NodeCompiler for ForeignLanguageCompiler {
    type TargetPattern = ForeignLanguage;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        _context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> GritResult<Self::TargetPattern> {
        node.text()?.try_into()
    }
}
