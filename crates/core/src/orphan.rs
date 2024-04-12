use anyhow::Result;
use grit_util::{traverse, Order};
use itertools::Itertools;
use marzano_language::{language::Language, target_language::TargetLanguage};
use marzano_util::cursor_wrapper::CursorWrapper;
use marzano_util::position::Range;
use tree_sitter::{Parser, Tree};

pub(crate) type Replacement = (Range, Option<String>);

fn merge_ranges(ranges: Vec<Replacement>) -> Vec<Replacement> {
    if ranges.is_empty() {
        return vec![];
    }

    let sorted: Vec<Replacement> = ranges
        .into_iter()
        .sorted_by(|a, b| b.0.start_byte.cmp(&a.0.start_byte))
        .collect_vec();
    let mut result = vec![];
    let mut current_range = sorted[0].to_owned();

    for range in sorted.into_iter().skip(1) {
        if current_range.0.start_byte <= range.0.end_byte && current_range.1 == range.1 {
            current_range.0.start_byte = current_range.0.start_byte.min(range.0.start_byte);
        } else {
            result.push(current_range);
            current_range = range.to_owned();
        }
    }
    result.push(current_range);
    result
}

pub(crate) fn replace_cleaned_ranges(
    parser: &mut Parser,
    replacement_ranges: Vec<Replacement>,
    src: &str,
) -> Result<Option<String>> {
    if replacement_ranges.is_empty() {
        return Ok(None);
    }
    let replacement_ranges = merge_ranges(replacement_ranges);
    let mut src = src.to_string();
    for range in &replacement_ranges {
        src.drain(range.0.start_byte as usize..range.0.end_byte as usize);
        // src.replace_range(range.start_byte as usize..range.start_byte as usize, " ".repeat(range.end_byte as usize - range.start_byte as usize).as_str());
    }
    Ok(Some(src))
}

pub fn get_orphaned_ranges(tree: &Tree, src: &str, lang: &TargetLanguage) -> Vec<Range> {
    let mut orphan_ranges = vec![];
    let cursor = tree.walk();
    for n in traverse(CursorWrapper::new(cursor, src), Order::Pre) {
        lang.check_orphaned(n, &mut orphan_ranges);
    }
    orphan_ranges
}
