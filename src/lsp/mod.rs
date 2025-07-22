//! Language Server Protocol implementation for PhotonDrift
//! 
//! This module provides LSP server functionality to enable IDE integration
//! for Architecture Decision Record (ADR) management with real-time drift detection.

#[cfg(feature = "lsp")]
pub mod server;
#[cfg(feature = "lsp")]
pub mod handlers;
#[cfg(feature = "lsp")]
pub mod diagnostics;
#[cfg(feature = "lsp")]
pub mod completion;
#[cfg(feature = "lsp")]
pub mod hover;
#[cfg(feature = "lsp")]
pub mod protocol;

#[cfg(feature = "lsp")]
pub use server::PhotonDriftLspServer;
#[cfg(feature = "lsp")]
pub use diagnostics::DriftDiagnosticsEngine;
#[cfg(feature = "lsp")]
pub use protocol::LspProtocolHelper;

#[cfg(feature = "lsp")]
use crate::Result;

/// Initialize and start the LSP server
#[cfg(feature = "lsp")]
pub async fn start_lsp_server() -> Result<()> {
    use tower_lsp::{LspService, Server};
    use tokio::io::{stdin, stdout};

    let (service, socket) = LspService::new(|client| PhotonDriftLspServer::new(client));
    Server::new(stdin(), stdout(), socket).serve(service).await;
    
    Ok(())
}