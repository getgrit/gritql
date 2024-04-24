use anyhow::Result;
use grit_util::{traverse, Order};
use itertools::Itertools;
use marzano_language::language::Language;
use marzano_language::language::Replacement;
use marzano_util::{cursor_wrapper::CursorWrapper, node_with_source::NodeWithSource};

fn merge_ranges(ranges: Vec<Replacement>) -> Vec<Replacement> {
    if ranges.is_empty() {
        return vec![];
    }

    let sorted: Vec<Replacement> = ranges
        .into_iter()
        .sorted_by(|a, b| b.range.start_byte.cmp(&a.range.start_byte))
        .collect_vec();
    let mut result = vec![];
    let mut current_range = sorted[0].to_owned();

    for range in sorted.into_iter().skip(1) {
        if current_range.range.start_byte <= range.range.end_byte
            && current_range.replacement == range.replacement
        {
            current_range.range.start_byte =
                current_range.range.start_byte.min(range.range.start_byte);
        } else {
            result.push(current_range);
            current_range = range.to_owned();
        }
    }
    result.push(current_range);
    result
}

pub(crate) fn replace_cleaned_ranges(
    replacement_ranges: Vec<Replacement>,
    src: &str,
) -> Result<Option<String>> {
    if replacement_ranges.is_empty() {
        return Ok(None);
    }
    let replacement_ranges = merge_ranges(replacement_ranges);
    let mut src = src.to_string();
    for range in &replacement_ranges {
        src.replace_range(
            range.range.start_byte as usize..range.range.end_byte as usize,
            range.replacement,
        );
    }
    Ok(Some(src))
}

pub fn get_replacement_ranges(node: NodeWithSource, lang: &impl Language) -> Vec<Replacement> {
    let mut replacement_ranges = vec![];
    let cursor = node.node.walk();
    for n in traverse(CursorWrapper::new(cursor, node.source), Order::Pre) {
        lang.check_replacements(n, &mut replacement_ranges);
    }
    replacement_ranges
}
