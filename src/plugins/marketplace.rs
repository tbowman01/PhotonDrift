//! Plugin marketplace integration for discovering and installing plugins

use crate::plugins::{PluginLoadError, PluginMetadata, SecurityPolicy};
use crate::{AdrscanError, Result};
use chrono::{DateTime, Utc};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// Plugin marketplace client for discovering and installing plugins
#[derive(Debug)]
pub struct MarketplaceClient {
    base_url: String,
    api_key: Option<String>,
    cache_dir: PathBuf,
    security_policy: SecurityPolicy,
    registries: Vec<PluginRegistry>,
}

/// Plugin package information from marketplace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginPackage {
    pub metadata: PluginMetadata,
    pub download_url: String,
    pub signature_url: Option<String>,
    pub checksum: String,
    pub size_bytes: u64,
    pub download_count: u64,
    pub rating: f32,
    pub tags: Vec<String>,
    pub dependencies: Vec<PluginDependency>,
    pub compatibility: CompatibilityInfo,
    pub screenshots: Vec<String>,
    pub documentation_url: Option<String>,
}

/// Plugin dependency information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    pub plugin_id: String,
    pub version_requirement: String,
    pub optional: bool,
    pub reason: Option<String>,
}

/// Plugin compatibility information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityInfo {
    pub min_adrscan_version: String,
    pub max_adrscan_version: Option<String>,
    pub supported_platforms: Vec<String>,
    pub supported_ides: Vec<String>,
    pub required_features: Vec<String>,
}

/// Plugin registry configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginRegistry {
    pub name: String,
    pub url: String,
    pub api_key: Option<String>,
    pub trusted: bool,
    pub enabled: bool,
    pub priority: u32,
    pub last_updated: Option<DateTime<Utc>>,
}

/// Plugin search criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchCriteria {
    pub query: Option<String>,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub min_rating: Option<f32>,
    pub compatible_only: bool,
    pub sort_by: SortBy,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

/// Sort criteria for plugin search
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortBy {
    Relevance,
    Rating,
    Downloads,
    Updated,
    Name,
    Size,
}

/// Plugin installation options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationOptions {
    pub verify_signature: bool,
    pub install_dependencies: bool,
    pub upgrade_existing: bool,
    pub target_directory: Option<PathBuf>,
    pub pre_release: bool,
}

/// Plugin installation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallationResult {
    pub success: bool,
    pub plugin_id: String,
    pub version: String,
    pub install_path: PathBuf,
    pub dependencies_installed: Vec<String>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

/// Plugin marketplace statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketplaceStats {
    pub total_plugins: u64,
    pub total_downloads: u64,
    pub average_rating: f32,
    pub categories: HashMap<String, u64>,
    pub top_plugins: Vec<PluginPackage>,
    pub recent_updates: Vec<PluginPackage>,
}

impl MarketplaceClient {
    /// Create a new marketplace client
    pub fn new<P: AsRef<Path>>(
        base_url: String,
        cache_dir: P,
        security_policy: SecurityPolicy,
    ) -> Result<Self> {
        let cache_path = cache_dir.as_ref().to_path_buf();

        // Create cache directory if it doesn't exist
        if !cache_path.exists() {
            std::fs::create_dir_all(&cache_path)?;
        }

        let default_registries = vec![
            PluginRegistry {
                name: "Official PhotonDrift Registry".to_string(),
                url: "https://plugins.photondrift.io/api/v1".to_string(),
                api_key: None,
                trusted: true,
                enabled: true,
                priority: 1,
                last_updated: None,
            },
            PluginRegistry {
                name: "Community Registry".to_string(),
                url: "https://community-plugins.photondrift.io/api/v1".to_string(),
                api_key: None,
                trusted: false,
                enabled: true,
                priority: 2,
                last_updated: None,
            },
        ];

        Ok(Self {
            base_url,
            api_key: None,
            cache_dir: cache_path,
            security_policy,
            registries: default_registries,
        })
    }

    /// Search for plugins in the marketplace
    pub async fn search_plugins(
        &self,
        criteria: &SearchCriteria,
    ) -> Result<Vec<PluginPackage>, PluginLoadError> {
        debug!("Searching plugins with criteria: {:?}", criteria);

        let mut all_results = Vec::new();

        for registry in &self.registries {
            if !registry.enabled {
                continue;
            }

            match self.search_in_registry(registry, criteria).await {
                Ok(mut results) => {
                    // Filter results based on security policy if not from trusted registry
                    if !registry.trusted {
                        results = self.filter_by_security_policy(results)?;
                    }
                    all_results.extend(results);
                }
                Err(e) => {
                    warn!("Failed to search in registry {}: {}", registry.name, e);
                }
            }
        }

        // Sort and limit results
        self.sort_and_limit_results(all_results, criteria)
    }

