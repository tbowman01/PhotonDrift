//! Performance tests for PhotonDrift LSP server
//! 
//! Tests to ensure the LSP server meets performance requirements (<100ms response time).

use std::time::{Duration, Instant};
use tempfile::TempDir;
use tower_lsp::lsp_types::*;
use url::Url;

use adrscan::lsp::PhotonDriftLspServer;

/// Mock client for performance testing
struct PerfTestClient;

#[tower_lsp::async_trait]
impl tower_lsp::Client for PerfTestClient {
    async fn log_message(&self, _typ: MessageType, _message: String) {}
    async fn show_message(&self, _typ: MessageType, _message: String) {}
    async fn show_message_request(
        &self,
        _typ: MessageType,
        _message: String,
        _actions: Option<Vec<MessageActionItem>>,
    ) -> tower_lsp::jsonrpc::Result<Option<MessageActionItem>> {
        Ok(None)
    }
    async fn publish_diagnostics(&self, _uri: Url, _diagnostics: Vec<Diagnostic>, _version: Option<i32>) {}
}

/// Performance benchmark configuration
struct BenchmarkConfig {
    max_duration: Duration,
    iterations: usize,
}

impl Default for BenchmarkConfig {
    fn default() -> Self {
        Self {
            max_duration: Duration::from_millis(100), // LSP response time requirement
            iterations: 10,
        }
    }
}

/// Run a performance benchmark on an async function
async fn benchmark_async<F, R>(name: &str, config: &BenchmarkConfig, mut f: F) -> Duration
where
    F: FnMut() -> std::pin::Pin<Box<dyn std::future::Future<Output = R> + Send>>,
{
    let mut total_duration = Duration::ZERO;
    let mut max_duration = Duration::ZERO;
    let mut min_duration = Duration::from_secs(999);

    for i in 0..config.iterations {
        let start = Instant::now();
        let _result = f().await;
        let duration = start.elapsed();
        
        total_duration += duration;
        max_duration = max_duration.max(duration);
        min_duration = min_duration.min(duration);
        
        if duration > config.max_duration {
            eprintln!(
                "⚠️  {} iteration {} took {}ms (exceeds {}ms limit)",
                name,
                i + 1,
                duration.as_millis(),
                config.max_duration.as_millis()
            );
        }
    }

    let average = total_duration / config.iterations as u32;
    
    println!(
        "📊 {} Performance:\n  Avg: {}ms | Min: {}ms | Max: {}ms | Target: <{}ms",
        name,
        average.as_millis(),
        min_duration.as_millis(),
        max_duration.as_millis(),
        config.max_duration.as_millis()
    );

    // Assert that average performance meets requirements
    assert!(
        average <= config.max_duration,
        "Average response time {}ms exceeds limit {}ms",
        average.as_millis(),
        config.max_duration.as_millis()
    );

    average
}

/// Test initialization performance
#[tokio::test]
async fn test_initialization_performance() {
    let config = BenchmarkConfig::default();

    benchmark_async("LSP Initialization", &config, || {
        Box::pin(async {
            let client = PerfTestClient;
            let server = PhotonDriftLspServer::new(client);

            let temp_dir = TempDir::new().unwrap();
            let workspace_uri = Url::from_file_path(temp_dir.path()).unwrap();

            let params = InitializeParams {
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

            server.initialize(params).await.unwrap()
        })
    }).await;
}

/// Test document open performance
#[tokio::test]
async fn test_document_open_performance() {
    let config = BenchmarkConfig {
        max_duration: Duration::from_millis(50), // Document operations should be faster
        ..Default::default()
    };

    // Create server once for all iterations
    let client = PerfTestClient;
    let server = PhotonDriftLspServer::new(client);
    
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
    
    server.initialize(init_params).await.unwrap();
    server.initialized(InitializedParams {}).await;

    let large_adr_content = create_large_adr_document();

    benchmark_async("Document Open", &config, || {
        let server = &server;
        let content = large_adr_content.clone();
        
        Box::pin(async move {
            let doc_uri = Url::parse(&format!("file:///test/adr-{}.md", rand::random::<u32>())).unwrap();
            
            let open_params = DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: doc_uri,
                    language_id: "markdown".to_string(),
                    version: 1,
                    text: content,
                },
            };

            server.did_open(open_params).await;
        })
    }).await;
}

