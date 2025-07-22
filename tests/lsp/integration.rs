//! Integration tests for PhotonDrift LSP server
//! 
//! Tests the complete LSP server functionality with real ADR documents.

use std::collections::HashMap;
use tempfile::TempDir;
use tokio::time::{timeout, Duration};
use tower_lsp::lsp_types::*;
use tower_lsp::{LspService, Server};
use tokio::io::{duplex, AsyncWriteExt};

use adrscan::lsp::PhotonDriftLspServer;
use adrscan::config::Config;

/// Mock client for testing LSP server
struct MockLspClient;

#[tower_lsp::async_trait]
impl tower_lsp::Client for MockLspClient {
    async fn log_message(&self, typ: MessageType, message: String) {
        println!("[LSP LOG {:?}] {}", typ, message);
    }

    async fn show_message(&self, typ: MessageType, message: String) {
        println!("[LSP SHOW {:?}] {}", typ, message);
    }

    async fn show_message_request(
        &self,
        typ: MessageType,
        message: String,
        actions: Option<Vec<MessageActionItem>>,
    ) -> tower_lsp::jsonrpc::Result<Option<MessageActionItem>> {
        println!("[LSP SHOW_REQUEST {:?}] {} (actions: {:?})", typ, message, actions);
        Ok(None)
    }

    async fn publish_diagnostics(&self, uri: Url, diagnostics: Vec<Diagnostic>, version: Option<i32>) {
        println!("[LSP DIAGNOSTICS] {} diagnostics for {} (version: {:?})", diagnostics.len(), uri, version);
        for diagnostic in diagnostics {
            println!("  {:?}: {}", diagnostic.severity, diagnostic.message);
        }
    }
}

/// Test LSP server initialization
#[tokio::test]
async fn test_lsp_initialization() {
    let client = MockLspClient;
    let server = PhotonDriftLspServer::new(client);

    // Create test workspace
    let temp_dir = TempDir::new().unwrap();
    let workspace_uri = Url::from_file_path(temp_dir.path()).unwrap();

    let params = InitializeParams {
        process_id: None,
        root_path: None,
        root_uri: Some(workspace_uri.clone()),
        initialization_options: None,
        capabilities: ClientCapabilities::default(),
        trace: None,
        workspace_folders: Some(vec![WorkspaceFolder {
            uri: workspace_uri,
            name: "test-workspace".to_string(),
        }]),
        client_info: None,
        locale: None,
    };

    let result = server.initialize(params).await.unwrap();

    // Verify server capabilities
    assert!(result.capabilities.text_document_sync.is_some());
    assert!(result.capabilities.completion_provider.is_some());
    assert!(result.capabilities.hover_provider.is_some());
    assert!(result.capabilities.diagnostic_provider.is_some());
    assert!(result.capabilities.workspace.is_some());

    // Verify server info
    assert!(result.server_info.is_some());
    let server_info = result.server_info.unwrap();
    assert_eq!(server_info.name, "photon-drift-lsp");
    assert!(server_info.version.is_some());
}

/// Test document lifecycle management
#[tokio::test]
async fn test_document_lifecycle() {
    let client = MockLspClient;
    let server = PhotonDriftLspServer::new(client);

    // Initialize server first
    let temp_dir = TempDir::new().unwrap();
    let workspace_uri = Url::from_file_path(temp_dir.path()).unwrap();
    
    let init_params = InitializeParams {
        process_id: None,
        root_path: None,
        root_uri: Some(workspace_uri),
        initialization_options: None,
        capabilities: ClientCapabilities::default(),
        trace: None,
        workspace_folders: None,
        client_info: None,
        locale: None,
    };

    let _init_result = server.initialize(init_params).await.unwrap();
    server.initialized(InitializedParams {}).await;

    // Test document opening
    let doc_uri = Url::parse("file:///test/adr-001.md").unwrap();
    let adr_content = r#"# ADR-001: Test Architecture Decision

## Status

Proposed

## Context

This is a test ADR for validating LSP functionality.

## Decision

We will use this test ADR to verify our LSP server works correctly.

## Consequences

This enables automated testing of ADR management features.
"#;

    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: doc_uri.clone(),
            language_id: "markdown".to_string(),
            version: 1,
            text: adr_content.to_string(),
        },
    };

    server.did_open(open_params).await;

    // Test document changes
    let change_params = DidChangeTextDocumentParams {
        text_document: VersionedTextDocumentIdentifier {
            uri: doc_uri.clone(),
            version: 2,
        },
        content_changes: vec![TextDocumentContentChangeEvent {
            range: None,
            range_length: None,
            text: adr_content.replace("Proposed", "Accepted"),
        }],
    };

    server.did_change(change_params).await;

    // Test document closing
    let close_params = DidCloseTextDocumentParams {
        text_document: TextDocumentIdentifier {
            uri: doc_uri.clone(),
        },
    };

    server.did_close(close_params).await;
}

