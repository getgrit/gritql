use anyhow::{anyhow, bail, Result};
use grit_util::{EffectRange, Language};
use itertools::Itertools;
use std::{cell::RefCell, collections::HashSet, ops::Range, rc::Rc};

/// Left hand side range, with the length of the replacement string
pub type ReplacementInfo = (Range<usize>, usize);

fn filter_out_nested(replacements: &mut Vec<(EffectRange, String)>) {
    let max_insert_index = match replacements.first() {
        Some((range, _)) => range.effective_range().end + 1,
        None => return,
    };
    let mut inserts = vec![HashSet::<String>::new(); max_insert_index];
    let mut i = 0;
    while i < replacements.len() {
        let insert_index = replacements[i].0.effective_range().end;
        inserts[insert_index].insert(replacements[i].1.clone());
        let mut j = i + 1;
        while j < replacements.len() {
            if replacements[i].0.effective_range().start
                <= replacements[j].0.effective_range().start
                && replacements[i].0.effective_range().end
                    >= replacements[j].0.effective_range().end
            {
                if replacements[j].0.effective_range().start
                    != replacements[j].0.effective_range().end
                    || (replacements[i].0.effective_range().start
                        < replacements[j].0.effective_range().start
                        && replacements[i].0.effective_range().end
                            > replacements[j].0.effective_range().end)
                {
                    replacements.remove(j);
                    continue;
                }
                let content = replacements[j].1.clone();
                if inserts[insert_index].contains(&content) {
                    replacements.remove(j);
                    continue;
                } else {
                    j += 1;
                }
            } else {
                j += 1;
            }
        }
        i += 1;
    }
}

/**
 * Manipulates a vector of replacements that are already correctly sorted by range end in-place to also
 * satisfy the secondary sorting mechanism of range start descending.
 * Ex. input [(12..23, "good: \"foo\", great: \"baz\""), (23..23, ", nice: \"bar\"")]
 * Output [(23..23, ", nice: \"bar\""), (12..23, "good: \"foo\", great: \"baz\"")]
 */
fn sort_range_start(replacements: &mut [(EffectRange, String)]) {
    let mut start_idx: Option<usize> = None;

    for i in 1..replacements.len() {
        if replacements[i].0.effective_range().end == replacements[i - 1].0.effective_range().end {
            if start_idx.is_none() {
                start_idx = Some(i - 1);
            }
        } else if let Some(start) = start_idx {
            replacements[start..i].sort_by(|a, b| {
                b.0.effective_range()
                    .start
                    .cmp(&a.0.effective_range().start)
            });
            start_idx = None;
        }
    }

    if let Some(start) = start_idx {
        replacements[start..].sort_by(|a, b| {
            b.0.effective_range()
                .start
                .cmp(&a.0.effective_range().start)
        });
    }
}

fn pad_snippet(
    language: &impl Language,
    context: &str,
    range_start: usize,
    snippet: &str,
) -> Result<String> {
    // Find last new-line character before range
    let newline_index = context
        .get(0..range_start)
        .ok_or(anyhow!(
            "Slice range does not align with character boundaries: This is probably a bug in Grit."
        ))?
        .rfind('\n');
    // Collect all leading whitespace, starting after the '\n' character
    let mut chars = context
        .get(newline_index.map(|n| n + 1).unwrap_or_default()..range_start)
        .ok_or(anyhow!(
            "Slice range does not align with character boundaries: This is probably a bug in Grit."
        ))?
        .chars()
        .peekable();
    let mut padding = Vec::new();
    while let Some(c) = chars.next() {
        if let Some(padding_value) = language.take_padding(c, chars.peek().copied()) {
            padding.push(padding_value);
        } else {
            break;
        }
    }

    let padding: String = padding.into_iter().collect();
    Ok(language.pad_snippet(snippet, &padding).to_string())
}