    /// Get detailed information about a specific plugin
    pub async fn get_plugin_details(
        &self,
        plugin_id: &str,
    ) -> Result<PluginPackage, PluginLoadError> {
        debug!("Getting details for plugin: {}", plugin_id);

        // Check cache first
        if let Ok(cached) = self.get_cached_plugin(plugin_id) {
            return Ok(cached);
        }

        // Search in registries
        for registry in &self.registries {
            if !registry.enabled {
                continue;
            }

            match self.get_plugin_from_registry(registry, plugin_id).await {
                Ok(plugin) => {
                    self.cache_plugin(&plugin)?;
                    return Ok(plugin);
                }
                Err(e) => {
                    debug!(
                        "Plugin {} not found in registry {}: {}",
                        plugin_id, registry.name, e
                    );
                }
            }
        }

        Err(PluginLoadError::NotFound(plugin_id.to_string()))
    }

    /// Install a plugin from the marketplace
    pub async fn install_plugin(
        &self,
        plugin_id: &str,
        version: Option<&str>,
        options: &InstallationOptions,
    ) -> Result<InstallationResult, PluginLoadError> {
        info!("Installing plugin: {} (version: {:?})", plugin_id, version);

        let plugin = self.get_plugin_details(plugin_id).await?;

        // Check compatibility
        if !self.is_plugin_compatible(&plugin)? {
            return Ok(InstallationResult {
                success: false,
                plugin_id: plugin_id.to_string(),
                version: plugin.metadata.version.clone(),
                install_path: PathBuf::new(),
                dependencies_installed: vec![],
                warnings: vec![],
                errors: vec!["Plugin is not compatible with current system".to_string()],
            });
        }

        let mut result = InstallationResult {
            success: false,
            plugin_id: plugin_id.to_string(),
            version: plugin.metadata.version.clone(),
            install_path: PathBuf::new(),
            dependencies_installed: vec![],
            warnings: vec![],
            errors: vec![],
        };

        // Install dependencies if requested
        if options.install_dependencies {
            for dep in &plugin.dependencies {
                if dep.optional {
                    continue;
                }

                match self.install_plugin(&dep.plugin_id, None, options).await {
                    Ok(dep_result) => {
                        if dep_result.success {
                            result.dependencies_installed.push(dep.plugin_id.clone());
                        } else {
                            result.errors.extend(dep_result.errors);
                            result
                                .warnings
                                .push(format!("Failed to install dependency: {}", dep.plugin_id));
                        }
                    }
                    Err(e) => {
                        result.errors.push(format!(
                            "Failed to install dependency {}: {}",
                            dep.plugin_id, e
                        ));
                    }
                }
            }
        }

        // Download and install the plugin
        match self.download_and_install(&plugin, options).await {
            Ok(install_path) => {
                result.success = true;
                result.install_path = install_path;
                info!("Successfully installed plugin: {}", plugin_id);
            }
            Err(e) => {
                result.errors.push(format!("Installation failed: {}", e));
                error!("Failed to install plugin {}: {}", plugin_id, e);
            }
        }

        Ok(result)
    }

    /// Uninstall a plugin
    pub async fn uninstall_plugin(&self, plugin_id: &str) -> Result<bool, PluginLoadError> {
        info!("Uninstalling plugin: {}", plugin_id);

        // Find plugin installation
        let plugin_dir = self.find_plugin_installation(plugin_id)?;

        // Remove plugin files
        std::fs::remove_dir_all(&plugin_dir).map_err(|e| PluginLoadError::IoError(e))?;

        // Clear cache
        self.clear_plugin_cache(plugin_id)?;

        info!("Successfully uninstalled plugin: {}", plugin_id);
        Ok(true)
    }

    /// Update a plugin to the latest version
    pub async fn update_plugin(
        &self,
        plugin_id: &str,
        options: &InstallationOptions,
    ) -> Result<InstallationResult, PluginLoadError> {
        info!("Updating plugin: {}", plugin_id);

        let latest = self.get_plugin_details(plugin_id).await?;

        // Check if update is needed
        let current_version = self.get_installed_version(plugin_id)?;
        if current_version >= latest.metadata.version {
            return Ok(InstallationResult {
                success: true,
                plugin_id: plugin_id.to_string(),
                version: latest.metadata.version,
                install_path: self.find_plugin_installation(plugin_id)?,
                dependencies_installed: vec![],
                warnings: vec!["Plugin is already up to date".to_string()],
                errors: vec![],
            });
        }

        // Backup current installation
        self.backup_plugin(plugin_id)?;

        // Install latest version
        let mut options = options.clone();
        options.upgrade_existing = true;

        match self.install_plugin(plugin_id, None, &options).await {
            Ok(result) => {
                if result.success {
                    self.cleanup_backup(plugin_id)?;
                } else {
                    self.restore_backup(plugin_id)?;
                }
                Ok(result)
            }
            Err(e) => {
                self.restore_backup(plugin_id)?;
                Err(e)
            }
        }
    }

