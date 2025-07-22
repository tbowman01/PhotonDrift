//! Core LSP server implementation for PhotonDrift
//! 
//! Provides the main Language Server Protocol server that handles client connections,
//! initialization, and orchestrates all LSP functionality.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::config::Config;
use crate::drift::detector::DriftDetector;
use super::diagnostics::DriftDiagnosticsEngine;
use super::handlers::*;
use super::protocol::LspProtocolHelper;

/// PhotonDrift Language Server implementation
pub struct PhotonDriftLspServer {
    /// LSP client handle
    client: Client,
    /// Server configuration
    config: Arc<RwLock<Option<Config>>>,
    /// Drift detection engine
    drift_detector: Arc<RwLock<Option<DriftDetector>>>,
    /// Diagnostics engine for real-time analysis
    diagnostics_engine: Arc<DriftDiagnosticsEngine>,
    /// Protocol helper utilities
    protocol_helper: Arc<LspProtocolHelper>,
    /// Document store for opened files
    document_map: Arc<RwLock<HashMap<Url, String>>>,
    /// Workspace folders
    workspace_folders: Arc<RwLock<Vec<WorkspaceFolder>>>,
}

impl PhotonDriftLspServer {
    /// Create a new PhotonDrift LSP server instance
    pub fn new(client: Client) -> Self {
        let config = Arc::new(RwLock::new(None));
        let diagnostics_engine = Arc::new(DriftDiagnosticsEngine::new(config.clone()));
        
        Self {
            client,
            config,
            drift_detector: Arc::new(RwLock::new(None)),
            diagnostics_engine,
            protocol_helper: Arc::new(LspProtocolHelper::new()),
            document_map: Arc::new(RwLock::new(HashMap::new())),
            workspace_folders: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// Initialize drift detector with workspace configuration
    async fn initialize_drift_detector(&self, workspace_root: &std::path::Path) -> crate::Result<()> {
        let config = Config::from_workspace_root(workspace_root)?;
        
        // Store configuration
        *self.config.write().await = Some(config.clone());
        
        // Initialize drift detector
        let detector = DriftDetector::new(config.clone())?;
        *self.drift_detector.write().await = Some(detector);
        
        Ok(())
    }

    /// Update diagnostics for a document
    async fn update_diagnostics(&self, uri: &Url, text: &str) {
        if let Some(diagnostics) = self.diagnostics_engine.analyze_document(uri, text).await {
            self.client.publish_diagnostics(uri.clone(), diagnostics, None).await;
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for PhotonDriftLspServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        // Store workspace folders
        if let Some(workspace_folders) = params.workspace_folders {
            *self.workspace_folders.write().await = workspace_folders;
        }

        // Initialize drift detector if workspace root is available
        if let Some(root_uri) = params.root_uri {
            if let Ok(root_path) = root_uri.to_file_path() {
                if let Err(e) = self.initialize_drift_detector(&root_path).await {
                    eprintln!("Failed to initialize drift detector: {}", e);
                }
            }
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![":".to_string(), "#".to_string(), "-".to_string()]),
                    work_done_progress_options: Default::default(),
                    all_commit_characters: None,
                    completion_item: None,
                }),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(DiagnosticOptions {
                    identifier: Some("photon-drift".to_string()),
                    inter_file_dependencies: true,
                    workspace_diagnostics: false,
                    work_done_progress_options: Default::default(),
                })),
                workspace: Some(WorkspaceServerCapabilities {
                    workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                        supported: Some(true),
                        change_notifications: Some(OneOf::Left(true)),
                    }),
                    file_operations: None,
                }),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "photon-drift-lsp".to_string(),
                version: Some(env!("CARGO_PKG_VERSION").to_string()),
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "PhotonDrift LSP server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        // Store document content
        self.document_map.write().await.insert(uri.clone(), text.clone());
        
        // Update diagnostics for the opened document
        self.update_diagnostics(&uri, &text).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        
        if let Some(change) = params.content_changes.into_iter().next() {
            // Update document content
            self.document_map.write().await.insert(uri.clone(), change.text.clone());
            
            // Update diagnostics for the changed document
            self.update_diagnostics(&uri, &change.text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        // Remove document from store
        self.document_map.write().await.remove(&params.text_document.uri);
        
        // Clear diagnostics for the closed document
        self.client
            .publish_diagnostics(params.text_document.uri, Vec::new(), None)
            .await;
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let uri = &params.text_document_position.text_document.uri;
        let position = params.text_document_position.position;

        if let Some(text) = self.document_map.read().await.get(uri) {
            let config = self.config.read().await;
            if let Some(ref config) = *config {
                return Ok(super::completion::provide_completions(text, position, config).await);
            }
        }

        Ok(None)
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        if let Some(text) = self.document_map.read().await.get(uri) {
            let config = self.config.read().await;
            if let Some(ref config) = *config {
                return Ok(super::hover::provide_hover_info(text, position, config).await);
            }
        }

        Ok(None)
    }
}