/// Test completion functionality
#[tokio::test]
async fn test_completion_functionality() {
    let client = MockLspClient;
    let server = PhotonDriftLspServer::new(client);

    // Initialize server
    let temp_dir = TempDir::new().unwrap();
    let workspace_uri = Url::from_file_path(temp_dir.path()).unwrap();
    
    let init_params = InitializeParams {
        process_id: None,
        root_path: None,
        root_uri: Some(workspace_uri),
        initialization_options: None,
        capabilities: ClientCapabilities::default(),
        trace: None,
        workspace_folders: None,
        client_info: None,
        locale: None,
    };

    let _init_result = server.initialize(init_params).await.unwrap();
    server.initialized(InitializedParams {}).await;

    // Open a document
    let doc_uri = Url::parse("file:///test/adr-002.md").unwrap();
    let partial_content = "# ADR-002: Test Decision\n\n## ";

    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: doc_uri.clone(),
            language_id: "markdown".to_string(),
            version: 1,
            text: partial_content.to_string(),
        },
    };

    server.did_open(open_params).await;

    // Test completion at section header position
    let completion_params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier {
                uri: doc_uri.clone(),
            },
            position: Position {
                line: 2,
                character: 3, // After "## "
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
        context: None,
    };

    let completion_result = server.completion(completion_params).await.unwrap();
    
    if let Some(CompletionResponse::Array(completions)) = completion_result {
        assert!(!completions.is_empty(), "Should provide completions for section headers");
        
        // Check that common ADR sections are suggested
        let section_labels: Vec<&str> = completions.iter()
            .map(|item| item.label.as_str())
            .collect();
        
        assert!(section_labels.iter().any(|&label| label.contains("Status")));
        assert!(section_labels.iter().any(|&label| label.contains("Context")));
        assert!(section_labels.iter().any(|&label| label.contains("Decision")));
    }
}

/// Test hover functionality
#[tokio::test]
async fn test_hover_functionality() {
    let client = MockLspClient;
    let server = PhotonDriftLspServer::new(client);

    // Initialize server
    let temp_dir = TempDir::new().unwrap();
    let workspace_uri = Url::from_file_path(temp_dir.path()).unwrap();
    
    let init_params = InitializeParams {
        process_id: None,
        root_path: None,
        root_uri: Some(workspace_uri),
        initialization_options: None,
        capabilities: ClientCapabilities::default(),
        trace: None,
        workspace_folders: None,
        client_info: None,
        locale: None,
    };

    let _init_result = server.initialize(init_params).await.unwrap();
    server.initialized(InitializedParams {}).await;

    // Open a document with status information
    let doc_uri = Url::parse("file:///test/adr-003.md").unwrap();
    let content_with_status = r#"# ADR-003: Test Decision

## Status

Accepted

## Context

Testing hover functionality.
"#;

    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: doc_uri.clone(),
            language_id: "markdown".to_string(),
            version: 1,
            text: content_with_status.to_string(),
        },
    };

    server.did_open(open_params).await;

    // Test hover over "Accepted" status
    let hover_params = HoverParams {
        text_document_position_params: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier {
                uri: doc_uri.clone(),
            },
            position: Position {
                line: 4, // Line with "Accepted"
                character: 2, // Within "Accepted"
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
    };

    let hover_result = server.hover(hover_params).await.unwrap();
    
    if let Some(hover) = hover_result {
        match hover.contents {
            HoverContents::Markup(content) => {
                assert!(content.value.contains("Accepted Status"));
                assert_eq!(content.kind, MarkupKind::Markdown);
            }
            _ => panic!("Expected markdown hover content"),
        }
    } else {
        panic!("Should provide hover information for status keywords");
    }
}

