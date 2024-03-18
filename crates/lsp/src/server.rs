use anyhow::bail;
use anyhow::Context;

use marzano_gritmodule::parser::extract_relative_path;
use marzano_language::target_language::PatternLanguage;
use serde_json::Value;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};
use tracing::instrument;

use crate::actions::get_code_actions;
use crate::apply::{apply_named_pattern, apply_pattern_body};
use crate::check::{fix_file, get_check_info};
use crate::commands::GritHighlightKind;
use crate::commands::ShowGritHighlights;
use crate::commands::ShowGritHighlightsRequest;
use crate::commands::{
    GritPongNotification, GritPongNotificationParams, LspCommand, ShowPatternSelector,
    ShowPatternSelectorParams,
};
use crate::definition::get_identifier;
use crate::diagnostics::get_diagnostics;
use crate::documents::run_doc_manager;
use crate::executor::IntenseExecutor;
use crate::language::language_id_to_pattern_language;
use crate::manager::GritServerManager;
use crate::patterns::{get_grit_files_from_uri, resolve_from_uri};
use crate::search::search_query;
use crate::testing::maybe_test_pattern;
use crate::util::uri_to_file_path;
use crate::util::{
    convert_grit_position_to_lsp_position, convert_lsp_range_to_grit_range, trim_one_match,
};

#[cfg(feature = "project_diagnostics")]
use crate::scan::{collect_initial_diagnostics, get_diagnostics_for_uri, make_pseudo_document};
#[cfg(feature = "project_diagnostics")]
use crate::watcher::get_watched_files;
#[cfg(feature = "project_diagnostics")]
use dashmap::DashSet;

#[derive(Debug)]
struct GritServer {
    client: Client,
    manager: GritServerManager,
    executor: IntenseExecutor,
    #[cfg(feature = "project_diagnostics")]
    watched_files: DashSet<String>,
}

impl GritServer {
    async fn do_apply(
        &self,
        body: String,
        uri: String,
        range: String,
        language: String,
    ) -> anyhow::Result<()> {
        let body = match serde_json::from_str::<[String; 1]>(&format!("[\"{}\"]", &body)) {
            Ok(body) => body[0].to_string(),
            Err(e) => {
                bail!("Invalid body: {}", e);
            }
        };
        let document = match self.manager.must_get_document(&self.client, uri).await {
            Ok(doc) => doc,
            Err(_) => {
                return Ok(());
            }
        };
        let range = serde_json::from_str::<Range>(&range).with_context(|| {
            format!(
                "Unable to parse range {} for document {}",
                range, document.uri
            )
        })?;
        let range = convert_lsp_range_to_grit_range(&range, &document.text);
        let grit_files = get_grit_files_from_uri(document.uri.as_str(), true).await;
        let language = PatternLanguage::from_string(language.as_str(), None).unwrap_or_default();

        apply_pattern_body(
            &document,
            &body,
            grit_files,
            language,
            &self.client,
            None,
            Some(range),
        )
        .await;
        Ok(())
    }

    async fn compute_search(&self, query: String) -> anyhow::Result<()> {
        let documents = self.manager.must_get_documents(&self.client).await?;
        let (errors, results) = self
            .executor
            .spawn(move || search_query(documents, query))
            .await?
            .await;
        for error in errors {
            self.client
                .show_message(
                    MessageType::ERROR,
                    format!("Error during search: {}", error),
                )
                .await;
        }
        self.client
            .send_request::<ShowGritHighlights>(ShowGritHighlightsRequest {
                kind: GritHighlightKind::Search,
                results,
            })
            .await?;
        Ok(())
    }

