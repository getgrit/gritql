use serde::{Deserialize, Serialize};
use std::fmt::Display;

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
        let mut pos = Self::first();
        for c in source[..byte_index].chars() {
            if c == '\n' {
                pos.line += 1;
                pos.column = 1;
            } else {
                pos.column += c.len_utf8() as u32;
            }
        }
        pos
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
    pub(crate) fn byte_position_to_char_position(self, context: &str) -> Self {
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
