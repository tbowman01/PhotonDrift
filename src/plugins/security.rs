//! Plugin security validation and sandboxing

use crate::plugins::{PluginMetadata, PluginConfig, PluginCapability, SecurityLevel, PluginLoadError};
use crate::{Result, AdrscanError};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use log::{info, warn, error, debug};
use sha2::{Sha256, Digest};
use std::fs;

/// Plugin security validator
#[derive(Debug, Clone)]
pub struct PluginValidator {
    security_policy: SecurityPolicy,
    trusted_signatures: HashMap<String, String>,
    security_rules: Vec<SecurityRule>,
}

/// Security policy for plugin validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityPolicy {
    pub enforce_signatures: bool,
    pub allow_untrusted_plugins: bool,
    pub default_security_level: SecurityLevel,
    pub blocked_capabilities: Vec<PluginCapability>,
    pub trusted_authors: Vec<String>,
    pub max_plugin_size_bytes: u64,
    pub scan_for_malware: bool,
    pub validate_dependencies: bool,
}

/// Security rule for plugin validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRule {
    pub name: String,
    pub description: String,
    pub rule_type: SecurityRuleType,
    pub severity: SecuritySeverity,
    pub action: SecurityAction,
}

/// Types of security rules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityRuleType {
    /// Check file signature/hash
    FileIntegrity,
    /// Validate capabilities against policy
    CapabilityCheck,
    /// Check author against trusted list
    AuthorValidation,
    /// Scan for suspicious patterns
    MalwarePattern,
    /// Validate dependencies
    DependencyCheck,
    /// Check file size limits
    SizeLimit,
    /// Custom rule with pattern
    Custom(String),
}

/// Security severity levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Ord, PartialOrd, Eq)]
pub enum SecuritySeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Actions to take when security rule is triggered
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityAction {
    /// Allow with warning
    Warn,
    /// Block loading
    Block,
    /// Force sandboxed execution
    Sandbox,
    /// Require explicit user approval
    RequireApproval,
}

/// Security validation result
#[derive(Debug, Clone)]
pub struct SecurityValidationResult {
    pub passed: bool,
    pub violations: Vec<SecurityViolation>,
    pub recommendations: Vec<String>,
    pub risk_score: u32,
}

/// Security violation details
#[derive(Debug, Clone)]
pub struct SecurityViolation {
    pub rule_name: String,
    pub severity: SecuritySeverity,
    pub message: String,
    pub action: SecurityAction,
    pub details: HashMap<String, String>,
}

/// Plugin sandbox manager for isolating plugin execution
#[derive(Debug)]
pub struct SandboxManager {
    sandboxes: HashMap<String, PluginSandbox>,
    wasm_runtime: Option<WasmRuntime>,
}

/// Individual plugin sandbox
#[derive(Debug)]
pub struct PluginSandbox {
    pub plugin_id: String,
    pub resource_limits: ResourceLimits,
    pub allowed_paths: Vec<PathBuf>,
    pub network_restrictions: NetworkRestrictions,
    pub is_active: bool,
}

/// Resource limits for sandboxed plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub max_memory_bytes: usize,
    pub max_cpu_time_ms: u64,
    pub max_file_descriptors: u32,
    pub max_network_connections: u32,
}

/// Network restrictions for sandboxed plugins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRestrictions {
    pub allow_outbound: bool,
    pub allowed_domains: Vec<String>,
    pub blocked_domains: Vec<String>,
    pub max_requests_per_minute: u32,
}

/// WASM runtime for secure plugin execution
#[derive(Debug)]
pub struct WasmRuntime {
    // In a real implementation, this would contain WASM runtime state
    // For now, we'll use a placeholder
    _placeholder: (),
}

impl PluginValidator {
    /// Create a new plugin validator with the given security policy
    pub fn new(security_policy: SecurityPolicy) -> Self {
        let mut validator = Self {
            security_policy,
            trusted_signatures: HashMap::new(),
            security_rules: Vec::new(),
        };
        
        validator.initialize_default_rules();
        validator
    }
    
