use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{
    container::Container,
    list::List,
    list_index::{ContainerOrIndex, ListIndex, ListOrContainer},
    variable::VariableSourceLocations,
};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct ListIndexCompiler;

impl NodeCompiler for ListIndexCompiler {
    type TargetPattern = ListIndex;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let list = node
            .child_by_field_name("list")
            .ok_or_else(|| anyhow!("missing list of listIndex"))?;
        let list = if list.kind() == "list" {
            ListOrContainer::List(List::from_node(
                &list,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                false,
                logs,
            )?)
        } else {
            ListOrContainer::Container(Container::from_node(
                &list,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?)
        };

        let index_node = node
            .child_by_field_name("index")
            .ok_or_else(|| anyhow!("missing index of listIndex"))?;

        let index = if let "signedIntConstant" = index_node.kind().as_ref() {
            ContainerOrIndex::Index(
                index_node
                    .utf8_text(context.src.as_bytes())?
                    .parse::<isize>()
                    .map_err(|_| anyhow!("list index must be an integer"))?,
            )
        } else {
            ContainerOrIndex::Container(Container::from_node(
                &index_node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?)
        };

        Ok(ListIndex { list, index })
    }
}
