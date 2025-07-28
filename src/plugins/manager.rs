//! Plugin manager for loading, validating, and managing plugins

use crate::plugins::{
    Plugin, PluginCapability, PluginContext, PluginMetadata, PluginResponse, PluginValidator,
    SandboxManager, SecurityPolicy, MAX_PLUGIN_EXECUTION_TIME_MS, MAX_PLUGIN_MEMORY_BYTES,
    PLUGIN_API_VERSION,
};
use crate::{AdrscanError, Result};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use thiserror::Error;

/// Plugin manager for handling plugin lifecycle
#[derive(Debug)]
pub struct PluginManager {
    plugins: Arc<Mutex<HashMap<String, Box<dyn Plugin>>>>,
    plugin_configs: HashMap<String, PluginConfig>,
    security_policy: SecurityPolicy,
    validator: PluginValidator,
    sandbox_manager: SandboxManager,
    plugin_directory: PathBuf,
    context: PluginContext,
}

/// Plugin loading and execution errors
#[derive(Error, Debug)]
pub enum PluginLoadError {
    #[error("Plugin not found: {0}")]
    NotFound(String),

    #[error("Plugin validation failed: {0}")]
    ValidationFailed(String),

    #[error("Plugin security check failed: {0}")]
    SecurityCheckFailed(String),

    #[error("Plugin API version mismatch: expected {expected}, got {actual}")]
    ApiVersionMismatch { expected: String, actual: String },

    #[error("Plugin initialization failed: {0}")]
    InitializationFailed(String),

    #[error("Plugin execution timeout after {0}ms")]
    ExecutionTimeout(u64),

    #[error("Plugin memory limit exceeded: {0} bytes")]
    MemoryLimitExceeded(usize),

    #[error("Plugin capability not allowed: {0:?}")]
    CapabilityNotAllowed(PluginCapability),

    #[error("Plugin dependency missing: {0}")]
    DependencyMissing(String),

    #[error("Plugin configuration error: {0}")]
    ConfigurationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Plugin configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginConfig {
    pub enabled: bool,
    pub auto_load: bool,
    pub security_level: SecurityLevel,
    pub allowed_capabilities: Vec<PluginCapability>,
    pub resource_limits: ResourceLimits,
    pub config: serde_json::Value,
}

/// Security levels for plugins
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SecurityLevel {
    /// Minimal security, full trust
    Trusted,
    /// Standard security with common restrictions
    Standard,
    /// High security with strict sandboxing
    Restricted,
    /// Maximum security, very limited capabilities
    Sandboxed,
}

/// Resource limits for plugin execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_bytes: usize,
    pub max_execution_time_ms: u64,
    pub max_file_operations: u32,
    pub max_network_requests: u32,
}

/// Plugin execution statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginStats {
    pub load_count: u32,
    pub execution_count: u32,
    pub total_execution_time_ms: u64,
    pub average_execution_time_ms: f64,
    pub error_count: u32,
    pub last_executed: Option<chrono::DateTime<chrono::Utc>>,
}

impl PluginManager {
    /// Create a new plugin manager
    pub fn new<P: AsRef<Path>>(
        plugin_directory: P,
        context: PluginContext,
        security_policy: SecurityPolicy,
    ) -> Result<Self> {
        let plugin_dir = plugin_directory.as_ref().to_path_buf();

        // Create plugin directory if it doesn't exist
        if !plugin_dir.exists() {
            std::fs::create_dir_all(&plugin_dir)?;
        }

        Ok(Self {
            plugins: Arc::new(Mutex::new(HashMap::new())),
            plugin_configs: HashMap::new(),
            security_policy: security_policy.clone(),
            validator: PluginValidator::new(security_policy.clone()),
            sandbox_manager: SandboxManager::new(),
            plugin_directory: plugin_dir,
            context,
        })
    }

    /// Load a plugin from the specified path
    pub fn load_plugin<P: AsRef<Path>>(
        &mut self,
        plugin_path: P,
    ) -> Result<String, PluginLoadError> {
        let path = plugin_path.as_ref();
        debug!("Loading plugin from: {}", path.display());

        // Validate plugin file exists
        if !path.exists() {
            return Err(PluginLoadError::NotFound(path.display().to_string()));
        }

        // Load plugin metadata
        let metadata = self.load_plugin_metadata(path)?;
        let plugin_id = metadata.id.clone();

        // Validate API version compatibility
        if metadata.api_version != PLUGIN_API_VERSION {
            return Err(PluginLoadError::ApiVersionMismatch {
                expected: PLUGIN_API_VERSION.to_string(),
                actual: metadata.api_version,
            });
        }

        // Get plugin configuration
        let config = self.get_plugin_config(&plugin_id);

        // Security validation
        self.validator.validate_plugin(path, &metadata, &config)?;

        // Load and initialize plugin
        let mut plugin = self.load_plugin_binary(path, metadata)?;

        // Initialize plugin in sandbox if required
        if config.security_level == SecurityLevel::Sandboxed {
            self.sandbox_manager.initialize_sandbox(&plugin_id)?;
        }

        // Initialize plugin
        plugin
            .initialize(&self.context)
            .map_err(|e| PluginLoadError::InitializationFailed(e.to_string()))?;

        // Verify plugin capabilities are allowed
        let capabilities = plugin.capabilities();
        for capability in &capabilities {
            if !config.allowed_capabilities.contains(capability) {
                return Err(PluginLoadError::CapabilityNotAllowed(capability.clone()));
            }
        }

        // Store plugin
        {
            let mut plugins = self.plugins.lock().unwrap();
            plugins.insert(plugin_id.clone(), plugin);
        }

        info!("Successfully loaded plugin: {}", plugin_id);
        Ok(plugin_id)
    }

