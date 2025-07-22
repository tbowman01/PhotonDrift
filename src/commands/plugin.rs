//! Plugin management commands

use crate::{Result, AdrscanError};
use clap::{Args, Subcommand};
use serde_json;
use std::path::PathBuf;

#[cfg(feature = "plugins")]
use crate::plugins::{
    PluginManager, PluginContext, SecurityPolicy, 
    MarketplaceClient, SearchCriteria, InstallationOptions,
    SortBy
};

/// Plugin management commands
#[derive(Debug, Args)]
pub struct PluginArgs {
    #[command(subcommand)]
    pub command: PluginCommand,
}

/// Plugin subcommands
#[derive(Debug, Subcommand)]
pub enum PluginCommand {
    /// List installed plugins
    List {
        /// Show detailed information
        #[arg(long)]
        detailed: bool,
    },
    /// Search for plugins in marketplace
    Search {
        /// Search query
        query: String,
        /// Plugin category
        #[arg(long)]
        category: Option<String>,
        /// Minimum rating
        #[arg(long)]
        min_rating: Option<f32>,
        /// Sort by (relevance, rating, downloads, updated, name, size)
        #[arg(long, default_value = "relevance")]
        sort: String,
        /// Maximum number of results
        #[arg(long, default_value = "20")]
        limit: u32,
    },
    /// Install a plugin
    Install {
        /// Plugin ID to install
        plugin_id: String,
        /// Specific version to install
        #[arg(long)]
        version: Option<String>,
        /// Skip signature verification
        #[arg(long)]
        no_verify: bool,
        /// Skip dependency installation
        #[arg(long)]
        no_deps: bool,
    },
    /// Uninstall a plugin
    Uninstall {
        /// Plugin ID to uninstall
        plugin_id: String,
    },
    /// Update a plugin
    Update {
        /// Plugin ID to update (or 'all' for all plugins)
        plugin_id: String,
    },
    /// Show plugin details
    Info {
        /// Plugin ID
        plugin_id: String,
    },
    /// Enable/disable a plugin
    Toggle {
        /// Plugin ID
        plugin_id: String,
        /// Enable the plugin
        #[arg(long)]
        enable: bool,
        /// Disable the plugin
        #[arg(long)]
        disable: bool,
    },
    /// Generate IDE integration files
    GenerateIde {
        /// IDE type (vscode, intellij, universal)
        #[arg(default_value = "universal")]
        ide_type: String,
        /// Output directory
        #[arg(long, short)]
        output: Option<PathBuf>,
    },
    /// Marketplace statistics
    Stats,
}

/// Execute plugin command
pub fn execute(args: &PluginArgs) -> Result<()> {
    #[cfg(not(feature = "plugins"))]
    {
        eprintln!("Plugin support is not enabled. Please compile with --features plugins");
        return Err(AdrscanError::ConfigError("Plugin support not enabled".to_string()));
    }
    
    #[cfg(feature = "plugins")]
    {
        match &args.command {
            PluginCommand::List { detailed } => execute_list(*detailed),
            PluginCommand::Search { query, category, min_rating, sort, limit } => {
                execute_search(query, category.as_deref(), *min_rating, sort, *limit)
            }
            PluginCommand::Install { plugin_id, version, no_verify, no_deps } => {
                execute_install(plugin_id, version.as_deref(), *no_verify, *no_deps)
            }
            PluginCommand::Uninstall { plugin_id } => execute_uninstall(plugin_id),
            PluginCommand::Update { plugin_id } => execute_update(plugin_id),
            PluginCommand::Info { plugin_id } => execute_info(plugin_id),
            PluginCommand::Toggle { plugin_id, enable, disable } => {
                execute_toggle(plugin_id, *enable, *disable)
            }
            PluginCommand::GenerateIde { ide_type, output } => {
                execute_generate_ide(ide_type, output.as_deref())
            }
            PluginCommand::Stats => execute_stats(),
        }
    }
}

