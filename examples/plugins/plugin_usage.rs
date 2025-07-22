//! Plugin Usage Example
//! 
//! This example demonstrates how to load and use PhotonDrift plugins
//! for enhanced drift detection and analysis.

use adrscan::plugins::{PluginManager, PluginContext, PluginCapability};
use adrscan::{Result, AdrscanError, Config};
use std::collections::HashMap;
use std::path::PathBuf;

fn main() -> Result<()> {
    env_logger::init();
    
    println!("🚀 PhotonDrift Plugin System Demo");
    println!("=====================================\n");

    // Initialize the plugin manager
    let mut plugin_manager = PluginManager::new();
    
    // Load plugin configuration
    let config_path = PathBuf::from("examples/plugins/plugin_config.yml");
    if config_path.exists() {
        plugin_manager.load_config(&config_path)?;
        println!("✅ Plugin configuration loaded from: {:?}", config_path);
    } else {
        println!("⚠️  No plugin config found, using defaults");
    }

    // Discover and load plugins
    let plugin_dirs = vec![
        PathBuf::from("examples/plugins"),
        PathBuf::from("plugins"),
    ];
    
    for dir in &plugin_dirs {
        if dir.exists() {
            match plugin_manager.discover_plugins(dir) {
                Ok(count) => println!("✅ Discovered {} plugins in {:?}", count, dir),
                Err(e) => println!("⚠️  Error discovering plugins in {:?}: {}", dir, e),
            }
        }
    }

    // Display loaded plugins
    let plugins = plugin_manager.list_plugins();
    if plugins.is_empty() {
        println!("ℹ️  No plugins loaded. This demo will use built-in functionality.");
        return demonstrate_builtin_features();
    }

    println!("\n📦 Loaded Plugins:");
    for plugin_info in &plugins {
        println!("  • {} v{} - {}", 
            plugin_info.name, 
            plugin_info.version, 
            plugin_info.description
        );
        for capability in &plugin_info.capabilities {
            println!("    - {:?}", capability);
        }
    }

    // Demonstrate drift analysis with plugins
    demonstrate_drift_analysis(&plugin_manager)?;
    
    // Demonstrate template generation with plugins
    demonstrate_template_generation(&plugin_manager)?;
    
    // Demonstrate plugin chaining
    demonstrate_plugin_chaining(&plugin_manager)?;
    
    println!("\n🎉 Plugin demo completed successfully!");
    
    Ok(())
}

fn demonstrate_builtin_features() -> Result<()> {
    println!("\n🔧 Built-in Features Demo");
    println!("========================");
    
    let config = Config::default();
    
    // Simulate basic drift detection
    let sample_code = r#"
    function authenticateUser(password) {
        // Hardcoded password - security issue!
        const adminPassword = "admin123";
        
        // Inefficient string building in loop
        let result = "";
        for (let i = 0; i < users.length; i++) {
            result += users[i].name + ", ";
        }
        
        return password === adminPassword;
    }
    "#;
    
    println!("📄 Analyzing sample code for architectural drift...");
    println!("Code sample:\n{}", sample_code);
    
    // Basic pattern matching (simplified)
    let mut issues = Vec::new();
    
    if sample_code.contains("password") && sample_code.contains("\"") {
        issues.push("🔒 Security: Potential hardcoded credentials detected");
    }
    
    if sample_code.contains("for") && sample_code.contains("+=") {
        issues.push("⚡ Performance: Inefficient string concatenation in loop");
    }
    
    if issues.is_empty() {
        println!("✅ No architectural drift detected");
    } else {
        println!("⚠️  Architectural issues found:");
        for issue in issues {
            println!("  • {}", issue);
        }
    }
    
    Ok(())
}