    /// Unload a plugin
    pub fn unload_plugin(&mut self, plugin_id: &str) -> Result<(), PluginLoadError> {
        debug!("Unloading plugin: {}", plugin_id);

        let plugin = {
            let mut plugins = self.plugins.lock().unwrap();
            plugins.remove(plugin_id)
        };

        if let Some(mut plugin) = plugin {
            // Shutdown plugin
            plugin
                .shutdown()
                .map_err(|e| PluginLoadError::ConfigurationError(e.to_string()))?;

            // Cleanup sandbox
            self.sandbox_manager.cleanup_sandbox(plugin_id)?;

            info!("Successfully unloaded plugin: {}", plugin_id);
            Ok(())
        } else {
            Err(PluginLoadError::NotFound(plugin_id.to_string()))
        }
    }

    /// Execute a plugin command
    pub fn execute_plugin_command(
        &self,
        plugin_id: &str,
        command: &str,
        params: &HashMap<String, String>,
    ) -> Result<PluginResponse, PluginLoadError> {
        debug!("Executing plugin command: {} -> {}", plugin_id, command);

        let plugins = self.plugins.lock().unwrap();
        let plugin = plugins
            .get(plugin_id)
            .ok_or_else(|| PluginLoadError::NotFound(plugin_id.to_string()))?;

        let config = self.get_plugin_config(plugin_id);

        // Execute with timeout and resource monitoring
        let start_time = Instant::now();
        let max_duration = Duration::from_millis(config.resource_limits.max_execution_time_ms);

        // TODO: In a real implementation, this would use proper sandboxing and resource monitoring
        let result = plugin
            .execute(command, params)
            .map_err(|e| PluginLoadError::ConfigurationError(e.to_string()))?;

        let execution_time = start_time.elapsed();
        if execution_time > max_duration {
            warn!(
                "Plugin {} execution time exceeded limit: {}ms",
                plugin_id,
                execution_time.as_millis()
            );
            return Err(PluginLoadError::ExecutionTimeout(
                execution_time.as_millis() as u64,
            ));
        }

        debug!(
            "Plugin command executed in {}ms",
            execution_time.as_millis()
        );
        Ok(result)
    }

    /// List all loaded plugins
    pub fn list_plugins(&self) -> Vec<PluginMetadata> {
        let plugins = self.plugins.lock().unwrap();
        plugins
            .values()
            .map(|plugin| plugin.metadata().clone())
            .collect()
    }

    /// Get plugin by ID
    pub fn get_plugin(&self, plugin_id: &str) -> Option<PluginMetadata> {
        let plugins = self.plugins.lock().unwrap();
        plugins
            .get(plugin_id)
            .map(|plugin| plugin.metadata().clone())
    }

    /// Discover plugins in the plugin directory
    pub fn discover_plugins(&self) -> Result<Vec<PathBuf>, PluginLoadError> {
        let mut plugin_paths = Vec::new();

        if !self.plugin_directory.exists() {
            return Ok(plugin_paths);
        }

        for entry in std::fs::read_dir(&self.plugin_directory)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && self.is_plugin_file(&path) {
                plugin_paths.push(path);
            } else if path.is_dir() {
                // Look for plugin manifest in subdirectory
                let manifest_path = path.join("plugin.json");
                if manifest_path.exists() {
                    plugin_paths.push(manifest_path);
                }
            }
        }