    /// List installed plugins
    pub fn list_installed(&self) -> Result<Vec<PluginMetadata>, PluginLoadError> {
        let mut installed = Vec::new();

        // Scan plugin directories
        // This is a simplified implementation
        // In a real implementation, this would scan actual plugin installation directories

        Ok(installed)
    }

    /// Get marketplace statistics
    pub async fn get_stats(&self) -> Result<MarketplaceStats, PluginLoadError> {
        debug!("Getting marketplace statistics");

        // This is a simplified implementation
        // In a real implementation, this would fetch actual statistics from registries

        Ok(MarketplaceStats {
            total_plugins: 42,
            total_downloads: 1337,
            average_rating: 4.2,
            categories: HashMap::from([
                ("IDE Integration".to_string(), 15),
                ("Drift Analysis".to_string(), 12),
                ("Templates".to_string(), 8),
                ("Utilities".to_string(), 7),
            ]),
            top_plugins: vec![],
            recent_updates: vec![],
        })
    }

    /// Add a custom registry
    pub fn add_registry(&mut self, registry: PluginRegistry) {
        self.registries.push(registry);
        self.registries.sort_by_key(|r| r.priority);
    }

    /// Remove a registry
    pub fn remove_registry(&mut self, name: &str) -> bool {
        let initial_len = self.registries.len();
        self.registries.retain(|r| r.name != name);
        self.registries.len() < initial_len
    }

    /// Update registry information
    pub async fn update_registries(&mut self) -> Result<(), PluginLoadError> {
        debug!("Updating registry information");

        for registry in &mut self.registries {
            if !registry.enabled {
                continue;
            }

            match self.fetch_registry_info(registry).await {
                Ok(updated_info) => {
                    registry.last_updated = Some(Utc::now());
                    debug!("Updated registry: {}", registry.name);
                }
                Err(e) => {
                    warn!("Failed to update registry {}: {}", registry.name, e);
                }
            }
        }

        Ok(())
    }

    // Private helper methods

    async fn search_in_registry(
        &self,
        _registry: &PluginRegistry,
        _criteria: &SearchCriteria,
    ) -> Result<Vec<PluginPackage>, PluginLoadError> {
        // This is a simplified implementation
        // In a real implementation, this would make HTTP requests to the registry API
        Ok(vec![])
    }

    async fn get_plugin_from_registry(
        &self,
        _registry: &PluginRegistry,
        _plugin_id: &str,
    ) -> Result<PluginPackage, PluginLoadError> {
        // This is a simplified implementation
        // In a real implementation, this would make HTTP requests to the registry API
        Err(PluginLoadError::NotFound("Not implemented".to_string()))
    }

    fn filter_by_security_policy(
        &self,
        plugins: Vec<PluginPackage>,
    ) -> Result<Vec<PluginPackage>, PluginLoadError> {
        // Filter plugins based on security policy
        let filtered: Vec<PluginPackage> = plugins
            .into_iter()
            .filter(|plugin| {
                // Check if author is trusted
                if !self
                    .security_policy
                    .trusted_authors
                    .contains(&plugin.metadata.author)
                {
                    return false;
                }

                // Check size limits
                if plugin.size_bytes > self.security_policy.max_plugin_size_bytes {
                    return false;
                }

                true
            })
            .collect();

        Ok(filtered)
    }

    fn sort_and_limit_results(
        &self,
        mut results: Vec<PluginPackage>,
        criteria: &SearchCriteria,
    ) -> Result<Vec<PluginPackage>, PluginLoadError> {
        // Sort results
        match criteria.sort_by {
            SortBy::Relevance => {
                // Default order (assumed to be relevance from search)
            }
            SortBy::Rating => {
                results.sort_by(|a, b| {
                    b.rating
                        .partial_cmp(&a.rating)
                        .unwrap_or(std::cmp::Ordering::Equal)
                });
            }
            SortBy::Downloads => {
                results.sort_by(|a, b| b.download_count.cmp(&a.download_count));
            }
            SortBy::Updated => {
                results.sort_by(|a, b| b.metadata.updated_at.cmp(&a.metadata.updated_at));
            }
            SortBy::Name => {
                results.sort_by(|a, b| a.metadata.name.cmp(&b.metadata.name));
            }
            SortBy::Size => {
                results.sort_by(|a, b| a.size_bytes.cmp(&b.size_bytes));
            }
        }

        // Apply offset and limit
        if let Some(offset) = criteria.offset {
            if offset as usize >= results.len() {
                return Ok(vec![]);
            }
            results = results.into_iter().skip(offset as usize).collect();
        }

        if let Some(limit) = criteria.limit {
            results.truncate(limit as usize);
        }

        Ok(results)
    }

