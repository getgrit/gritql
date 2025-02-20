use crate::constants::MAX_FILE_SIZE;
use grit_util::{AnalysisLog, Position};
use marzano_util::rich_path::RichFile;
use std::env;

pub(crate) fn is_file_too_big(file: &RichFile) -> Option<AnalysisLog> {
    let max_size = env::var("GRIT_MAX_FILE_SIZE_BYTES")
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(MAX_FILE_SIZE);

    // Skip the check if max_size is 0
    if max_size == 0 {
        return None;
    }

    if file.path.len() > max_size || file.content.len() > max_size {
        Some(AnalysisLog {
            // TODO: standardize levels
            level: Some(310),
            message: format!("Skipped {}, it is too big.", file.path),
            file: Some(file.path.to_owned().into()),
            engine_id: Some("marzano".to_owned()),
            position: Some(Position::first()),
            syntax_tree: None,
            range: None,
            source: None,
        })
    } else {
        None
    }
}
