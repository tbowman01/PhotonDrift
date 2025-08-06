//! Core LSP server implementation for PhotonDrift

use std::collections::HashMap;
use std::sync::Arc;
use tokio::signal;
use tokio::sync::RwLock;

use lsp_types::{
    CompletionOptions, CompletionParams, CompletionResponse, DidChangeTextDocumentParams,
    DidCloseTextDocumentParams, DidOpenTextDocumentParams, Hover, HoverParams,
    HoverProviderCapability, InitializeParams, InitializeResult, MessageType, ServerCapabilities,
    TextDocumentSyncCapability, TextDocumentSyncKind, Url, WorkDoneProgressOptions,
};
use tower_lsp::jsonrpc::Result;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::lsp::{CompletionProvider, DiagnosticEngine, DocumentStore, HoverProvider, LspConfig};

/// PhotonDrift Language Server implementation
pub struct PhotonDriftLspServer {
    client: Client,
    documents: DocumentStore,
    config: Arc<RwLock<LspConfig>>,
    diagnostic_engine: DiagnosticEngine,
    completion_provider: CompletionProvider,
    hover_provider: HoverProvider,
}

impl PhotonDriftLspServer {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            documents: DocumentStore::default(),
            config: Arc::new(RwLock::new(LspConfig::default())),
            diagnostic_engine: DiagnosticEngine::new(),
            completion_provider: CompletionProvider::new(),
            hover_provider: HoverProvider::new(),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for PhotonDriftLspServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // Log initialization
        self.client
            .log_message(MessageType::INFO, "PhotonDrift LSP server initializing")
            .await;

        // Set workspace root if provided
        if let Some(workspace_folders) = params.workspace_folders {
            if let Some(folder) = workspace_folders.first() {
                if let Ok(path) = folder.uri.to_file_path() {
                    let mut config = self.config.write().await;
                    config.workspace_root = Some(path);
                }
            }
        }

