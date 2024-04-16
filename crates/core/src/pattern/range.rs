use super::{
    patterns::{Matcher, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::QueryContext;
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use marzano_util::position::UtilRange;

#[derive(Debug, Clone)]
pub struct Point {
    line: u32,
    column: Option<u32>,
}

impl Point {
    pub fn new(line: Option<u32>, column: Option<u32>) -> Result<Option<Self>> {
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
    pub start: Option<Point>,
    pub end: Option<Point>,
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

impl<Q: QueryContext> Matcher<Q> for Range {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        _state: &mut State<'a, Q>,
        _context: &'a Q::ExecContext<'a>,
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

impl PatternName for Range {
    fn name(&self) -> &'static str {
        "RANGE"
    }
}