    fn is_plugin_compatible(&self, _plugin: &PluginPackage) -> Result<bool, PluginLoadError> {
        // Check compatibility with current system
        // This is simplified - in a real implementation, this would check:
        // - ADRScan version compatibility
        // - Platform compatibility
        // - Required features availability
        Ok(true)
    }

    async fn download_and_install(
        &self,
        _plugin: &PluginPackage,
        _options: &InstallationOptions,
    ) -> Result<PathBuf, PluginLoadError> {
        // This is a simplified implementation
        // In a real implementation, this would:
        // 1. Download the plugin file
        // 2. Verify signature if required
        // 3. Extract to installation directory
        // 4. Set up plugin configuration

        let install_path = self.cache_dir.join("plugins").join(&_plugin.metadata.id);
        std::fs::create_dir_all(&install_path)?;

        Ok(install_path)
    }

    fn get_cached_plugin(&self, plugin_id: &str) -> Result<PluginPackage, PluginLoadError> {
        let cache_file = self.cache_dir.join(format!("{}.json", plugin_id));
        if !cache_file.exists() {
            return Err(PluginLoadError::NotFound("Not in cache".to_string()));
        }

        let content = std::fs::read_to_string(cache_file)?;
        let plugin: PluginPackage = serde_json::from_str(&content)?;
        Ok(plugin)
    }

    fn cache_plugin(&self, plugin: &PluginPackage) -> Result<(), PluginLoadError> {
        let cache_file = self.cache_dir.join(format!("{}.json", plugin.metadata.id));
        let content = serde_json::to_string_pretty(plugin)?;
        std::fs::write(cache_file, content)?;
        Ok(())
    }

    fn clear_plugin_cache(&self, plugin_id: &str) -> Result<(), PluginLoadError> {
        let cache_file = self.cache_dir.join(format!("{}.json", plugin_id));
        if cache_file.exists() {
            std::fs::remove_file(cache_file)?;
        }
        Ok(())
    }

    fn find_plugin_installation(&self, plugin_id: &str) -> Result<PathBuf, PluginLoadError> {
        let plugin_dir = self.cache_dir.join("plugins").join(plugin_id);
        if plugin_dir.exists() {
            Ok(plugin_dir)
        } else {
            Err(PluginLoadError::NotFound(format!(
                "Plugin {} not installed",
                plugin_id
            )))
        }
    }

    fn get_installed_version(&self, plugin_id: &str) -> Result<String, PluginLoadError> {
        // This is simplified - in a real implementation, this would read version from plugin metadata
        Ok("0.1.0".to_string())
    }

    fn backup_plugin(&self, plugin_id: &str) -> Result<(), PluginLoadError> {
        let plugin_dir = self.find_plugin_installation(plugin_id)?;
        let backup_dir = self.cache_dir.join("backups").join(plugin_id);

        std::fs::create_dir_all(backup_dir.parent().unwrap())?;

        // Copy plugin directory to backup location
        // This is simplified - in a real implementation, this would be a proper recursive copy
        std::fs::create_dir_all(&backup_dir)?;

        Ok(())
    }

    fn cleanup_backup(&self, plugin_id: &str) -> Result<(), PluginLoadError> {
        let backup_dir = self.cache_dir.join("backups").join(plugin_id);
        if backup_dir.exists() {
            std::fs::remove_dir_all(backup_dir)?;
        }
        Ok(())
    }

    fn restore_backup(&self, plugin_id: &str) -> Result<(), PluginLoadError> {
        let plugin_dir = self.find_plugin_installation(plugin_id)?;
        let backup_dir = self.cache_dir.join("backups").join(plugin_id);

        if backup_dir.exists() {
            // Remove current installation
            if plugin_dir.exists() {
                std::fs::remove_dir_all(&plugin_dir)?;
            }

            // Restore from backup
            // This is simplified - in a real implementation, this would be a proper recursive copy
            std::fs::create_dir_all(&plugin_dir)?;
        }

        Ok(())
    }

    async fn fetch_registry_info(&self, _registry: &PluginRegistry) -> Result<(), PluginLoadError> {
        // This is a simplified implementation
        // In a real implementation, this would fetch registry metadata and update information
        Ok(())
    }
}

impl Default for SearchCriteria {
    fn default() -> Self {
        Self {
            query: None,
            category: None,
            tags: vec![],
            min_rating: None,
            compatible_only: true,
            sort_by: SortBy::Relevance,
            limit: Some(20),
            offset: None,
        }
    }
}

impl Default for InstallationOptions {
    fn default() -> Self {
        Self {
            verify_signature: true,
            install_dependencies: true,
            upgrade_existing: false,
            target_directory: None,
            pre_release: false,
        }
    }
}
