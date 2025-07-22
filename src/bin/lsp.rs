//! ADRScan Language Server Protocol binary
//! 
//! This binary provides LSP server functionality for IDE integration
//! with real-time ADR analysis and drift detection.

use std::process;

#[cfg(feature = "lsp")]
use adrscan::lsp::start_lsp_server;

#[cfg(feature = "lsp")]
#[tokio::main]
async fn main() {
    env_logger::init();

    if let Err(e) = start_lsp_server().await {
        eprintln!("LSP server error: {}", e);
        process::exit(1);
    }
}

#[cfg(not(feature = "lsp"))]
fn main() {
    eprintln!("LSP feature not enabled. Build with --features lsp");
    process::exit(1);
}