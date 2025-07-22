//! LSP protocol compliance tests
//! 
//! Tests to ensure the PhotonDrift LSP server complies with the Language Server Protocol specification.

use tower_lsp::lsp_types::*;
use serde_json::{Value, json};
use adrscan::lsp::protocol::{LspProtocolHelper, error_codes};

/// Test LSP message validation
#[test]
fn test_lsp_message_validation() {
    let helper = LspProtocolHelper::new();

    // Valid request message
    let valid_request = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "textDocument/completion",
        "params": {}
    });
    assert!(helper.validate_message(&valid_request).is_ok());

    // Valid notification message
    let valid_notification = json!({
        "jsonrpc": "2.0",
        "method": "textDocument/didOpen",
        "params": {}
    });
    assert!(helper.validate_message(&valid_notification).is_ok());

    // Valid response message
    let valid_response = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {}
    });
    assert!(helper.validate_message(&valid_response).is_ok());

    // Valid error response
    let valid_error = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "error": {
            "code": -32601,
            "message": "Method not found"
        }
    });
    assert!(helper.validate_message(&valid_error).is_ok());

    // Invalid messages
    let invalid_messages = vec![
        // Missing jsonrpc
        json!({"id": 1, "method": "test"}),
        // Wrong jsonrpc version
        json!({"jsonrpc": "1.0", "id": 1, "method": "test"}),
        // Invalid structure (has both result and error)
        json!({"jsonrpc": "2.0", "id": 1, "result": {}, "error": {}}),
        // Not an object
        json!("invalid"),
    ];

    for invalid_msg in invalid_messages {
        assert!(helper.validate_message(&invalid_msg).is_err());
    }
}

/// Test error response creation
#[test]
fn test_error_response_creation() {
    let helper = LspProtocolHelper::new();

    let error_response = helper.create_error_response(
        Some(json!(123)),
        error_codes::METHOD_NOT_FOUND,
        "Method not found"
    );

    assert_eq!(error_response["jsonrpc"], "2.0");
    assert_eq!(error_response["id"], 123);
    assert_eq!(error_response["error"]["code"], error_codes::METHOD_NOT_FOUND);
    assert_eq!(error_response["error"]["message"], "Method not found");
}

/// Test position and offset conversions
#[test]
fn test_position_conversions() {
    let helper = LspProtocolHelper::new();
    let text = "Hello\nWorld\nTest\n";

    // Test position to offset
    let position = Position { line: 1, character: 2 };
    let offset = helper.position_to_offset(text, position).unwrap();
    assert_eq!(offset, 8); // "Hello\nWo"

    // Test offset to position
    let converted_position = helper.offset_to_position(text, offset);
    assert_eq!(converted_position, position);

    // Test end of file
    let eof_position = Position { line: 3, character: 0 };
    let eof_offset = helper.position_to_offset(text, eof_position).unwrap();
    assert_eq!(eof_offset, text.len());
}

/// Test range creation utilities
#[test]
fn test_range_creation() {
    let helper = LspProtocolHelper::new();

    let range = helper.create_range(0, 5, 2, 10);
    assert_eq!(range.start.line, 0);
    assert_eq!(range.start.character, 5);
    assert_eq!(range.end.line, 2);
    assert_eq!(range.end.character, 10);

    // Test line range extraction
    let text = "Line 1\nLine 2 with more content\nLine 3\n";
    let line_range = helper.get_line_range(text, 1).unwrap();
    
    assert_eq!(line_range.start.line, 1);
    assert_eq!(line_range.start.character, 0);
    assert_eq!(line_range.end.line, 1);
    assert_eq!(line_range.end.character, "Line 2 with more content".chars().count() as u32);
}

/// Test word extraction at position
#[test]
fn test_word_extraction() {
    let helper = LspProtocolHelper::new();
    let text = "Hello world-test_case";

    // Test word at beginning
    let position = Position { line: 0, character: 2 };
    let (word, range) = helper.get_word_at_position(text, position).unwrap();
    assert_eq!(word, "Hello");
    assert_eq!(range.start.character, 0);
    assert_eq!(range.end.character, 5);

    // Test hyphenated word
    let position = Position { line: 0, character: 8 };
    let (word, range) = helper.get_word_at_position(text, position).unwrap();
    assert_eq!(word, "world-test_case");
    assert_eq!(range.start.character, 6);
    assert_eq!(range.end.character, 21);

    // Test position beyond text
    let position = Position { line: 0, character: 100 };
    assert!(helper.get_word_at_position(text, position).is_none());
}

/// Test diagnostic creation
#[test]
fn test_diagnostic_creation() {
    let helper = LspProtocolHelper::new();

    let range = Range {
        start: Position { line: 0, character: 0 },
        end: Position { line: 0, character: 10 },
    };

    let diagnostic = helper.create_diagnostic(
        range.clone(),
        DiagnosticSeverity::ERROR,
        "test-error",
        "This is a test error"
    );

    assert_eq!(diagnostic.range, range);
    assert_eq!(diagnostic.severity, Some(DiagnosticSeverity::ERROR));
    assert_eq!(diagnostic.code, Some(NumberOrString::String("test-error".to_string())));
    assert_eq!(diagnostic.source, Some("photon-drift".to_string()));
    assert_eq!(diagnostic.message, "This is a test error");
}