/// Test completion performance
#[tokio::test]
async fn test_completion_performance() {
    let config = BenchmarkConfig::default();

    let client = PerfTestClient;
    let server = PhotonDriftLspServer::new(client);
    
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
    
    server.initialize(init_params).await.unwrap();
    server.initialized(InitializedParams {}).await;

    // Open a document first
    let doc_uri = Url::parse("file:///test/completion-perf.md").unwrap();
    let content = "# ADR-001: Performance Test\n\n## ";
    
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: doc_uri.clone(),
            language_id: "markdown".to_string(),
            version: 1,
            text: content.to_string(),
        },
    };
    
    server.did_open(open_params).await;

    benchmark_async("Completion Request", &config, || {
        let server = &server;
        let uri = doc_uri.clone();
        
        Box::pin(async move {
            let completion_params = CompletionParams {
                text_document_position: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri },
                    position: Position { line: 2, character: 3 },
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
                context: None,
            };

            server.completion(completion_params).await.unwrap()
        })
    }).await;
}

/// Test hover performance
#[tokio::test]
async fn test_hover_performance() {
    let config = BenchmarkConfig::default();

    let client = PerfTestClient;
    let server = PhotonDriftLspServer::new(client);
    
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
    
    server.initialize(init_params).await.unwrap();
    server.initialized(InitializedParams {}).await;

    // Open a document with status keywords
    let doc_uri = Url::parse("file:///test/hover-perf.md").unwrap();
    let content = r#"# ADR-001: Performance Test

## Status

Accepted

## Context

This is for testing hover performance.
"#;
    
    let open_params = DidOpenTextDocumentParams {
        text_document: TextDocumentItem {
            uri: doc_uri.clone(),
            language_id: "markdown".to_string(),
            version: 1,
            text: content.to_string(),
        },
    };
    
    server.did_open(open_params).await;

    benchmark_async("Hover Request", &config, || {
        let server = &server;
        let uri = doc_uri.clone();
        
        Box::pin(async move {
            let hover_params = HoverParams {
                text_document_position_params: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri },
                    position: Position { line: 4, character: 2 }, // "Accepted"
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
            };

            server.hover(hover_params).await.unwrap()
        })
    }).await;
}

/// Test diagnostic performance with large documents
#[tokio::test]
async fn test_diagnostics_performance() {
    let config = BenchmarkConfig {
        max_duration: Duration::from_millis(200), // Diagnostics can take slightly longer
        iterations: 5, // Fewer iterations for expensive operation
    };

    let client = PerfTestClient;
    let server = PhotonDriftLspServer::new(client);
    
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
    
    server.initialize(init_params).await.unwrap();
    server.initialized(InitializedParams {}).await;

    let complex_content = create_complex_adr_document();

    benchmark_async("Diagnostics Processing", &config, || {
        let server = &server;
        let content = complex_content.clone();
        
        Box::pin(async move {
            let doc_uri = Url::parse(&format!("file:///test/diagnostics-{}.md", rand::random::<u32>())).unwrap();
            
            let open_params = DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: doc_uri,
                    language_id: "markdown".to_string(),
                    version: 1,
                    text: content,
                },
            };

            server.did_open(open_params).await;
            
            // Give diagnostics time to process
            tokio::time::sleep(Duration::from_millis(10)).await;
        })
    }).await;
}

