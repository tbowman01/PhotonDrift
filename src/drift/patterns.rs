//! Technology Detection Patterns
//! 
//! This module defines patterns for detecting various technologies,
//! frameworks, and architectural elements in codebases.

use serde::{Deserialize, Serialize};
use std::path::Path;
use regex::Regex;
use crate::error::AdrscanError;
use crate::drift::DriftResult;

/// Re-export the DetectionPattern from config for use in drift detection
pub use crate::config::DetectionPattern;

/// A technology match found in the codebase
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnologyMatch {
    /// The detection pattern that matched
    pub pattern: DetectionPattern,
    
    /// File where the match was found
    pub file_path: String,
    
    /// Line number where the match was found
    pub line_number: usize,
    
    /// The actual text that matched
    pub matched_text: String,
    
    /// Context around the match (full line or snippet)
    pub context: String,
    
    /// Confidence level of the match (0.0 to 1.0)
    pub confidence: f64,
}

/// Built-in technology detection patterns
pub struct BuiltinPatterns;

impl BuiltinPatterns {
    /// Get default patterns for common technologies
    pub fn default_patterns() -> Vec<DetectionPattern> {
        vec![
            // Database Technologies
            DetectionPattern {
                name: "PostgreSQL Database".to_string(),
                file_pattern: "**/Cargo.toml".to_string(),
                content_pattern: r#"(postgres|tokio-postgres|diesel.*postgres)"#.to_string(),
                category: "database".to_string(),
            },
            DetectionPattern {
                name: "MySQL Database".to_string(),
                file_pattern: "**/Cargo.toml".to_string(),
                content_pattern: r#"(mysql|tokio-mysql|diesel.*mysql)"#.to_string(),
                category: "database".to_string(),
            },
            DetectionPattern {
                name: "SQLite Database".to_string(),
                file_pattern: "**/Cargo.toml".to_string(),
                content_pattern: r#"(sqlite|rusqlite|diesel.*sqlite)"#.to_string(),
                category: "database".to_string(),
            },
            DetectionPattern {
                name: "MongoDB Database".to_string(),
                file_pattern: "**/Cargo.toml".to_string(),
                content_pattern: r#"(mongodb|bson)"#.to_string(),
                category: "database".to_string(),
            },
            DetectionPattern {
                name: "Redis Cache".to_string(),
                file_pattern: "**/Cargo.toml".to_string(),
                content_pattern: r#"(redis|darkredis)"#.to_string(),
                category: "database".to_string(),
            },
            
            // Web Frameworks (Rust)
            DetectionPattern {
                name: "Axum Web Framework".to_string(),
                file_pattern: "**/Cargo.toml".to_string(),
                content_pattern: r#"axum\s*="#.to_string(),
                category: "framework".to_string(),
            },
            DetectionPattern {
                name: "Actix Web Framework".to_string(),
                file_pattern: "**/Cargo.toml".to_string(),
                content_pattern: r#"actix-web\s*="#.to_string(),
                category: "framework".to_string(),
            },
            DetectionPattern {
                name: "Warp Web Framework".to_string(),
                file_pattern: "**/Cargo.toml".to_string(),
                content_pattern: r#"warp\s*="#.to_string(),
                category: "framework".to_string(),
            },
            DetectionPattern {
                name: "Rocket Web Framework".to_string(),
                file_pattern: "**/Cargo.toml".to_string(),
                content_pattern: r#"rocket\s*="#.to_string(),
                category: "framework".to_string(),
            },
            
            // Cloud Providers (Infrastructure as Code)
            DetectionPattern {
                name: "AWS Provider".to_string(),
                file_pattern: "**/*.tf".to_string(),
                content_pattern: r#"provider\s+"aws""#.to_string(),
                category: "cloud".to_string(),
            },
            DetectionPattern {
                name: "Azure Provider".to_string(),
                file_pattern: "**/*.tf".to_string(),
                content_pattern: r#"provider\s+"azurerm""#.to_string(),
                category: "cloud".to_string(),
            },
            DetectionPattern {
                name: "Google Cloud Provider".to_string(),
                file_pattern: "**/*.tf".to_string(),
                content_pattern: r#"provider\s+"google""#.to_string(),
                category: "cloud".to_string(),
            },
            
            // JavaScript/TypeScript Frameworks
            DetectionPattern {
                name: "React Framework".to_string(),
                file_pattern: "**/package.json".to_string(),
                content_pattern: r#""react"\s*:"#.to_string(),
                category: "framework".to_string(),
            },
            DetectionPattern {
                name: "Vue.js Framework".to_string(),
                file_pattern: "**/package.json".to_string(),
                content_pattern: r#""vue"\s*:"#.to_string(),
                category: "framework".to_string(),
            },
            DetectionPattern {
                name: "Angular Framework".to_string(),
                file_pattern: "**/package.json".to_string(),
                content_pattern: r#""@angular/core"\s*:"#.to_string(),
                category: "framework".to_string(),
            },
            DetectionPattern {
                name: "Express.js Framework".to_string(),
                file_pattern: "**/package.json".to_string(),
                content_pattern: r#""express"\s*:"#.to_string(),
                category: "framework".to_string(),
            },
            DetectionPattern {
                name: "Next.js Framework".to_string(),
                file_pattern: "**/package.json".to_string(),
                content_pattern: r#""next"\s*:"#.to_string(),
                category: "framework".to_string(),
            },
            
            // Authentication Libraries
            DetectionPattern {
                name: "JWT Authentication".to_string(),
                file_pattern: "**/*.{rs,js,ts,py}".to_string(),
                content_pattern: r#"(jsonwebtoken|jwt|JWT)"#.to_string(),
                category: "authentication".to_string(),
            },
            DetectionPattern {
                name: "OAuth Implementation".to_string(),
                file_pattern: "**/*.{rs,js,ts,py}".to_string(),
                content_pattern: r#"(oauth|OAuth|passport)"#.to_string(),
                category: "authentication".to_string(),
            },
            
            // Container Technology
            DetectionPattern {
                name: "Docker Usage".to_string(),
                file_pattern: "**/Dockerfile".to_string(),
                content_pattern: r#"FROM\s+"#.to_string(),
                category: "infrastructure".to_string(),
            },
            DetectionPattern {
                name: "Kubernetes Deployment".to_string(),
                file_pattern: "**/*.{yaml,yml}".to_string(),
                content_pattern: r#"apiVersion:\s*(apps/v1|v1)"#.to_string(),
                category: "infrastructure".to_string(),
            },
            
            // Message Queues
            DetectionPattern {
                name: "RabbitMQ".to_string(),
                file_pattern: "**/*.{rs,js,ts,py,java}".to_string(),
                content_pattern: r#"(rabbitmq|amqp)"#.to_string(),
                category: "messaging".to_string(),
            },
            DetectionPattern {
                name: "Apache Kafka".to_string(),
                file_pattern: "**/*.{rs,js,ts,py,java}".to_string(),
                content_pattern: r#"(kafka|rdkafka)"#.to_string(),
                category: "messaging".to_string(),
            },
            
            // Monitoring and Observability
            DetectionPattern {
                name: "Prometheus Metrics".to_string(),
                file_pattern: "**/*.{rs,js,ts,py}".to_string(),
                content_pattern: r#"(prometheus|metrics)"#.to_string(),
                category: "monitoring".to_string(),
            },
            DetectionPattern {
                name: "OpenTelemetry".to_string(),
                file_pattern: "**/*.{rs,js,ts,py}".to_string(),
                content_pattern: r#"(opentelemetry|tracing)"#.to_string(),
                category: "monitoring".to_string(),
            },
        ]
    }
    
