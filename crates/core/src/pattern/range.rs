use super::{
    patterns::{Matcher, Name},
    resolved_pattern::ResolvedPattern,
    state,
};
use crate::context::Context;
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use marzano_util::position::UtilRange;
use tree_sitter::Node;

#[derive(Debug, Clone)]
struct Point {
    line: u32,
    column: Option<u32>,
}

impl Point {
    fn new(line: Option<u32>, column: Option<u32>) -> Result<Option<Self>> {
        if let Some(line) = line {
            Ok(Some(Self { line, column }))
        } else {
            column
                .map(|_| {
                    Err(anyhow!(
                        "cannot have a point with a column index, but no line"
                    ))
                })
                .unwrap_or(Ok(None))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Range {
    start: Option<Point>,
    end: Option<Point>,
}

fn node_to_int(node: &Node, field: &str, src: &str) -> Result<Option<u32>> {
    node.child_by_field_name(field)
        .map(|n| Ok(n.utf8_text(src.as_bytes())?.parse::<u32>()?))
        .map_or(Ok(None), |v| v.map(Some))
}

impl Range {
    pub(crate) fn from_node(node: &Node, src: &str) -> Result<Self> {
        let start_line = node_to_int(node, "start_line", src)?;
        let start_column = node_to_int(node, "start_column", src)?;
        let end_line = node_to_int(node, "end_line", src)?;
        let end_column = node_to_int(node, "end_column", src)?;
        Ok(Self {
            start: Point::new(start_line, start_column)?,
            end: Point::new(end_line, end_column)?,
        })
    }
}

impl From<UtilRange> for Range {
    fn from(util_range: UtilRange) -> Self {
        // TODO: there must be a smarter way
        match util_range {
            UtilRange::Range(range) => Self {
                start: Some(Point {
                    line: range.start.line,
                    column: Some(range.start.column),
                }),
                end: Some(Point {
                    line: range.end.line,
                    column: Some(range.end.column),
                }),
            },
            UtilRange::RangeWithoutByte(range_without_byte) => Self {
                start: Some(Point {
                    line: range_without_byte.start.line,
                    column: Some(range_without_byte.start.column),
                }),
                end: Some(Point {
                    line: range_without_byte.end.line,
                    column: Some(range_without_byte.end.column),
                }),
            },
        }
    }
}

impl Matcher for Range {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        _state: &mut state::State<'a>,
        _context: &'a impl Context,
        _logs: &mut AnalysisLogs,
    ) -> anyhow::Result<bool> {
        if let Some(range) = binding.position() {
            if let Some(start) = &self.start {
                if start.line > range.start.line {
                    return Ok(false);
                }
                if let Some(column) = start.column {
                    if start.line == range.start.line && column > range.start.column {
                        return Ok(false);
                    }
                }
            }
            if let Some(end) = &self.end {
                if end.line < range.end.line {
                    return Ok(false);
                }
                if let Some(column) = end.column {
                    if end.line == range.end.line && column < range.end.column {
                        return Ok(false);
                    }
                }
            }
            return Ok(true);
        }
        Ok(false)
    }
}

impl Name for Range {
    fn name(&self) -> &'static str {
        "RANGE"
    }
}
