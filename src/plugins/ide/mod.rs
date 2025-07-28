//! IDE integration plugins for PhotonDrift

pub mod intellij;
pub mod universal;
pub mod vscode;

pub use intellij::IntelliJPlugin;
pub use universal::UniversalLSPPlugin;
pub use vscode::VSCodePlugin;

use crate::plugins::{IDEConfig, IDEEvent, IDEResponse, IDEType};
use crate::Result;
use serde::{Deserialize, Serialize};

/// Factory for creating IDE-specific plugins
pub struct IDEPluginFactory;

impl IDEPluginFactory {
    /// Create an IDE plugin for the specified type
    pub fn create_plugin(
        ide_type: IDEType,
    ) -> Result<Box<dyn crate::plugins::IDEIntegrationPlugin>> {
        match ide_type {
            IDEType::VSCode => Ok(Box::new(VSCodePlugin::new())),
            IDEType::IntelliJ => Ok(Box::new(IntelliJPlugin::new())),
            IDEType::Universal => Ok(Box::new(UniversalLSPPlugin::new())),
            _ => {
                // For other IDEs, fall back to Universal LSP
                Ok(Box::new(UniversalLSPPlugin::new()))
            }
        }
    }

    /// Detect IDE type from environment or configuration
    pub fn detect_ide() -> IDEType {
        // Check environment variables and common IDE indicators
        if std::env::var("VSCODE_PID").is_ok() || std::env::var("VSCODE_IPC_HOOK").is_ok() {
            return IDEType::VSCode;
        }

        if std::env::var("IDEA_INITIAL_DIRECTORY").is_ok() {
            return IDEType::IntelliJ;
        }

        // Check for common IDE processes (simplified)
        if let Ok(processes) = get_running_processes() {
            if processes.contains("code") || processes.contains("code.exe") {
                return IDEType::VSCode;
            }
            if processes.contains("idea") || processes.contains("idea.exe") {
                return IDEType::IntelliJ;
            }
            if processes.contains("vim") || processes.contains("nvim") {
                return IDEType::Vim;
            }
            if processes.contains("emacs") {
                return IDEType::Emacs;
            }
        }

        // Default to Universal LSP
        IDEType::Universal
    }

    /// Get recommended configuration for IDE type
    pub fn get_recommended_config(ide_type: IDEType) -> IDEConfig {
        match ide_type {
            IDEType::VSCode => IDEConfig {
                ide_type,
                version: None,
                installation_path: detect_vscode_installation(),
                settings: serde_json::json!({
                    "enableRealTimeAnalysis": true,
                    "showInlineHints": true,
                    "autoFormat": true,
                    "enableCodeLens": true
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            IDEType::IntelliJ => IDEConfig {
                ide_type,
                version: None,
                installation_path: detect_intellij_installation(),
                settings: serde_json::json!({
                    "enableInspections": true,
                    "showQuickFixes": true,
                    "autoImport": true,
                    "enableRefactoring": true
                })
                .as_object()
                .unwrap()
                .clone(),
            },
            _ => IDEConfig {
                ide_type,
                version: None,
                installation_path: None,
                settings: serde_json::json!({
                    "enableBasicFeatures": true,
                    "useTextMateGrammar": true
                })
                .as_object()
                .unwrap()
                .clone(),
            },
        }
    }
}

/// Common IDE features that all plugins can implement
pub trait CommonIDEFeatures {
    /// Show notification in IDE
    fn show_notification(&self, message: &str, level: crate::plugins::MessageLevel) -> Result<()>;

    /// Open file at specific line
    fn open_file(&self, path: &std::path::Path, line: Option<u32>) -> Result<()>;

    /// Insert text at position
    fn insert_text(&self, path: &std::path::Path, line: u32, column: u32, text: &str)
        -> Result<()>;

    /// Get current selection
    fn get_selection(&self) -> Result<Option<TextSelection>>;

    /// Set status bar message
    fn set_status_message(&self, message: &str) -> Result<()>;
}

/// Text selection information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSelection {
    pub file_path: std::path::PathBuf,
    pub start_line: u32,
    pub start_column: u32,
    pub end_line: u32,
    pub end_column: u32,
    pub text: String,
}

/// IDE capability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IDECapabilities {
    pub supports_lsp: bool,
    pub supports_debugging: bool,
    pub supports_extensions: bool,
    pub supports_git_integration: bool,
    pub supports_terminal: bool,
    pub supported_languages: Vec<String>,
}

// Helper functions

fn get_running_processes() -> Result<Vec<String>> {
    // Simplified process detection
    // In a real implementation, this would use proper system APIs
    Ok(vec![])
}

fn detect_vscode_installation() -> Option<std::path::PathBuf> {
    // Common VS Code installation paths
    let common_paths = [
        "/Applications/Visual Studio Code.app/Contents/MacOS/Electron",
        "/usr/bin/code",
        "/snap/bin/code",
        "C:\\Program Files\\Microsoft VS Code\\Code.exe",
        "C:\\Users\\{user}\\AppData\\Local\\Programs\\Microsoft VS Code\\Code.exe",
    ];

    for path in &common_paths {
        let path_buf = std::path::PathBuf::from(path);
        if path_buf.exists() {
            return Some(path_buf);
        }
    }

    None
}

fn detect_intellij_installation() -> Option<std::path::PathBuf> {
    // Common IntelliJ IDEA installation paths
    let common_paths = [
        "/Applications/IntelliJ IDEA.app/Contents/MacOS/idea",
        "/opt/idea/bin/idea.sh",
        "/usr/local/bin/idea",
        "C:\\Program Files\\JetBrains\\IntelliJ IDEA\\bin\\idea64.exe",
    ];

    for path in &common_paths {
        let path_buf = std::path::PathBuf::from(path);
        if path_buf.exists() {
            return Some(path_buf);
        }
    }

    None
}
