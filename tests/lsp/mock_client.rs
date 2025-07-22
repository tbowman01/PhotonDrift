//! Mock LSP client for testing PhotonDrift LSP server
//! 
//! Provides a comprehensive mock client that can simulate IDE behavior
//! and capture server responses for validation.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tower_lsp::lsp_types::*;
use url::Url;

/// Mock LSP client that captures all server communications
#[derive(Clone)]
pub struct MockLspClient {
    /// Captured log messages
    pub logs: Arc<Mutex<Vec<LogMessage>>>,
    /// Captured show message requests
    pub messages: Arc<Mutex<Vec<ShowMessage>>>,
    /// Captured diagnostics
    pub diagnostics: Arc<Mutex<HashMap<Url, Vec<Diagnostic>>>>,
    /// Configuration for mock behavior
    pub config: MockClientConfig,
}

/// Configuration for mock client behavior
#[derive(Clone, Debug)]
pub struct MockClientConfig {
    /// Whether to print messages to stdout
    pub verbose: bool,
    /// Whether to simulate slow client responses
    pub simulate_latency: bool,
    /// Latency to simulate (if enabled)
    pub latency_ms: u64,
}

impl Default for MockClientConfig {
    fn default() -> Self {
        Self {
            verbose: false,
            simulate_latency: false,
            latency_ms: 10,
        }
    }
}

/// Captured log message
#[derive(Debug, Clone)]
pub struct LogMessage {
    pub message_type: MessageType,
    pub message: String,
    pub timestamp: std::time::Instant,
}

/// Captured show message
#[derive(Debug, Clone)]
pub struct ShowMessage {
    pub message_type: MessageType,
    pub message: String,
    pub actions: Option<Vec<MessageActionItem>>,
    pub timestamp: std::time::Instant,
}

impl MockLspClient {
    /// Create a new mock client with default configuration
    pub fn new() -> Self {
        Self::with_config(MockClientConfig::default())
    }

    /// Create a new mock client with custom configuration
    pub fn with_config(config: MockClientConfig) -> Self {
        Self {
            logs: Arc::new(Mutex::new(Vec::new())),
            messages: Arc::new(Mutex::new(Vec::new())),
            diagnostics: Arc::new(Mutex::new(HashMap::new())),
            config,
        }
    }

    /// Create a verbose mock client for debugging
    pub fn verbose() -> Self {
        Self::with_config(MockClientConfig {
            verbose: true,
            ..Default::default()
        })
    }

    /// Get all captured log messages
    pub fn get_logs(&self) -> Vec<LogMessage> {
        self.logs.lock().unwrap().clone()
    }

    /// Get all captured show messages
    pub fn get_messages(&self) -> Vec<ShowMessage> {
        self.messages.lock().unwrap().clone()
    }

    /// Get diagnostics for a specific URI
    pub fn get_diagnostics(&self, uri: &Url) -> Option<Vec<Diagnostic>> {
        self.diagnostics.lock().unwrap().get(uri).cloned()
    }

    /// Get all diagnostics
    pub fn get_all_diagnostics(&self) -> HashMap<Url, Vec<Diagnostic>> {
        self.diagnostics.lock().unwrap().clone()
    }

    /// Clear all captured data
    pub fn clear(&self) {
        self.logs.lock().unwrap().clear();
        self.messages.lock().unwrap().clear();
        self.diagnostics.lock().unwrap().clear();
    }

    /// Count messages of a specific type
    pub fn count_log_messages(&self, message_type: MessageType) -> usize {
        self.logs.lock().unwrap()
            .iter()
            .filter(|log| log.message_type == message_type)
            .count()
    }

    /// Check if any log message contains a specific text
    pub fn has_log_message_containing(&self, text: &str) -> bool {
        self.logs.lock().unwrap()
            .iter()
            .any(|log| log.message.contains(text))
    }

    /// Check if any diagnostic has a specific code
    pub fn has_diagnostic_with_code(&self, code: &str) -> bool {
        self.diagnostics.lock().unwrap()
            .values()
            .flatten()
            .any(|diag| {
                if let Some(NumberOrString::String(diag_code)) = &diag.code {
                    diag_code == code
                } else {
                    false
                }
            })
    }

    /// Get total number of diagnostics across all documents
    pub fn total_diagnostics_count(&self) -> usize {
        self.diagnostics.lock().unwrap()
            .values()
            .map(|diagnostics| diagnostics.len())
            .sum()
    }

