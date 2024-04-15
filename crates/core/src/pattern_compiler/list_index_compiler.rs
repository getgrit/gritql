use super::{
    compiler::NodeCompilationContext, container_compiler::ContainerCompiler,
    list_compiler::ListCompiler, node_compiler::NodeCompiler,
};
use crate::pattern::list_index::{ContainerOrIndex, ListIndex, ListOrContainer};
use crate::problem::MarzanoQueryContext;
use anyhow::{anyhow, Result};
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct ListIndexCompiler;

impl NodeCompiler for ListIndexCompiler {
    type TargetPattern = ListIndex<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let list = node
            .child_by_field_name("list")
            .ok_or_else(|| anyhow!("missing list of listIndex"))?;
        let list = if list.node.kind() == "list" {
            ListOrContainer::List(ListCompiler::from_node(&list, context)?)
        } else {
            ListOrContainer::Container(ContainerCompiler::from_node(&list, context)?)
        };

        let index_node = node
            .child_by_field_name("index")
            .ok_or_else(|| anyhow!("missing index of listIndex"))?;

        let index = if index_node.node.kind() == "signedIntConstant" {
            ContainerOrIndex::Index(
                index_node
                    .text()?
                    .parse::<isize>()
                    .map_err(|_| anyhow!("list index must be an integer"))?,
            )
        } else {
            ContainerOrIndex::Container(ContainerCompiler::from_node(&index_node, context)?)
        };

        Ok(ListIndex { list, index })
    }
}