    async fn compute_code_action(
        &self,
        params: CodeActionParams,
    ) -> anyhow::Result<Option<Vec<CodeActionOrCommand>>> {
        let document = match self
            .manager
            .maybe_get_document(&self.client, params.text_document.uri.into())
            .await
        {
            Some(doc) => doc,
            None => return Ok(None),
        };

        let check_info = match get_check_info(&document).await {
            Ok(Some(info)) => info,
            Ok(None) => return Ok(None),
            Err(e) => {
                self.client
                    .show_message(MessageType::ERROR, format!("Error: {}", e))
                    .await;
                return Ok(None);
            }
        };

        let actions = self
            .executor
            .spawn(move || get_code_actions(document, check_info, params.range))
            .await??;

        Ok(Some(actions))
    }

    async fn on_change(&self, params: &TextDocumentItem) -> anyhow::Result<()> {
        let check_info = match get_check_info(params).await? {
            Some(info) => info,
            None => return Ok(()),
        };

        // TODO: remove these clones
        let check_clone = check_info.clone();
        let doc_clone = params.clone();
        let Some(our_repo) = self.manager.get_root_module().await else {
            self.client
                .log_message(
                    MessageType::INFO,
                    "No repo found when checking document".to_string(),
                )
                .await;
            return Ok(());
        };
        let Some(our_path) = self.manager.get_root_path() else {
            self.client
                .log_message(
                    MessageType::INFO,
                    "No path found when checking document".to_string(),
                )
                .await;
            return Ok(());
        };

        let diagnostics = self
            .executor
            .spawn(move || {
                get_diagnostics(
                    doc_clone,
                    check_clone,
                    &our_repo,
                    &our_path,
                    #[cfg(feature = "caching")]
                    &None,
                )
            })
            .await??;

        self.client
            .log_message(
                MessageType::INFO,
                format!("Found {} diagnostics for {}", diagnostics.len(), params.uri),
            )
            .await;

        self.client
            .publish_diagnostics(params.uri.to_owned(), diagnostics, None)
            .await;

        match maybe_test_pattern(&self.client, &self.manager, params).await {
            Ok(_) => {}
            Err(e) => {
                self.client
                    .show_message(MessageType::ERROR, format!("Test error: {}", e))
                    .await;
            }
        };

        Ok(())
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for GritServer {
    #[instrument(skip(self, params))]
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        if let Some(root_uri) = params.root_uri {
            self.manager.set_root_uri(root_uri.clone());
            #[cfg(feature = "project_diagnostics")]
            {
                let watched = get_watched_files(root_uri);
                for file in watched {
                    self.watched_files.insert(file);
                }
            }
        }
        self.manager.load_client_configuration(&params.capabilities);
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                code_action_provider: Some(CodeActionProviderCapability::Simple(true)),
                definition_provider: Some(OneOf::Left(true)),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                execute_command_provider: Some(ExecuteCommandOptions {
                    commands: vec![
                        LspCommand::ShowDebug.to_string(),
                        LspCommand::OpenPatternSelector.to_string(),
                        LspCommand::ApplyNamedPattern.to_string(),
                        LspCommand::ApplyPattern.to_string(),
                        LspCommand::ApplyResult.to_string(),
                        LspCommand::FixFile.to_string(),
                        LspCommand::PingGrit.to_string(),
                    ],
                    ..Default::default()
                }),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                ..ServerCapabilities::default()
            },
            server_info: Some(ServerInfo {
                name: "grit-integrated-language-server".to_string(),
                version: Some("0.1.0".to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized")
            .await;

        #[cfg(feature = "project_diagnostics")]
        {
            let do_collect = self
                .manager
                .check_client_configuration(&self.client, "grit.project_diagnostics".to_string())
                .await;
            if do_collect {
                let workspace_root = self
                    .manager
                    .get_root_uri()
                    .map(|uri| uri_to_file_path(uri.as_ref()).unwrap());
                collect_initial_diagnostics(&self.client, &self.watched_files, workspace_root)
                    .await;
            }
        }
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    #[instrument(skip(self, params), fields(path = params.text_document.uri.to_string()))]
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        match self.on_change(&params.text_document).await {
            Ok(_) => {}
            Err(e) => {
                self.client
                    .show_message(MessageType::ERROR, format!("Error: {}", e))
                    .await;
            }
        };
        let _ = self
            .manager
            .must_set_document(&self.client, params.text_document)
            .await;
    }

    #[instrument(skip(self, params), fields(extension, language, path=params.text_document.uri.to_string()))]
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let DidChangeTextDocumentParams {
            text_document,
            mut content_changes,
        } = params;
        let uri = text_document.uri;
        let new_content = match content_changes.pop() {
            // Doing this is safe, since we specify `TextDocumentSyncKind::FULL` in `initialize`.
            Some(change) => change.text,
            None => {
                self.client
                    .log_message(MessageType::ERROR, "no content chang found")
                    .await;
                return;
            }
        };

        let Some(document) = self
            .manager
            .maybe_upsert_document(&self.client, uri.into(), new_content)
            .await
        else {
            return;
        };
        #[cfg(feature = "grit_tracing")]
        {
            tracing::Span::current().record("extension", &extension);
            tracing::Span::current().record("language", language.to_string());
        }

        match self.on_change(&document).await {
            Ok(_) => {}
            Err(e) => {
                self.client
                    .show_message(MessageType::ERROR, format!("Error: {}", e))
                    .await;
            }
        };
    }

    #[cfg(feature = "project_diagnostics")]
    async fn did_change_watched_files(&self, params: DidChangeWatchedFilesParams) {
        for change in params.changes {
            let uri_string: String = change.uri.to_string();
            let is_watched = match uri_to_file_path(&uri_string) {
                Ok(path) => {
                    let path_string = path.to_string_lossy().to_string();
                    self.watched_files.contains(&path_string)
                }
                Err(_) => false,
            };
            if !is_watched {
                continue;
            }
            if change.typ == FileChangeType::DELETED {
                let _ = self.manager.drop_document(uri_string).await;
                return;
            }

            let pseudo_document = match make_pseudo_document(&change.uri, &self.client).await {
                Some(document) => document,
                None => {
                    return;
                }
            };
            let check_info = match get_check_info(&pseudo_document).await {
                Some(info) => info,
                None => {
                    return;
                }
            };
            let diagnostics =
                get_diagnostics_for_uri(&self.client, &change.uri, false, &check_info, &None).await;
            if let Some(diagnostics) = diagnostics {
                self.client
                    .publish_diagnostics(change.uri.to_owned(), diagnostics, None)
                    .await;
            }
        }
    }

    #[instrument(skip(self, params), fields(
        uri,
        position,
        line = params.text_document_position_params.position.line
        ))]
    async fn goto_definition(
        &self,
        params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        let document = match self
            .manager
            .must_get_document(
                &self.client,
                params
                    .text_document_position_params
                    .text_document
                    .uri
                    .to_string(),
            )
            .await
        {
            Ok(doc) => doc,
            Err(_e) => {
                return Ok(None);
            }
        };
        match document.language_id.as_str() {
            "markdown" | "grit" | "yaml" => {}
            _ => {
                return Ok(None);
            }
        }

        let position = params.text_document_position_params.position;
        let identifier = get_identifier(&document, &position);

        #[cfg(feature = "grit_tracing")]
        {
            tracing::Span::current().record("uri", &uri);
            tracing::Span::current().record("position", format!("{position:?}"));
        }

        let resolved = resolve_from_uri(document.uri.as_ref(), None, true).await;
        let Some(root_path) = self.manager.get_root_path() else {
            return Ok(None);
        };
        let our_path = match uri_to_file_path(document.uri.as_ref()) {
            Ok(path) => {
                let maybe_path = root_path.to_string_lossy().to_string();
                let maybe_path = if maybe_path.starts_with("/var") {
                    None
                } else {
                    Some(maybe_path)
                };
                extract_relative_path(&path.to_string_lossy(), &maybe_path)
            }
            Err(_) => {
                return Ok(None);
            }
        };
        let our_language = resolved
            .iter()
            .find(|p| p.config.path == our_path)
            .map(|p| &p.language);
        let definition = match resolved.iter().find(|p| {
            p.local_name == identifier
                && (our_language.is_none()
                    || our_language.is_some_and(|l| {
                        l.language_name() == p.language.language_name()
                            || matches!(p.language, PatternLanguage::Universal)
                    }))
        }) {
            Some(definition) => definition,
            None => {
                return Ok(None);
            }
        };
        let Some(our_repo) = self.manager.get_root_module().await else {
            return Ok(None);
        };
        let url = Url::parse(definition.url(&our_repo, &root_path).as_str()).unwrap();
        let definition_position = match &definition.config.position {
            Some(position) => convert_grit_position_to_lsp_position(position),
            None => {
                return Ok(None);
            }
        };
        Ok(Some(GotoDefinitionResponse::Scalar(Location {
            uri: url,
            range: Range {
                start: definition_position,
                end: definition_position,
            },
        })))
    }

    #[instrument(skip(self, params), fields(path = params.text_document.uri.to_string()))]
    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri_string = params.text_document.uri.to_string();
        let _ = self.manager.drop_document(uri_string).await;
    }

    #[instrument(skip(self), fields(command=params.command))]
    async fn execute_command(&self, params: ExecuteCommandParams) -> Result<Option<Value>> {
        let ExecuteCommandParams {
            command, arguments, ..
        } = params;
        let mut args = arguments
            .iter()
            .map(|a| trim_one_match(&a.to_string(), '"').to_string())
            .collect::<Vec<_>>();
        if let Some(cmd) = LspCommand::maybe_from_str(&command) {
            match cmd {
                LspCommand::PingGrit => {
                    self.client
                        .send_notification::<GritPongNotification>(GritPongNotificationParams {})
                        .await;
                }
                LspCommand::ShowDebug => {
                    let server_info = "grit-integrated-language-server";
                    let flags = if cfg!(feature = "project_diagnostics") {
                        "project_diagnostics"
                    } else {
                        "none"
                    };
                    let message =
                        format!("Grit language server: {}, flags: {}", server_info, flags);
                    self.client.show_message(MessageType::INFO, message).await;
                }
                LspCommand::OpenPatternSelector => {
                    let Some(uri) = args.pop() else {
                        return Ok(None);
                    };
                    let document = match self.manager.must_get_document(&self.client, uri).await {
                        Ok(doc) => doc,
                        Err(_) => {
                            return Ok(None);
                        }
                    };
                    let Some(language) = language_id_to_pattern_language(&document.language_id)
                    else {
                        let message =
                            format!("Unable to find language for document {}", &document.uri);
                        self.client.show_message(MessageType::ERROR, message).await;
                        return Ok(None);
                    };
                    let resolved =
                        resolve_from_uri(document.uri.as_str(), Some(language), true).await;
                    let params: ShowPatternSelectorParams = ShowPatternSelectorParams {
                        file: document.uri.to_string(),
                        patterns: resolved,
                    };
                    let _ = self
                        .client
                        .send_request::<ShowPatternSelector>(params)
                        .await?;
                }
                LspCommand::ApplyNamedPattern => {
                    let (Some(pattern), Some(uri)) = (args.pop(), args.pop()) else {
                        return Ok(None);
                    };
                    let document = match self.manager.must_get_document(&self.client, uri).await {
                        Ok(doc) => doc,
                        Err(_) => {
                            return Ok(None);
                        }
                    };

                    apply_named_pattern(&document, &pattern, &self.client).await;
                }
                LspCommand::ApplyPattern => {
                    let (Some(body), Some(uri)) = (args.pop(), args.pop()) else {
                        return Ok(None);
                    };
                    let grit_files = get_grit_files_from_uri(&uri, true).await;
                    let lang = PatternLanguage::get_language(&body).unwrap_or_default();
                    let document = match self.manager.must_get_document(&self.client, uri).await {
                        Ok(doc) => doc,
                        Err(_) => {
                            return Ok(None);
                        }
                    };
                    self.client
                        .show_message(MessageType::INFO, format!("Applying pattern {}", body))
                        .await;
                    apply_pattern_body(
                        &document,
                        &body,
                        grit_files,
                        lang,
                        &self.client,
                        None,
                        None,
                    )
                    .await;
                }
                LspCommand::ApplyResult => {
                    let (Some(range), Some(language), Some(body), Some(uri)) =
                        (args.pop(), args.pop(), args.pop(), args.pop())
                    else {
                        return Ok(None);
                    };
                    match self.do_apply(body, uri, range, language).await {
                        Ok(_) => {}
                        Err(e) => {
                            self.client
                                .show_message(MessageType::ERROR, format!("Error: {}", e))
                                .await;
                        }
                    };
                }
                LspCommand::FixFile => {
                    let Some(uri) = args.pop() else {
                        return Ok(None);
                    };
                    let document = match self.manager.must_get_document(&self.client, uri).await {
                        Ok(doc) => doc,
                        Err(_) => {
                            return Ok(None);
                        }
                    };
                    match fix_file(&document, &self.client).await {
                        Ok(did_edit) => {
                            if !did_edit {
                                self.client
                                    .show_message(MessageType::INFO, "No changes found for file.")
                                    .await;
                            }
                        }
                        Err(e) => {
                            self.client
                                .show_message(MessageType::ERROR, format!("Error: {}", e))
                                .await;
                        }
                    };
                }
                LspCommand::SearchGritQL => {
                    let Some(query) = args.pop() else {
                        return Ok(None);
                    };
                    match self.compute_search(query).await {
                        Ok(_) => {}
                        Err(e) => {
                            self.client
                                .show_message(MessageType::ERROR, format!("Error: {}", e))
                                .await;
                        }
                    };
                }
            }
        }
        Ok(None)
    }

    async fn code_action(
        &self,
        params: CodeActionParams,
    ) -> Result<Option<Vec<CodeActionOrCommand>>> {
        match self.compute_code_action(params).await {
            Ok(action) => Ok(action),
            Err(e) => {
                self.client
                    .show_message(MessageType::ERROR, format!("Error: {}", e))
                    .await;
                Ok(None)
            }
        }
    }
}

