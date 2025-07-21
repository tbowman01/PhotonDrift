use serde::{Deserialize, Serialize};
use std::env;
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

/// Configuration overrides that can be applied from command line or environment variables
#[derive(Debug, Clone, Default)]
pub struct ConfigOverrides {
    #[allow(dead_code)] // Planned for CLI integration
    pub adr_dir: Option<PathBuf>,
    #[allow(dead_code)] // Planned for CLI integration
    pub include_patterns: Option<Vec<String>>,
    #[allow(dead_code)] // Planned for CLI integration
    pub exclude_patterns: Option<Vec<String>>,
    #[allow(dead_code)] // Planned for CLI integration
    pub snapshot_file: Option<PathBuf>,
    #[allow(dead_code)] // Planned for CLI integration
    pub template_format: Option<String>,
    #[allow(dead_code)] // Planned for CLI integration
    pub template_custom_path: Option<PathBuf>,
    #[allow(dead_code)] // Planned for CLI integration
    pub drift_enabled: Option<bool>,
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
    /// Load configuration from file, environment variables, and apply overrides
    pub fn load(config_path: Option<&Path>) -> Result<Self> {
        let mut config = Self::load_base_config(config_path)?;

        // Apply environment variable overrides
        config.apply_env_overrides()?;

        // Validate the final configuration
        config.validate()?;

        Ok(config)
    }

    /// Load configuration with additional overrides
    #[allow(dead_code)] // Planned for CLI integration
    pub fn load_with_overrides(
        config_path: Option<&Path>,
        overrides: &ConfigOverrides,
    ) -> Result<Self> {
        let mut config = Self::load_base_config(config_path)?;

        // Apply environment variable overrides
        config.apply_env_overrides()?;

        // Apply provided overrides
        config.apply_overrides(overrides);

        // Validate the final configuration
        config.validate()?;

        Ok(config)
    }

    /// Load base configuration from file or defaults
    fn load_base_config(config_path: Option<&Path>) -> Result<Self> {
        let config_file = if let Some(path) = config_path {
            path.to_path_buf()
        } else {
            // Try to find config file in current directory and parent directories
            let mut current_dir = env::current_dir().map_err(|e| {
                AdrscanError::ConfigError(format!("Cannot get current directory: {e}"))
            })?;

            loop {
                for filename in &[".adrscan.yml", ".adrscan.yaml", ".adrscan.toml"] {
                    let config_path = current_dir.join(filename);
                    if config_path.exists() {
                        return Self::load_from_file(&config_path);
                    }
                }

                // Try parent directory
                if let Some(parent) = current_dir.parent() {
                    current_dir = parent.to_path_buf();
                } else {
                    // Reached filesystem root, use default config
                    break;
                }
            }

            // Return default config if no file found
            return Ok(Self::default());
        };

        Self::load_from_file(&config_file)
    }

