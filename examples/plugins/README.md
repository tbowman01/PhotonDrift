# PhotonDrift Plugin Examples

This directory contains example plugins and usage demonstrations for PhotonDrift's extensible plugin system.

## 🔌 Available Example Plugins

### 1. Security Analyzer Plugin (`security_analyzer.rs`)
A comprehensive security-focused plugin that provides:

- **Security Drift Detection**: Identifies hardcoded credentials, weak hashing, SQL injection risks, and more
- **Security Templates**: Specialized ADR templates for security architecture decisions
- **Compliance Validation**: Checks ADRs for security compliance requirements
- **Risk Assessment**: Categorizes security issues by severity and provides recommendations

**Capabilities:**
- `DriftAnalysis` - Detects security-related architectural violations
- `TemplateGeneration` - Provides security-focused ADR templates

**Usage:**
```rust
let plugin = SecurityAnalyzerPlugin::new();
let drift_results = plugin.analyze_drift("auth.js", source_code)?;
```

### 2. Performance Analyzer Plugin (`performance_analyzer.rs`)
A performance-focused plugin that provides:

- **Performance Drift Detection**: Identifies N+1 queries, inefficient string operations, missing caching
- **Performance Templates**: ADR templates for performance decisions and caching strategies
- **Benchmark Analysis**: Analyzes performance metrics and provides optimization recommendations
- **Performance Scoring**: Calculates performance health scores

**Capabilities:**
- `DriftAnalysis` - Detects performance-related issues
- `TemplateGeneration` - Provides performance-focused ADR templates

**Usage:**
```rust
let plugin = PerformanceAnalyzerPlugin::new();
let perf_analysis = plugin.analyze_performance_drift(&context)?;
```

## 📋 Plugin Configuration

### Configuration File (`plugin_config.yml`)
Comprehensive configuration example showing:

- Plugin discovery and loading settings
- Security and sandboxing configuration
- Performance monitoring and thresholds
- Integration with IDEs and CI/CD systems
- Custom plugin types and API settings

### Key Configuration Sections:

```yaml
plugins:
  security_analyzer:
    enabled: true
    priority: 1
    config:
      security_levels: ["low", "medium", "high", "critical"]
      compliance_frameworks: ["GDPR", "CCPA", "SOX", "HIPAA"]
    triggers:
      - "file_change"
      - "adr_create"

security:
  sandbox_enabled: true
  max_execution_time_ms: 5000
  max_memory_bytes: 52428800  # 50MB
  verify_signatures: true
```

## 🚀 Usage Demonstration

### Plugin Usage Example (`plugin_usage.rs`)
A complete demonstration showing:

1. **Plugin Discovery and Loading**
   - Scanning plugin directories
   - Loading configuration
   - Initializing plugin manager

2. **Drift Analysis with Plugins**
   - Security analysis of JavaScript code
   - Performance analysis with multiple patterns
   - Plugin chaining for comprehensive analysis

3. **Template Generation**
   - Security ADR templates
   - Performance decision templates
   - Custom template requests

4. **Plugin Chaining**
   - Sequential analysis with multiple plugins
   - Combining security and performance insights
   - Comprehensive reporting

### Running the Demo

```bash
# From the PhotonDrift root directory
cargo run --example plugin_usage --features plugins

# Or build and run
cargo build --features plugins
./target/debug/examples/plugin_usage
```

## 🛠️ Plugin Development Guide

### Creating a Custom Plugin

1. **Implement Required Traits**
```rust
use adrscan::plugins::{Plugin, DriftAnalysisPlugin, PluginMetadata};

pub struct MyCustomPlugin {
    metadata: PluginMetadata,
}

impl Plugin for MyCustomPlugin {
    fn metadata(&self) -> &PluginMetadata { &self.metadata }
    fn initialize(&mut self, context: &PluginContext) -> PluginResult<()> { Ok(()) }
    fn execute(&self, context: &PluginContext) -> PluginResult<PluginResponse> { 
        // Plugin logic here
    }
    fn shutdown(&mut self) -> PluginResult<()> { Ok(()) }
}
```