    /// Validate a plugin against security policies
    pub fn validate_plugin(
        &self,
        plugin_path: &Path,
        metadata: &PluginMetadata,
        config: &PluginConfig,
    ) -> Result<SecurityValidationResult, PluginLoadError> {
        debug!("Validating plugin security: {}", metadata.id);
        
        let mut violations = Vec::new();
        let mut recommendations = Vec::new();
        let mut risk_score = 0u32;
        
        // Run all security rules
        for rule in &self.security_rules {
            if let Some(violation) = self.apply_security_rule(rule, plugin_path, metadata, config)? {
                risk_score += self.calculate_risk_score(&violation);
                violations.push(violation);
            }
        }
        
        // Generate recommendations based on violations
        if !violations.is_empty() {
            recommendations.extend(self.generate_security_recommendations(&violations));
        }
        
        let passed = violations.iter().all(|v| !matches!(v.action, SecurityAction::Block));
        
        let result = SecurityValidationResult {
            passed,
            violations,
            recommendations,
            risk_score,
        };
        
        info!("Plugin {} security validation: {} (risk score: {})", 
              metadata.id, if result.passed { "PASSED" } else { "FAILED" }, result.risk_score);
        
        Ok(result)
    }
    
    /// Check if a plugin signature is trusted
    pub fn verify_signature(&self, plugin_path: &Path) -> Result<bool, PluginLoadError> {
        if !self.security_policy.enforce_signatures {
            return Ok(true);
        }
        
        let file_hash = self.calculate_file_hash(plugin_path)?;
        let signature_path = plugin_path.with_extension("sig");
        
        if !signature_path.exists() {
            warn!("No signature file found for plugin: {}", plugin_path.display());
            return Ok(self.security_policy.allow_untrusted_plugins);
        }
        
        let signature = fs::read_to_string(signature_path)?;
        Ok(self.trusted_signatures.get(&file_hash) == Some(&signature))
    }
    
    /// Scan plugin for malware patterns
    pub fn scan_for_malware(&self, plugin_path: &Path) -> Result<Vec<String>, PluginLoadError> {
        if !self.security_policy.scan_for_malware {
            return Ok(Vec::new());
        }
        
        let content = fs::read(plugin_path)?;
        let mut suspicious_patterns = Vec::new();
        
        // Define suspicious patterns to look for
        let patterns = [
            (b"eval(", "Dynamic code execution"),
            (b"exec(", "Command execution"),
            (b"system(", "System command execution"),
            (b"shell_exec", "Shell command execution"),
            (b"/etc/passwd", "System file access"),
            (b"rm -rf", "Destructive file operations"),
            (b"curl ", "Network requests"),
            (b"wget ", "File downloads"),
        ];
        
        for (pattern, description) in &patterns {
            if content.windows(pattern.len()).any(|window| window == *pattern) {
                suspicious_patterns.push(description.to_string());
            }
        }
        
        if !suspicious_patterns.is_empty() {
            warn!("Suspicious patterns found in plugin {}: {:?}", 
                  plugin_path.display(), suspicious_patterns);
        }
        
        Ok(suspicious_patterns)
    }
    
    /// Add a trusted signature for a plugin
    pub fn add_trusted_signature(&mut self, file_hash: String, signature: String) {
        self.trusted_signatures.insert(file_hash, signature);
    }
    
    /// Add a custom security rule
    pub fn add_security_rule(&mut self, rule: SecurityRule) {
        self.security_rules.push(rule);
    }
    
    // Private helper methods
    