    /// Get patterns for Python projects
    pub fn python_patterns() -> Vec<DetectionPattern> {
        vec![
            DetectionPattern {
                name: "Django Framework".to_string(),
                file_pattern: "**/requirements.txt".to_string(),
                content_pattern: r#"[Dd]jango"#.to_string(),
                category: "framework".to_string(),
            },
            DetectionPattern {
                name: "Flask Framework".to_string(),
                file_pattern: "**/requirements.txt".to_string(),
                content_pattern: r#"[Ff]lask"#.to_string(),
                category: "framework".to_string(),
            },
            DetectionPattern {
                name: "FastAPI Framework".to_string(),
                file_pattern: "**/requirements.txt".to_string(),
                content_pattern: r#"fastapi"#.to_string(),
                category: "framework".to_string(),
            },
            DetectionPattern {
                name: "SQLAlchemy ORM".to_string(),
                file_pattern: "**/requirements.txt".to_string(),
                content_pattern: r#"[Ss][Qq][Ll][Aa]lchemy"#.to_string(),
                category: "database".to_string(),
            },
        ]
    }
    
    /// Get patterns for Java projects
    pub fn java_patterns() -> Vec<DetectionPattern> {
        vec![
            DetectionPattern {
                name: "Spring Framework".to_string(),
                file_pattern: "**/pom.xml".to_string(),
                content_pattern: r#"<groupId>org\.springframework"#.to_string(),
                category: "framework".to_string(),
            },
            DetectionPattern {
                name: "Spring Boot".to_string(),
                file_pattern: "**/pom.xml".to_string(),
                content_pattern: r#"spring-boot-starter"#.to_string(),
                category: "framework".to_string(),
            },
            DetectionPattern {
                name: "Hibernate ORM".to_string(),
                file_pattern: "**/pom.xml".to_string(),
                content_pattern: r#"<groupId>org\.hibernate"#.to_string(),
                category: "database".to_string(),
            },
        ]
    }
}