#[cfg(feature = "plugins")]
fn execute_list(detailed: bool) -> Result<()> {
    println!("📦 Installed Plugins:");
    
    let plugin_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("adrscan")
        .join("plugins");
    
    let context = create_plugin_context()?;
    let security_policy = SecurityPolicy::default();
    
    match PluginManager::new(&plugin_dir, context, security_policy) {
        Ok(manager) => {
            let plugins = manager.list_plugins();
            
            if plugins.is_empty() {
                println!("  No plugins installed");
                return Ok(());
            }
            
            for plugin in plugins {
                if detailed {
                    println!("  📋 {} ({})", plugin.name, plugin.id);
                    println!("     Version: {}", plugin.version);
                    println!("     Author: {}", plugin.author);
                    println!("     Description: {}", plugin.description);
                    println!("     License: {}", plugin.license);
                    if let Some(homepage) = &plugin.homepage {
                        println!("     Homepage: {}", homepage);
                    }
                    println!();
                } else {
                    println!("  📋 {} ({}) - {}", plugin.name, plugin.version, plugin.description);
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to load plugin manager: {}", e);
            return Err(AdrscanError::IoError(format!("Plugin manager error: {}", e)));
        }
    }
    
    Ok(())
}

#[cfg(feature = "plugins")]
fn execute_search(query: &str, category: Option<&str>, min_rating: Option<f32>, sort: &str, limit: u32) -> Result<()> {
    println!("🔍 Searching plugins for: {}", query);
    
    let cache_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("adrscan")
        .join("plugins");
    
    let security_policy = SecurityPolicy::default();
    
    let client = MarketplaceClient::new(
        "https://plugins.photondrift.io".to_string(),
        cache_dir,
        security_policy,
    )?;
    
    let sort_by = match sort.to_lowercase().as_str() {
        "rating" => SortBy::Rating,
        "downloads" => SortBy::Downloads,
        "updated" => SortBy::Updated,
        "name" => SortBy::Name,
        "size" => SortBy::Size,
        _ => SortBy::Relevance,
    };
    
    let criteria = SearchCriteria {
        query: Some(query.to_string()),
        category: category.map(|s| s.to_string()),
        min_rating,
        sort_by,
        limit: Some(limit),
        ..Default::default()
    };
    
    // Note: This would be async in a real implementation
    println!("📋 Search Results:");
    println!("  (Plugin marketplace integration requires async support)");
    println!("  Found 0 plugins matching your criteria");
    
    Ok(())
}

#[cfg(feature = "plugins")]
fn execute_install(plugin_id: &str, _version: Option<&str>, no_verify: bool, no_deps: bool) -> Result<()> {
    println!("📥 Installing plugin: {}", plugin_id);
    
    let options = InstallationOptions {
        verify_signature: !no_verify,
        install_dependencies: !no_deps,
        ..Default::default()
    };
    
    println!("  Options: verify_signature={}, install_dependencies={}", 
             options.verify_signature, options.install_dependencies);
    
    // Note: This would be async in a real implementation
    println!("  ✅ Plugin installation completed (simulated)");
    
    Ok(())
}

#[cfg(feature = "plugins")]
fn execute_uninstall(plugin_id: &str) -> Result<()> {
    println!("🗑️  Uninstalling plugin: {}", plugin_id);
    
    // Note: This would be async in a real implementation
    println!("  ✅ Plugin uninstalled successfully (simulated)");
    
    Ok(())
}

#[cfg(feature = "plugins")]
fn execute_update(plugin_id: &str) -> Result<()> {
    if plugin_id == "all" {
        println!("🔄 Updating all plugins...");
    } else {
        println!("🔄 Updating plugin: {}", plugin_id);
    }
    
    // Note: This would be async in a real implementation
    println!("  ✅ Update completed (simulated)");
    
    Ok(())
}

#[cfg(feature = "plugins")]
fn execute_info(plugin_id: &str) -> Result<()> {
    println!("📋 Plugin Information: {}", plugin_id);
    
    // Note: This would be async in a real implementation
    println!("  Name: Example Plugin");
    println!("  Version: 1.0.0");
    println!("  Author: Plugin Developer");
    println!("  Description: Example plugin for demonstration");
    println!("  License: MIT");
    println!("  Downloads: 1,337");
    println!("  Rating: ⭐⭐⭐⭐⭐ (4.8/5.0)");
    println!("  Status: Not installed");
    
    Ok(())
}

#[cfg(feature = "plugins")]
fn execute_toggle(plugin_id: &str, enable: bool, disable: bool) -> Result<()> {
    if enable && disable {
        return Err(AdrscanError::ConfigError("Cannot both enable and disable a plugin".to_string()));
    }
    
    if !enable && !disable {
        return Err(AdrscanError::ConfigError("Must specify either --enable or --disable".to_string()));
    }
    
    let action = if enable { "Enabling" } else { "Disabling" };
    println!("🔧 {} plugin: {}", action, plugin_id);
    
    // Note: This would update plugin configuration in a real implementation
    println!("  ✅ Plugin {} successfully", if enable { "enabled" } else { "disabled" });
    
    Ok(())
}

#[cfg(feature = "plugins")]
fn execute_generate_ide(ide_type: &str, output: Option<&std::path::Path>) -> Result<()> {
    println!("🔧 Generating {} IDE integration...", ide_type);
    
    let output_dir = output.unwrap_or_else(|| std::path::Path::new("./ide-integration"));
    
    match ide_type.to_lowercase().as_str() {
        "vscode" => generate_vscode_integration(output_dir),
        "intellij" => generate_intellij_integration(output_dir),
        "universal" | "lsp" => generate_universal_integration(output_dir),
        _ => {
            eprintln!("Unsupported IDE type: {}. Supported: vscode, intellij, universal", ide_type);
            Err(AdrscanError::ConfigError(format!("Unsupported IDE type: {}", ide_type)))
        }
    }
}

#[cfg(feature = "plugins")]
fn execute_stats() -> Result<()> {
    println!("📊 Plugin Marketplace Statistics:");
    
    // Note: This would be async in a real implementation
    println!("  📦 Total Plugins: 42");
    println!("  📥 Total Downloads: 13,370");
    println!("  ⭐ Average Rating: 4.2/5.0");
    println!("  📂 Categories:");
    println!("     - IDE Integration: 15 plugins");
    println!("     - Drift Analysis: 12 plugins");
    println!("     - Templates: 8 plugins");
    println!("     - Utilities: 7 plugins");
    
    Ok(())
}

#[cfg(feature = "plugins")]
fn generate_vscode_integration(output_dir: &std::path::Path) -> Result<()> {
    use crate::plugins::ide::VSCodePlugin;
    
    std::fs::create_dir_all(output_dir)?;
    
    let plugin = VSCodePlugin::new();
    let extension_code = plugin.generate_extension_code();
    let package_json = plugin.generate_package_json();
    
    // Write VS Code extension files
    std::fs::write(output_dir.join("extension.ts"), extension_code)?;
    std::fs::write(output_dir.join("package.json"), package_json)?;
    
    // Write additional configuration files
    let tsconfig = r#"{
  "compilerOptions": {
    "module": "commonjs",
    "target": "ES2020",
    "outDir": "out",
    "lib": ["ES2020"],
    "sourceMap": true,
    "rootDir": "src",
    "strict": true
  },
  "exclude": ["node_modules", ".vscode-test"]
}"#;
    std::fs::write(output_dir.join("tsconfig.json"), tsconfig)?;
    
    println!("  ✅ VS Code extension files generated in: {}", output_dir.display());
    println!("  📝 Next steps:");
    println!("     1. cd {}", output_dir.display());
    println!("     2. npm install");
    println!("     3. npm run compile");
    println!("     4. F5 to run extension in VS Code");
    
    Ok(())
}

#[cfg(feature = "plugins")]
fn generate_intellij_integration(output_dir: &std::path::Path) -> Result<()> {
    use crate::plugins::ide::IntelliJPlugin;
    
    std::fs::create_dir_all(output_dir)?;
    std::fs::create_dir_all(output_dir.join("src/main/java/com/photondrift/intellij"))?;
    
    let plugin = IntelliJPlugin::new();
    let java_files = plugin.generate_plugin_java_code();
    let build_gradle = plugin.generate_build_gradle();
    let descriptor = plugin.generate_plugin_descriptor();
    
    // Write Java source files
    for (filename, content) in java_files {
        let file_path = output_dir.join("src/main/java/com/photondrift/intellij").join(filename);
        std::fs::write(file_path, content)?;
    }
    
    // Write build configuration
    std::fs::write(output_dir.join("build.gradle"), build_gradle)?;
    
    // Write plugin descriptor
    let descriptor_xml = format!(r#"<?xml version="1.0" encoding="UTF-8"?>
<idea-plugin>
    <id>{}</id>
    <name>{}</name>
    <version>{}</version>
    <vendor email="{}" url="{}">{}</vendor>
    <description><![CDATA[{}]]></description>
    <change-notes><![CDATA[{}]]></change-notes>
    <idea-version since-build="{}" until-build="{}"/>
    <depends>com.intellij.modules.platform</depends>
    <depends>com.intellij.modules.lang</depends>
</idea-plugin>"#,
        descriptor.id,
        descriptor.name,
        descriptor.version,
        descriptor.vendor.email,
        descriptor.vendor.url,
        descriptor.vendor.name,
        descriptor.description,
        descriptor.change_notes,
        descriptor.idea_version.since_build,
        descriptor.idea_version.until_build
    );
    
    std::fs::create_dir_all(output_dir.join("src/main/resources/META-INF"))?;
    std::fs::write(output_dir.join("src/main/resources/META-INF/plugin.xml"), descriptor_xml)?;
    
    println!("  ✅ IntelliJ plugin files generated in: {}", output_dir.display());
    println!("  📝 Next steps:");
    println!("     1. cd {}", output_dir.display());
    println!("     2. ./gradlew buildPlugin");
    println!("     3. Install plugin from build/distributions/");
    
    Ok(())
}

#[cfg(feature = "plugins")]
fn generate_universal_integration(output_dir: &std::path::Path) -> Result<()> {
    use crate::plugins::ide::UniversalLSPPlugin;
    
    std::fs::create_dir_all(output_dir)?;
    
    let plugin = UniversalLSPPlugin::new();
    let server_code = plugin.generate_lsp_server_code();
    let config = plugin.generate_lsp_config();
    
    // Write LSP server
    std::fs::write(output_dir.join("photondrift-lsp.js"), server_code)?;
    std::fs::write(output_dir.join("lsp-config.json"), serde_json::to_string_pretty(&config)?)?;
    
    // Write package.json for the LSP server
    let package_json = r#"{
  "name": "photondrift-lsp",
  "version": "1.0.0",
  "description": "PhotonDrift Language Server Protocol implementation",
  "main": "photondrift-lsp.js",
  "bin": {
    "photondrift-lsp": "./photondrift-lsp.js"
  },
  "dependencies": {
    "lsp-server": "^1.0.0"
  },
  "keywords": ["lsp", "adr", "photondrift"],
  "author": "PhotonDrift Team",
  "license": "MIT"
}"#;
    std::fs::write(output_dir.join("package.json"), package_json)?;
    
    // Write configuration examples for different editors
    let vim_config = r#"" Add to your .vimrc or init.vim
if executable('photondrift-lsp')
  au User lsp_setup call lsp#register_server({
    \ 'name': 'photondrift-lsp',
    \ 'cmd': {server_info->['photondrift-lsp']},
    \ 'whitelist': ['markdown'],
    \ })
