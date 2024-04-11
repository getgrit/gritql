use anyhow::Result;
use grit_util::{traverse, Order};
use itertools::Itertools;
use marzano_language::{language::Language, target_language::TargetLanguage};
use marzano_util::cursor_wrapper::CursorWrapper;
use marzano_util::position::Range;
use tree_sitter::{Parser, Tree};

pub(crate) type Replacement = (Range, String);

fn merge_ranges(ranges: Vec<Range>) -> Vec<Range> {
    if ranges.is_empty() {
        return vec![];
    }

    let sorted: Vec<Range> = ranges
        .into_iter()
        .sorted_by(|a, b| b.start_byte.cmp(&a.start_byte))
        .collect_vec();
    let mut result = vec![];
    let mut current_range = sorted[0].to_owned();

    for range in sorted.into_iter().skip(1) {
        if current_range.start_byte <= range.end_byte {
            current_range.start_byte = current_range.start_byte.min(range.start_byte);
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
    orphan_ranges: Vec<Replacement>,
    src: &str,
) -> Result<Option<String>> {
    let mut removable_ranges = orphan_ranges;
    let mut src = src.to_string();
    for range in &removable_ranges {
        src.drain(range.start_byte as usize..range.end_byte as usize);
        // src.replace_range(range.start_byte as usize..range.start_byte as usize, " ".repeat(range.end_byte as usize - range.start_byte as usize).as_str());
    }
    let new_tree = if !removable_ranges.is_empty() {
        Some(parser.parse(src.as_bytes(), None).unwrap().unwrap())
    } else {
        None
    };
    let new_src = if new_tree.is_some() { Some(src) } else { None };
    Ok((new_tree, new_src))
}

pub fn get_orphaned_ranges(tree: &Tree, src: &str, lang: &TargetLanguage) -> Vec<Range> {
    let mut orphan_ranges = vec![];
    let cursor = tree.walk();
    for n in traverse(CursorWrapper::new(cursor, src), Order::Pre) {
        lang.check_orphaned(n, &mut orphan_ranges);
    }
    orphan_ranges
}
