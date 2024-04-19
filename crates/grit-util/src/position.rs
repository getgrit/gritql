use serde::{Deserialize, Serialize};
use std::{fmt::Display, path::PathBuf};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]

pub struct Position {
    /// 1-based line number in the source file.
    pub line: u32,

    /// 1-based column number in the source file.
    pub column: u32,
}

impl Position {
    pub fn new(line: u32, column: u32) -> Self {
        Self { line, column }
    }

    /// Returns the first position in any source string.
    pub fn first() -> Self {
        Self::new(1, 1)
    }

    /// Returns the last position in the given `source`.
    pub fn last(source: &str) -> Self {
        Self::from_byte_index(source, source.len())
    }

    /// Adds another position to this one.
    pub fn add(&mut self, other: Position) {
        self.line += other.line - 1;
        self.column += other.column - 1;
    }

    /// Creates a position for the given `byte_index` in the given `source`.
    pub fn from_byte_index(source: &str, byte_index: usize) -> Self {
        let mut line_count = 0;
        let mut last_line = source;
        for line in source[..byte_index].lines() {
            line_count += 1;
            last_line = line;
        }
        let last_line_length = last_line.len() as u32;
        Self::new(line_count, last_line_length + 1)
    }

    /// Returns the byte index for this `Position` within the given `source`.
    pub fn byte_index(&self, source: &str) -> usize {
        let line_start_index: usize = source
            .split('\n')
            .take((self.line as usize).saturating_sub(1))
            .map(|line| line.len() + 1)
            .sum();
        line_start_index + (self.column as usize) - 1
    }

    /// Converts a position expressed in byte indices to a position expressed in
    /// character offsets.
    fn byte_position_to_char_position(self, context: &str) -> Self {
        let mut char_pos = Position { line: 1, column: 1 };
        let mut bytes_processed = 0;

        for c in context.chars() {
            bytes_processed += c.len_utf8();

            if self.line == char_pos.line && bytes_processed >= self.column as usize {
                break;
            }

            if c == '\n' {
                char_pos.line += 1;
                char_pos.column = 1;
            } else {
                char_pos.column += 1;
            }
        }

        char_pos
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub start: Position,
    pub end: Position,
    pub start_byte: u32,
    pub end_byte: u32,
}

impl Range {
    pub fn abbreviated_debug(&self) -> String {
        format!(
            "[{}-{}]/[{}-{}]",
            self.start, self.end, self.start_byte, self.end_byte
        )
    }

    pub fn new(start: Position, end: Position, start_byte: u32, end_byte: u32) -> Self {
        Self {
            start,
            end,
            start_byte,
            end_byte,
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

    /// Converts a range expressed in byte indices to a range expressed in
    /// character offets.
    pub fn byte_range_to_char_range(self, context: &str) -> Self {
        let start = self.start.byte_position_to_char_position(context);
        let end = self.end.byte_position_to_char_position(context);
        let start_byte = byte_index_to_char_offset(self.start_byte, context);
        let end_byte = byte_index_to_char_offset(self.end_byte, context);
        Self {
            start,
            end,
            start_byte,
            end_byte,
        }
    }
}

// A simple range, without byte information
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

#[derive(Clone, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FileRange {
    pub file_path: PathBuf,
    pub range: UtilRange,
}

fn byte_index_to_char_offset(index: u32, text: &str) -> u32 {
    text.char_indices()
        .take_while(|(i, _)| *i < index as usize)
        .count() as u32
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
        let range = Range::new(Position::new(1, 8), Position::new(1, 10), 7, 9);
        let new_range = range.byte_range_to_char_range("const [µb, fµa]");
        assert_eq!(
            new_range,
            Range::new(Position::new(1, 8), Position::new(1, 9), 7, 8)
        );
        let range = Range::new(Position::new(1, 16), Position::new(1, 18), 15, 17);
        let new_range = range.byte_range_to_char_range("const [µb, fµa]");
        assert_eq!(
            new_range,
            Range::new(Position::new(1, 14), Position::new(1, 16), 13, 15)
        );
    }
}