// checks on this one are likely redundant as
// we already sort and check in the linearization step
// maybe convert these to a debug assertion?
// also probably worth merging with the originial above.
pub(crate) fn inline_sorted_snippets_with_offset(
    language: &impl Language,
    code: String,
    offset: usize,
    replacements: &mut Vec<(EffectRange, String)>,
    should_pad_snippet: bool,
) -> Result<(String, Vec<Range<usize>>, Vec<ReplacementInfo>)> {
    if !is_sorted_descending(replacements) {
        bail!("Replacements must be in descending order.");
    }

    filter_out_nested(replacements);
    sort_range_start(replacements);

    if has_overlapping_ranges(replacements) {
        bail!("Ranges must not be partially overlapping. They must be either disjoint or nested.");
    }
    // this could probably be integrated with the following loop
    // top level snippets are not already padded, so we need to pad again here
    if should_pad_snippet && offset == 0 {
        for (range, snippet) in replacements.iter_mut() {
            let start = adjust_index(range.start(), offset)?;
            let padded_snippet = pad_snippet(language, &code, start, snippet)?;
            *snippet = padded_snippet;
        }
    }
    for (range, snippet) in replacements.iter_mut() {
        let old = code.get(range.range.start..range.range.end);
        if old.is_none() {
            continue;
        }
        let old_content = old.unwrap();
        let preceding_newline = code
            .get(0..range.range.start)
            .ok_or(anyhow!(
                "Slice range does not align with character boundaries: This is probably a bug in Grit."
            ))?
            .rfind('\n');
        let subsequent_newline = code
            .get(range.range.end..)
            .ok_or(anyhow!(
                "Slice range does not align with character boundaries: This is probably a bug in Grit."
            ))?
            .find('\n')
            .map(|i| i + range.range.end);
        if let Some(preceding) = preceding_newline {
            if let Some(subsequent) = subsequent_newline {
                let old_lines = code.get(preceding..subsequent);
                if old_lines.is_none() {
                    continue;
                }
                let old_lines = old_lines.unwrap();
                if old_lines.trim() != old_content.trim()
                    && old_lines.trim() != format!("{};", old_content.trim())
                {
                    continue;
                }
                if !old_content.is_empty() && snippet.trim().is_empty() {
                    range.range.start = preceding;
                    range.range.end = subsequent;
                }
            }
        }
    }

    let (mut code, original_ranges) = delete_hanging_comma(&code, replacements, offset)?;

    // we could optimize by checking if offset is zero, or some other flag
    // so we only compute if top level.
    let mut output_ranges: Vec<Range<usize>> = vec![];
    {
        let mut offset: isize = 0;
        for (range, len) in replacements.iter().map(|(r, s)| (r, s.len())).rev() {
            let range = range.effective_range();
            let start = range
                .start
                .checked_add_signed(offset)
                .ok_or_else(|| anyhow!("offset {} overflows start {}", offset, range.start))?;
            let end = start + len;
            // offset = offset + len - (range.end - range.start)
            offset = offset
                .checked_add_unsigned(len)
                .ok_or_else(|| {
                    anyhow!(
                        "{} + {} - ({} - {}) overflowed",
                        offset,
                        len,
                        range.end,
                        range.start
                    )
                })?
                .checked_add_unsigned(range.start)
                .ok_or_else(|| {
                    anyhow!(
                        "{} + {} - ({} - {}) overflowed",
                        offset,
                        len,
                        range.end,
                        range.start
                    )
                })?
                .checked_sub_unsigned(range.end)
                .ok_or_else(|| {
                    anyhow!(
                        "{} + {} - ({} - {}) overflowed",
                        offset,
                        len,
                        range.end,
                        range.start
                    )
                })?;
            output_ranges.push(start..end);
        }
    }

    for (range, snippet) in replacements {
        let range = adjust_range(&range.effective_range(), offset, &code)?;
        if range.start > code.len() || range.end > code.len() {
            bail!("Range {:?} is out of bounds for code:\n{}\n", range, code);
        }
        code.replace_range(range, snippet);
    }

    Ok((code, output_ranges, original_ranges))
}

fn adjust_range(range: &Range<usize>, offset: usize, code: &str) -> Result<Range<usize>> {
    if range.start < offset || range.end < offset {
        bail!("offset must not be greater than the range.");
    }
    if !code.is_char_boundary(range.start - offset) || !code.is_char_boundary(range.end - offset) {
        bail!("Offset range does not align with character boundaries: This is probably a bug in Grit.");
    }
    Ok((range.start - offset)..(range.end - offset))
}

