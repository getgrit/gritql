use std::path::PathBuf;

use anyhow::{anyhow, Result};
use marzano_core::pattern::{api::Rewrite, built_in_functions::BuiltIns};
use marzano_util::{
    position::{
        map_one_indexed_position_to_zero_indexed, map_zero_indexed_position_to_one_indexed,
    },
    rich_path::RichFile,
};
use tower_lsp::lsp_types::{Position, Range, TextDocumentItem, TextEdit, Url};

pub fn uri_to_file_path(uri: &str) -> Result<PathBuf> {
    let url = Url::parse(uri)?;
    match url.to_file_path() {
        Ok(path) => Ok(path),
        Err(_) => Err(anyhow!("Unable to find file corresponding to uri {}", uri)),
    }
}

pub fn convert_grit_position_to_lsp_position(pos: &marzano_util::position::Position) -> Position {
    let new_pos = map_one_indexed_position_to_zero_indexed(pos);
    Position::new(new_pos.line, new_pos.column)
}

pub fn convert_lsp_position_to_grit_position(pos: &Position) -> marzano_util::position::Position {
    map_zero_indexed_position_to_one_indexed(&marzano_util::position::Position::new(
        pos.line,
        pos.character,
    ))
}

pub fn convert_grit_range_to_lsp_range(
    pos: &marzano_util::position::Range,
) -> tower_lsp::lsp_types::Range {
    let start = convert_grit_position_to_lsp_position(&pos.start);
    let end = convert_grit_position_to_lsp_position(&pos.end);
    tower_lsp::lsp_types::Range { start, end }
}

pub fn convert_lsp_range_to_grit_range(
    range: &tower_lsp::lsp_types::Range,
    src: &str,
) -> marzano_util::position::Range {
    let start = convert_lsp_position_to_grit_position(&range.start);
    let end = convert_lsp_position_to_grit_position(&range.end);
    let start_byte = one_based_position_to_byte(&start, src);
    let end_byte = one_based_position_to_byte(&end, src);
    marzano_util::position::Range {
        start,
        end,
        start_byte: start_byte as u32,
        end_byte: end_byte as u32,
    }
}

fn one_based_position_to_byte(position: &marzano_util::position::Position, content: &str) -> usize {
    let mut line = 1;
    let mut column = 0;
    let mut byte = 0;
    for (i, c) in content.char_indices() {
        if line == position.line && column == position.column {
            break;
        }
        if c == '\n' {
            line += 1;
            column = 0;
        } else {
            column += 1;
        }
        byte = i;
    }
    byte
}

pub fn trim_one_match(s: &str, pattern: char) -> &str {
    let mut start = 0;
    let mut end = s.len();

    if let Some(first) = s.chars().next() {
        if first == pattern {
            start += first.len_utf8();
        }
    }

    if let Some(last) = s.chars().last() {
        if last == pattern {
            end -= last.len_utf8();
        }
    }

    &s[start..end]
}

pub fn document_as_rich_file(document: TextDocumentItem) -> Result<RichFile> {
    Ok(RichFile {
        path: uri_to_file_path(document.uri.as_ref())?
            .to_string_lossy()
            .to_string(),
        content: document.text,
    })
}

pub fn rewrite_as_edit(doc: &TextDocumentItem, result: Rewrite) -> TextEdit {
    let current_content = &doc.text;
    let old_range = {
        let start = offset_to_zero_based_position(0, current_content);
        let end = offset_to_zero_based_position(current_content.len(), current_content);
        tower_lsp::lsp_types::Range { start, end }
    };
    let new_text = result.rewritten.content;
    TextEdit::new(old_range, new_text)
}

fn offset_to_zero_based_position(offset: usize, content: &str) -> Position {
    let mut line = 0;
    let mut column = 0;
    for (i, c) in content.char_indices() {
        if i == offset {
            break;
        }
        if c == '\n' {
            line += 1;
            column = 0;
        } else {
            column += 1;
        }
    }
    Position::new(line, column)
}

/// Confirm if two LSP ranges intersect
pub fn check_intersection(range1: &Range, range2: &Range) -> bool {
    range1.start.line <= range2.end.line
        && range1.end.line >= range2.start.line
        && range1.start.character <= range2.end.character
        && range1.end.character >= range2.start.character
}

pub(crate) fn get_ai_built_in_functions_for_feature() -> Option<BuiltIns> {
    #[cfg(not(feature = "ai_builtins"))]
    return None;
    #[cfg(feature = "ai_builtins")]
    return Some(ai_builtins::ai_builtins::get_ai_built_in_functions());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn zero_offset_to_position() {
        let content = "function foo() {\n    return 1;\n}\n";
        let position = offset_to_zero_based_position(0, content);
        assert_eq!(
            position,
            Position {
                line: 0,
                character: 0
            }
        );
    }

    #[test]
    fn text_length_offset_to_position() {
        let content = "function foo() {\n    return 1;\n}";
        let position = offset_to_zero_based_position(content.len(), content);
        assert_eq!(
            position,
            Position {
                line: 2,
                character: 1
            }
        );
    }

    #[test]
    fn position_to_byte() {
        let content = "function foo() {\n    return 1;\n}";
        let position = marzano_util::position::Position::new(3, 1);
        let byte = one_based_position_to_byte(&position, content);
        assert_eq!(byte, content.len() - 1);
    }

    #[test]
    fn position_to_middle_byte() {
        let content = "function foo() {\n    return 1;\n}";
        let position = marzano_util::position::Position::new(2, 5);
        let byte = one_based_position_to_byte(&position, content);
        assert_eq!(byte, 21);
    }
}