/// Test completion item creation
#[test]
fn test_completion_item_creation() {
    let helper = LspProtocolHelper::new();

    // Test snippet completion
    let snippet_item = helper.create_completion_item(
        "test-snippet",
        CompletionItemKind::SNIPPET,
        Some("A test snippet"),
        Some("This is a test snippet"),
        Some("test ${1:placeholder}"),
        true
    );

    assert_eq!(snippet_item.label, "test-snippet");
    assert_eq!(snippet_item.kind, Some(CompletionItemKind::SNIPPET));
    assert_eq!(snippet_item.detail, Some("A test snippet".to_string()));
    assert_eq!(snippet_item.insert_text, Some("test ${1:placeholder}".to_string()));
    assert_eq!(snippet_item.insert_text_format, Some(InsertTextFormat::SNIPPET));

    // Test plain text completion
    let text_item = helper.create_completion_item(
        "plain-text",
        CompletionItemKind::TEXT,
        None,
        None,
        Some("simple text"),
        false
    );

    assert_eq!(text_item.label, "plain-text");
    assert_eq!(text_item.kind, Some(CompletionItemKind::TEXT));
    assert_eq!(text_item.detail, None);
    assert_eq!(text_item.insert_text_format, Some(InsertTextFormat::PLAIN_TEXT));
}

/// Test hover creation
#[test]
fn test_hover_creation() {
    let helper = LspProtocolHelper::new();

    let range = Some(Range {
        start: Position { line: 0, character: 0 },
        end: Position { line: 0, character: 5 },
    });

    let hover = helper.create_hover("# Test Hover\n\nThis is **markdown** content.", range.clone());

    match hover.contents {
        HoverContents::Markup(content) => {
            assert_eq!(content.kind, MarkupKind::Markdown);
            assert!(content.value.contains("Test Hover"));
            assert!(content.value.contains("**markdown**"));
        }
        _ => panic!("Expected markup hover content"),
    }

    assert_eq!(hover.range, range);
}

/// Test URI and path conversions
#[test]
fn test_uri_path_conversions() {
    let helper = LspProtocolHelper::new();

    // Test path to URI conversion
    let path = std::path::Path::new("/tmp/test.md");
    let uri = helper.path_to_uri(path);
    assert!(uri.is_some());
    
    if let Some(uri) = uri {
        assert!(uri.as_str().starts_with("file://"));
        assert!(uri.as_str().ends_with("test.md"));

        // Test URI to path conversion
        let converted_path = helper.uri_to_path(&uri);
        assert!(converted_path.is_some());
        
        if let Some(converted_path) = converted_path {
            assert_eq!(converted_path.file_name().unwrap(), "test.md");
        }
    }
}

/// Test performance measurement
#[test]
fn test_performance_measurement() {
    let helper = LspProtocolHelper::new();

    let result = helper.measure_performance("test-operation", || {
        // Simulate some work
        std::thread::sleep(std::time::Duration::from_millis(1));
        42
    });

    assert_eq!(result, 42);
}

/// Test LSP capabilities compliance
#[test]
fn test_server_capabilities_compliance() {
    use adrscan::lsp::handlers::InitializationHandler;

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

    let capabilities = InitializationHandler::process_initialization(&init_params);

    // Verify required capabilities are present
    assert!(capabilities.text_document_sync.is_some());
    
    // Verify specific PhotonDrift capabilities
    if let Some(completion_provider) = capabilities.completion_provider {
        assert!(completion_provider.trigger_characters.is_some());
        let triggers = completion_provider.trigger_characters.unwrap();
        assert!(triggers.contains(&":".to_string()));
        assert!(triggers.contains(&"#".to_string()));
    }

    assert!(capabilities.hover_provider.is_some());
    assert!(capabilities.diagnostic_provider.is_some());
    
    if let Some(workspace) = capabilities.workspace {
        assert!(workspace.workspace_folders.is_some());
        assert!(workspace.file_operations.is_some());
    }
}

/// Test error code constants
#[test]
fn test_error_codes() {
    // Verify LSP standard error codes
    assert_eq!(error_codes::PARSE_ERROR, -32700);
    assert_eq!(error_codes::INVALID_REQUEST, -32600);
    assert_eq!(error_codes::METHOD_NOT_FOUND, -32601);
    assert_eq!(error_codes::INVALID_PARAMS, -32602);
    assert_eq!(error_codes::INTERNAL_ERROR, -32603);

    // Verify LSP specific error codes
    assert_eq!(error_codes::SERVER_NOT_INITIALIZED, -32002);
    assert_eq!(error_codes::REQUEST_CANCELLED, -32800);
    assert_eq!(error_codes::CONTENT_MODIFIED, -32801);
}