fn adjust_index(index: usize, offset: usize) -> Result<usize> {
    if index < offset {
        bail!("offset must not be greater than the index.");
    }
    Ok(index - offset)
}

fn is_sorted_descending(replacements: &[(EffectRange, String)]) -> bool {
    replacements
        .iter()
        .map(|r| r.0.effective_range())
        .tuple_windows()
        .all(|(l, r)| l.end >= r.end)
}

fn has_overlapping_ranges(replacements: &[(EffectRange, String)]) -> bool {
    for i in 1..replacements.len() {
        if replacements[i - 1].0.effective_range().start < replacements[i].0.effective_range().end {
            return true;
        }
    }
    false
}

#[derive(Debug, PartialEq)]
enum RewriteRange {
    Rewrite,
    PostRewrite,
    Unknown,
}

fn is_in_deletion_range(index: usize, ranges: &[&Range<usize>]) -> bool {
    for range in ranges {
        if range.start <= index && index < range.end {
            return true;
        }
    }
    false
}

fn delete_hanging_comma(
    code: &str,
    replacements: &mut [(EffectRange, String)],
    offset: usize,
) -> Result<(String, Vec<ReplacementInfo>)> {
    let deletion_ranges = replacements
        .iter()
        .filter_map(|r| {
            if r.1.is_empty() {
                Some(adjust_range(&r.0.effective_range(), offset, code))
            } else {
                None
            }
        })
        .collect::<Result<Vec<Range<usize>>>>()?;

    // we reverse because in the event of multiple insertions we only want to delete
    // a comma for the first insertion, the unique at the end stably removes duplicates
    // we do not currently deduplicate commas when inserting with a comma at the end, as
    // that would require using the original order, since what matters is the last comma
    // to be inserted. wouldn't be hard to add, just another iteration with a new filter.
    let comma_inserts = replacements
        .iter_mut()
        .rev()
        .filter_map(|r| {
            if (r.1.trim().starts_with(',')) && r.0.effective_range().is_empty() {
                match adjust_range(&r.0.effective_range(), offset, code) {
                    Err(e) => Some(Err(e)),
                    Ok(range) => Some(Ok((range.start, Rc::new(RefCell::new(&mut r.1))))),
                }
            } else {
                None
            }
        })
        .collect::<Result<Vec<(usize, Rc<RefCell<&mut String>>)>>>()?
        .into_iter()
        .unique_by(|x| x.0)
        .collect::<Vec<(usize, Rc<RefCell<&mut String>>)>>();
    update_comma_insertion_strings(code, &comma_inserts);
    let to_delete = get_deletion_indices(code, &deletion_ranges);
    let ranges = replacements.iter().map(|r| r.0.clone()).collect::<Vec<_>>();
    let mut ranges_updates: Vec<(usize, usize)> = ranges.iter().map(|_| (0, 0)).collect();
    let mut to_delete = to_delete.iter();
    let mut result = String::new();

    let chars = code.chars().enumerate();

    let mut next_comma = to_delete.next();

    let mut replacement_ranges: Vec<(Range<usize>, usize)> = replacements
        .iter()
        .map(|r| (r.0.effective_range(), r.1.len()))
        .collect();
    
    // Flag to track if the last character was a comma
    let mut last_was_comma = false;
    
    for (index, c) in chars {
        if Some(&index) != next_comma {
            if c == ',' && last_was_comma {
                continue;
            }
            result.push(c);
            last_was_comma = c == ',';
        } else {
            // Keep track of ranges we need to expand into, since we deleted code in the range
            // This isn't perfect, but it's good enough for tracking cell boundaries
            for (range, ..) in replacement_ranges.iter_mut().rev() {
                if range.end >= index {
                    range.end += 1;
                    break;
                }
            }
            ranges_updates = update_range_shifts(index + offset, &ranges_updates, &ranges);
            next_comma = to_delete.next();
            last_was_comma = false;
        }
    }

    for (r, u) in replacements.iter_mut().zip(ranges_updates) {
        r.0.range.start -= u.0;
        r.0.range.end -= u.1;
    }
    Ok((result, replacement_ranges))
}

