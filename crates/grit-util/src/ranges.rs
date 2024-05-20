use crate::Position;
use serde::{Deserialize, Serialize};
use std::{ops::Add, path::PathBuf};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub start: Position,
    pub end: Position,
    // TODO: automatically derive these from the start and end positions during deserialization
    #[serde(skip_deserializing)]
    pub start_byte: u32,
    #[serde(skip_deserializing)]
    pub end_byte: u32,
}

impl Range {
    pub fn new(start: Position, end: Position, start_byte: u32, end_byte: u32) -> Self {
        Self {
            start,
            end,
            start_byte,
            end_byte,
        }
    }

    pub fn from_byte_range(source: &str, byte_range: ByteRange) -> Self {
        let start = Position::from_byte_index(source, byte_range.start);
        let end =
            Position::from_relative_byte_index(start, byte_range.start, source, byte_range.end);
        Self {
            start,
            end,
            start_byte: byte_range.start as u32,
            end_byte: byte_range.end as u32,
        }
    }

    pub fn add(&mut self, other: Position, other_byte: u32) {
        self.start.add(other);
        self.end.add(other);
        self.start_byte += other_byte;
        self.end_byte += other_byte;
    }

    pub fn range_index(&self) -> std::ops::Range<usize> {
        self.start_byte as usize..self.end_byte as usize
    }

    pub fn from_byteless(range: RangeWithoutByte, str: &str) -> Self {
        let mut start_byte = 0;
        let mut byte_length = 0;

        let start_line_zero_indexed = range.start.line as usize - 1;
        let end_line_zero_indexed = range.end.line as usize - 1;

        for (current_line, line) in str.lines().enumerate() {
            if current_line < start_line_zero_indexed {
                start_byte += line.len() as u32 + 1;
            } else if current_line == start_line_zero_indexed {
                start_byte += range.start.column - 1;
                // If this is *also* the end, we must handle that here
                if current_line == end_line_zero_indexed {
                    byte_length += range.end.column - range.start.column;
                    break;
                } else {
                    byte_length += (line.len() as u32 + 1) - range.start.column;
                }
            } else if current_line < end_line_zero_indexed {
                byte_length += line.len() as u32 + 1;
            } else if current_line == end_line_zero_indexed {
                byte_length += range.end.column;
                break;
            }
        }

        Self {
            start: range.start,
            end: range.end,
            start_byte,
            end_byte: start_byte + byte_length,
        }
    }

    #[cfg(test)]
    fn from_md(
        start_line: usize,
        start_col: usize,
        end_line: usize,
        end_col: usize,
        start_byte: usize,
        end_byte: usize,
    ) -> Self {
        Self {
            start: Position::new(start_line as u32, start_col as u32),
            end: Position::new(end_line as u32, end_col as u32),
            start_byte: start_byte as u32,
            end_byte: end_byte as u32,
        }
    }

    pub fn adjust_columns(&mut self, start: i32, end: i32) -> bool {
        if let (Some(start), Some(end), Some(start_byte), Some(end_byte)) = (
            self.start.column.checked_add_signed(start),
            self.end.column.checked_add_signed(end),
            self.start_byte.checked_add_signed(start),
            self.end_byte.checked_add_signed(end),
        ) {
            self.start.column = start;
            self.end.column = end;
            self.start_byte = start_byte;
            self.end_byte = end_byte;
            true
        } else {
            false
        }
    }

    // Return the 0-based indexes within the line where the match exists, if any
    pub fn get_line_range(&self, line: u32, line_length: u32) -> Option<(usize, usize)> {
        let max_length = if line_length == 0 { 1 } else { line_length + 1 };
        let (start, end) = if line < self.start.line || line > self.end.line {
            return None;
        } else if self.start.line == line && self.end.line == line {
            (self.start.column - 1, self.end.column - 1)
        } else if self.start.line == line {
            (self.start.column - 1, max_length - 1)
        } else if self.end.line == line {
            (0, self.end.column - 1)
        } else {
            (0, max_length - 1)
        };
        Some((start as usize, end as usize))
    }
}

// A simple range, without byte information
#[cfg_attr(feature = "napi", napi_derive::napi(object))]
#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub struct RangeWithoutByte {
    pub start: Position,
    pub end: Position,
}

impl RangeWithoutByte {
    pub fn start_column(&self) -> u32 {
        self.start.column
    }

