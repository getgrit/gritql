use std::path::PathBuf;

use anyhow::anyhow;
use anyhow::bail;
use anyhow::Result;
use dashmap::DashMap;
use marzano_gritmodule::fetcher::ModuleRepo;
use tokio::sync::oneshot;
use tower_lsp::{
    lsp_types::{ClientCapabilities, ConfigurationItem, MessageType, TextDocumentItem, Url},
    Client,
};

use crate::documents::{DocumentAction, DocumentCommander, DocumentKey};
use crate::util::uri_to_file_path;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SettingsKey {
    ClientConfiguration,
    RootUri,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum SettingsValue {
    ClientConfiguration(bool),
    RootUri(Option<Url>),
}

#[derive(Debug)]
pub struct GritServerManager {
    settings: DashMap<SettingsKey, SettingsValue>,
    documents: DocumentCommander,
}

impl GritServerManager {
    pub fn new(documents: DocumentCommander) -> Self {
        Self {
            settings: DashMap::new(),
            documents,
        }
    }

    pub async fn drop_document(&self, uri: DocumentKey) -> Result<()> {
        let cmd = DocumentAction::Drop { uri };
        if self.documents.send(cmd).await.is_err() {
            bail!("Could not send document action to document manager");
        };
        Ok(())
    }

    pub async fn must_set_document(&self, client: &Client, document: TextDocumentItem) {
        let cmd = DocumentAction::Upsert { document };
        if self.documents.send(cmd).await.is_err() {
            client
                .show_message(MessageType::ERROR, "error setting document")
                .await;
        };
    }

    async fn get_document(&self, uri: DocumentKey) -> Result<Option<TextDocumentItem>> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = DocumentAction::Get { uri, resp: resp_tx };

        if self.documents.send(cmd).await.is_err() {
            bail!("Could not send document action to document manager");
        };

        let res = resp_rx.await?;
        Ok(res)
    }

    /// Fetch a document from the document manager if it exists
    /// This handles sending the error to the client as well
    pub async fn maybe_upsert_document(
        &self,
        client: &Client,
        uri: DocumentKey,
        content: String,
    ) -> Option<TextDocumentItem> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = DocumentAction::UpsertContent {
            uri,
            content,
            resp: resp_tx,
        };

        match self.documents.send(cmd).await {
            Ok(_) => {}
            Err(e) => {
                client
                    .log_message(
                        MessageType::ERROR,
                        format!("error upserting document: {}", e),
                    )
                    .await;
                return None;
            }
        };

        match resp_rx.await {
            Ok(res) => res,
            Err(e) => {
                client
                    .log_message(
                        MessageType::ERROR,
                        format!("error upserting document: {}", e),
                    )
                    .await;
                None
            }
        }
    }

    /// Fetch a document from the document manager if it exists
    /// This handles sending the error to the client as well
    pub async fn maybe_get_document(
        &self,
        client: &Client,
        uri: DocumentKey,
    ) -> Option<TextDocumentItem> {
        match self.get_document(uri.clone()).await {
            Ok(doc) => doc,
            Err(_e) => {
                client
                    .show_message(MessageType::ERROR, "error getting document")
                    .await;
                None
            }
        }
    }

    /// Fetch a document from the document manager, or error if it doesn't exist
    /// This handles sending the error to the client as well
    pub async fn must_get_document(
        &self,
        client: &Client,
        uri: DocumentKey,
    ) -> Result<TextDocumentItem> {
        let doc = match self.get_document(uri.clone()).await {
            Ok(doc) => doc,
            Err(_e) => {
                client
                    .show_message(MessageType::ERROR, "error getting document")
                    .await;
                return Err(anyhow!("Could not find document for uri {}", uri));
            }
        };
        match doc {
            Some(doc) => Ok(doc),
            None => {
                client
                    .show_message(MessageType::ERROR, "document not found")
                    .await;
                Err(anyhow!("Could not find document for uri {}", uri))
            }
        }
    }

    /// Fetch all documents from the document manager
    pub async fn must_get_documents(&self, client: &Client) -> Result<Vec<TextDocumentItem>> {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = DocumentAction::GetAll { resp: resp_tx };

        match self.documents.send(cmd).await {
            Ok(_) => {}
            Err(e) => {
                client
                    .log_message(
                        MessageType::ERROR,
                        format!("error getting documents: {}", e),
                    )
                    .await;
                bail!("Could not send document action to document manager");
            }
        };

        match resp_rx.await {
            Ok(res) => Ok(res),
            Err(e) => {
                client
                    .log_message(
                        MessageType::ERROR,
                        format!("error getting documents: {}", e),
                    )
                    .await;
                bail!("Could not get documents from document manager");
            }
        }
    }

    fn check_setting(&self, key: SettingsKey) -> SettingsValue {
        let val = self.settings.get(&key);
        match val {
            Some(val) => {
                let val = val.value();
                val.clone()
            }
            None => match key {
                SettingsKey::ClientConfiguration => SettingsValue::ClientConfiguration(false),
                SettingsKey::RootUri => SettingsValue::RootUri(None),
            },
        }
    }

    pub async fn check_client_configuration(&self, client: &Client, section: String) -> bool {
        let can_check = self.check_setting(SettingsKey::ClientConfiguration);
        if let SettingsValue::ClientConfiguration(false) = can_check {
            return false;
        }

        match client
            .configuration(vec![ConfigurationItem {
                scope_uri: None,
                section: Some(section.clone()),
            }])
            .await
        {
            Ok(config) => {
                let value = config.first();
                match value {
                    Some(value) => value.as_bool().unwrap_or(false),
                    None => {
                        client
                            .log_message(
                                MessageType::WARNING,
                                format!("Grit could not find configured {:?}", section),
                            )
                            .await;
                        false
                    }
                }
            }
            e => {
                client
                    .log_message(
                        MessageType::WARNING,
                        format!("Grit could not find configured {:?}", e),
                    )
                    .await;
                false
            }
        }
    }

    pub fn get_root_uri(&self) -> Option<Url> {
        let root_uri = self.check_setting(SettingsKey::RootUri);
        if let SettingsValue::RootUri(Some(uri)) = root_uri {
            Some(uri)
        } else {
            None
        }
    }

    pub fn get_root_path(&self) -> Option<PathBuf> {
        let root_uri = self.get_root_uri()?;
        match uri_to_file_path(root_uri.as_ref()) {
            Ok(path) => Some(path),
            Err(_) => None,
        }
    }

    pub async fn get_root_module(&self) -> Option<ModuleRepo> {
        let root_path = self.get_root_path();
        match root_path {
            Some(root_path) => {
                let root_module = ModuleRepo::from_dir(&root_path).await;
                Some(root_module)
            }
            None => None,
        }
    }

    pub fn set_root_uri(&self, uri: Url) {
        self.settings
            .insert(SettingsKey::RootUri, SettingsValue::RootUri(Some(uri)));
    }

    pub fn load_client_configuration(&self, client_capabilities: &ClientCapabilities) {
        if let Some(value) = client_capabilities
            .workspace
            .as_ref()
            .and_then(|w| w.configuration)
        {
            self.settings.insert(
                SettingsKey::ClientConfiguration,
                SettingsValue::ClientConfiguration(value),
            );
        }
    }
}