/// Pattern matcher for detecting technologies in files
pub struct PatternMatcher {
    compiled_patterns: Vec<CompiledPattern>,
}

/// A compiled detection pattern with cached regex
struct CompiledPattern {
    pattern: DetectionPattern,
    regex: Regex,
    file_matcher: glob::Pattern,
}

impl PatternMatcher {
    /// Create a new pattern matcher with the given patterns
    pub fn new(patterns: &[DetectionPattern]) -> DriftResult<Self> {
        let mut compiled_patterns = Vec::new();
        
        for pattern in patterns {
            let regex = Regex::new(&pattern.content_pattern)
                .map_err(|e| AdrscanError::DriftError(
                    format!("Invalid regex pattern '{}': {}", pattern.content_pattern, e)
                ))?;
            
            let file_matcher = glob::Pattern::new(&pattern.file_pattern)
                .map_err(|e| AdrscanError::DriftError(
                    format!("Invalid file pattern '{}': {}", pattern.file_pattern, e)
                ))?;
            
            compiled_patterns.push(CompiledPattern {
                pattern: pattern.clone(),
                regex,
                file_matcher,
            });
        }
        
        Ok(Self { compiled_patterns })
    }
    
    /// Check if a file path matches any pattern
    pub fn matches_file(&self, file_path: &Path) -> Vec<&DetectionPattern> {
        let path_str = file_path.to_string_lossy();
        self.compiled_patterns
            .iter()
            .filter(|cp| cp.file_matcher.matches(&path_str))
            .map(|cp| &cp.pattern)
            .collect()
    }
    
    /// Find all technology matches in a file
    pub fn find_matches(&self, file_path: &Path, content: &str) -> DriftResult<Vec<TechnologyMatch>> {
        let mut matches = Vec::new();
        let path_str = file_path.to_string_lossy().to_string();
        
        for compiled_pattern in &self.compiled_patterns {
            if !compiled_pattern.file_matcher.matches(&path_str) {
                continue;
            }
            
            for (line_number, line) in content.lines().enumerate() {
                if let Some(captures) = compiled_pattern.regex.captures(line) {
                    let matched_text = captures.get(0).unwrap().as_str().to_string();
                    
                    matches.push(TechnologyMatch {
                        pattern: compiled_pattern.pattern.clone(),
                        file_path: path_str.clone(),
                        line_number: line_number + 1,
                        matched_text,
                        context: line.to_string(),
                        confidence: self.calculate_confidence(&compiled_pattern.pattern, line),
                    });
                }
            }
        }
        
        Ok(matches)
    }
    