/// After commas are deleted, calculate how much each range has shifted
/// (start shift amount, end shift amount)
fn update_range_shifts(
    index: usize,
    shifts: &[(usize, usize)],
    ranges: &[EffectRange],
) -> Vec<(usize, usize)> {
    ranges
        .iter()
        .zip(shifts.iter())
        .map(|(range, shift)| {
            let l = range.range.start;
            let r = range.range.end;
            let mut sl = shift.0;
            let mut sr = shift.1;
            if l > index {
                sl += 1;
            }
            if r > index {
                sr += 1;
            }

            (sl, sr)
        })
        .collect()
}

fn get_insertion_at_index<'a>(
    index: usize,
    insertions: &[(usize, Rc<RefCell<&'a mut String>>)],
) -> Option<Rc<RefCell<&'a mut String>>> {
    for (range, insertion) in insertions.iter() {
        if index == *range {
            return Some(insertion.to_owned());
        }
    }
    None
}

fn update_comma_insertion_strings(input: &str, ranges: &[(usize, Rc<RefCell<&mut String>>)]) {
    let mut in_single_line_comment = false;
    let mut in_multiline_comment = false;
    let mut is_double_quoted_string_literal = false;
    let mut is_single_quoted_string_literal = false;
    let mut delete_future_comma = false;
    let mut chars = input.chars().enumerate().peekable();
    while let Some((index, c)) = chars.next() {
        if let Some(insertion) = get_insertion_at_index(index, ranges) {
            let mut comma_index = None;
            {
                let borrow = insertion.borrow();
                let chars = borrow.chars().enumerate();
                for (i, c) in chars {
                    if !c.is_whitespace() {
                        if c == ',' && delete_future_comma {
                            comma_index = Some(i);
                        }
                        break;
                    }
                }
            }
            if let Some(index) = comma_index {
                insertion.borrow_mut().remove(index);
            }
        }

        match c {
            '/' => {
                if let Some((_, '/')) = chars.peek() {
                    if !in_multiline_comment {
                        in_single_line_comment = true;
                    }
                } else if let Some((_, '*')) = chars.peek() {
                    in_multiline_comment = true;
                }
                if !in_single_line_comment && !in_multiline_comment {
                    delete_future_comma = false;
                }
            }
            '\n' => {
                in_single_line_comment = false;
            }
            '*' => {
                if is_double_quoted_string_literal || is_single_quoted_string_literal {
                    continue;
                }
                if in_multiline_comment && matches!(chars.peek(), Some((_, '/'))) {
                    in_multiline_comment = false;
                    chars.next().unwrap();
                } else if !in_single_line_comment && !in_multiline_comment {
                    delete_future_comma = false;
                }
            }
            '"' => {
                if !in_single_line_comment && !in_multiline_comment {
                    is_double_quoted_string_literal = !is_double_quoted_string_literal;
                    delete_future_comma = false;
                }
            }
            '\'' => {
                if !in_single_line_comment && !in_multiline_comment {
                    is_single_quoted_string_literal = !is_single_quoted_string_literal;
                    delete_future_comma = false;
                }
            }
            '(' | '[' | '{' => {
                if !in_single_line_comment
                    && !in_multiline_comment
                    && !is_double_quoted_string_literal
                    && !is_single_quoted_string_literal
                {
                    delete_future_comma = true;
                }
            }
            ',' if !in_single_line_comment
                && !in_multiline_comment
                && !is_double_quoted_string_literal
                && !is_single_quoted_string_literal =>
            {
                delete_future_comma = true;
            }
            '}' | ']' | ')' => {
                if !in_single_line_comment
                    && !in_multiline_comment
                    && !is_double_quoted_string_literal
                    && !is_single_quoted_string_literal
                {
                    delete_future_comma = false;
                }
            }
            _ => {
                if !in_single_line_comment && !in_multiline_comment && !c.is_whitespace() {
                    delete_future_comma = false;
                }
            }
        }
    }
}

