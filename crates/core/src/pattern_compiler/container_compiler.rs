use super::{
    accessor_compiler::AccessorCompiler, compiler::NodeCompilationContext,
    list_index_compiler::ListIndexCompiler, node_compiler::NodeCompiler,
    variable_compiler::VariableCompiler,
};
use crate::problem::MarzanoQueryContext;
use anyhow::{bail, Result};
use grit_pattern_matcher::pattern::Container;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct ContainerCompiler;

impl NodeCompiler for ContainerCompiler {
    type TargetPattern = Container<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        match node.node.kind().as_ref() {
            "variable" => Ok(Container::Variable(VariableCompiler::from_node(
                node, context,
            )?)),
            "mapAccessor" => Ok(Container::Accessor(Box::new(AccessorCompiler::from_node(
                node, context,
            )?))),
            "listIndex" => Ok(Container::ListIndex(Box::new(
                ListIndexCompiler::from_node(node, context)?,
            ))),
            s => return Err(GritPatternError::new(format!("Invalid kind for container: {}", s))),
        }
    }
}