        Ok(plugin_paths)
    }

    /// Auto-load plugins marked for auto-loading
    pub fn auto_load_plugins(&mut self) -> Result<Vec<String>, PluginLoadError> {
        let plugin_paths = self.discover_plugins()?;
        let mut loaded_plugins = Vec::new();

        for path in plugin_paths {
            match self.should_auto_load(&path) {
                Ok(true) => match self.load_plugin(&path) {
                    Ok(plugin_id) => loaded_plugins.push(plugin_id),
                    Err(e) => {
                        error!("Failed to auto-load plugin from {}: {}", path.display(), e);
                    }
                },
                Ok(false) => {
                    debug!("Skipping auto-load for plugin: {}", path.display());
                }
                Err(e) => {
                    warn!(
                        "Error checking auto-load for plugin {}: {}",
                        path.display(),
                        e
                    );
                }
            }
        }

        info!("Auto-loaded {} plugins", loaded_plugins.len());
        Ok(loaded_plugins)
    }

    /// Update plugin configuration
    pub fn update_plugin_config(
        &mut self,
        plugin_id: &str,
        config: PluginConfig,
    ) -> Result<(), PluginLoadError> {
        self.plugin_configs.insert(plugin_id.to_string(), config);
        self.save_plugin_configs()?;
        Ok(())
    }

    /// Get plugin statistics
    pub fn get_plugin_stats(&self, plugin_id: &str) -> Option<PluginStats> {
        // TODO: Implement actual statistics tracking
        Some(PluginStats {
            load_count: 1,
            execution_count: 0,
            total_execution_time_ms: 0,
            average_execution_time_ms: 0.0,
            error_count: 0,
            last_executed: None,
        })
    }

    /// Shutdown all plugins
    pub fn shutdown(&mut self) -> Result<(), PluginLoadError> {
        info!("Shutting down plugin manager");

        let plugin_ids: Vec<String> = {
            let plugins = self.plugins.lock().unwrap();
            plugins.keys().cloned().collect()
        };

        for plugin_id in plugin_ids {
            if let Err(e) = self.unload_plugin(&plugin_id) {
                error!("Error unloading plugin {}: {}", plugin_id, e);
            }
        }

        Ok(())
    }

    // Private helper methods

    fn load_plugin_metadata(&self, plugin_path: &Path) -> Result<PluginMetadata, PluginLoadError> {
        // This is a simplified implementation
        // In a real implementation, this would parse actual plugin metadata
        let metadata = PluginMetadata {
            id: plugin_path
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .to_string(),
            name: "Example Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Example plugin description".to_string(),
            author: "Plugin Author".to_string(),
            license: "MIT".to_string(),
            homepage: None,
            repository: None,
            keywords: vec!["adrscan".to_string(), "plugin".to_string()],
            api_version: PLUGIN_API_VERSION.to_string(),
            min_adrscan_version: "0.2.0".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        Ok(metadata)
    }

    fn load_plugin_binary(
        &self,
        _plugin_path: &Path,
        metadata: PluginMetadata,
    ) -> Result<Box<dyn Plugin>, PluginLoadError> {
        // This is a simplified implementation
        // In a real implementation, this would load the actual plugin binary/library
        Ok(Box::new(ExamplePlugin { metadata }))
    }

    fn get_plugin_config(&self, plugin_id: &str) -> PluginConfig {
        self.plugin_configs
            .get(plugin_id)
            .cloned()
            .unwrap_or_else(|| PluginConfig {
                enabled: true,
                auto_load: false,
                security_level: SecurityLevel::Standard,
                allowed_capabilities: vec![
                    PluginCapability::DriftAnalysis,
                    PluginCapability::TemplateProvider,
                    PluginCapability::FileSystemRead,
                ],
                resource_limits: ResourceLimits {
                    max_memory_bytes: MAX_PLUGIN_MEMORY_BYTES,
                    max_execution_time_ms: MAX_PLUGIN_EXECUTION_TIME_MS,
                    max_file_operations: 1000,
                    max_network_requests: 100,
                },
                config: serde_json::Value::Object(serde_json::Map::new()),
            })
    }

    fn is_plugin_file(&self, path: &Path) -> bool {
        if let Some(extension) = path.extension() {
            matches!(
                extension.to_str(),
                Some("dll") | Some("so") | Some("dylib") | Some("wasm")
            )
        } else {
            false
        }
    }

    fn should_auto_load(&self, plugin_path: &Path) -> Result<bool, PluginLoadError> {
        let metadata = self.load_plugin_metadata(plugin_path)?;
        let config = self.get_plugin_config(&metadata.id);
        Ok(config.enabled && config.auto_load)
    }

    fn save_plugin_configs(&self) -> Result<(), PluginLoadError> {
        let config_file = self.plugin_directory.join("plugins.json");
        let json = serde_json::to_string_pretty(&self.plugin_configs)?;
        std::fs::write(config_file, json)?;
        Ok(())
    }
}

// Example plugin implementation for testing
#[derive(Debug)]
struct ExamplePlugin {
    metadata: PluginMetadata,
}

impl Plugin for ExamplePlugin {
    fn initialize(&mut self, _context: &PluginContext) -> Result<()> {
        Ok(())
    }

    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn capabilities(&self) -> Vec<PluginCapability> {
        vec![
            PluginCapability::DriftAnalysis,
            PluginCapability::TemplateProvider,
        ]
    }

    fn execute(&self, command: &str, _params: &HashMap<String, String>) -> Result<PluginResponse> {
        Ok(PluginResponse {
            success: true,
            data: Some(serde_json::json!({
                "command": command,
                "plugin": self.metadata.name
            })),
            message: Some(format!("Executed command: {}", command)),
            warnings: vec![],
            errors: vec![],
        })
    }

    fn shutdown(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_bytes: MAX_PLUGIN_MEMORY_BYTES,
            max_execution_time_ms: MAX_PLUGIN_EXECUTION_TIME_MS,
            max_file_operations: 1000,
            max_network_requests: 100,
        }
    }
}

impl Default for SecurityLevel {
    fn default() -> Self {
        SecurityLevel::Standard
    }
}