fn get_deletion_indices(input: &str, ranges: &[Range<usize>]) -> Vec<usize> {
    let deletion_ranges = ranges
        .iter()
        .filter(|r| input.as_bytes()[r.start] as char != ':')
        .collect_vec();
    let mut in_single_line_comment = false;
    let mut in_multiline_comment = false;
    let mut is_double_quoted_string_literal = false;
    let mut is_single_quoted_string_literal = false;
    let mut deletion_range = RewriteRange::Unknown;
    let mut last_comma: Option<usize> = None;
    let mut chars = input.chars().enumerate().peekable();
    let mut result: Vec<usize> = vec![];
    while let Some((index, c)) = chars.next() {
        if is_in_deletion_range(index, &deletion_ranges) {
            deletion_range = RewriteRange::Rewrite;
        } else if let RewriteRange::Rewrite = deletion_range {
            deletion_range = RewriteRange::PostRewrite
        }
        match c {
            '/' => {
                if let Some((_, '/')) = chars.peek() {
                    if !in_multiline_comment {
                        in_single_line_comment = true;
                    }
                } else if let Some((_, '*')) = chars.peek() {
                    in_multiline_comment = true;
                }
                if !in_single_line_comment && !in_multiline_comment {
                    if RewriteRange::Rewrite != deletion_range {
                        last_comma = None;
                    }
                    if RewriteRange::PostRewrite == deletion_range {
                        deletion_range = RewriteRange::Unknown;
                    }
                }
            }
            '\n' => {
                in_single_line_comment = false;
            }
            '*' => {
                if is_double_quoted_string_literal || is_single_quoted_string_literal {
                    continue;
                }
                if in_multiline_comment && matches!(chars.peek(), Some((_, '/'))) {
                    in_multiline_comment = false;
                    chars.next().unwrap();
                } else if !in_single_line_comment && !in_multiline_comment {
                    if RewriteRange::Rewrite != deletion_range {
                        last_comma = None;
                    }
                    if RewriteRange::PostRewrite == deletion_range {
                        deletion_range = RewriteRange::Unknown;
                    }
                }
            }
            '"' => {
                if !in_single_line_comment && !in_multiline_comment {
                    is_double_quoted_string_literal = !is_double_quoted_string_literal;
                    if RewriteRange::Rewrite != deletion_range {
                        last_comma = None;
                    }
                    if RewriteRange::PostRewrite == deletion_range {
                        deletion_range = RewriteRange::Unknown;
                    }
                }
            }
            '\'' => {
                if !in_single_line_comment && !in_multiline_comment {
                    is_single_quoted_string_literal = !is_single_quoted_string_literal;
                    if RewriteRange::Rewrite != deletion_range {
                        last_comma = None;
                    }
                    if RewriteRange::PostRewrite == deletion_range {
                        deletion_range = RewriteRange::Unknown;
                    }
                }
            }
            '(' | '[' | '{' => {
                if !in_single_line_comment
                    && !in_multiline_comment
                    && !is_double_quoted_string_literal
                    && !is_single_quoted_string_literal
                {
                    if RewriteRange::Rewrite != deletion_range {
                        last_comma = None;
                    }
                    if RewriteRange::PostRewrite == deletion_range {
                        deletion_range = RewriteRange::Unknown;
                    }
                }
            }
            ',' if !in_single_line_comment
                && !in_multiline_comment
                && !is_double_quoted_string_literal
                && !is_single_quoted_string_literal =>
            {
                match deletion_range {
                    RewriteRange::Rewrite => {}
                    RewriteRange::PostRewrite => {
                        result.push(index);
                        last_comma = None;
                        deletion_range = RewriteRange::Unknown;
                    }
                    RewriteRange::Unknown => {
                        last_comma = Some(index);
                    }
                }
            }
            '}' | ']' | ')' => {
                if !in_single_line_comment
                    && !in_multiline_comment
                    && !is_double_quoted_string_literal
                    && !is_single_quoted_string_literal
                {
                    if RewriteRange::PostRewrite == deletion_range {
                        if let Some(last_comma) = last_comma {
                            result.push(last_comma);
                        }
                        deletion_range = RewriteRange::Unknown;
                    }
                    if RewriteRange::Rewrite != deletion_range {
                        last_comma = None;
                    }
                }
            }
            _ => {
                if !in_single_line_comment && !in_multiline_comment && !c.is_whitespace() {
                    if RewriteRange::Rewrite != deletion_range {
                        last_comma = None;
                    }
                    if RewriteRange::PostRewrite == deletion_range {
                        deletion_range = RewriteRange::Unknown;
                    }
                }
            }
        }
    }
    result
}
