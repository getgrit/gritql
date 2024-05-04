use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[cfg_attr(feature = "napi", napi_derive::napi(object))]
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
        Self::from_relative_byte_index(Self::first(), 0, source, byte_index)
    }

    /// Create a position for the given `byte_index` in the given `source`,
    /// counting from the given other position. This avoids double work in case
    /// one position lies after another.
    pub fn from_relative_byte_index(
        mut prev: Self,
        prev_byte_index: usize,
        source: &str,
        byte_index: usize,
    ) -> Self {
        for c in source[prev_byte_index..byte_index].chars() {
            if c == '\n' {
                prev.line += 1;
                prev.column = 1;
            } else {
                prev.column += c.len_utf8() as u32;
            }
        }
        prev
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
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}
