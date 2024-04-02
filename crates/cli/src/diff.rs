use anyhow::Result;
use marzano_util::position::FileRange;

pub fn parse_modified_ranges(diff: &str) -> Result<Vec<FileRange>> {
    let mut results = Vec::new();
    let lines = diff.lines();

    let mut current_file = String::new();
    let mut start_pos = Position { line: 0, column: 0 };
    let mut end_pos = Position { line: 0, column: 0 };

    for line in lines {
        if line.starts_with("+++") {
            current_file = line.split_whitespace().nth(1).unwrap_or("").to_string();
            if current_file.starts_with("b/") {
                current_file = current_file[2..].to_string();
            }
        } else if line.starts_with("@@") {
            let range_part = line.split_whitespace().nth(2).unwrap_or("");
            let range_parts: Vec<&str> = range_part.split(',').collect();
            if let Ok(line_num) = u32::from_str(range_parts[0].trim_start_matches('+')) {
                start_pos.line = line_num;
                end_pos.line = line_num + range_parts.get(1).map_or(0, |&x| x.parse::<u32>().unwrap_or(0)) - 1;
            }

            results.push(FileRange {
                file_path: current_file.clone(),
                range: UtilRange::RangeWithoutByte(RangeWithoutByte {
                    start: start_pos.clone(),
                    end: end_pos.clone(),
                }),
            });
        }
    }

    Ok(results)
}
