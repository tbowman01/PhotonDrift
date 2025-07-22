//! Plugin System for PhotonDrift
//!
//! This module provides a comprehensive plugin architecture for extending PhotonDrift
//! with IDE integrations, custom drift analysis rules, and third-party extensions.

pub mod manager;
pub mod traits;
pub mod security;
pub mod marketplace;
pub mod ide;

pub use manager::{PluginManager, PluginLoadError};
pub use traits::{
    Plugin, DriftAnalysisPlugin, IDEIntegrationPlugin, TemplatePlugin,
    PluginCapability, PluginMetadata, PluginContext
};
pub use security::{PluginValidator, SecurityPolicy, SandboxManager};
pub use marketplace::{MarketplaceClient, PluginPackage, PluginRegistry};

/// Plugin system result type
pub type PluginResult<T> = std::result::Result<T, PluginLoadError>;

/// Plugin API version
pub const PLUGIN_API_VERSION: &str = "1.0.0";

/// Maximum plugin execution time in milliseconds
pub const MAX_PLUGIN_EXECUTION_TIME_MS: u64 = 5000;

/// Maximum memory allocation per plugin in bytes
pub const MAX_PLUGIN_MEMORY_BYTES: usize = 50 * 1024 * 1024; // 50MB