    /// Simulate client latency if configured
    async fn simulate_latency(&self) {
        if self.config.simulate_latency {
            tokio::time::sleep(tokio::time::Duration::from_millis(self.config.latency_ms)).await;
        }
    }
}

impl Default for MockLspClient {
    fn default() -> Self {
        Self::new()
    }
}

#[tower_lsp::async_trait]
impl tower_lsp::Client for MockLspClient {
    async fn log_message(&self, typ: MessageType, message: String) {
        self.simulate_latency().await;

        let log_message = LogMessage {
            message_type: typ,
            message: message.clone(),
            timestamp: std::time::Instant::now(),
        };

        if self.config.verbose {
            println!("[MOCK CLIENT LOG {:?}] {}", typ, message);
        }

        self.logs.lock().unwrap().push(log_message);
    }

    async fn show_message(&self, typ: MessageType, message: String) {
        self.simulate_latency().await;

        let show_message = ShowMessage {
            message_type: typ,
            message: message.clone(),
            actions: None,
            timestamp: std::time::Instant::now(),
        };

        if self.config.verbose {
            println!("[MOCK CLIENT SHOW {:?}] {}", typ, message);
        }

        self.messages.lock().unwrap().push(show_message);
    }

    async fn show_message_request(
        &self,
        typ: MessageType,
        message: String,
        actions: Option<Vec<MessageActionItem>>,
    ) -> tower_lsp::jsonrpc::Result<Option<MessageActionItem>> {
        self.simulate_latency().await;

        let show_message = ShowMessage {
            message_type: typ,
            message: message.clone(),
            actions: actions.clone(),
            timestamp: std::time::Instant::now(),
        };

        if self.config.verbose {
            println!("[MOCK CLIENT REQUEST {:?}] {} (actions: {:?})", typ, message, actions);
        }

        self.messages.lock().unwrap().push(show_message);

        // Return the first action if available (simulating user clicking first option)
        Ok(actions.and_then(|mut actions| actions.pop()))
    }

    async fn publish_diagnostics(&self, uri: Url, diagnostics: Vec<Diagnostic>, version: Option<i32>) {
        self.simulate_latency().await;

        if self.config.verbose {
            println!("[MOCK CLIENT DIAGNOSTICS] {} diagnostics for {} (version: {:?})", 
                    diagnostics.len(), uri, version);
            for diagnostic in &diagnostics {
                println!("  {:?} at {}:{}-{}:{}: {}", 
                        diagnostic.severity.unwrap_or(DiagnosticSeverity::INFORMATION),
                        diagnostic.range.start.line,
                        diagnostic.range.start.character,
                        diagnostic.range.end.line,
                        diagnostic.range.end.character,
                        diagnostic.message);
            }
        }

        self.diagnostics.lock().unwrap().insert(uri, diagnostics);
    }
}

/// Builder for creating mock clients with specific configurations
pub struct MockClientBuilder {
    config: MockClientConfig,
}

impl MockClientBuilder {
    pub fn new() -> Self {
        Self {
            config: MockClientConfig::default(),
        }
    }

    pub fn verbose(mut self, verbose: bool) -> Self {
        self.config.verbose = verbose;
        self
    }

    pub fn with_latency(mut self, latency_ms: u64) -> Self {
        self.config.simulate_latency = true;
        self.config.latency_ms = latency_ms;
        self
    }

    pub fn build(self) -> MockLspClient {
        MockLspClient::with_config(self.config)
    }
}

impl Default for MockClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Test utilities for working with mock clients
pub mod test_utils {
    use super::*;
    use std::time::Duration;

    /// Wait for a specific number of diagnostics to be published
    pub async fn wait_for_diagnostics(client: &MockLspClient, uri: &Url, expected_count: usize, timeout: Duration) -> Result<Vec<Diagnostic>, &'static str> {
        let start = std::time::Instant::now();
        
