//! Language Server Protocol (LSP) implementation for PhotonDrift
//!
//! This module provides LSP server functionality for IDE integration,
//! enabling real-time ADR analysis, drift detection, and intelligent
//! code assistance features.

mod completion;
mod diagnostics;
mod handlers;
mod hover;
mod protocol;
mod server;

pub use completion::{CompletionProvider, ADR_TEMPLATE_COMPLETIONS};
pub use diagnostics::{create_drift_diagnostic, DiagnosticEngine};
pub use hover::{create_hover_info, HoverProvider};
pub use protocol::{normalize_line_endings, path_to_uri, uri_to_path};
pub use server::{start_lsp_server, PhotonDriftLspServer};

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
