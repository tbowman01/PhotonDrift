//! Plugin trait definitions for the PhotonDrift plugin system

use crate::{AdrscanError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

/// Core plugin trait that all plugins must implement
pub trait Plugin: Send + Sync {
    /// Initialize the plugin with the given context
    fn initialize(&mut self, context: &PluginContext) -> Result<()>;

    /// Get plugin metadata
    fn metadata(&self) -> &PluginMetadata;

    /// Get plugin capabilities
    fn capabilities(&self) -> Vec<PluginCapability>;

    /// Execute a plugin command with the given parameters
    fn execute(&self, command: &str, params: &HashMap<String, String>) -> Result<PluginResponse>;

    /// Cleanup plugin resources
    fn shutdown(&mut self) -> Result<()>;

    /// Check if plugin is compatible with the current system
    fn is_compatible(&self) -> bool {
        true
    }

    /// Get plugin configuration schema
    fn config_schema(&self) -> Option<serde_json::Value> {
        None
    }
}

/// Plugin for custom drift analysis rules
pub trait DriftAnalysisPlugin: Plugin {
    /// Analyze drift in the given ADR content
    fn analyze_drift(
        &self,
        adr_content: &str,
        context: &DriftAnalysisContext,
    ) -> Result<Vec<DriftAlert>>;

    /// Get supported drift pattern types
    fn supported_patterns(&self) -> Vec<String>;

    /// Validate drift analysis configuration
    fn validate_config(&self, config: &serde_json::Value) -> Result<()>;
}

/// Plugin for IDE-specific functionality
pub trait IDEIntegrationPlugin: Plugin {
    /// Get supported IDE type
    fn ide_type(&self) -> IDEType;

    /// Initialize IDE-specific features
    fn setup_ide_integration(&self, config: &IDEConfig) -> Result<()>;

    /// Handle IDE events
    fn handle_ide_event(&self, event: &IDEEvent) -> Result<IDEResponse>;

    /// Get IDE-specific configuration
    fn get_ide_config(&self) -> IDEConfig;

    /// Register IDE commands
    fn register_commands(&self) -> Vec<IDECommand>;
}

/// Plugin for custom ADR templates
pub trait TemplatePlugin: Plugin {
    /// Get available templates
    fn get_templates(&self) -> Vec<TemplateInfo>;

    /// Render template with given variables
    fn render_template(
        &self,
        template_id: &str,
        variables: &HashMap<String, String>,
    ) -> Result<String>;

    /// Validate template variables
    fn validate_template_variables(
        &self,
        template_id: &str,
        variables: &HashMap<String, String>,
    ) -> Result<()>;

    /// Get template schema
    fn get_template_schema(&self, template_id: &str) -> Result<serde_json::Value>;
}

/// Plugin metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub license: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub keywords: Vec<String>,
    pub api_version: String,
    pub min_adrscan_version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Plugin capabilities that define what the plugin can do
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginCapability {
    /// Analyze drift in ADRs
    DriftAnalysis,
    /// Integrate with IDEs
    IDEIntegration,
    /// Provide custom templates
    TemplateProvider,
    /// Extend CLI commands
    CommandExtension,
    /// Process file events
    FileWatcher,
    /// Network access for external APIs
    NetworkAccess,
    /// File system read access
    FileSystemRead,
    /// File system write access
    FileSystemWrite,
    /// Execute system commands
    SystemExecution,
    /// Custom capability with name
    Custom(String),
}

/// Plugin execution context
#[derive(Debug, Clone)]
pub struct PluginContext {
    pub plugin_dir: std::path::PathBuf,
    pub config_dir: std::path::PathBuf,
    pub work_dir: std::path::PathBuf,
    pub adrscan_version: String,
    pub api_version: String,
    pub environment: HashMap<String, String>,
}

/// Response from plugin execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginResponse {
    pub success: bool,
    pub data: Option<serde_json::Value>,
    pub message: Option<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Drift analysis context for drift analysis plugins
#[derive(Debug, Clone)]
pub struct DriftAnalysisContext {
    pub file_path: std::path::PathBuf,
    pub project_root: std::path::PathBuf,
    pub previous_analysis: Option<serde_json::Value>,
    pub config: serde_json::Value,
}

/// Drift alert from analysis plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftAlert {
    pub severity: AlertSeverity,
    pub pattern: String,
    pub message: String,
    pub location: Option<TextLocation>,
    pub suggestions: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Text location for alerts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextLocation {
    pub line: u32,
    pub column: u32,
    pub length: u32,
}

/// Supported IDE types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IDEType {
    VSCode,
    IntelliJ,
    Vim,
    Emacs,
    Sublime,
    Atom,
    Universal, // LSP-based
}

/// IDE configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDEConfig {
    pub ide_type: IDEType,
    pub version: Option<String>,
    pub installation_path: Option<std::path::PathBuf>,
    pub settings: HashMap<String, serde_json::Value>,
}

/// IDE events that plugins can handle
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IDEEvent {
    FileOpened {
        path: std::path::PathBuf,
    },
    FileSaved {
        path: std::path::PathBuf,
    },
    FileModified {
        path: std::path::PathBuf,
        changes: Vec<TextChange>,
    },
    ProjectOpened {
        root: std::path::PathBuf,
    },
    ProjectClosed {
        root: std::path::PathBuf,
    },
    Command {
        name: String,
        args: Vec<String>,
    },
    Custom {
        event_type: String,
        data: serde_json::Value,
    },
}

