//! Integration tests for PhotonDrift LSP server

#[cfg(feature = "lsp")]
mod lsp_tests {
    use adrscan::lsp::{PhotonDriftLspServer, DiagnosticEngine, CompletionProvider, HoverProvider};
    use lsp_types::*;
    use tower_lsp::{Client, LanguageServer};
    use std::collections::HashMap;

    // Mock client for testing
    struct MockClient;

    #[tower_lsp::async_trait]
    impl Client for MockClient {
        async fn log_message(&self, _typ: MessageType, _message: String) {}
        async fn show_message(&self, _typ: MessageType, _message: String) {}
        async fn publish_diagnostics(&self, _uri: Url, _diagnostics: Vec<Diagnostic>, _version: Option<i32>) {}
    }

    #[tokio::test]
    async fn test_lsp_server_lifecycle() {
        let client = MockClient;
        let server = PhotonDriftLspServer::new(Box::new(client));

        // Test initialization
        let init_params = InitializeParams {
            process_id: None,
            root_path: None,
            root_uri: Some(Url::parse("file:///tmp/test-workspace").unwrap()),
            initialization_options: None,
            capabilities: ClientCapabilities {
                text_document: Some(TextDocumentClientCapabilities {
                    completion: Some(CompletionClientCapabilities {
                        completion_item: Some(CompletionItemCapability {
                            snippet_support: Some(true),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }),
                    hover: Some(HoverClientCapabilities {
                        content_format: Some(vec![MarkupKind::Markdown]),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            },
            trace: Some(TraceValue::Off),
            workspace_folders: Some(vec![WorkspaceFolder {
                uri: Url::parse("file:///tmp/test-workspace").unwrap(),
                name: "test-workspace".to_string(),
            }]),
            client_info: None,
            locale: None,
        };

        let result = server.initialize(init_params).await.unwrap();
        
        // Verify server info
        assert_eq!(result.server_info.as_ref().unwrap().name, "PhotonDrift LSP");
        
        // Verify capabilities
        let caps = result.capabilities;
        assert!(caps.text_document_sync.is_some());
        assert!(caps.completion_provider.is_some());
        assert!(caps.hover_provider.is_some());
        
        // Test initialized notification
        server.initialized(InitializedParams {}).await;
        
        // Test shutdown
        server.shutdown().await.unwrap();
    }

    #[tokio::test]
    async fn test_document_operations() {
        let client = MockClient;
        let server = PhotonDriftLspServer::new(Box::new(client));

        let uri = Url::parse("file:///tmp/test-adr.md").unwrap();
        let initial_content = r#"# ADR-001: Use Rust for Backend

## Status
Proposed

## Context
We need a performant backend language.

## Decision
We will use Rust for our backend services.
"#;

        // Test document open
        let open_params = DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: uri.clone(),
                language_id: "markdown".to_string(),
                version: 1,
                text: initial_content.to_string(),
            },
        };

        server.did_open(open_params).await;

        // Verify document is stored (we can't directly access the internal state,
        // but we can test through completion requests)
        let completion_params = CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position { line: 8, character: 0 },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
            partial_result_params: PartialResultParams::default(),
            context: None,
        };

        let completions = server.completion(completion_params).await.unwrap();
        assert!(completions.is_some());

        // Test document change
        let change_params = DidChangeTextDocumentParams {
            text_document: VersionedTextDocumentIdentifier {
                uri: uri.clone(),
                version: 2,
            },
            content_changes: vec![TextDocumentContentChangeEvent {
                range: None,
                range_length: None,
                text: format!("{}\n\n## Consequences\nBetter performance.", initial_content),
            }],
        };

        server.did_change(change_params).await;

        // Test document close
        let close_params = DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
        };

        server.did_close(close_params).await;
    }

    #[tokio::test]
    async fn test_completion_functionality() {
        let provider = CompletionProvider::new();

        // Test template completion at document start
        let content = "";
        let position = Position { line: 0, character: 0 };
        let completions = provider.get_completions(content, position).await;

        assert!(!completions.is_empty());
        assert!(completions.iter().any(|c| c.label.contains("Full ADR Template")));
        assert!(completions.iter().any(|c| c.label.contains("Simple ADR Template")));

        // Test section completion
        let content = "# ADR-001: Test\n\n## ";
        let position = Position { line: 2, character: 3 };
        let completions = provider.get_completions(content, position).await;

        assert!(completions.iter().any(|c| c.label.contains("Status")));
        assert!(completions.iter().any(|c| c.label.contains("Context")));
        assert!(completions.iter().any(|c| c.label.contains("Decision")));

        // Test status value completion
        let content = "# ADR-001: Test\n\n## Status\n";
        let position = Position { line: 3, character: 0 };
        let completions = provider.get_completions(content, position).await;

        assert!(completions.iter().any(|c| c.label == "Proposed"));
        assert!(completions.iter().any(|c| c.label == "Accepted"));
        assert!(completions.iter().any(|c| c.label == "Deprecated"));
    }

    #[tokio::test]
    async fn test_hover_functionality() {
        let provider = HoverProvider::new();

        // Test status hover
        let content = "## Status\nAccepted\n\n## Context\nSome context";
        let position = Position { line: 1, character: 3 }; // Position in "Accepted"
        
        let hover = provider.get_hover_info(content, position).await;
        assert!(hover.is_some());

        if let Some(h) = hover {
            if let HoverContents::Markup(markup) = h.contents {
                assert!(markup.value.contains("ACCEPTED"));
                assert!(markup.value.contains("approved"));
            }
        }

        // Test ADR reference hover
        let content = "Related to ADR-001 for database decisions";
        let position = Position { line: 0, character: 12 }; // Position in "ADR-001"
        
        let hover = provider.get_hover_info(content, position).await;
        assert!(hover.is_some());

        if let Some(h) = hover {
            if let HoverContents::Markup(markup) = h.contents {
                assert!(markup.value.contains("Architecture Decision Record Reference"));
            }
        }

        // Test section hover
        let content = "## Context\nThis is the context section";
        let position = Position { line: 0, character: 5 }; // Position in "Context"
        
        let hover = provider.get_hover_info(content, position).await;
        assert!(hover.is_some());

        if let Some(h) = hover {
            if let HoverContents::Markup(markup) = h.contents {
                assert!(markup.value.contains("Context Section"));
            }
        }
    }

    #[tokio::test]
    async fn test_diagnostics_engine() {
        let engine = DiagnosticEngine::new();
        
        // Test well-formed ADR (should have minimal diagnostics)
        let good_content = r#"# ADR-001: Use Rust for Backend

## Status
Accepted

## Context
We need a performant backend language for our services.

## Decision
We will use Rust for our backend services due to its performance and safety.

## Consequences
Better performance and memory safety, but steeper learning curve for the team.
"#;

        let uri = Url::parse("file:///tmp/good-adr.md").unwrap();
        let diagnostics = engine.analyze_content(good_content, &uri).await.unwrap();
        
        // Should have few or no warnings for a well-formed ADR
        let warnings = diagnostics.iter().filter(|d| {
            matches!(d.severity, Some(DiagnosticSeverity::WARNING))
        }).count();
        assert!(warnings <= 1);

        // Test malformed ADR (should have many diagnostics)
        let bad_content = "This is not a proper ADR at all";
        let diagnostics = engine.analyze_content(bad_content, &uri).await.unwrap();
        
        assert!(!diagnostics.is_empty());
        assert!(diagnostics.iter().any(|d| {
            d.code == Some(NumberOrString::String("missing-title".to_string()))
        }));
        assert!(diagnostics.iter().any(|d| {
            d.code == Some(NumberOrString::String("missing-status".to_string()))
        }));

        // Test empty sections
        let empty_sections_content = r#"# ADR-001: Test

## Status

## Context
Some context here

## Decision

"#;
        
        let diagnostics = engine.analyze_content(empty_sections_content, &uri).await.unwrap();
        let empty_section_warnings = diagnostics.iter().filter(|d| {
            d.code == Some(NumberOrString::String("empty-section".to_string()))
        }).count();
        
        assert!(empty_section_warnings >= 1); // Should detect empty Status and Decision sections
    }

    #[tokio::test]
    async fn test_end_to_end_lsp_workflow() {
        let client = MockClient;
        let server = PhotonDriftLspServer::new(Box::new(client));

        // Initialize server
        let init_params = InitializeParams {
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

        server.initialize(init_params).await.unwrap();
        server.initialized(InitializedParams {}).await;

        // Open a document
        let uri = Url::parse("file:///tmp/workflow-test.md").unwrap();
        let content = "# ADR-001: Test Decision\n\n";

        let open_params = DidOpenTextDocumentParams {
            text_document: TextDocumentItem {
                uri: uri.clone(),
                language_id: "markdown".to_string(),
                version: 1,
                text: content.to_string(),
            },
        };

        server.did_open(open_params).await;

        // Test completion at the end of the document
        let completion_params = CompletionParams {
            text_document_position: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position { line: 2, character: 0 },
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
            partial_result_params: PartialResultParams::default(),
            context: None,
        };

        let completions = server.completion(completion_params).await.unwrap();
        assert!(completions.is_some());

        // Test hover on the title
        let hover_params = HoverParams {
            text_document_position_params: TextDocumentPositionParams {
                text_document: TextDocumentIdentifier { uri: uri.clone() },
                position: Position { line: 0, character: 5 }, // On "ADR-001"
            },
            work_done_progress_params: WorkDoneProgressParams::default(),
        };

        let hover = server.hover(hover_params).await.unwrap();
        assert!(hover.is_some());

        // Close the document
        let close_params = DidCloseTextDocumentParams {
            text_document: TextDocumentIdentifier { uri: uri.clone() },
        };

        server.did_close(close_params).await;

        // Shutdown server
        server.shutdown().await.unwrap();
    }

    #[test]
    fn test_lsp_configuration_validation() {
        use adrscan::lsp::LspConfig;

        let config = LspConfig::default();
        assert!(config.diagnostics_enabled);
        assert!(config.completion_enabled);
        assert!(config.hover_enabled);
        assert_eq!(config.max_diagnostics, 100);
        assert!(config.workspace_root.is_none());

        // Test custom configuration
        let custom_config = LspConfig {
            diagnostics_enabled: false,
            max_diagnostics: 50,
            completion_enabled: true,
            hover_enabled: false,
            workspace_root: Some("/tmp/test".into()),
        };

        assert!(!custom_config.diagnostics_enabled);
        assert_eq!(custom_config.max_diagnostics, 50);
        assert!(!custom_config.hover_enabled);
        assert!(custom_config.workspace_root.is_some());
    }
}

#[cfg(not(feature = "lsp"))]
mod no_lsp_tests {
    #[test]
    fn test_lsp_not_available() {
        // This test ensures the test suite runs even without LSP features
        assert!(true, "LSP feature not enabled");
    }
}