        loop {
            if let Some(diagnostics) = client.get_diagnostics(uri) {
                if diagnostics.len() >= expected_count {
                    return Ok(diagnostics);
                }
            }
            
            if start.elapsed() > timeout {
                return Err("Timeout waiting for diagnostics");
            }
            
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    /// Wait for a log message containing specific text
    pub async fn wait_for_log_message(client: &MockLspClient, text: &str, timeout: Duration) -> Result<LogMessage, &'static str> {
        let start = std::time::Instant::now();
        
        loop {
            let logs = client.get_logs();
            if let Some(log) = logs.iter().find(|log| log.message.contains(text)) {
                return Ok(log.clone());
            }
            
            if start.elapsed() > timeout {
                return Err("Timeout waiting for log message");
            }
            
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    }

    /// Assert that diagnostics contain expected codes
    pub fn assert_diagnostics_contain_codes(diagnostics: &[Diagnostic], expected_codes: &[&str]) {
        let diagnostic_codes: Vec<String> = diagnostics
            .iter()
            .filter_map(|diag| {
                if let Some(NumberOrString::String(code)) = &diag.code {
                    Some(code.clone())
                } else {
                    None
                }
            })
            .collect();

        for expected_code in expected_codes {
            assert!(
                diagnostic_codes.iter().any(|code| code == expected_code),
                "Expected diagnostic code '{}' not found. Available codes: {:?}",
                expected_code,
                diagnostic_codes
            );
        }
    }

    /// Assert that diagnostics have expected severity levels
    pub fn assert_diagnostics_have_severities(diagnostics: &[Diagnostic], expected_severities: &[DiagnosticSeverity]) {
        let severities: Vec<DiagnosticSeverity> = diagnostics
            .iter()
            .filter_map(|diag| diag.severity)
            .collect();

        for expected_severity in expected_severities {
            assert!(
                severities.contains(expected_severity),
                "Expected diagnostic severity {:?} not found. Available severities: {:?}",
                expected_severity,
                severities
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_client_creation() {
        let client = MockLspClient::new();
        assert_eq!(client.get_logs().len(), 0);
        assert_eq!(client.get_messages().len(), 0);
        assert_eq!(client.total_diagnostics_count(), 0);
    }

    #[test]
    fn test_mock_client_builder() {
        let client = MockClientBuilder::new()
            .verbose(true)
            .with_latency(50)
            .build();

        assert!(client.config.verbose);
        assert!(client.config.simulate_latency);
        assert_eq!(client.config.latency_ms, 50);
    }

    #[tokio::test]
    async fn test_mock_client_log_capture() {
        let client = MockLspClient::new();
        
        client.log_message(MessageType::INFO, "Test message".to_string()).await;
        client.log_message(MessageType::ERROR, "Error message".to_string()).await;

        let logs = client.get_logs();
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0].message_type, MessageType::INFO);
        assert_eq!(logs[0].message, "Test message");
        assert_eq!(logs[1].message_type, MessageType::ERROR);
        assert_eq!(logs[1].message, "Error message");
    }

    #[tokio::test]
    async fn test_mock_client_diagnostics_capture() {
        let client = MockLspClient::new();
        let uri = Url::parse("file:///test.md").unwrap();
        
        let diagnostics = vec![
            Diagnostic {
                range: Range {
                    start: Position { line: 0, character: 0 },
                    end: Position { line: 0, character: 10 },
                },
                severity: Some(DiagnosticSeverity::ERROR),
                code: Some(NumberOrString::String("test-error".to_string())),
                source: Some("photon-drift".to_string()),
                message: "Test diagnostic".to_string(),
                related_information: None,
                tags: None,
                data: None,
                code_description: None,
            }
        ];

        client.publish_diagnostics(uri.clone(), diagnostics.clone(), Some(1)).await;

        let captured_diagnostics = client.get_diagnostics(&uri).unwrap();
        assert_eq!(captured_diagnostics.len(), 1);
        assert_eq!(captured_diagnostics[0].message, "Test diagnostic");
        assert!(client.has_diagnostic_with_code("test-error"));
    }

    #[tokio::test]
    async fn test_mock_client_utility_methods() {
        let client = MockLspClient::new();
        
        client.log_message(MessageType::INFO, "Info message".to_string()).await;
        client.log_message(MessageType::ERROR, "Error message".to_string()).await;
        client.log_message(MessageType::INFO, "Another info".to_string()).await;

        assert_eq!(client.count_log_messages(MessageType::INFO), 2);
        assert_eq!(client.count_log_messages(MessageType::ERROR), 1);
        assert!(client.has_log_message_containing("Error"));
        assert!(!client.has_log_message_containing("Warning"));

        client.clear();
        assert_eq!(client.get_logs().len(), 0);
    }
}