use serde::Deserialize;
use serde::Serialize;
use tree_sitter::Node;
use tree_sitter::Point as TPoint;
use tree_sitter::Range as TRange;

// same as TS functionality but well want to impl more functionality

// note that byte if present must match the coordinates in the file.
// in this sense maybe we want to add a file name to the struct?
// Leave for later as currently we only operate on one file at at time.

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]

pub struct Position {
    // line/column are now 1-indexed
    pub line: u32,
    pub column: u32,
}

impl Position {
    pub fn char_position_to_byte_position(self, context: &str) -> Self {
        let line = self.line;
        let column = context
            .lines()
            .nth(self.line as usize - 1)
            .unwrap()
            .chars()
            .take(self.column as usize - 1)
            .map(|c| c.len_utf8())
            .sum::<usize>() as u32
            + 1;
        Self { line, column }
    }

    pub fn byte_position_to_char_position(self, context: &str) -> Self {
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

    pub fn abbreviated_debug(&self) -> String {
        format!("({},{})", self.line, self.column)
    }

    pub fn new(row: u32, col: u32) -> Self {
        Self {
            line: row,
            column: col,
        }
    }

    pub fn first() -> Self {
        Self::new(1, 1)
    }

    pub fn last(content: &str) -> Self {
        let lines = content.lines();
        let line_count = lines.clone().count() as u32;
        let last_line = lines.last().unwrap_or("");
        let last_line_length = last_line.len() as u32;
        Self::new(line_count, last_line_length + 1)
    }

    pub fn add(&mut self, other: Position) {
        self.line += other.line - 1;
        self.column += other.column - 1;
    }

    pub fn from_byte_index(source: &str, old: Option<(Position, u32)>, byte_index: u32) -> Self {
        let (mut pos, mut start_byte) = old.unwrap_or((Self::first(), 0));
        for c in source[start_byte as usize..byte_index as usize].chars() {
            if c == '\n' {
                pos.line += 1;
                pos.column = 1;
            } else {
                pos.column += c.len_utf8() as u32;
            }
            start_byte += c.len_utf8() as u32;
        }
        pos
    }
}

impl From<TPoint> for Position {
    fn from(point: TPoint) -> Self {
        Self::new(point.row() + 1, point.column() + 1)
    }
}

pub trait ByteInterval {
    fn interval(&self) -> (usize, usize);

    fn earliest_deadline_sort<T>(list: &mut Vec<T>) -> bool
    where
        T: ByteInterval,
    {
        list.sort_by_key(|t| t.interval().1);
        for pair in list.windows(2) {
            if pair[0].interval().1 > pair[1].interval().0 {
                return false;
            }
        }
        true
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub start: Position,
    pub end: Position,
    pub start_byte: u32,
    pub end_byte: u32,
}

pub fn char_index_to_byte_index(index: u32, text: &str) -> u32 {
    text.chars()
        .take(index as usize)
        .map(|c| c.len_utf8())
        .sum::<usize>() as u32
}

pub fn byte_index_to_char_index(index: u32, text: &str) -> u32 {
    text.char_indices()
        .take_while(|(i, _)| *i < index as usize)
        .count() as u32
}

#[cfg(not(target_arch = "wasm32"))]
pub fn start_byte(node: &Node, _text: &str) -> u32 {
    node.start_byte()
}

#[cfg(target_arch = "wasm32")]
pub fn start_byte(node: &Node, text: &str) -> u32 {
    char_index_to_byte_index(node.start_byte(), text)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn end_byte(node: &Node, _text: &str) -> u32 {
    node.end_byte()
}

#[cfg(target_arch = "wasm32")]
pub fn end_byte(node: &Node, text: &str) -> u32 {
    char_index_to_byte_index(node.end_byte(), text)
}

impl Range {
    pub fn char_range_to_byte_range(self, context: &str) -> Self {
        let start = self.start.char_position_to_byte_position(context);
        let end = self.end.char_position_to_byte_position(context);
        let start_byte = char_index_to_byte_index(self.start_byte, context);
        let end_byte = char_index_to_byte_index(self.end_byte, context);
        Self {
            start,
            end,
            start_byte,
            end_byte,
        }
    }

    pub fn byte_range_to_char_range(self, context: &str) -> Self {
        let start = self.start.byte_position_to_char_position(context);
        let end = self.end.byte_position_to_char_position(context);
        let start_byte = byte_index_to_char_index(self.start_byte, context);
        let end_byte = byte_index_to_char_index(self.end_byte, context);
        Self {
            start,
            end,
            start_byte,
            end_byte,
        }
    }

    pub fn abbreviated_debug(&self) -> String {
        format!(
            "[{}-{}]/[{}-{}]",
            self.start.abbreviated_debug(),
            self.end.abbreviated_debug(),
            self.start_byte,
            self.end_byte
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

    pub fn from_md(
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

impl From<TRange> for Range {
    fn from(range: TRange) -> Self {
        let start = range.start_point();
        let end = range.end_point();
        Self::new(
            Position::new(start.row() + 1, start.column() + 1),
            Position::new(end.row() + 1, end.column() + 1),
            range.start_byte(),
            range.end_byte(),
        )
    }
}

// A simple range, without byte information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RangeWithoutByte {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UtilRange {
    Range(Range),
    RangeWithoutByte(RangeWithoutByte),
}

impl UtilRange {
    pub fn start_line(&self) -> u32 {
        match self {
            UtilRange::Range(range) => range.start.line,
            UtilRange::RangeWithoutByte(range) => range.start.line,
        }
    }

    pub fn end_line(&self) -> u32 {
        match self {
            UtilRange::Range(range) => range.end.line,
            UtilRange::RangeWithoutByte(range) => range.end.line,
        }
    }
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct FileRange {
    pub file_path: String,
    pub range: UtilRange,
}

pub fn map_one_indexed_position_to_zero_indexed(position: &Position) -> Position {
    Position::new(position.line - 1, position.column - 1)
}

pub fn map_zero_indexed_position_to_one_indexed(position: &Position) -> Position {
    Position::new(position.line + 1, position.column + 1)
}

pub fn get_one_indexed_position_offset(position: &Position, content: &str) -> usize {
    let lines: Vec<&str> = content.split('\n').collect();
    let mut offsets: Vec<usize> = vec![0; lines.len()];
    for i in 1..lines.len() {
        offsets[i] = offsets[i - 1] + (lines[i - 1].len() + 1);
    }
    offsets[(position.line - 1) as usize] + (position.column - 1) as usize
}

#[cfg(not(target_arch = "wasm32"))]
pub fn len(s: &str) -> u32 {
    s.len() as u32
}

// wasm version of tree-sitter counts characters not bytes
#[cfg(target_arch = "wasm32")]
pub fn len(s: &str) -> u32 {
    s.chars().count() as u32
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
    fn test_char_to_byte_range() {
        // [(1,8)-(1,10)]/[7-9] not found in map ["[(1,14)-(1,17)]/[13-16]", "[(1,8)-(1,11)]/[7-10]"]
        let range = Range::new(Position::new(1, 8), Position::new(1, 10), 7, 9);
        let new_range = range.char_range_to_byte_range("const [µb, fµa]");
        assert_eq!(
            new_range,
            Range::new(Position::new(1, 8), Position::new(1, 11), 7, 10)
        );
        let range = Range::new(Position::new(1, 13), Position::new(1, 15), 12, 14);
        let new_range = range.char_range_to_byte_range("const [µb, fµa]");
        assert_eq!(
            new_range,
            Range::new(Position::new(1, 14), Position::new(1, 17), 13, 16)
        );
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
