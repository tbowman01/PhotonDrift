use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

use crate::error::AdrscanError;
type Result<T> = std::result::Result<T, AdrscanError>;

/// ADRScan configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// ADR directory path
    pub adr_dir: PathBuf,
    
    /// File patterns to include in scans
    pub include_patterns: Vec<String>,
    
    /// File patterns to exclude from scans
    pub exclude_patterns: Vec<String>,
    
    /// Snapshot file location
    pub snapshot_file: PathBuf,
    
    /// ADR template configuration
    pub template: TemplateConfig,
    
    /// Drift detection configuration
    pub drift: DriftConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateConfig {
    /// ADR template format (madr, custom)
    pub format: String,
    
    /// Custom template path
    pub custom_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftConfig {
    /// Enable drift detection
    pub enabled: bool,
    
    /// Patterns for detecting architectural elements
    pub detection_patterns: Vec<DetectionPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectionPattern {
    /// Pattern name/description
    pub name: String,
    
    /// File glob pattern
    pub file_pattern: String,
    
    /// Regex pattern to match in file content
    pub content_pattern: String,
    
    /// Category for grouping (database, framework, cloud, etc.)
    pub category: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            adr_dir: PathBuf::from("docs/adr"),
            include_patterns: vec![
                "**/*.md".to_string(),
                "**/*.rs".to_string(),
                "**/*.py".to_string(),
                "**/*.js".to_string(),
                "**/*.ts".to_string(),
                "**/*.java".to_string(),
                "**/*.tf".to_string(),
                "**/*.yml".to_string(),
                "**/*.yaml".to_string(),
                "**/*.json".to_string(),
            ],
            exclude_patterns: vec![
                "**/target/**".to_string(),
                "**/node_modules/**".to_string(),
                "**/.git/**".to_string(),
                "**/build/**".to_string(),
                "**/dist/**".to_string(),
            ],
            snapshot_file: PathBuf::from(".adrscan_snapshot.json"),
            template: TemplateConfig {
                format: "madr".to_string(),
                custom_path: None,
            },
            drift: DriftConfig {
                enabled: true,
                detection_patterns: vec![
                    DetectionPattern {
                        name: "Database Dependencies".to_string(),
                        file_pattern: "**/Cargo.toml".to_string(),
                        content_pattern: r#"(postgres|mysql|sqlite|mongodb)"#.to_string(),
                        category: "database".to_string(),
                    },
                    DetectionPattern {
                        name: "Cloud Provider".to_string(),
                        file_pattern: "**/*.tf".to_string(),
                        content_pattern: r#"(aws|azure|gcp|google)"#.to_string(),
                        category: "cloud".to_string(),
                    },
                ],
            },
        }
    }
}

impl Config {
    /// Load configuration from file or create default
    pub fn load(config_path: Option<&Path>) -> Result<Self> {
        let config_file = if let Some(path) = config_path {
            path.to_path_buf()
        } else {
            // Try to find config file in current directory
            for filename in &[".adrscan.yml", ".adrscan.yaml", ".adrscan.toml"] {
                let path = PathBuf::from(filename);
                if path.exists() {
                    return Self::load_from_file(&path);
                }
            }
            // Return default config if no file found
            return Ok(Self::default());
        };

        Self::load_from_file(&config_file)
    }

    fn load_from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| AdrscanError::ConfigError(format!("Failed to read config file: {}", e)))?;

        let config = if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            toml::from_str(&content)
                .map_err(|e| AdrscanError::ConfigError(format!("Invalid TOML config: {}", e)))?
        } else {
            // Assume YAML
            serde_yaml::from_str(&content)
                .map_err(|e| AdrscanError::ConfigError(format!("Invalid YAML config: {}", e)))?
        };

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            toml::to_string_pretty(self)
                .map_err(|e| AdrscanError::ConfigError(format!("Failed to serialize TOML: {}", e)))?
        } else {
            // Default to YAML
            serde_yaml::to_string(self)
                .map_err(|e| AdrscanError::ConfigError(format!("Failed to serialize YAML: {}", e)))?
        };

        std::fs::write(path, content)
            .map_err(|e| AdrscanError::ConfigError(format!("Failed to write config file: {}", e)))?;

        Ok(())
    }
}