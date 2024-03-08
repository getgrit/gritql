use marzano_util::position::get_one_indexed_position_offset;
use tower_lsp::lsp_types::{Position, TextDocumentItem};

use crate::util::convert_lsp_position_to_grit_position;

pub fn get_identifier(document: &TextDocumentItem, position: &Position) -> String {
    let content = &document.text;
    let query_position =
        get_one_indexed_position_offset(&convert_lsp_position_to_grit_position(position), content);
    let start_offset = content[..query_position]
        .rfind(|c: char| !c.is_alphanumeric() && c != '_')
        .unwrap_or(0);
    let end_offset = content[query_position..]
        .find(|c: char| !c.is_alphanumeric() && c != '_')
        .map(|i| i + query_position)
        .unwrap_or(content.len());
    content[(start_offset + 1)..end_offset].to_string()
}