pub async fn start_language_server() -> anyhow::Result<()> {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (docs, manager) = run_doc_manager();

    let (service, socket) = tower_lsp::LspService::new(|client| GritServer {
        client,
        manager: GritServerManager::new(docs),
        executor: IntenseExecutor::new(),
        #[cfg(feature = "project_diagnostics")]
        watched_files: DashSet::new(),
    });
    let server = tower_lsp::Server::new(stdin, stdout, socket).serve(service);

    // Await manager and server
    let (r, _) = tokio::join!(manager, server);

    r?;
    Ok(())
}

#[cfg(test)]
mod test {
    use serde_json::json;
    use tower::util::ServiceExt;
    use tower::Service;
    use tower_lsp::jsonrpc::Request;

    use super::*;

    #[tokio::test(flavor = "current_thread")]
    async fn test_lsp_simple() {
        let temp_root_dir = tempfile::tempdir().unwrap();
        let root_dir = temp_root_dir.path().to_string_lossy().to_string();

        let (docs, _) = run_doc_manager();

        let (mut service, _socket) = tower_lsp::LspService::new(|client| GritServer {
            client,
            manager: GritServerManager::new(docs),
            executor: IntenseExecutor::new(),
            #[cfg(feature = "project_diagnostics")]
            watched_files: DashSet::new(),
        });

        let req_init = Request::build("initialize")
            .params(json!({"capabilities":{}, "rootUri": format!("file://{}", root_dir)}))
            .id(1)
            .finish();
        let _ = service.ready().await.unwrap().call(req_init).await;

        let req_initialized = Request::build("initialized").params(json!({})).finish();
        let _ = service.call(req_initialized).await;
    }
}
