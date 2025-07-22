//! Security Analyzer Plugin
//! 
//! A sample plugin that demonstrates drift detection for security-related decisions
//! and provides security-specific ADR templates and analysis.

use adrscan::plugins::{
    Plugin, DriftAnalysisPlugin, TemplatePlugin, PluginCapability, PluginMetadata,
    PluginContext, PluginResponse, PluginResult
};
use adrscan::{Result, AdrscanError};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use regex::Regex;

/// Security-focused drift analysis and template plugin
pub struct SecurityAnalyzerPlugin {
    metadata: PluginMetadata,
    security_patterns: Vec<SecurityPattern>,
    templates: Vec<SecurityTemplate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SecurityPattern {
    name: String,
    description: String,
    pattern: String,
    severity: SecuritySeverity,
    category: SecurityCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum SecuritySeverity {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum SecurityCategory {
    Authentication,
    Authorization,
    DataProtection,
    NetworkSecurity,
    Cryptography,
    InputValidation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SecurityTemplate {
    name: String,
    description: String,
    category: SecurityCategory,
    template_content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SecurityDriftResult {
    pattern_name: String,
    severity: SecuritySeverity,
    category: SecurityCategory,
    description: String,
    file_path: String,
    line_number: Option<usize>,
    recommendation: String,
}

impl SecurityAnalyzerPlugin {
    pub fn new() -> Self {
        let security_patterns = vec![
            SecurityPattern {
                name: "Hardcoded Credentials".to_string(),
                description: "Detection of hardcoded passwords, API keys, or tokens".to_string(),
                pattern: r"(?i)(password|pwd|pass|token|key|secret|auth)\s*[:=]\s*['\"][^'\"]{8,}['\"]".to_string(),
                severity: SecuritySeverity::Critical,
                category: SecurityCategory::Authentication,
            },
            SecurityPattern {
                name: "Weak Hashing Algorithm".to_string(),
                description: "Usage of weak or deprecated hashing algorithms".to_string(),
                pattern: r"(?i)(md5|sha1|des|rc4|md4)".to_string(),
                severity: SecuritySeverity::High,
                category: SecurityCategory::Cryptography,
            },
            SecurityPattern {
                name: "SQL Injection Risk".to_string(),
                description: "Potential SQL injection vulnerability patterns".to_string(),
                pattern: r"(?i)query\s*\(\s*['\"].*\+.*['\"]".to_string(),
                severity: SecuritySeverity::High,
                category: SecurityCategory::InputValidation,
            },
            SecurityPattern {
                name: "Insecure HTTP Usage".to_string(),
                description: "Usage of insecure HTTP instead of HTTPS".to_string(),
                pattern: r"http://(?!localhost|127\.0\.0\.1|0\.0\.0\.0)".to_string(),
                severity: SecuritySeverity::Medium,
                category: SecurityCategory::NetworkSecurity,
            },
            SecurityPattern {
                name: "Missing Input Validation".to_string(),
                description: "User input processed without validation".to_string(),
                pattern: r"(?i)(request\.|params\.|query\.).*(exec|eval|system)".to_string(),
                severity: SecuritySeverity::High,
                category: SecurityCategory::InputValidation,
            },
        ];

        let templates = vec![
            SecurityTemplate {
                name: "Security Architecture Decision".to_string(),
                description: "Template for security-related architectural decisions".to_string(),
                category: SecurityCategory::Authentication,
                template_content: r#"---
title: ${1:Security Decision Title}
status: proposed
date: ${CURRENT_DATE}
tags: [security, ${2:authentication}]
security_impact: ${3:high}
---

# ADR-${4:0001}: ${1:Security Decision Title}

## Status

${5|Proposed,Accepted,Rejected,Superseded,Deprecated|}

## Security Context

<!-- Describe the security challenge or threat that needs to be addressed -->

${6:Security context and threat landscape}

## Security Requirements

<!-- List specific security requirements and compliance needs -->

* ${7:Requirement 1}
* ${8:Requirement 2}

## Threat Model

<!-- Identify potential threats and attack vectors -->

### Identified Threats
* ${9:Threat 1}
* ${10:Threat 2}

### Attack Vectors
* ${11:Attack vector 1}
* ${12:Attack vector 2}

## Decision

${13:Security architecture decision}

## Security Controls

<!-- Describe specific security controls and measures -->

### Preventive Controls
* ${14:Preventive control 1}

### Detective Controls  
* ${15:Detective control 1}

### Corrective Controls
* ${16:Corrective control 1}

## Consequences

### Security Benefits
* ${17:Security benefit 1}

### Security Risks
* ${18:Security risk 1}

### Compliance Impact
* ${19:Compliance consideration 1}

## Implementation Notes

<!-- Security implementation details and considerations -->

${20:Implementation notes}

## Monitoring & Alerting

<!-- How security violations will be detected and handled -->

${21:Monitoring approach}
"#.to_string(),
            },
            SecurityTemplate {
                name: "Data Protection Decision".to_string(),
                description: "Template for data protection and privacy decisions".to_string(),
                category: SecurityCategory::DataProtection,
                template_content: r#"---
title: ${1:Data Protection Decision}
status: proposed
date: ${CURRENT_DATE}
tags: [security, data-protection, privacy]
compliance: [${2:GDPR}, ${3:CCPA}]
---

# ADR-${4:0001}: ${1:Data Protection Decision}

## Status

${5|Proposed,Accepted,Rejected,Superseded,Deprecated|}

## Data Classification

<!-- Classify the data types and sensitivity levels -->

| Data Type | Classification | Retention Period | Access Level |
|-----------|---------------|------------------|-------------|
| ${6:Personal Data} | ${7:Sensitive} | ${8:7 years} | ${9:Restricted} |

## Privacy Impact Assessment

${10:Privacy impact analysis}

## Legal & Compliance Requirements

* ${11:GDPR Article reference}
* ${12:Other compliance requirements}

## Decision

${13:Data protection decision}

## Data Lifecycle Management

### Collection
${14:Data collection practices}

### Storage
${15:Data storage approach}

### Processing
${16:Data processing methods}

### Retention
${17:Data retention policy}

### Disposal
${18:Secure data disposal}

## Consequences

### Privacy Benefits
* ${19:Privacy benefit 1}

### Compliance Risks
* ${20:Compliance risk 1}

## Audit Trail

${21:Audit and monitoring approach}
"#.to_string(),
            },
        ];

        Self {
            metadata: PluginMetadata {
                name: "Security Analyzer".to_string(),
                version: "1.0.0".to_string(),
                description: "Advanced security-focused drift detection and ADR templates".to_string(),
                author: "PhotonDrift Security Team".to_string(),
                capabilities: vec![
                    PluginCapability::DriftAnalysis,
                    PluginCapability::TemplateGeneration,
                ],
                api_version: "1.0.0".to_string(),
            },
            security_patterns,
            templates,
        }
    }
}

impl Plugin for SecurityAnalyzerPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &self.metadata
    }

    fn initialize(&mut self, _context: &PluginContext) -> PluginResult<()> {
        log::info!("Security Analyzer Plugin initialized");
        Ok(())
    }

    fn execute(&self, context: &PluginContext) -> PluginResult<PluginResponse> {
        match context.action.as_str() {
            "analyze_security_drift" => self.analyze_security_drift(context),
            "get_security_templates" => self.get_security_templates(),
            "validate_security_compliance" => self.validate_security_compliance(context),
            _ => Err(adrscan::plugins::PluginLoadError::InvalidAction(context.action.clone())),
        }
    }

    fn shutdown(&mut self) -> PluginResult<()> {
        log::info!("Security Analyzer Plugin shutting down");
        Ok(())
    }
}

impl DriftAnalysisPlugin for SecurityAnalyzerPlugin {
    fn analyze_drift(&self, file_path: &str, content: &str) -> PluginResult<Vec<serde_json::Value>> {
        let mut results = Vec::new();

        for pattern in &self.security_patterns {
            if let Ok(regex) = Regex::new(&pattern.pattern) {
                for (line_num, line) in content.lines().enumerate() {
                    if regex.is_match(line) {
                        let result = SecurityDriftResult {
                            pattern_name: pattern.name.clone(),
                            severity: pattern.severity.clone(),
                            category: pattern.category.clone(),
                            description: pattern.description.clone(),
                            file_path: file_path.to_string(),
                            line_number: Some(line_num + 1),
                            recommendation: self.get_security_recommendation(&pattern.category),
                        };
                        results.push(serde_json::to_value(result).unwrap());
                    }
                }
            }
        }

        Ok(results)
    }

    fn get_severity_level(&self, _drift_type: &str) -> String {
        "security".to_string()
    }
}

impl TemplatePlugin for SecurityAnalyzerPlugin {
    fn get_templates(&self) -> PluginResult<Vec<serde_json::Value>> {
        Ok(self.templates.iter()
            .map(|t| serde_json::to_value(t).unwrap())
            .collect())
    }

    fn generate_template(&self, template_name: &str, _context: &HashMap<String, String>) -> PluginResult<String> {
        self.templates.iter()
            .find(|t| t.name == template_name)
            .map(|t| t.template_content.clone())
            .ok_or_else(|| adrscan::plugins::PluginLoadError::TemplateNotFound(template_name.to_string()))
    }
}

impl SecurityAnalyzerPlugin {
    fn analyze_security_drift(&self, context: &PluginContext) -> PluginResult<PluginResponse> {
        let file_path = context.parameters.get("file_path")
            .ok_or_else(|| adrscan::plugins::PluginLoadError::MissingParameter("file_path".to_string()))?;
        
        let content = context.parameters.get("content")
            .ok_or_else(|| adrscan::plugins::PluginLoadError::MissingParameter("content".to_string()))?;

        let drift_results = self.analyze_drift(file_path, content)?;
        
        Ok(PluginResponse {
            success: true,
            data: Some(serde_json::json!({
                "security_drift_results": drift_results,
                "total_issues": drift_results.len(),
                "high_severity_count": drift_results.iter()
                    .filter(|r| {
                        r.get("severity").and_then(|s| s.as_str()) == Some("Critical") ||
                        r.get("severity").and_then(|s| s.as_str()) == Some("High")
                    })
                    .count()
            })),
            message: Some(format!("Analyzed security drift: {} issues found", drift_results.len())),
            error: None,
        })
    }

    fn get_security_templates(&self) -> PluginResult<PluginResponse> {
        let templates = self.get_templates()?;
        
        Ok(PluginResponse {
            success: true,
            data: Some(serde_json::json!({
                "templates": templates,
                "categories": ["Authentication", "Authorization", "DataProtection", "NetworkSecurity", "Cryptography", "InputValidation"]
            })),
            message: Some(format!("Retrieved {} security templates", templates.len())),
            error: None,
        })
    }

    fn validate_security_compliance(&self, context: &PluginContext) -> PluginResult<PluginResponse> {
        let adr_content = context.parameters.get("adr_content")
            .ok_or_else(|| adrscan::plugins::PluginLoadError::MissingParameter("adr_content".to_string()))?;

        let compliance_issues = self.check_security_compliance(adr_content);
        
        Ok(PluginResponse {
            success: true,
            data: Some(serde_json::json!({
                "compliance_issues": compliance_issues,
                "is_compliant": compliance_issues.is_empty(),
                "security_score": self.calculate_security_score(&compliance_issues)
            })),
            message: Some(format!("Security compliance validation complete: {} issues", compliance_issues.len())),
            error: None,
        })
    }

    fn get_security_recommendation(&self, category: &SecurityCategory) -> String {
        match category {
            SecurityCategory::Authentication => "Consider implementing ADR for authentication mechanisms and multi-factor authentication".to_string(),
            SecurityCategory::Authorization => "Document authorization model and access control decisions in dedicated ADR".to_string(),
            SecurityCategory::DataProtection => "Create ADR for data classification and protection strategies".to_string(),
            SecurityCategory::NetworkSecurity => "Document network security architecture and transport encryption decisions".to_string(),
            SecurityCategory::Cryptography => "Establish ADR for cryptographic standards and key management practices".to_string(),
            SecurityCategory::InputValidation => "Create ADR for input validation strategies and sanitization approaches".to_string(),
        }
    }

    fn check_security_compliance(&self, adr_content: &str) -> Vec<String> {
        let mut issues = Vec::new();
        
        // Check for required security sections
        if !adr_content.contains("## Security") && !adr_content.contains("security") {
            issues.push("Missing security considerations section".to_string());
        }
        
        if !adr_content.contains("threat") && !adr_content.contains("risk") {
            issues.push("No threat analysis or risk assessment mentioned".to_string());
        }
        
        if !adr_content.contains("compliance") && !adr_content.contains("regulation") {
            issues.push("No compliance requirements addressed".to_string());
        }
        
        if adr_content.contains("http://") && !adr_content.contains("localhost") {
            issues.push("Insecure HTTP references found".to_string());
        }
        
        issues
    }

    fn calculate_security_score(&self, issues: &[String]) -> u8 {
        match issues.len() {
            0 => 100,
            1 => 85,
            2 => 70,
            3 => 55,
            4 => 40,
            _ => 25,
        }
    }
}

// Plugin entry point for dynamic loading
#[no_mangle]
pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
    Box::new(SecurityAnalyzerPlugin::new())
}