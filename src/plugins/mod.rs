//! Plugin System for PhotonDrift
//!
//! This module provides a comprehensive plugin architecture for extending PhotonDrift
//! with IDE integrations, custom drift analysis rules, and third-party extensions.

pub mod ide;
pub mod manager;
pub mod marketplace;
pub mod security;
pub mod traits;

pub use manager::{PluginConfig, PluginLoadError, PluginManager, SecurityLevel};
pub use marketplace::{
    InstallationOptions, MarketplaceClient, PluginPackage, PluginRegistry, SearchCriteria, SortBy,
};
pub use security::{PluginValidator, SandboxManager, SecurityPolicy};
pub use traits::{
    ArgumentType, CommandArgument, DiagnosticSeverity, DriftAnalysisPlugin, IDEAction, IDECommand,
    IDEConfig, IDEDiagnostic, IDEEvent, IDEIntegrationPlugin, IDEResponse, IDEType, MessageLevel,
    Plugin, PluginCapability, PluginContext, PluginMetadata, PluginResponse, TemplatePlugin,
    TextPosition, TextRange,
};

/// Plugin system result type
pub type PluginResult<T> = std::result::Result<T, PluginLoadError>;

/// Plugin API version
pub const PLUGIN_API_VERSION: &str = "1.0.0";

/// Maximum plugin execution time in milliseconds
pub const MAX_PLUGIN_EXECUTION_TIME_MS: u64 = 5000;

/// Maximum memory allocation per plugin in bytes
pub const MAX_PLUGIN_MEMORY_BYTES: usize = 50 * 1024 * 1024; // 50MB
