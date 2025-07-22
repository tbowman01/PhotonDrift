//! LSP request and notification handlers for PhotonDrift
//! 
//! This module contains specialized handlers for different LSP requests
//! and notifications, providing focused functionality for each operation.

use tower_lsp::lsp_types::*;
use crate::config::Config;

/// Handle LSP initialization parameters and setup
pub struct InitializationHandler;

impl InitializationHandler {
    /// Process initialization parameters and prepare server capabilities
    pub fn process_initialization(params: &InitializeParams) -> ServerCapabilities {
        let mut capabilities = ServerCapabilities::default();
        
        // Configure text document synchronization
        capabilities.text_document_sync = Some(TextDocumentSyncCapability::Kind(
            TextDocumentSyncKind::FULL,
        ));

        // Enable completion with ADR-specific triggers
        capabilities.completion_provider = Some(CompletionOptions {
            resolve_provider: Some(true),
            trigger_characters: Some(vec![
                ":".to_string(),  // For ADR field completion
                "#".to_string(),  // For section headers
                "-".to_string(),  // For list items
                "@".to_string(),  // For references
            ]),
            work_done_progress_options: Default::default(),
            all_commit_characters: None,
            completion_item: None,
        });

        // Enable hover for contextual information
        capabilities.hover_provider = Some(HoverProviderCapability::Simple(true));

        // Enable diagnostics for drift detection
        capabilities.diagnostic_provider = Some(DiagnosticServerCapabilities::Options(
            DiagnosticOptions {
                identifier: Some("photon-drift".to_string()),
                inter_file_dependencies: true,
                workspace_diagnostics: true,
                work_done_progress_options: Default::default(),
            }
        ));

        // Configure workspace capabilities
        capabilities.workspace = Some(WorkspaceServerCapabilities {
            workspace_folders: Some(WorkspaceFoldersServerCapabilities {
                supported: Some(true),
                change_notifications: Some(OneOf::Left(true)),
            }),
            file_operations: Some(WorkspaceFileOperationsServerCapabilities {
                did_create: Some(FileOperationRegistrationOptions {
                    filters: vec![FileOperationFilter {
                        scheme: Some("file".to_string()),
                        pattern: FileOperationPattern {
                            glob: "**/*.md".to_string(),
                            matches: Some(FileOperationPatternKind::File),
                            options: None,
                        },
                    }],
                }),
                will_create: None,
                did_rename: Some(FileOperationRegistrationOptions {
                    filters: vec![FileOperationFilter {
                        scheme: Some("file".to_string()),
                        pattern: FileOperationPattern {
                            glob: "**/*.md".to_string(),
                            matches: Some(FileOperationPatternKind::File),
                            options: None,
                        },
                    }],
                }),
                will_rename: None,
                did_delete: Some(FileOperationRegistrationOptions {
                    filters: vec![FileOperationFilter {
                        scheme: Some("file".to_string()),
                        pattern: FileOperationPattern {
                            glob: "**/*.md".to_string(),
                            matches: Some(FileOperationPatternKind::File),
                            options: None,
                        },
                    }],
                }),
                will_delete: None,
            }),
        });

        capabilities
    }

    /// Extract workspace information from initialization parameters
    pub fn extract_workspace_info(params: &InitializeParams) -> Option<WorkspaceInfo> {
        if let Some(root_uri) = &params.root_uri {
            if let Ok(root_path) = root_uri.to_file_path() {
                return Some(WorkspaceInfo {
                    root_path,
                    workspace_folders: params.workspace_folders.clone().unwrap_or_default(),
                });
            }
        }
        None
    }
}

/// Workspace information extracted from initialization
pub struct WorkspaceInfo {
    pub root_path: std::path::PathBuf,
    pub workspace_folders: Vec<WorkspaceFolder>,
}

/// Handle document lifecycle events
pub struct DocumentHandler;

impl DocumentHandler {
    /// Process document open event
    pub fn handle_did_open(params: &DidOpenTextDocumentParams) -> DocumentEvent {
        DocumentEvent::Opened {
            uri: params.text_document.uri.clone(),
            text: params.text_document.text.clone(),
            language_id: params.text_document.language_id.clone(),
            version: params.text_document.version,
        }
    }

    /// Process document change event
    pub fn handle_did_change(params: &DidChangeTextDocumentParams) -> Option<DocumentEvent> {
        if let Some(change) = params.content_changes.first() {
            return Some(DocumentEvent::Changed {
                uri: params.text_document.uri.clone(),
                version: params.text_document.version,
                text: change.text.clone(),
            });
        }
        None
    }

    /// Process document close event
    pub fn handle_did_close(params: &DidCloseTextDocumentParams) -> DocumentEvent {
        DocumentEvent::Closed {
            uri: params.text_document.uri.clone(),
        }
    }
}

/// Document lifecycle events
#[derive(Debug, Clone)]
pub enum DocumentEvent {
    Opened {
        uri: Url,
        text: String,
        language_id: String,
        version: i32,
    },
    Changed {
        uri: Url,
        version: i32,
        text: String,
    },
    Closed {
        uri: Url,
    },
}

/// Handle workspace events and operations
pub struct WorkspaceHandler;

impl WorkspaceHandler {
    /// Process workspace folder changes
    pub fn handle_workspace_folders_change(
        params: &DidChangeWorkspaceFoldersParams,
    ) -> WorkspaceFoldersChange {
        WorkspaceFoldersChange {
            added: params.event.added.clone(),
            removed: params.event.removed.clone(),
        }
    }

    /// Process file creation events
    pub fn handle_file_create(params: &CreateFilesParams) -> Vec<CreatedFile> {
        params.files.iter().map(|file| CreatedFile {
            uri: file.uri.clone(),
        }).collect()
    }

    /// Process file deletion events  
    pub fn handle_file_delete(params: &DeleteFilesParams) -> Vec<DeletedFile> {
        params.files.iter().map(|file| DeletedFile {
            uri: file.uri.clone(),
        }).collect()
    }

    /// Process file rename events
    pub fn handle_file_rename(params: &RenameFilesParams) -> Vec<RenamedFile> {
        params.files.iter().map(|file| RenamedFile {
            old_uri: file.old_uri.clone(),
            new_uri: file.new_uri.clone(),
        }).collect()
    }
}

/// Workspace folder change event
#[derive(Debug, Clone)]
pub struct WorkspaceFoldersChange {
    pub added: Vec<WorkspaceFolder>,
    pub removed: Vec<WorkspaceFolder>,
}

/// File operation events
#[derive(Debug, Clone)]
pub struct CreatedFile {
    pub uri: Url,
}

#[derive(Debug, Clone)]
pub struct DeletedFile {
    pub uri: Url,
}

#[derive(Debug, Clone)]
pub struct RenamedFile {
    pub old_uri: Url,
    pub new_uri: Url,
}

/// Error handling utilities for LSP handlers
pub struct ErrorHandler;

impl ErrorHandler {
    /// Convert internal errors to LSP error responses
    pub fn handle_error(error: crate::AdrscanError) -> tower_lsp::jsonrpc::Error {
        tower_lsp::jsonrpc::Error {
            code: tower_lsp::jsonrpc::ErrorCode::InternalError,
            message: error.to_string().into(),
            data: None,
        }
    }

    /// Create a generic LSP error
    pub fn generic_error(message: &str) -> tower_lsp::jsonrpc::Error {
        tower_lsp::jsonrpc::Error {
            code: tower_lsp::jsonrpc::ErrorCode::InternalError,
            message: message.into(),
            data: None,
        }
    }
}