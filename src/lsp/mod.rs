//! Language Server Protocol (LSP) implementation for PhotonDrift
//! 
//! This module provides LSP server functionality for IDE integration,
//! enabling real-time ADR analysis, drift detection, and intelligent
//! code assistance features.

mod server;
mod handlers;
mod diagnostics;
mod completion;
mod hover;
mod protocol;

pub use server::{PhotonDriftLspServer, start_lsp_server};
pub use diagnostics::{DiagnosticEngine, create_drift_diagnostic};
pub use completion::{CompletionProvider, ADR_TEMPLATE_COMPLETIONS};
pub use hover::{HoverProvider, create_hover_info};
pub use protocol::{uri_to_path, path_to_uri, normalize_line_endings};

use crate::Result;
use lsp_types::Url;
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::sync::RwLock;

/// Document store for managing open ADR files
pub type DocumentStore = RwLock<HashMap<Url, String>>;

/// LSP server configuration
#[derive(Debug, Clone)]
pub struct LspConfig {
    /// Enable real-time drift diagnostics
    pub diagnostics_enabled: bool,
    
    /// Maximum number of diagnostics per file
    pub max_diagnostics: usize,
    
    /// Enable template completion
    pub completion_enabled: bool,
    
    /// Enable hover information
    pub hover_enabled: bool,
    
    /// Workspace root directory
    pub workspace_root: Option<PathBuf>,
}

impl Default for LspConfig {
    fn default() -> Self {
        Self {
            diagnostics_enabled: true,
            max_diagnostics: 100,
            completion_enabled: true,
            hover_enabled: true,
            workspace_root: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_config_default() {
        let config = LspConfig::default();
        assert!(config.diagnostics_enabled);
        assert!(config.completion_enabled);
        assert!(config.hover_enabled);
        assert_eq!(config.max_diagnostics, 100);
    }
}