    fn initialize_default_rules(&mut self) {
        // File integrity rule
        self.security_rules.push(SecurityRule {
            name: "file_integrity".to_string(),
            description: "Verify plugin file integrity".to_string(),
            rule_type: SecurityRuleType::FileIntegrity,
            severity: SecuritySeverity::High,
            action: SecurityAction::Block,
        });
        
        // Capability check rule
        self.security_rules.push(SecurityRule {
            name: "capability_check".to_string(),
            description: "Validate plugin capabilities against policy".to_string(),
            rule_type: SecurityRuleType::CapabilityCheck,
            severity: SecuritySeverity::Medium,
            action: SecurityAction::Sandbox,
        });
        
        // Author validation rule
        self.security_rules.push(SecurityRule {
            name: "author_validation".to_string(),
            description: "Check if plugin author is trusted".to_string(),
            rule_type: SecurityRuleType::AuthorValidation,
            severity: SecuritySeverity::Low,
            action: SecurityAction::Warn,
        });
        
        // Malware pattern rule
        self.security_rules.push(SecurityRule {
            name: "malware_scan".to_string(),
            description: "Scan for suspicious code patterns".to_string(),
            rule_type: SecurityRuleType::MalwarePattern,
            severity: SecuritySeverity::Critical,
            action: SecurityAction::Block,
        });
        
        // Size limit rule
        self.security_rules.push(SecurityRule {
            name: "size_limit".to_string(),
            description: "Check plugin file size limits".to_string(),
            rule_type: SecurityRuleType::SizeLimit,
            severity: SecuritySeverity::Low,
            action: SecurityAction::Warn,
        });
    }
    
    fn apply_security_rule(
        &self,
        rule: &SecurityRule,
        plugin_path: &Path,
        metadata: &PluginMetadata,
        _config: &PluginConfig,
    ) -> Result<Option<SecurityViolation>, PluginLoadError> {
        match &rule.rule_type {
            SecurityRuleType::FileIntegrity => {
                if !self.verify_signature(plugin_path)? {
                    return Ok(Some(SecurityViolation {
                        rule_name: rule.name.clone(),
                        severity: rule.severity.clone(),
                        message: "Plugin signature verification failed".to_string(),
                        action: rule.action.clone(),
                        details: HashMap::from([
                            ("file".to_string(), plugin_path.display().to_string()),
                        ]),
                    }));
                }
            }
            
            SecurityRuleType::CapabilityCheck => {
                // This would be implemented with actual plugin loading
                // For now, assume all capabilities are allowed
            }
            
            SecurityRuleType::AuthorValidation => {
                if !self.security_policy.trusted_authors.contains(&metadata.author) {
                    return Ok(Some(SecurityViolation {
                        rule_name: rule.name.clone(),
                        severity: rule.severity.clone(),
                        message: format!("Plugin author '{}' is not in trusted list", metadata.author),
                        action: rule.action.clone(),
                        details: HashMap::from([
                            ("author".to_string(), metadata.author.clone()),
                        ]),
                    }));
                }
            }
            
            SecurityRuleType::MalwarePattern => {
                let suspicious_patterns = self.scan_for_malware(plugin_path)?;
                if !suspicious_patterns.is_empty() {
                    return Ok(Some(SecurityViolation {
                        rule_name: rule.name.clone(),
                        severity: rule.severity.clone(),
                        message: format!("Suspicious patterns detected: {:?}", suspicious_patterns),
                        action: rule.action.clone(),
                        details: HashMap::from([
                            ("patterns".to_string(), suspicious_patterns.join(", ")),
                        ]),
                    }));
                }
            }
            
            SecurityRuleType::SizeLimit => {
                let file_size = fs::metadata(plugin_path)?.len();
                if file_size > self.security_policy.max_plugin_size_bytes {
                    return Ok(Some(SecurityViolation {
                        rule_name: rule.name.clone(),
                        severity: rule.severity.clone(),
                        message: format!("Plugin size {} exceeds limit {}", 
                                       file_size, self.security_policy.max_plugin_size_bytes),
                        action: rule.action.clone(),
                        details: HashMap::from([
                            ("size".to_string(), file_size.to_string()),
                            ("limit".to_string(), self.security_policy.max_plugin_size_bytes.to_string()),
                        ]),
                    }));
                }
            }
            
            SecurityRuleType::DependencyCheck => {
                // TODO: Implement dependency validation
            }
            
            SecurityRuleType::Custom(_pattern) => {
                // TODO: Implement custom rule pattern matching
            }
        }
        
        Ok(None)
    }
    
    fn calculate_risk_score(&self, violation: &SecurityViolation) -> u32 {
        match violation.severity {
            SecuritySeverity::Low => 1,
            SecuritySeverity::Medium => 5,
            SecuritySeverity::High => 15,
            SecuritySeverity::Critical => 50,
        }
    }
    