2. **Define Plugin Capabilities**
```rust
let metadata = PluginMetadata {
    name: "My Custom Plugin".to_string(),
    version: "1.0.0".to_string(),
    description: "Custom drift detection plugin".to_string(),
    author: "Your Name".to_string(),
    capabilities: vec![
        PluginCapability::DriftAnalysis,
        PluginCapability::TemplateGeneration,
    ],
    api_version: "1.0.0".to_string(),
};
```

3. **Export Plugin Entry Point**
```rust
#[no_mangle]
pub extern "C" fn create_plugin() -> Box<dyn Plugin> {
    Box::new(MyCustomPlugin::new())
}
```

### Plugin Types

#### Drift Analysis Plugin
Implement `DriftAnalysisPlugin` for custom architectural violation detection:
```rust
impl DriftAnalysisPlugin for MyCustomPlugin {
    fn analyze_drift(&self, file_path: &str, content: &str) -> PluginResult<Vec<serde_json::Value>> {
        // Custom drift analysis logic
    }
    
    fn get_severity_level(&self, drift_type: &str) -> String {
        // Return severity classification
    }
}
```

#### Template Plugin  
Implement `TemplatePlugin` for custom ADR templates:
```rust
impl TemplatePlugin for MyCustomPlugin {
    fn get_templates(&self) -> PluginResult<Vec<serde_json::Value>> {
        // Return available templates
    }
    
    fn generate_template(&self, template_name: &str, context: &HashMap<String, String>) -> PluginResult<String> {
        // Generate template with context substitution
    }
}
```

## 🔒 Plugin Security

### Security Features
- **Sandboxed Execution**: Plugins run in isolated environments
- **Resource Limits**: Memory and execution time constraints
- **Code Signing**: Digital signature verification for trusted plugins
- **File System Restrictions**: Limited file access permissions
- **Network Isolation**: No external network access by default

### Best Practices
1. **Validate Inputs**: Always validate plugin inputs and parameters
2. **Handle Errors Gracefully**: Use proper error handling and recovery
3. **Respect Resource Limits**: Stay within memory and execution time bounds
4. **Use Safe APIs**: Avoid unsafe operations and system calls
5. **Document Security Requirements**: Clearly document plugin security needs

## 📊 Plugin Performance

### Performance Monitoring
- **Execution Time Tracking**: Monitor plugin response times
- **Memory Usage Monitoring**: Track plugin memory consumption
- **Success Rate Metrics**: Monitor plugin reliability
- **Drift Detection Accuracy**: Track false positive rates

### Optimization Tips
1. **Cache Results**: Cache expensive computations and analyses
2. **Batch Operations**: Process multiple files together when possible
3. **Lazy Loading**: Load resources only when needed
4. **Async Operations**: Use async patterns for I/O operations
5. **Resource Cleanup**: Properly cleanup resources and memory

## 🧪 Testing Plugins

### Unit Testing
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_security_drift_detection() {
        let plugin = SecurityAnalyzerPlugin::new();
        let code_with_issue = "const password = 'hardcoded123';";
        let results = plugin.analyze_drift("test.js", code_with_issue).unwrap();
        assert!(!results.is_empty());
    }
}
```

### Integration Testing
```rust
#[test]
fn test_plugin_manager_integration() {
    let mut manager = PluginManager::new();
    let plugin = Box::new(SecurityAnalyzerPlugin::new());
    manager.register_plugin("security_analyzer", plugin).unwrap();
    
    let context = PluginContext::new("analyze_drift");
    let response = manager.execute_plugin("security_analyzer", &context).unwrap();
    assert!(response.success);
}
```

## 📚 Additional Resources

- [Plugin API Documentation](../../docs/PLUGIN_API.md)
- [Security Guidelines](../../docs/PLUGIN_SECURITY.md)
- [Performance Best Practices](../../docs/PLUGIN_PERFORMANCE.md)
- [Plugin Marketplace](https://plugins.photondrift.io)

## 🤝 Contributing

1. Fork the repository
2. Create your plugin in `examples/plugins/`
3. Add comprehensive tests
4. Update documentation
5. Submit a pull request

## 📄 License

These examples are provided under the same MIT license as PhotonDrift.

---

**Ready to extend PhotonDrift with your own plugins? 🚀**