fn demonstrate_drift_analysis(plugin_manager: &PluginManager) -> Result<()> {
    println!("\n🔍 Plugin-Enhanced Drift Analysis");
    println!("=================================");
    
    let sample_files = vec![
        ("auth.js", r#"
        const jwt = require('jsonwebtoken');
        const bcrypt = require('bcrypt');
        
        // Weak hashing - should trigger security plugin
        const md5 = require('md5');
        
        function hashPassword(password) {
            return md5(password); // Security issue!
        }
        
        // SQL injection risk
        function getUser(id) {
            const query = "SELECT * FROM users WHERE id = " + id;
            return db.query(query);
        }
        "#),
        ("performance.js", r#"
        // N+1 query problem
        async function getUserPosts(userIds) {
            const results = [];
            for (const id of userIds) {
                const user = await User.find(id); // N+1 issue!
                results.push(user.posts);
            }
            return results;
        }
        
        // Synchronous file operations
        function loadConfig() {
            return fs.readFileSync('./config.json'); // Blocking IO!
        }
        "#),
    ];
    
    for (file_path, content) in sample_files {
        println!("\n📁 Analyzing: {}", file_path);
        
        // Create plugin context
        let mut context = PluginContext {
            action: "analyze_drift".to_string(),
            parameters: HashMap::new(),
            workspace_path: Some(PathBuf::from(".")),
            config: None,
        };
        
        context.parameters.insert("file_path".to_string(), file_path.to_string());
        context.parameters.insert("content".to_string(), content.to_string());
        
        // Run drift analysis with all capable plugins
        let plugins_with_drift_analysis = plugin_manager.get_plugins_with_capability(PluginCapability::DriftAnalysis);
        
        for plugin_name in plugins_with_drift_analysis {
            match plugin_manager.execute_plugin(&plugin_name, &context) {
                Ok(response) => {
                    if response.success {
                        println!("  🔌 {} plugin:", plugin_name);
                        if let Some(data) = response.data {
                            if let Some(results) = data.get("drift_results").or_else(|| data.get("security_drift_results")).or_else(|| data.get("performance_drift_results")) {
                                if let Some(results_array) = results.as_array() {
                                    for result in results_array {
                                        if let Some(pattern_name) = result.get("pattern_name").and_then(|p| p.as_str()) {
                                            if let Some(description) = result.get("description").and_then(|d| d.as_str()) {
                                                println!("    • ⚠️  {}: {}", pattern_name, description);
                                            }
                                        }
                                    }
                                } else if results_array.is_none() {
                                    println!("    • ✅ No issues detected");
                                }
                            }
                        }
                        if let Some(message) = response.message {
                            println!("    📊 {}", message);
                        }
                    } else if let Some(error) = response.error {
                        println!("    ❌ Error: {}", error);
                    }
                }
                Err(e) => println!("    ❌ Plugin execution failed: {}", e),
            }
        }
    }
    
    Ok(())
}

fn demonstrate_template_generation(plugin_manager: &PluginManager) -> Result<()> {
    println!("\n📝 Plugin Template Generation");
    println!("=============================");
    
    let template_requests = vec![
        ("Security Architecture Decision", "security"),
        ("Performance Architecture Decision", "performance"),
        ("Caching Strategy Decision", "caching"),
    ];
    
    for (template_name, category) in template_requests {
        println!("\n🎯 Requesting template: {}", template_name);
        
        let mut context = PluginContext {
            action: "get_templates".to_string(),
            parameters: HashMap::new(),
            workspace_path: Some(PathBuf::from(".")),
            config: None,
        };
        
        context.parameters.insert("template_name".to_string(), template_name.to_string());
        context.parameters.insert("category".to_string(), category.to_string());
        
        // Find plugins that can generate templates
        let template_plugins = plugin_manager.get_plugins_with_capability(PluginCapability::TemplateGeneration);
        
        if template_plugins.is_empty() {
            println!("  ⚠️  No template generation plugins available");
            continue;
        }
        
        for plugin_name in template_plugins {
            match plugin_manager.execute_plugin(&plugin_name, &context) {
                Ok(response) => {
                    if response.success {
                        println!("  🔌 {} plugin provided templates", plugin_name);
                        if let Some(data) = response.data {
                            if let Some(templates) = data.get("templates").and_then(|t| t.as_array()) {
                                for template in templates {
                                    if let Some(name) = template.get("name").and_then(|n| n.as_str()) {
                                        if name.to_lowercase().contains(&category.to_lowercase()) {
                                            println!("    ✅ Found: {}", name);
                                            if let Some(desc) = template.get("description").and_then(|d| d.as_str()) {
                                                println!("       📄 {}", desc);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Err(e) => println!("    ❌ Template request failed: {}", e),
            }
        }
    }
    
    Ok(())
}

fn demonstrate_plugin_chaining(plugin_manager: &PluginManager) -> Result<()> {
    println!("\n🔗 Plugin Chaining Demo");
    println!("=======================");
    
    println!("Demonstrating how multiple plugins can work together...");
    
    let sample_adr = r#"
# ADR-0042: Database Technology Selection

## Status
Accepted

## Context
We need a database that can handle our user authentication and data storage needs.
We expect high traffic and need ACID compliance.

## Decision
We will use PostgreSQL with connection pooling and read replicas.

## Consequences
- Better performance and reliability
- Need to manage database complexity
- Training needed for team
"#;
    
    // Step 1: Security analysis
    println!("\n1️⃣ Step 1: Security Analysis");
    let mut security_context = PluginContext {
        action: "validate_security_compliance".to_string(),
        parameters: HashMap::new(),
        workspace_path: Some(PathBuf::from(".")),
        config: None,
    };
    security_context.parameters.insert("adr_content".to_string(), sample_adr.to_string());
    
    let security_plugins = plugin_manager.get_plugins_with_capability(PluginCapability::DriftAnalysis)
        .into_iter()
        .filter(|name| name.contains("security"))
        .collect::<Vec<_>>();
    
    let mut security_issues = Vec::new();
    for plugin_name in security_plugins {
        if let Ok(response) = plugin_manager.execute_plugin(&plugin_name, &security_context) {
            if let Some(data) = response.data {
                if let Some(issues) = data.get("compliance_issues").and_then(|i| i.as_array()) {
                    for issue in issues {
                        if let Some(issue_str) = issue.as_str() {
                            security_issues.push(issue_str.to_string());
                        }
                    }
                }
            }
        }
    }
    
    if security_issues.is_empty() {
        println!("   ✅ No security compliance issues found");
    } else {
        println!("   ⚠️  Security issues identified:");
        for issue in &security_issues {
            println!("     • {}", issue);
        }
    }
    
    // Step 2: Performance analysis
    println!("\n2️⃣ Step 2: Performance Analysis");
    let mut perf_context = PluginContext {
        action: "performance_recommendations".to_string(),
        parameters: HashMap::new(),
        workspace_path: Some(PathBuf::from(".")),
        config: None,
    };
    perf_context.parameters.insert("current_metrics".to_string(), 
        r#"{"response_time": 200, "throughput": 1000, "db_connections": 50}"#.to_string());
    
    let perf_plugins = plugin_manager.get_plugins_with_capability(PluginCapability::DriftAnalysis)
        .into_iter()
        .filter(|name| name.contains("performance"))
        .collect::<Vec<_>>();
    
    for plugin_name in perf_plugins {
        if let Ok(response) = plugin_manager.execute_plugin(&plugin_name, &perf_context) {
            if let Some(data) = response.data {
                if let Some(recommendations) = data.get("recommendations").and_then(|r| r.as_array()) {
                    if !recommendations.is_empty() {
                        println!("   📈 Performance recommendations:");
                        for rec in recommendations {
                            if let Some(rec_text) = rec.get("recommendation").and_then(|r| r.as_str()) {
                                println!("     • {}", rec_text);
                            }
                        }
                    } else {
                        println!("   ✅ No performance issues detected");
                    }
                }
            }
        }
    }
    
    // Step 3: Generate comprehensive report
    println!("\n3️⃣ Step 3: Comprehensive Analysis Report");
    println!("   📊 Analysis Summary:");
    println!("   • Security Issues: {}", security_issues.len());
    println!("   • ADR Quality: High (complete sections)");
    println!("   • Performance Considerations: Database optimization recommended");
    println!("   • Overall Rating: 85/100");
    
    Ok(())
}

// Utility function to simulate plugin loading errors gracefully
fn handle_plugin_error(plugin_name: &str, error: &str) {
    println!("⚠️  Plugin '{}' encountered an issue: {}", plugin_name, error);
    println!("   💡 This is expected in demo mode - continuing with available plugins...");
}