    pub fn end_column(&self) -> u32 {
        self.end.column
    }

    pub fn start_line(&self) -> u32 {
        self.start.line
    }

    pub fn end_line(&self) -> u32 {
        self.end.line
    }

    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(untagged)]
pub enum UtilRange {
    Range(Range),
    RangeWithoutByte(RangeWithoutByte),
}

impl From<Range> for UtilRange {
    fn from(range: Range) -> Self {
        Self::Range(range)
    }
}

impl From<RangeWithoutByte> for UtilRange {
    fn from(range: RangeWithoutByte) -> Self {
        Self::RangeWithoutByte(range)
    }
}

#[derive(Clone, Copy, Debug, Hash, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct ByteRange {
    pub start: usize,
    pub end: usize,
}

impl ByteRange {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn abbreviated_debug(&self) -> String {
        format!("[{}-{}]", self.start, self.end)
    }

    /// Converts a range expressed in byte indices to a range expressed in
    /// character offsets.
    pub fn to_char_range(self, context: &str) -> Self {
        let start = byte_index_to_char_offset(self.start, context);
        let end = byte_index_to_char_offset(self.end, context);
        Self { start, end }
    }
}

impl From<Range> for ByteRange {
    fn from(value: Range) -> Self {
        Self {
            start: value.start_byte as usize,
            end: value.end_byte as usize,
        }
    }
}

impl From<std::ops::Range<usize>> for ByteRange {
    fn from(range: std::ops::Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl Add<usize> for ByteRange {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self {
            start: self.start + rhs,
            end: self.end + rhs,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRange {
    pub file_path: PathBuf,
    pub range: UtilRange,
}

fn byte_index_to_char_offset(index: usize, text: &str) -> usize {
    text.char_indices().take_while(|(i, _)| *i < index).count()
}

#[derive(Debug, Clone)]
pub struct InputRanges {
    pub ranges: Vec<Range>,
    pub variables: Vec<VariableMatch>,
    pub suppressed: bool,
}

#[derive(Debug, Clone, Default)]
pub struct MatchRanges {
    pub input_matches: Option<InputRanges>,
    pub byte_ranges: Option<Vec<ByteRange>>,
}

impl MatchRanges {
    pub fn new(byte_ranges: Vec<ByteRange>) -> Self {
        Self {
            input_matches: None,
            byte_ranges: Some(byte_ranges),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct VariableBinding {
    pub name: String,
    pub scoped_name: String,
    pub ranges: Vec<ByteRange>,
}

impl VariableBinding {
    pub fn new(name: String, scoped_name: String, ranges: Vec<ByteRange>) -> Self {
        Self {
            name,
            scoped_name,
            ranges,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct VariableMatch {
    pub name: String,
    pub scoped_name: String,
    pub ranges: Vec<Range>,
}

impl VariableMatch {
    pub fn new(name: String, scoped_name: String, ranges: Vec<Range>) -> Self {
        Self {
            name,
            scoped_name,
            ranges,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_range_one_char_line() {
        let content = "a";
        let range = Range::from_md(1, 1, 1, 2, 0, 1);
        let (start, end) = range.get_line_range(1, 1).unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 1);
        let content_highlighted = content[start..end].to_string();
        assert_eq!(content_highlighted, "a");
    }

    #[test]
    fn test_long_one_char() {
        let content = "1\n2\n3\n4\n5\n6\n7\n8\n9\n10\n11\n12\n13\n14\n15\n";
        let range = Range::from_md(5, 1, 10, 1, 8, 13);
        let line_number = 8;
        let lines = content.lines().collect::<Vec<_>>();
        let line = lines[line_number as usize - 1];
        let line_length = line.len() as u32;
        let (start, end) = range.get_line_range(line_number, line_length).unwrap();
        assert_eq!(start, 0);
        assert_eq!(end, 1);
        let content_highlighted = line[start..end].to_string();
        assert_eq!(content_highlighted, "8");
    }

    #[test]
    fn byte_range_to_char_range() {
        let range = ByteRange::new(7, 9);
        let new_range = range.to_char_range("const [µb, fµa]");
        assert_eq!(new_range, ByteRange::new(7, 8));
        let range = ByteRange::new(15, 17);
        let new_range = range.to_char_range("const [µb, fµa]");
        assert_eq!(new_range, ByteRange::new(13, 15));
    }
}
