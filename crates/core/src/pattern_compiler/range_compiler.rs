use super::{compiler::NodeCompilationContext, node_compiler::NodeCompiler};
use crate::pattern::range::{Point, Range};
use anyhow::Result;
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct RangeCompiler;

impl NodeCompiler for RangeCompiler {
    type TargetPattern = Range;

    fn from_node_with_rhs(
        node: NodeWithSource,
        _context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let start_line = node_to_int(&node, "start_line")?;
        let start_column = node_to_int(&node, "start_column")?;
        let end_line = node_to_int(&node, "end_line")?;
        let end_column = node_to_int(&node, "end_column")?;
        Ok(Range {
            start: Point::new(start_line, start_column)?,
            end: Point::new(end_line, end_column)?,
        })
    }
}

fn node_to_int(node: &NodeWithSource, field: &str) -> Result<Option<u32>> {
    node.child_by_field_name(field)
        .map(|n| Ok(n.text().parse::<u32>()?))
        .map_or(Ok(None), |v| v.map(Some))
}