    fn generate_security_recommendations(&self, violations: &[SecurityViolation]) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        for violation in violations {
            match &violation.action {
                SecurityAction::Block => {
                    recommendations.push(format!(
                        "Plugin blocked due to {}: {}. Consider using a trusted version.",
                        violation.rule_name, violation.message
                    ));
                }
                SecurityAction::Sandbox => {
                    recommendations.push(format!(
                        "Plugin should run in sandbox mode due to {}: {}",
                        violation.rule_name, violation.message
                    ));
                }
                SecurityAction::Warn => {
                    recommendations.push(format!(
                        "Consider reviewing plugin due to {}: {}",
                        violation.rule_name, violation.message
                    ));
                }
                SecurityAction::RequireApproval => {
                    recommendations.push(format!(
                        "Manual approval required for {}: {}",
                        violation.rule_name, violation.message
                    ));
                }
            }
        }
        
        recommendations
    }
    
    fn calculate_file_hash(&self, path: &Path) -> Result<String, PluginLoadError> {
        let content = fs::read(path)?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        Ok(format!("{:x}", hasher.finalize()))
    }
}

impl SandboxManager {
    /// Create a new sandbox manager
    pub fn new() -> Self {
        Self {
            sandboxes: HashMap::new(),
            wasm_runtime: Some(WasmRuntime { _placeholder: () }),
        }
    }
    
    /// Initialize sandbox for a plugin
    pub fn initialize_sandbox(&mut self, plugin_id: &str) -> Result<(), PluginLoadError> {
        debug!("Initializing sandbox for plugin: {}", plugin_id);
        
        let sandbox = PluginSandbox {
            plugin_id: plugin_id.to_string(),
            resource_limits: ResourceLimits::default(),
            allowed_paths: vec![],
            network_restrictions: NetworkRestrictions::default(),
            is_active: true,
        };
        
        self.sandboxes.insert(plugin_id.to_string(), sandbox);
        info!("Sandbox initialized for plugin: {}", plugin_id);
        Ok(())
    }
    
    /// Execute plugin in sandbox
    pub fn execute_in_sandbox<F, T>(
        &mut self,
        plugin_id: &str,
        operation: F,
    ) -> Result<T, PluginLoadError>
    where
        F: FnOnce() -> Result<T, PluginLoadError>,
    {
        let _sandbox = self.sandboxes.get(plugin_id)
            .ok_or_else(|| PluginLoadError::NotFound(format!("Sandbox for {}", plugin_id)))?;
        
        // TODO: Implement actual sandboxing
        // For now, just execute the operation directly
        debug!("Executing operation in sandbox for plugin: {}", plugin_id);
        operation()
    }
    
    /// Cleanup sandbox for a plugin
    pub fn cleanup_sandbox(&mut self, plugin_id: &str) -> Result<(), PluginLoadError> {
        if let Some(_sandbox) = self.sandboxes.remove(plugin_id) {
            debug!("Cleaned up sandbox for plugin: {}", plugin_id);
        }
        Ok(())
    }
    
    /// Get sandbox status
    pub fn get_sandbox_status(&self, plugin_id: &str) -> Option<&PluginSandbox> {
        self.sandboxes.get(plugin_id)
    }
}

impl Default for SecurityPolicy {
    fn default() -> Self {
        Self {
            enforce_signatures: false,
            allow_untrusted_plugins: true,
            default_security_level: SecurityLevel::Standard,
            blocked_capabilities: vec![
                PluginCapability::SystemExecution,
            ],
            trusted_authors: vec![
                "photondrift-team".to_string(),
                "official-plugins".to_string(),
            ],
            max_plugin_size_bytes: 50 * 1024 * 1024, // 50MB
            scan_for_malware: true,
            validate_dependencies: true,
        }
    }
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            max_memory_bytes: 50 * 1024 * 1024, // 50MB
            max_cpu_time_ms: 5000, // 5 seconds
            max_file_descriptors: 100,
            max_network_connections: 10,
        }
    }
}

impl Default for NetworkRestrictions {
    fn default() -> Self {
        Self {
            allow_outbound: false,
            allowed_domains: vec![
                "api.github.com".to_string(),
                "registry.npmjs.org".to_string(),
            ],
            blocked_domains: vec![],
            max_requests_per_minute: 60,
        }
    }
}