/// Test diagnostic functionality
#[tokio::test]
async fn test_diagnostic_functionality() {
    let client = MockLspClient;
    let server = PhotonDriftLspServer::new(client);

    // Initialize server
    let temp_dir = TempDir::new().unwrap();
    let workspace_uri = Url::from_file_path(temp_dir.path()).unwrap();
    
    let init_params = InitializeParams {
        process_id: None,
        root_path: None,
        root_uri: Some(workspace_uri),
        initialization_options: None,
        capabilities: ClientCapabilities::default(),
        trace: None,
        workspace_folders: None,
        client_info: None,
        locale: None,
    };

    let _init_result = server.initialize(init_params).await.unwrap();
    server.initialized(InitializedParams {}).await;

    // Open a document with issues
    let doc_uri = Url::parse("file:///test/adr-004.md").unwrap();
    let problematic_content = r#"# ADR-004: Test Decision

This ADR is missing required sections and has other issues.

We decided to use jQuery for our frontend framework.
"#;

    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: doc_uri.clone(),
            language_id: "markdown".to_string(),
            version: 1,
            text: problematic_content.to_string(),
        },
    };

    server.did_open(open_params).await;

    // Wait a moment for diagnostics to be processed
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Note: In a real test, we would capture the diagnostics sent to the client
    // For this integration test, we're verifying the server doesn't crash
    // and processes the document without errors
}

/// Test workspace functionality
#[tokio::test]
async fn test_workspace_functionality() {
    let client = MockLspClient;
    let server = PhotonDriftLspServer::new(client);

    // Create temporary workspace with ADR files
    let temp_dir = TempDir::new().unwrap();
    let workspace_path = temp_dir.path();
    
    // Create an ADR directory structure
    std::fs::create_dir_all(workspace_path.join("docs/adrs")).unwrap();
    
    // Write a test ADR
    let adr_content = r#"# ADR-001: Use Microservices Architecture

## Status

Accepted

## Context

Our monolithic application is becoming difficult to maintain.

## Decision

We will migrate to a microservices architecture.

## Consequences

- Better scalability
- Increased operational complexity
- Need for service discovery
"#;
    
    std::fs::write(workspace_path.join("docs/adrs/001-microservices.md"), adr_content).unwrap();

    let workspace_uri = Url::from_file_path(workspace_path).unwrap();
    
    let init_params = InitializeParams {
        process_id: None,
        root_path: None,
        root_uri: Some(workspace_uri.clone()),
        initialization_options: None,
        capabilities: ClientCapabilities::default(),
        trace: None,
        workspace_folders: Some(vec![WorkspaceFolder {
            uri: workspace_uri,
            name: "test-workspace".to_string(),
        }]),
        client_info: None,
        locale: None,
    };

    let init_result = server.initialize(init_params).await.unwrap();
    
    // Verify workspace capabilities
    assert!(init_result.capabilities.workspace.is_some());
    let workspace_caps = init_result.capabilities.workspace.unwrap();
    assert!(workspace_caps.workspace_folders.is_some());

    server.initialized(InitializedParams {}).await;

    // Test opening the ADR file from workspace
    let adr_uri = Url::from_file_path(workspace_path.join("docs/adrs/001-microservices.md")).unwrap();
    
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: adr_uri.clone(),
            language_id: "markdown".to_string(),
            version: 1,
            text: adr_content.to_string(),
        },
    };

    server.did_open(open_params).await;

    // Test completion in workspace context
    let completion_params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier {
                uri: adr_uri,
            },
            position: Position {
                line: 20, // After the content
                character: 0,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
        context: None,
    };

    let _completion_result = server.completion(completion_params).await.unwrap();
}

/// Test error handling
#[tokio::test]
async fn test_error_handling() {
    let client = MockLspClient;
    let server = PhotonDriftLspServer::new(client);

    // Test operation before initialization
    let completion_params = CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier {
                uri: Url::parse("file:///nonexistent.md").unwrap(),
            },
            position: Position {
                line: 0,
                character: 0,
            },
        },
        work_done_progress_params: WorkDoneProgressParams::default(),
        partial_result_params: PartialResultParams::default(),
        context: None,
    };

    // This should not crash even without initialization
    let result = server.completion(completion_params).await;
    assert!(result.is_ok()); // Should return None gracefully
    assert_eq!(result.unwrap(), None);
}