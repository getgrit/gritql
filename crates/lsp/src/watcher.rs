use marzano_language::target_language::expand_paths;
use marzano_language::target_language::PatternLanguage;
use tower_lsp::lsp_types::Url;

pub fn get_watched_files(root_uri: Url) -> Vec<String> {
    let root = match root_uri.to_file_path() {
        Ok(path) => path,
        Err(_) => {
            return vec![];
        }
    };
    let paths = vec![root];
    let languages = PatternLanguage::enumerate();
    let walker = match expand_paths(&paths, Some(&languages)) {
        Ok(walker) => walker,
        Err(_) => {
            return vec![];
        }
    };
    let mut files = Vec::new();
    for entry in walker {
        let entry = match entry {
            Ok(entry) => entry,
            Err(_) => {
                continue;
            }
        };
        if entry.file_type().is_some_and(|ft| ft.is_dir()) {
            continue;
        }
        let path = entry.path().to_owned();
        files.push(path.to_string_lossy().to_string());
    }
    files
}