        Ok(InitializeResult {
            server_info: Some(lsp_types::ServerInfo {
                name: "PhotonDrift LSP".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(false),
                    trigger_characters: Some(vec![
                        "#".to_string(),
                        "-".to_string(),
                        ":".to_string(),
                    ]),
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn initialized(&self, _: lsp_types::InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "PhotonDrift LSP server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        let content = params.text_document.text.clone();

        // Store document
        self.documents
            .write()
            .await
            .insert(uri.clone(), content.clone());

        // Run diagnostics if enabled
        let config = self.config.read().await;
        if config.diagnostics_enabled {
            drop(config);
            if let Ok(diagnostics) = self.diagnostic_engine.analyze_content(&content, &uri).await {
                self.client
                    .publish_diagnostics(uri, diagnostics, None)
                    .await;
            }
        }
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri.clone();

        if let Some(change) = params.content_changes.into_iter().next() {
            // Update document store
            self.documents
                .write()
                .await
                .insert(uri.clone(), change.text.clone());

            // Run diagnostics if enabled
            let config = self.config.read().await;
            if config.diagnostics_enabled {
                drop(config);
                if let Ok(diagnostics) = self
                    .diagnostic_engine
                    .analyze_content(&change.text, &uri)
                    .await
                {
                    self.client
                        .publish_diagnostics(uri, diagnostics, None)
                        .await;
                }
            }
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        // Remove from document store
        self.documents
            .write()
            .await
            .remove(&params.text_document.uri);

        // Clear diagnostics
        self.client
            .publish_diagnostics(params.text_document.uri, Vec::new(), None)
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let config = self.config.read().await;
        if !config.completion_enabled {
            return Ok(None);
        }
        drop(config);

        let uri = params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        // Get document content
        let documents = self.documents.read().await;
        if let Some(content) = documents.get(&uri) {
            let completions = self
                .completion_provider
                .get_completions(content, position)
                .await;
            Ok(Some(CompletionResponse::Array(completions)))
        } else {
            Ok(None)
        }
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let config = self.config.read().await;
        if !config.hover_enabled {
            return Ok(None);
        }
        drop(config);

        let uri = params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        // Get document content
        let documents = self.documents.read().await;
        if let Some(content) = documents.get(&uri) {
            Ok(self.hover_provider.get_hover_info(content, position).await)
        } else {
            Ok(None)
        }
    }
}

/// Start the LSP server with proper error handling and graceful shutdown
pub async fn start_lsp_server() -> crate::Result<()> {
    // Initialize logging
    eprintln!("Starting PhotonDrift LSP server...");

    // Setup async I/O
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    // Create LSP service
    let (service, socket) = LspService::new(PhotonDriftLspServer::new);
    let server = Server::new(stdin, stdout, socket);

    eprintln!("LSP server initialized, waiting for connections...");

    // Run server with graceful shutdown handling
    let server_task = tokio::spawn(async move {
        if let Err(e) = server.serve(service).await {
            eprintln!("LSP server error: {}", e);
            return Err(crate::AdrscanError::RealtimeError(format!(
                "LSP server failed: {}",
                e
            )));
        }
        Ok(())
    });

    // Handle shutdown signals
    let shutdown_task = tokio::spawn(async {
        #[cfg(unix)]
        {
            use tokio::signal::unix::{signal, SignalKind};
            let mut sigterm = signal(SignalKind::terminate()).map_err(|e| {
                crate::AdrscanError::RealtimeError(format!(
                    "Failed to setup SIGTERM handler: {}",
                    e
                ))
            })?;
            let mut sigint = signal(SignalKind::interrupt()).map_err(|e| {
                crate::AdrscanError::RealtimeError(format!("Failed to setup SIGINT handler: {}", e))
            })?;

            tokio::select! {
                _ = sigterm.recv() => {
                    eprintln!("Received SIGTERM, shutting down LSP server...");
                }
                _ = sigint.recv() => {
                    eprintln!("Received SIGINT, shutting down LSP server...");
                }
            }
        }

        #[cfg(not(unix))]
        {
            if let Err(e) = signal::ctrl_c().await {
                return Err(crate::AdrscanError::RealtimeError(format!(
                    "Failed to setup Ctrl+C handler: {}",
                    e
                )));
            }
            eprintln!("Received Ctrl+C, shutting down LSP server...");
        }

        Ok(())
    });

    // Wait for either server completion or shutdown signal
    tokio::select! {
        result = server_task => {
            match result {
                Ok(Ok(())) => {
                    eprintln!("LSP server completed successfully");
                    Ok(())
                }
                Ok(Err(e)) => {
                    eprintln!("LSP server failed: {}", e);
                    Err(e)
                }
                Err(e) => {
                    eprintln!("LSP server task panicked: {}", e);
                    Err(crate::AdrscanError::RealtimeError(format!("Server task failed: {}", e)))
                }
            }
        }
        result = shutdown_task => {
            match result {
                Ok(Ok(())) => {
                    eprintln!("LSP server shutdown gracefully");
                    Ok(())
                }
                Ok(Err(e)) => {
                    eprintln!("Shutdown handler failed: {}", e);
                    Err(e)
                }
                Err(e) => {
                    eprintln!("Shutdown task panicked: {}", e);
                    Err(crate::AdrscanError::RealtimeError(format!("Shutdown task failed: {}", e)))
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lsp_types::ClientCapabilities;
    use tower_lsp::jsonrpc::Id;

    // Mock client for testing
    struct MockClient;

    #[tower_lsp::async_trait]
    impl Client for MockClient {
        async fn log_message(&self, _typ: MessageType, _message: String) {}
        async fn show_message(&self, _typ: MessageType, _message: String) {}
        async fn publish_diagnostics(
            &self,
            _uri: Url,
            _diagnostics: Vec<lsp_types::Diagnostic>,
            _version: Option<i32>,
        ) {
        }
    }

    #[tokio::test]
    async fn test_server_initialization() {
        let client = MockClient;
        let server = PhotonDriftLspServer::new(client);

        let params = InitializeParams {
            process_id: None,
            root_path: None,
            root_uri: None,
            initialization_options: None,
            capabilities: ClientCapabilities::default(),
            trace: None,
            workspace_folders: None,
            client_info: None,
            locale: None,
        };

        let result = server.initialize(params).await.unwrap();
        assert_eq!(result.server_info.as_ref().unwrap().name, "PhotonDrift LSP");
        assert!(result.capabilities.text_document_sync.is_some());
        assert!(result.capabilities.completion_provider.is_some());
        assert!(result.capabilities.hover_provider.is_some());
    }

    #[tokio::test]
    async fn test_document_lifecycle() {
        let client = MockClient;
        let server = PhotonDriftLspServer::new(client);

        let uri = Url::parse("file:///test.md").unwrap();
        let content = "# ADR-001: Test Decision\n\n## Status\nProposed";

        // Test document open
        let open_params = DidOpenTextDocumentParams {
            text_document: lsp_types::TextDocumentItem {
                uri: uri.clone(),
                language_id: "markdown".to_string(),
                version: 1,
                text: content.to_string(),
            },
        };

        server.did_open(open_params).await;

        // Verify document is stored
        let documents = server.documents.read().await;
        assert_eq!(documents.get(&uri), Some(&content.to_string()));
        drop(documents);

        // Test document close
        let close_params = DidCloseTextDocumentParams {
            text_document: lsp_types::TextDocumentIdentifier { uri: uri.clone() },
        };

        server.did_close(close_params).await;

        // Verify document is removed
        let documents = server.documents.read().await;
        assert!(!documents.contains_key(&uri));
    }
}
