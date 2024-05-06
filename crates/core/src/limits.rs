use grit_util::{AnalysisLog, Position};
use marzano_util::rich_path::RichFile;

use crate::constants::MAX_FILE_SIZE;

pub(crate) fn is_file_too_big(file: &RichFile) -> Option<AnalysisLog> {
    if file.path.len() > MAX_FILE_SIZE || file.content.len() > MAX_FILE_SIZE {
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
