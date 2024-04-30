use anyhow::Result;
use grit_util::Position;
use marzano_core::api::{ByteRange, MatchResult};
use std::process::Command;

const JAVASCRIPT_EXTENSIONS: [&str; 6] = ["js", "jsx", "ts", "tsx", "cjs", "mjs"];

fn get_extended_byte_range(content: &str, range: &ByteRange) -> ByteRange {
    let Position {
        line: start_line, ..
    } = character_offset_to_range(content, range.start);
    let Position { line: end_line, .. } = character_offset_to_range(content, range.end);
    let total_lines = count_lines(content);
    let lines: Vec<&str> = content.split('\n').collect();

    let mut start_line_to_check = start_line.saturating_sub(2) as usize;
    while start_line_to_check > 0 {
        if let Some(line) = lines.get(start_line_to_check) {
            if !line.trim().is_empty() {
                break;
            }
        } else {
            break;
        }
        start_line_to_check = start_line_to_check.saturating_sub(1);
    }

    // Extend the end offset downwards until a non-empty line is found
    let mut end_line_to_check = end_line as usize;
    while end_line_to_check < total_lines.saturating_sub(1) {
        if let Some(line) = lines.get(end_line_to_check) {
            if !line.trim().is_empty() {
                break;
            }
        } else {
            break;
        }
        end_line_to_check = end_line_to_check.saturating_add(1);
    }

    let start_offset = Position {
        line: start_line_to_check as u32 + 1,
        column: 1,
    }
    .byte_index(content);

    let end_offset = if end_line_to_check >= total_lines - 1 {
        content.len()
    } else {
        Position {
            line: end_line_to_check as u32 + 2,
            column: 1,
        }
        .byte_index(content)
    };

    ByteRange::new(start_offset, end_offset)
}

fn character_offset_to_range(content: &str, offset: usize) -> Position {
    if offset > content.len() {
        panic!("Offset is out of file bounds");
    }

    let mut line = 1;
    let mut column = 1;

    for c in content.chars().take(offset) {
        if c == '\n' {
            line += 1;
            column = 1;
        } else {
            column += 1;
        }
    }

    Position { line, column }
}

fn count_lines(content: &str) -> usize {
    content.matches('\n').count() + 1
}

fn format_ranges(path_to_format: &str, extended_ranges: &Vec<ByteRange>) {
    for extended_range in extended_ranges {
        let start_range = extended_range.start.to_string();
        let end_range = extended_range.end.to_string();

        let _ = Command::new("npx")
            .arg("prettier")
            .arg("--range-start")
            .arg(start_range)
            .arg("--range-end")
            .arg(end_range)
            .arg("--parser")
            .arg("typescript")
            .arg(path_to_format)
            .arg("--write")
            .output();
    }
}

fn format_entire_file(path_to_format: &str) {
    let _ = Command::new("prettier")
        .arg("--parser")
        .arg("typescript")
        .arg(path_to_format)
        .output();
}

pub fn format_result(r: MatchResult) -> Result<()> {
    match r {
        MatchResult::CreateFile(res) => {
            if !JAVASCRIPT_EXTENSIONS
                .iter()
                .any(|ext| res.rewritten.source_file.ends_with(ext))
            {
                return Ok(());
            }
            format_entire_file(&res.rewritten.source_file);
        }
        MatchResult::Rewrite(res) => {
            if !JAVASCRIPT_EXTENSIONS
                .iter()
                .any(|ext| res.rewritten.source_file.ends_with(ext))
            {
                return Ok(());
            }
            if let Some(byte_ranges) = &res.rewritten.byte_ranges {
                let extended_ranges: Vec<ByteRange> = byte_ranges
                    .iter()
                    .map(|range| get_extended_byte_range(&res.rewritten.content, range))
                    .collect();
                format_ranges(&res.rewritten.source_file, &extended_ranges);
            }
        }
        _ => {}
    };

    Ok(())
}