/// Text changes in IDE events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextChange {
    pub range: TextRange,
    pub new_text: String,
}

/// Text range for IDE operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRange {
    pub start: TextPosition,
    pub end: TextPosition,
}

/// Text position for IDE operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextPosition {
    pub line: u32,
    pub character: u32,
}

/// IDE response from plugin
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDEResponse {
    pub handled: bool,
    pub actions: Vec<IDEAction>,
    pub diagnostics: Vec<IDEDiagnostic>,
}

/// IDE actions that plugins can request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IDEAction {
    ShowMessage {
        level: MessageLevel,
        message: String,
    },
    ShowProgress {
        title: String,
        message: String,
        percentage: Option<u32>,
    },
    OpenFile {
        path: std::path::PathBuf,
        line: Option<u32>,
    },
    InsertText {
        path: std::path::PathBuf,
        position: TextPosition,
        text: String,
    },
    ReplaceText {
        path: std::path::PathBuf,
        range: TextRange,
        text: String,
    },
    ExecuteCommand {
        command: String,
        args: Vec<String>,
    },
    Custom {
        action_type: String,
        data: serde_json::Value,
    },
}

/// Message levels for IDE messages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageLevel {
    Info,
    Warning,
    Error,
}

/// IDE diagnostics from plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDEDiagnostic {
    pub range: TextRange,
    pub severity: DiagnosticSeverity,
    pub message: String,
    pub source: String,
    pub code: Option<String>,
}

/// Diagnostic severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiagnosticSeverity {
    Error,
    Warning,
    Information,
    Hint,
}

/// IDE command that can be registered by plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDECommand {
    pub id: String,
    pub title: String,
    pub category: String,
    pub description: Option<String>,
    pub arguments: Vec<CommandArgument>,
}

/// Command argument definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandArgument {
    pub name: String,
    pub arg_type: ArgumentType,
    pub description: Option<String>,
    pub required: bool,
    pub default_value: Option<String>,
}

/// Command argument types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ArgumentType {
    String,
    Integer,
    Boolean,
    File,
    Directory,
    Choice(Vec<String>),
}

/// Template information for template plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub variables: Vec<TemplateVariable>,
    pub preview: Option<String>,
}

/// Template variable definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub name: String,
    pub description: String,
    pub var_type: VariableType,
    pub required: bool,
    pub default_value: Option<String>,
    pub validation: Option<VariableValidation>,
}

/// Template variable types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VariableType {
    String,
    Integer,
    Boolean,
    Date,
    Choice(Vec<String>),
    Array(Box<VariableType>),
}

/// Variable validation rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableValidation {
    pub pattern: Option<String>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub min_value: Option<i64>,
    pub max_value: Option<i64>,
}

// Additional missing types that are imported in other modules

/// Supported IDE types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum IDEType {
    VSCode,
    IntelliJ,
    Vim,
    Emacs,
    Universal,
    Other(String),
}

/// IDE configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDEConfig {
    pub ide_type: IDEType,
    pub version: Option<String>,
    pub installation_path: Option<PathBuf>,
    pub settings: serde_json::Map<String, Value>,
}

/// IDE event data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IDEEvent {
    FileOpened { path: PathBuf },
    FileSaved { path: PathBuf },
    FileClosing { path: PathBuf },
    SelectionChanged { range: TextRange },
    CursorMoved { position: TextPosition },
    ProjectOpened { path: PathBuf },
    ProjectClosed { path: PathBuf },
    Custom { event_type: String, data: Value },
}

/// IDE response to events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IDEResponse {
    Success { message: Option<String> },
    Error { error: String },
    ShowMessage { message: String, level: MessageLevel },
    OpenFile { path: PathBuf, line: Option<u32> },
    ShowDiagnostics { diagnostics: Vec<IDEDiagnostic> },
    Custom { response_type: String, data: Value },
}

/// Text position in a document
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct TextPosition {
    pub line: u32,
    pub column: u32,
}

/// Text range in a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRange {
    pub start: TextPosition,
    pub end: TextPosition,
}

/// Installation options for plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationOptions {
    pub verify_signature: bool,
    pub install_dependencies: bool,
    pub allow_prerelease: bool,
    pub force_reinstall: bool,
}

impl Default for InstallationOptions {
    fn default() -> Self {
        Self {
            verify_signature: true,
            install_dependencies: true,
            allow_prerelease: false,
            force_reinstall: false,
        }
    }
}

/// Search criteria for plugin marketplace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCriteria {
    pub query: Option<String>,
    pub category: Option<String>,
    pub min_rating: Option<f32>,
    pub sort_by: SortBy,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Default for SearchCriteria {
    fn default() -> Self {
        Self {
            query: None,
            category: None,
            min_rating: None,
            sort_by: SortBy::Relevance,
            limit: Some(20),
            offset: None,
        }
    }
}

/// Sort criteria for search results
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SortBy {
    Relevance,
    Rating,
    Downloads,
    Updated,
    Name,
    Size,
}
