use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

use dashmap::{DashMap, DashSet};

use grit_cache::cache::{cache_dir, Cache};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use tokio::{runtime::Runtime, task::spawn_blocking};
use tower_lsp::{
    lsp_types::{Diagnostic, MessageType, TextDocumentItem, Url},
    Client,
};

use crate::diagnostics::get_diagnostics;
use crate::{check::get_check_info, language::extension_to_language_id};
use crate::{check::CheckInfo, util::uri_to_file_path};

pub async fn get_diagnostics_for_uri(
    client: &Client,
    uri: &Url,
    store: bool,
    check_info: &CheckInfo,
    cache: &Option<Arc<RwLock<Cache>>>,
) -> Option<Vec<Diagnostic>> {
    let pseudo_document = match make_pseudo_document(uri, client).await {
        Some(document) => document,
        None => {
            return None;
        }
    };
    let diagnostics = get_diagnostics(client, &pseudo_document, store, check_info, cache).await;
    Some(diagnostics)
}

pub async fn collect_initial_diagnostics(
    client: &Client,
    watched_files: &DashSet<String>,
    workspace_root: Option<PathBuf>,
) {
    let grouped_by_extension: DashMap<String, Vec<String>> = DashMap::new();
    for file in watched_files.iter() {
        let ext = Path::new(file.key())
            .extension()
            .map(|os_str| os_str.to_string_lossy().to_string());
        let ext = match ext {
            Some(ext) => ext,
            None => {
                continue;
            }
        };
        grouped_by_extension
            .entry(ext)
            .or_default()
            .push(file.to_string());
    }

    let cache = match workspace_root {
        Some(workspace_root) => match cache_dir(workspace_root).await {
            Ok(cache_dir) => {
                let cache = Cache::new(cache_dir);
                match cache {
                    Ok(cache) => Some(Arc::new(RwLock::new(cache))),
                    Err(_) => {
                        client
                            .log_message(
                                MessageType::ERROR,
                                "Could not create cache directory".to_string(),
                            )
                            .await;
                        None
                    }
                }
            }
            Err(_) => None,
        },
        None => None,
    };

    for (_, watched) in grouped_by_extension {
        let first_uri = match watched.first() {
            Some(f) => Url::parse(&format!("file://{}", f)).unwrap(),
            None => {
                continue;
            }
        };

        let pseudo_document = match make_pseudo_document(&first_uri, client).await {
            Some(document) => document,
            None => {
                continue;
            }
        };

        let check_info = if let Ok(Some(info)) = get_check_info(&pseudo_document).await {
            info
        } else {
            continue;
        };

        let client_clone = client.clone();
        let cache_clone = cache.clone();
        spawn_blocking(move || {
            watched.par_iter().for_each(|f| {
                let url = Url::parse(&format!("file://{}", f)).unwrap();
                Runtime::new().unwrap().block_on(async {
                    let diagnostics = get_diagnostics_for_uri(
                        &client_clone,
                        &url,
                        false,
                        &check_info,
                        &cache_clone,
                    )
                    .await;
                    if let Some(diagnostics) = diagnostics {
                        client_clone
                            .publish_diagnostics(url, diagnostics, None)
                            .await;
                    }
                });
            });
            if let Some(cache) = cache_clone {
                let mut cache_lock = cache.write().unwrap();
                let _ = cache_lock.write();
            }
        });
    }
}

pub async fn make_pseudo_document(uri: &Url, client: &Client) -> Option<TextDocumentItem> {
    let uri_string = uri.to_string();
    let file_path = match uri_to_file_path(&uri_string) {
        Ok(path) => path,
        Err(e) => {
            client
                .log_message(
                    MessageType::ERROR,
                    format!("Could not get file path for uri {}: {:?}", uri_string, e),
                )
                .await;
            return None;
        }
    };
    let extension = file_path.extension().unwrap().to_string_lossy();
    let language_id = match extension_to_language_id(&extension) {
        Some(id) => id,
        None => {
            return None;
        }
    };
    let pseudo_document = TextDocumentItem::new(
        uri.clone(),
        language_id,
        0,
        std::fs::read_to_string(file_path).unwrap(),
    );
    Some(pseudo_document)
}
