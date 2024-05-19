use ignore::Walk;
use log::warn;
use std::path::PathBuf;

use crate::{hasher::hash, rich_path::RichPath};

pub fn get_files(walker: Walk) -> Vec<PathBuf> {
    walker
        .filter_map(|entry| {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => {
                    return None;
                }
            };
            if entry.file_type().is_some_and(|ft| ft.is_dir()) {
                return None;
            }
            let path = entry.path().to_owned();
            Some(path)
        })
        .collect()
}

pub fn get_canonical_paths(walker: Walk) -> Result<Vec<PathBuf>, std::io::Error> {
    let paths = get_files(walker);
    paths.iter().map(|path| path.canonicalize()).collect()
}

pub fn get_input_files(files: &[PathBuf]) -> Vec<RichPath> {
    files
        .iter()
        .filter_map(|p| match fs_err::read_to_string(p) {
            Ok(content) => {
                let hash = hash(&content);
                Some(RichPath::new(p.to_owned(), Some(hash)))
            }
            Err(_) => {
                warn!("Failed to read file {:?}", p);
                None
            }
        })
        .collect()
}