    fn load_from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| AdrscanError::ConfigError(format!("Failed to read config file: {e}")))?;

        let config = if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            toml::from_str(&content)
                .map_err(|e| AdrscanError::ConfigError(format!("Invalid TOML config: {e}")))?
        } else {
            // Assume YAML
            serde_yaml::from_str(&content)
                .map_err(|e| AdrscanError::ConfigError(format!("Invalid YAML config: {e}")))?
        };

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            toml::to_string_pretty(self)
                .map_err(|e| AdrscanError::ConfigError(format!("Failed to serialize TOML: {e}")))?
        } else {
            // Default to YAML
            serde_yaml::to_string(self)
                .map_err(|e| AdrscanError::ConfigError(format!("Failed to serialize YAML: {e}")))?
        };

        std::fs::write(path, content)
            .map_err(|e| AdrscanError::ConfigError(format!("Failed to write config file: {e}")))?;

        Ok(())
    }

    /// Apply environment variable overrides to configuration
    fn apply_env_overrides(&mut self) -> Result<()> {
        // ADR_DIR or ADRSCAN_ADR_DIR
        if let Ok(adr_dir) = env::var("ADRSCAN_ADR_DIR").or_else(|_| env::var("ADR_DIR")) {
            self.adr_dir = PathBuf::from(adr_dir);
        }

        // ADRSCAN_INCLUDE_PATTERNS (comma-separated)
        if let Ok(patterns) = env::var("ADRSCAN_INCLUDE_PATTERNS") {
            self.include_patterns = patterns
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        // ADRSCAN_EXCLUDE_PATTERNS (comma-separated)
        if let Ok(patterns) = env::var("ADRSCAN_EXCLUDE_PATTERNS") {
            self.exclude_patterns = patterns
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        // ADRSCAN_SNAPSHOT_FILE
        if let Ok(snapshot_file) = env::var("ADRSCAN_SNAPSHOT_FILE") {
            self.snapshot_file = PathBuf::from(snapshot_file);
        }

        // ADRSCAN_TEMPLATE_FORMAT
        if let Ok(format) = env::var("ADRSCAN_TEMPLATE_FORMAT") {
            self.template.format = format;
        }

        // ADRSCAN_TEMPLATE_CUSTOM_PATH
        if let Ok(custom_path) = env::var("ADRSCAN_TEMPLATE_CUSTOM_PATH") {
            self.template.custom_path = Some(PathBuf::from(custom_path));
        }

        // ADRSCAN_DRIFT_ENABLED
        if let Ok(enabled) = env::var("ADRSCAN_DRIFT_ENABLED") {
            match enabled.to_lowercase().as_str() {
                "true" | "1" | "yes" | "on" => self.drift.enabled = true,
                "false" | "0" | "no" | "off" => self.drift.enabled = false,
                _ => return Err(AdrscanError::ConfigError(
                    format!("Invalid value for ADRSCAN_DRIFT_ENABLED: '{enabled}'. Expected true/false, 1/0, yes/no, or on/off")
                )),
            }
        }

        Ok(())
    }

    /// Apply configuration overrides
    #[allow(dead_code)] // Planned for CLI integration
    fn apply_overrides(&mut self, overrides: &ConfigOverrides) {
        if let Some(ref adr_dir) = overrides.adr_dir {
            self.adr_dir = adr_dir.clone();
        }
        if let Some(ref include_patterns) = overrides.include_patterns {
            self.include_patterns = include_patterns.clone();
        }
        if let Some(ref exclude_patterns) = overrides.exclude_patterns {
            self.exclude_patterns = exclude_patterns.clone();
        }
        if let Some(ref snapshot_file) = overrides.snapshot_file {
            self.snapshot_file = snapshot_file.clone();
        }
        if let Some(ref template_format) = overrides.template_format {
            self.template.format = template_format.clone();
        }
        if let Some(ref template_custom_path) = overrides.template_custom_path {
            self.template.custom_path = Some(template_custom_path.clone());
        }
        if let Some(drift_enabled) = overrides.drift_enabled {
            self.drift.enabled = drift_enabled;
        }
    }

    /// Validate configuration values
    fn validate(&self) -> Result<()> {
        // Validate ADR directory path
        if self.adr_dir.to_string_lossy().is_empty() {
            return Err(AdrscanError::ConfigError(
                "ADR directory path cannot be empty".to_string(),
            ));
        }

        // Validate include patterns
        if self.include_patterns.is_empty() {
            return Err(AdrscanError::ConfigError(
                "At least one include pattern must be specified".to_string(),
            ));
        }

        // Validate patterns for basic glob syntax
        for pattern in &self.include_patterns {
            if pattern.is_empty() {
                return Err(AdrscanError::ConfigError(
                    "Include patterns cannot be empty".to_string(),
                ));
            }
        }

        for pattern in &self.exclude_patterns {
            if pattern.is_empty() {
                return Err(AdrscanError::ConfigError(
                    "Exclude patterns cannot be empty".to_string(),
                ));
            }
        }

        // Validate snapshot file path
        if self.snapshot_file.to_string_lossy().is_empty() {
            return Err(AdrscanError::ConfigError(
                "Snapshot file path cannot be empty".to_string(),
            ));
        }

        // Validate template format
        match self.template.format.as_str() {
            "madr" | "custom" => {}
            _ => {
                return Err(AdrscanError::ConfigError(format!(
                    "Invalid template format '{}'. Supported formats: 'madr', 'custom'",
                    self.template.format
                )))
            }
        }

        // Validate custom template path if format is custom
        if self.template.format == "custom" && self.template.custom_path.is_none() {
            return Err(AdrscanError::ConfigError(
                "Custom template path must be specified when format is 'custom'".to_string(),
            ));
        }

        // Validate custom template path exists if specified
        if let Some(ref custom_path) = self.template.custom_path {
            if !custom_path.exists() {
                return Err(AdrscanError::ConfigError(format!(
                    "Custom template file does not exist: {}",
                    custom_path.display()
                )));
            }
        }

        // Validate detection patterns
        for pattern in &self.drift.detection_patterns {
            if pattern.name.is_empty() {
                return Err(AdrscanError::ConfigError(
                    "Detection pattern name cannot be empty".to_string(),
                ));
            }
            if pattern.file_pattern.is_empty() {
                return Err(AdrscanError::ConfigError(
                    "Detection pattern file_pattern cannot be empty".to_string(),
                ));
            }
            if pattern.content_pattern.is_empty() {
                return Err(AdrscanError::ConfigError(
                    "Detection pattern content_pattern cannot be empty".to_string(),
                ));
            }
            if pattern.category.is_empty() {
                return Err(AdrscanError::ConfigError(
                    "Detection pattern category cannot be empty".to_string(),
                ));
            }

            // Validate regex pattern
            if let Err(e) = regex::Regex::new(&pattern.content_pattern) {
                return Err(AdrscanError::ConfigError(format!(
                    "Invalid regex pattern '{}': {}",
                    pattern.content_pattern, e
                )));
            }
        }

        Ok(())
    }

    /// Create a sample configuration file
    #[allow(dead_code)] // Planned for CLI integration
    pub fn create_sample_config(path: &Path, format: &str) -> Result<()> {
        let config = Self::default();

        let content = match format {
            "toml" => toml::to_string_pretty(&config).map_err(|e| {
                AdrscanError::ConfigError(format!("Failed to serialize sample TOML: {e}"))
            })?,
            "yaml" | "yml" => serde_yaml::to_string(&config).map_err(|e| {
                AdrscanError::ConfigError(format!("Failed to serialize sample YAML: {e}"))
            })?,
            _ => {
                return Err(AdrscanError::ConfigError(format!(
                    "Unsupported config format: {format}. Use 'toml' or 'yaml'"
                )))
            }
        };

        std::fs::write(path, content).map_err(|e| {
            AdrscanError::ConfigError(format!("Failed to write sample config: {e}"))
        })?;

        Ok(())
    }

    /// Get configuration from environment variables only (for testing/debugging)
    #[allow(dead_code)] // Planned for CLI integration
    pub fn from_env() -> Result<Self> {
        let mut config = Self::default();
        config.apply_env_overrides()?;
        config.validate()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.adr_dir, PathBuf::from("docs/adr"));
        assert!(config.include_patterns.contains(&"**/*.md".to_string()));
        assert!(config
            .exclude_patterns
            .contains(&"**/target/**".to_string()));
        assert_eq!(config.template.format, "madr");
        assert!(config.drift.enabled);
    }

    #[test]
    fn test_load_yaml_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".adrscan.yml");

        let yaml_content = r#"