    /// Calculate confidence score for a match
    fn calculate_confidence(&self, pattern: &DetectionPattern, line: &str) -> f64 {
        let mut confidence = 0.8; // Base confidence
        
        // Increase confidence for exact matches
        if line.contains(&pattern.name) {
            confidence += 0.1;
        }
        
        // Decrease confidence for comments
        if line.trim_start().starts_with('#') || 
           line.trim_start().starts_with("//") ||
           line.trim_start().starts_with("/*") {
            confidence -= 0.2;
        }
        
        // Increase confidence for dependency declarations
        if line.contains("=") || line.contains(":") {
            confidence += 0.1;
        }
        
        let result: f64 = confidence;
        result.max(0.0).min(1.0)
    }
}

impl TechnologyMatch {
    /// Check if this match is likely in a comment
    pub fn is_likely_comment(&self) -> bool {
        let trimmed = self.context.trim_start();
        trimmed.starts_with('#') || 
        trimmed.starts_with("//") || 
        trimmed.starts_with("/*") ||
        trimmed.starts_with('*')
    }
    
    /// Get a short description of this match
    pub fn description(&self) -> String {
        format!(
            "{} detected in {} at line {} (confidence: {:.1}%)",
            self.pattern.name,
            self.file_path,
            self.line_number,
            self.confidence * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_builtin_patterns() {
        let patterns = BuiltinPatterns::default_patterns();
        assert!(!patterns.is_empty());
        
        // Check for specific technology patterns
        assert!(patterns.iter().any(|p| p.name.contains("PostgreSQL")));
        assert!(patterns.iter().any(|p| p.name.contains("Redis")));
        assert!(patterns.iter().any(|p| p.name.contains("Docker")));
        assert!(patterns.iter().any(|p| p.name.contains("AWS")));
        
        // Check categories
        assert!(patterns.iter().any(|p| p.category == "database"));
        assert!(patterns.iter().any(|p| p.category == "framework"));
        assert!(patterns.iter().any(|p| p.category == "cloud"));
    }

    #[test]
    fn test_python_patterns() {
        let patterns = BuiltinPatterns::python_patterns();
        assert!(!patterns.is_empty());
        
        assert!(patterns.iter().any(|p| p.name.contains("Django")));
        assert!(patterns.iter().any(|p| p.name.contains("Flask")));
        assert!(patterns.iter().any(|p| p.name.contains("FastAPI")));
    }

    #[test]
    fn test_java_patterns() {
        let patterns = BuiltinPatterns::java_patterns();
        assert!(!patterns.is_empty());
        
        assert!(patterns.iter().any(|p| p.name.contains("Spring")));
        assert!(patterns.iter().any(|p| p.name.contains("Hibernate")));
    }

    #[test]
    fn test_pattern_matcher_creation() {
        let patterns = vec![
            DetectionPattern {
                name: "Test Pattern".to_string(),
                file_pattern: "**/*.rs".to_string(),
                content_pattern: r"use\s+test".to_string(),
                category: "test".to_string(),
            }
        ];
        
        let matcher = PatternMatcher::new(&patterns);
        assert!(matcher.is_ok());
    }

    #[test]
    fn test_pattern_matcher_invalid_regex() {
        let patterns = vec![
            DetectionPattern {
                name: "Invalid Pattern".to_string(),
                file_pattern: "**/*.rs".to_string(),
                content_pattern: "[invalid_regex".to_string(), // Invalid regex
                category: "test".to_string(),
            }
        ];
        
        let matcher = PatternMatcher::new(&patterns);
        assert!(matcher.is_err());
    }

    #[test]
    fn test_pattern_matcher_invalid_glob() {
        let patterns = vec![
            DetectionPattern {
                name: "Invalid Glob".to_string(),
                file_pattern: "[".to_string(), // Invalid glob
                content_pattern: "test".to_string(),
                category: "test".to_string(),
            }
        ];
        
        let matcher = PatternMatcher::new(&patterns);
        assert!(matcher.is_err());
    }

    #[test]
    fn test_file_matching() {
        let patterns = vec![
            DetectionPattern {
                name: "Rust Pattern".to_string(),
                file_pattern: "**/*.rs".to_string(),
                content_pattern: "fn".to_string(),
                category: "rust".to_string(),
            },
            DetectionPattern {
                name: "JSON Pattern".to_string(),
                file_pattern: "**/*.json".to_string(),
                content_pattern: r#""name""#.to_string(),
                category: "config".to_string(),
            }
        ];
        
        let matcher = PatternMatcher::new(&patterns).unwrap();
        
        // Test matching files
        let rust_file = Path::new("src/main.rs");
        let matches = matcher.matches_file(rust_file);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].name, "Rust Pattern");
        
        let json_file = Path::new("package.json");
        let matches = matcher.matches_file(json_file);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].name, "JSON Pattern");
        
        // Test non-matching file
        let txt_file = Path::new("readme.txt");
        let matches = matcher.matches_file(txt_file);
        assert_eq!(matches.len(), 0);
    }

    #[test]
    fn test_content_matching() {
        let patterns = vec![
            DetectionPattern {
                name: "Rust Function".to_string(),
                file_pattern: "**/*.rs".to_string(),
                content_pattern: r"fn\s+\w+".to_string(),
                category: "rust".to_string(),
            },
            DetectionPattern {
                name: "Use Statement".to_string(),
                file_pattern: "**/*.rs".to_string(),
                content_pattern: r"use\s+\w+".to_string(),
                category: "rust".to_string(),
            }
        ];
        
        let matcher = PatternMatcher::new(&patterns).unwrap();
        
        let rust_content = r#"
use std::collections::HashMap;
use serde::Serialize;

fn main() {
    println!("Hello, world!");
}

fn helper_function() -> i32 {
    42
}
"#;
        
        let rust_file = Path::new("src/main.rs");
        let matches = matcher.find_matches(rust_file, rust_content).unwrap();
        
        // Should find multiple matches
        assert!(!matches.is_empty());
        
        // Check for function matches
        let fn_matches: Vec<_> = matches.iter()
            .filter(|m| m.pattern.name == "Rust Function")
            .collect();
        assert!(fn_matches.len() >= 2); // main and helper_function
        
        // Check for use statement matches
        let use_matches: Vec<_> = matches.iter()
            .filter(|m| m.pattern.name == "Use Statement")
            .collect();
        assert!(use_matches.len() >= 2); // std and serde imports
        
        // Verify match details
        for tech_match in &matches {
            assert!(!tech_match.matched_text.is_empty());
            assert!(!tech_match.context.is_empty());
            assert!(tech_match.line_number > 0);
            assert!(tech_match.confidence > 0.0 && tech_match.confidence <= 1.0);
        }
    }

    #[test]
    fn test_confidence_calculation() {
        let patterns = vec![
            DetectionPattern {
                name: "Test Pattern".to_string(),
                file_pattern: "**/*.rs".to_string(),
                content_pattern: "test".to_string(),
                category: "test".to_string(),
            }
        ];
        
        let matcher = PatternMatcher::new(&patterns).unwrap();
        
        // Test normal line
        let normal_content = "let test = 42;";
        let matches = matcher.find_matches(Path::new("test.rs"), normal_content).unwrap();
        assert!(!matches.is_empty());
        let normal_confidence = matches[0].confidence;
        
        // Test comment line (should have lower confidence)
        let comment_content = "// This is a test comment";
        let matches = matcher.find_matches(Path::new("test.rs"), comment_content).unwrap();
        assert!(!matches.is_empty());
        let comment_confidence = matches[0].confidence;
        
        assert!(comment_confidence < normal_confidence);
        
        // Test dependency line (should have higher confidence)
        let dep_content = r#"test = "1.0""#;
        let matches = matcher.find_matches(Path::new("test.rs"), dep_content).unwrap();
        assert!(!matches.is_empty());
        let dep_confidence = matches[0].confidence;
        
        assert!(dep_confidence >= normal_confidence);
    }

    #[test]
    fn test_technology_match_properties() {
        let tech_match = TechnologyMatch {
            pattern: DetectionPattern {
                name: "Test Tech".to_string(),
                file_pattern: "**/*.rs".to_string(),
                content_pattern: "test".to_string(),
                category: "test".to_string(),
            },
            file_path: "src/test.rs".to_string(),
            line_number: 10,
            matched_text: "test".to_string(),
            context: "let test = 42;".to_string(),
            confidence: 0.85,
        };
        
        assert!(!tech_match.is_likely_comment());
        
        let description = tech_match.description();
        assert!(description.contains("Test Tech"));
        assert!(description.contains("src/test.rs"));
        assert!(description.contains("line 10"));
        assert!(description.contains("85.0%"));
        
        // Test comment detection
        let comment_match = TechnologyMatch {
            pattern: tech_match.pattern.clone(),
            file_path: "src/test.rs".to_string(),
            line_number: 5,
            matched_text: "test".to_string(),
            context: "// This is a test comment".to_string(),
            confidence: 0.6,
        };
        
        assert!(comment_match.is_likely_comment());
    }

    #[test]
    fn test_real_world_cargo_toml() {
        let patterns = BuiltinPatterns::default_patterns();
        let matcher = PatternMatcher::new(&patterns).unwrap();
        
        let cargo_content = r#"
[package]
name = "my-app"
version = "0.1.0"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
axum = "0.6"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
diesel = { version = "2.0", features = ["postgres"] }
redis = "0.23"
"#;
        
        let cargo_file = Path::new("Cargo.toml");
        let matches = matcher.find_matches(cargo_file, cargo_content).unwrap();
        
        // Should detect multiple technologies
        let tech_names: Vec<String> = matches.iter()
            .map(|m| m.pattern.name.clone())
            .collect();
        
        // Check for expected detections
        assert!(tech_names.iter().any(|name| name.contains("Axum")));
        assert!(tech_names.iter().any(|name| name.contains("PostgreSQL")));
        assert!(tech_names.iter().any(|name| name.contains("Redis")));
    }

    #[test]
    fn test_real_world_terraform() {
        let patterns = BuiltinPatterns::default_patterns();
        let matcher = PatternMatcher::new(&patterns).unwrap();
        
        let tf_content = r#"
provider "aws" {
  region = "us-west-2"
}

resource "aws_instance" "web" {
  ami           = "ami-0c55b159cbfafe1d0"
  instance_type = "t2.micro"
}

provider "azurerm" {
  features {}
}
"#;
        
        let tf_file = Path::new("main.tf");
        let matches = matcher.find_matches(tf_file, tf_content).unwrap();
        
        let tech_names: Vec<String> = matches.iter()
            .map(|m| m.pattern.name.clone())
            .collect();
        
        // Should detect cloud providers
        assert!(tech_names.iter().any(|name| name.contains("AWS")));
        assert!(tech_names.iter().any(|name| name.contains("Azure")));
    }

    #[test]
    fn test_real_world_package_json() {
        let patterns = BuiltinPatterns::default_patterns();
        let matcher = PatternMatcher::new(&patterns).unwrap();
        
        let package_content = r#"
{
  "name": "my-app",
  "version": "1.0.0",
  "dependencies": {
    "react": "^18.0.0",
    "express": "^4.18.0",
    "@angular/core": "^15.0.0",
    "next": "^13.0.0"
  }
}
"#;
        
        let package_file = Path::new("package.json");
        let matches = matcher.find_matches(package_file, package_content).unwrap();
        
        let tech_names: Vec<String> = matches.iter()
            .map(|m| m.pattern.name.clone())
            .collect();
        
        // Should detect multiple frameworks
        assert!(tech_names.iter().any(|name| name.contains("React")));
        assert!(tech_names.iter().any(|name| name.contains("Express")));
        assert!(tech_names.iter().any(|name| name.contains("Angular")));
        assert!(tech_names.iter().any(|name| name.contains("Next")));
    }
}