/// Test concurrent operation performance
#[tokio::test]
async fn test_concurrent_operations_performance() {
    let client = PerfTestClient;
    let server = std::sync::Arc::new(PhotonDriftLspServer::new(client));
    
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
    
    server.initialize(init_params).await.unwrap();
    server.initialized(InitializedParams {}).await;

    let start = Instant::now();
    
    // Simulate concurrent operations from multiple clients
    let mut tasks = Vec::new();
    
    for i in 0..20 {
        let server_clone = server.clone();
        let task = tokio::spawn(async move {
            let doc_uri = Url::parse(&format!("file:///test/concurrent-{}.md", i)).unwrap();
            let content = format!("# ADR-{}: Concurrent Test\n\n## Status\n\nProposed\n", i);
            
            // Open document
            let open_params = DidOpenTextDocumentParams {
                text_document: TextDocumentItem {
                    uri: doc_uri.clone(),
                    language_id: "markdown".to_string(),
                    version: 1,
                    text: content,
                },
            };
            server_clone.did_open(open_params).await;
            
            // Request completion
            let completion_params = CompletionParams {
                text_document_position: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri: doc_uri.clone() },
                    position: Position { line: 6, character: 0 },
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
                partial_result_params: PartialResultParams::default(),
                context: None,
            };
            let _completion = server_clone.completion(completion_params).await;
            
            // Request hover
            let hover_params = HoverParams {
                text_document_position_params: TextDocumentPositionParams {
                    text_document: TextDocumentIdentifier { uri: doc_uri },
                    position: Position { line: 4, character: 2 },
                },
                work_done_progress_params: WorkDoneProgressParams::default(),
            };
            let _hover = server_clone.hover(hover_params).await;
        });
        
        tasks.push(task);
    }
    
    // Wait for all concurrent operations to complete
    for task in tasks {
        task.await.unwrap();
    }
    
    let total_duration = start.elapsed();
    
    println!(
        "📊 Concurrent Operations Performance:\n  Total time: {}ms for 20 concurrent operations\n  Average per operation: {}ms",
        total_duration.as_millis(),
        total_duration.as_millis() / 20
    );
    
    // Should complete all operations reasonably quickly
    assert!(
        total_duration < Duration::from_secs(5),
        "Concurrent operations took too long: {}ms",
        total_duration.as_millis()
    );
}

/// Create a large ADR document for performance testing
fn create_large_adr_document() -> String {
    let mut content = String::with_capacity(10000);
    
    content.push_str("# ADR-001: Large Performance Test Document\n\n");
    content.push_str("## Status\n\nAccepted\n\n");
    content.push_str("## Context\n\n");
    
    // Add a lot of context content
    for i in 0..50 {
        content.push_str(&format!(
            "This is context paragraph {}. It contains detailed information about the architectural decision that needs to be made. The context explains the current situation, problems, and constraints that influence this decision.\n\n",
            i + 1
        ));
    }
    
    content.push_str("## Decision\n\n");
    content.push_str("After careful consideration of all the factors mentioned in the context, we have decided to implement the following architectural solution.\n\n");
    
    content.push_str("## Consequences\n\n");
    
    // Add many consequence items
    for i in 0..30 {
        content.push_str(&format!(
            "- Consequence {}: This will impact the system in various ways that need to be considered.\n",
            i + 1
        ));
    }
    
    content.push_str("\n## Related\n\n");
    content.push_str("- [ADR-002](./002-related-decision.md)\n");
    content.push_str("- [ADR-003](./003-another-decision.md)\n");
    
    content
}

/// Create a complex ADR document with potential issues for diagnostics testing
fn create_complex_adr_document() -> String {
    r#"# ADR-001: Complex Test Document

This document has various issues that should be detected by diagnostics.

## Status

Maybe-Accepted-Or-Not

## Context

We need to choose between jQuery and React for our frontend.
The decision involves using Internet Explorer compatibility and Flash support.
We also need to consider Java 8 compatibility.

Links to check:
- [Broken Link](http://example.com/broken)
- [TODO Link](TODO: add real link)
- [Another Link](https://secure-site.com)

Reference to ADR-999 which probably doesn't exist.

## Decision

We decided to use the old technology stack because it works.

## Consequences

### Positive

- It works for now
- No changes needed

### Negative

- Technical debt
- Security issues
- Performance problems

### Neutral

- Documentation needs updating

## References

- ADR-123: Another high number reference
- ADR-500: Yet another reference

---
Date: 2030-01-01
Author: Future Person
"#.to_string()
}