adr_dir: "custom/adr"
include_patterns:
  - "**/*.md"
  - "**/*.txt"
exclude_patterns:
  - "**/build/**"
snapshot_file: "custom_snapshot.json"
template:
  format: "custom"
  custom_path: "templates/custom.md"
drift:
  enabled: false
  detection_patterns: []
"#;

        fs::write(&config_path, yaml_content).unwrap();

        let config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(config.adr_dir, PathBuf::from("custom/adr"));
        assert_eq!(config.include_patterns, vec!["**/*.md", "**/*.txt"]);
        assert_eq!(config.exclude_patterns, vec!["**/build/**"]);
        assert_eq!(config.template.format, "custom");
        assert!(!config.drift.enabled);
    }

    #[test]
    fn test_load_toml_config() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".adrscan.toml");

        let toml_content = r#"
adr_dir = "toml/adr"
include_patterns = ["**/*.md"]
exclude_patterns = ["**/node_modules/**"]
snapshot_file = "toml_snapshot.json"

[template]
format = "madr"

[drift]
enabled = true
detection_patterns = []
"#;

        fs::write(&config_path, toml_content).unwrap();

        let config = Config::load_from_file(&config_path).unwrap();
        assert_eq!(config.adr_dir, PathBuf::from("toml/adr"));
        assert_eq!(config.include_patterns, vec!["**/*.md"]);
        assert_eq!(config.template.format, "madr");
        assert!(config.drift.enabled);
    }

    #[test]
    fn test_env_overrides() {
        env::set_var("ADRSCAN_ADR_DIR", "env/adr");
        env::set_var("ADRSCAN_INCLUDE_PATTERNS", "*.md,*.txt");
        env::set_var("ADRSCAN_EXCLUDE_PATTERNS", "build/**,tmp/**");
        env::set_var("ADRSCAN_TEMPLATE_FORMAT", "custom");
        env::set_var("ADRSCAN_DRIFT_ENABLED", "false");

        let mut config = Config::default();
        config.apply_env_overrides().unwrap();

        assert_eq!(config.adr_dir, PathBuf::from("env/adr"));
        assert_eq!(config.include_patterns, vec!["*.md", "*.txt"]);
        assert_eq!(config.exclude_patterns, vec!["build/**", "tmp/**"]);
        assert_eq!(config.template.format, "custom");
        assert!(!config.drift.enabled);

        // Clean up
        env::remove_var("ADRSCAN_ADR_DIR");
        env::remove_var("ADRSCAN_INCLUDE_PATTERNS");
        env::remove_var("ADRSCAN_EXCLUDE_PATTERNS");
        env::remove_var("ADRSCAN_TEMPLATE_FORMAT");
        env::remove_var("ADRSCAN_DRIFT_ENABLED");
    }

    #[test]
    fn test_config_overrides() {
        let mut config = Config::default();
        let overrides = ConfigOverrides {
            adr_dir: Some(PathBuf::from("override/adr")),
            include_patterns: Some(vec!["*.override".to_string()]),
            template_format: Some("custom".to_string()),
            drift_enabled: Some(false),
            ..Default::default()
        };

        config.apply_overrides(&overrides);

        assert_eq!(config.adr_dir, PathBuf::from("override/adr"));
        assert_eq!(config.include_patterns, vec!["*.override"]);
        assert_eq!(config.template.format, "custom");
        assert!(!config.drift.enabled);
    }

    #[test]
    fn test_validation_success() {
        let config = Config::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_validation_empty_adr_dir() {
        let mut config = Config::default();
        config.adr_dir = PathBuf::from("");
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_empty_include_patterns() {
        let mut config = Config::default();
        config.include_patterns = vec![];
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_invalid_template_format() {
        let mut config = Config::default();
        config.template.format = "invalid".to_string();
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_custom_template_without_path() {
        let mut config = Config::default();
        config.template.format = "custom".to_string();
        config.template.custom_path = None;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_validation_invalid_regex_pattern() {
        let mut config = Config::default();
        config.drift.detection_patterns.push(DetectionPattern {
            name: "Invalid Pattern".to_string(),
            file_pattern: "*.rs".to_string(),
            content_pattern: "[invalid_regex".to_string(), // Invalid regex
            category: "test".to_string(),
        });
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_create_sample_config_yaml() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("sample.yml");

        Config::create_sample_config(&config_path, "yaml").unwrap();
        assert!(config_path.exists());

        // Verify it can be loaded back
        let loaded_config = Config::load_from_file(&config_path).unwrap();
        let default_config = Config::default();
        assert_eq!(loaded_config.adr_dir, default_config.adr_dir);
    }

    #[test]
    fn test_create_sample_config_toml() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join("sample.toml");

        Config::create_sample_config(&config_path, "toml").unwrap();
        assert!(config_path.exists());

        // Verify it can be loaded back
        let loaded_config = Config::load_from_file(&config_path).unwrap();
        let default_config = Config::default();
        assert_eq!(
            loaded_config.template.format,
            default_config.template.format
        );
    }

    #[test]
    fn test_env_drift_enabled_variations() {
        let test_cases = vec![
            ("true", true),
            ("TRUE", true),
            ("1", true),
            ("yes", true),
            ("YES", true),
            ("on", true),
            ("ON", true),
            ("false", false),
            ("FALSE", false),
            ("0", false),
            ("no", false),
            ("NO", false),
            ("off", false),
            ("OFF", false),
        ];

        for (env_val, expected) in test_cases {
            // Clear any existing environment variables
            env::remove_var("ADRSCAN_DRIFT_ENABLED");

            env::set_var("ADRSCAN_DRIFT_ENABLED", env_val);
            let mut config = Config::default();
            // Set initial state to opposite of expected to ensure the env var actually changes it
            config.drift.enabled = !expected;
            config.apply_env_overrides().unwrap();
            assert_eq!(
                config.drift.enabled, expected,
                "Failed for value: {} (original: {})",
                env_val, !expected
            );

            // Clean up after each test case
            env::remove_var("ADRSCAN_DRIFT_ENABLED");
        }

        // Test invalid value
        env::set_var("ADRSCAN_DRIFT_ENABLED", "maybe");
        let mut config = Config::default();
        assert!(config.apply_env_overrides().is_err());

        env::remove_var("ADRSCAN_DRIFT_ENABLED");
    }

    #[test]
    fn test_load_with_overrides() {
        let temp_dir = TempDir::new().unwrap();
        let config_path = temp_dir.path().join(".adrscan.yml");

        let yaml_content = r#"
adr_dir: "yaml/adr"
include_patterns: ["**/*.md"]
exclude_patterns: []
snapshot_file: "test_snapshot.json"
template:
  format: "madr"
drift:
  enabled: true
  detection_patterns: []
"#;
        fs::write(&config_path, yaml_content).unwrap();

        let overrides = ConfigOverrides {
            adr_dir: Some(PathBuf::from("override/adr")),
            include_patterns: Some(vec!["*.override".to_string()]),
            ..Default::default()
        };

        let config = Config::load_with_overrides(Some(&config_path), &overrides).unwrap();

        // Overrides should take precedence
        assert_eq!(config.adr_dir, PathBuf::from("override/adr"));
        assert_eq!(config.include_patterns, vec!["*.override"]);
    }

    #[test]
    fn test_config_search_in_parent_directories() {
        // This test is more complex to set up, so we'll just test the logic
        // In a real scenario, it would search parent directories for config files
        let temp_dir = TempDir::new().unwrap();
        let sub_dir = temp_dir.path().join("subdir");
        fs::create_dir(&sub_dir).unwrap();

        let config_path = temp_dir.path().join(".adrscan.yml");
        let yaml_content = r#"
adr_dir: "parent/adr"
include_patterns: ["**/*.md"]
exclude_patterns: []
snapshot_file: "parent_snapshot.json"
template:
  format: "madr"
drift:
  enabled: true
  detection_patterns: []
"#;
        fs::write(&config_path, yaml_content).unwrap();

        // Change to subdirectory
        let original_dir = env::current_dir().unwrap();
        env::set_current_dir(&sub_dir).unwrap();

        // Should find config in parent directory
        let config = Config::load_base_config(None).unwrap();
        assert_eq!(config.adr_dir, PathBuf::from("parent/adr"));

        // Restore original directory
        env::set_current_dir(original_dir).unwrap();
    }
}
