use std::collections::HashMap;

use tokio::sync::{mpsc, oneshot};
use tower_lsp::lsp_types::TextDocumentItem;

/// A document manager that handles all the documents in the workspace.
/// The document manager goes on its own thread, so we don't have to deal with locking
pub type DocumentCommander = mpsc::Sender<DocumentAction>;

pub type DocumentKey = String;
type Responder = oneshot::Sender<Option<TextDocumentItem>>;

#[derive(Debug)]
pub enum DocumentAction {
    /// Upsert a full document into the document manager
    Upsert { document: TextDocumentItem },
    /// Get a document from the document manager
    Get { uri: DocumentKey, resp: Responder },
    /// Drop a document from the document manager, such as when closing a file
    Drop { uri: DocumentKey },
    /// Upsert content by key and return the old content
    UpsertContent {
        uri: DocumentKey,
        content: String,
        resp: Responder,
    },
    /// Fetch a copy of all current documents
    GetAll {
        resp: oneshot::Sender<Vec<TextDocumentItem>>,
    },
}

pub fn run_doc_manager() -> (DocumentCommander, tokio::task::JoinHandle<()>) {
    let (tx, mut rx) = mpsc::channel(32);

    let manager = tokio::spawn(async move {
        let mut document_map = HashMap::new();

        while let Some(cmd) = rx.recv().await {
            match cmd {
                DocumentAction::Upsert { document } => {
                    let uri = document.uri.to_string();
                    document_map.insert(uri, document);
                }
                DocumentAction::Get { uri, resp } => {
                    let doc = document_map.get(&uri).cloned();
                    let _ = resp.send(doc);
                }
                DocumentAction::Drop { uri } => {
                    document_map.remove(&uri);
                }
                DocumentAction::UpsertContent { uri, content, resp } => {
                    let doc = document_map.get_mut(&uri);
                    match doc {
                        Some(doc) => {
                            doc.text = content;
                            let _ = resp.send(Some(doc.clone()));
                        }
                        None => {
                            let _ = resp.send(None);
                        }
                    }
                }
                DocumentAction::GetAll { resp } => {
                    let docs = document_map.values().cloned().collect();
                    let _ = resp.send(docs);
                }
            }
        }
    });

    (tx, manager)
}