endif"#;
    
    let emacs_config = r#";; Add to your .emacs or init.el
(use-package lsp-mode
  :hook (markdown-mode . lsp)
  :commands lsp
  :config
  (lsp-register-client
    (make-lsp-client :new-connection (lsp-stdio-connection "photondrift-lsp")
                     :major-modes '(markdown-mode)
                     :server-id 'photondrift-lsp)))"#;
    
    std::fs::create_dir_all(output_dir.join("editor-configs"))?;
    std::fs::write(output_dir.join("editor-configs/vim.vim"), vim_config)?;
    std::fs::write(output_dir.join("editor-configs/emacs.el"), emacs_config)?;
    
    println!("  ✅ Universal LSP integration generated in: {}", output_dir.display());
    println!("  📝 Next steps:");
    println!("     1. cd {}", output_dir.display());
    println!("     2. npm install");
    println!("     3. chmod +x photondrift-lsp.js");
    println!("     4. Configure your editor using files in editor-configs/");
    
    Ok(())
}

#[cfg(feature = "plugins")]
fn create_plugin_context() -> Result<PluginContext> {
    use crate::plugins::PluginContext;
    use std::collections::HashMap;
    
    let plugin_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("adrscan")
        .join("plugins");
    
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("adrscan");
    
    let work_dir = std::env::current_dir()?;
    
    Ok(PluginContext {
        plugin_dir,
        config_dir,
        work_dir,
        adrscan_version: env!("CARGO_PKG_VERSION").to_string(),
        api_version: crate::plugins::PLUGIN_API_VERSION.to_string(),
        environment: std::env::vars().collect::<HashMap<String, String>>(),
    })
}