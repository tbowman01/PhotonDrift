//! PhotonDrift Language Server Protocol binary
//! 
//! This binary starts the PhotonDrift LSP server for IDE integration.
//! Usage: adrscan-lsp

use std::io;
use tokio::io::{stdin, stdout};
use tower_lsp::{LspService, Server};

use adrscan::lsp::PhotonDriftLspServer;

#[tokio::main]
async fn main() {
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .target(env_logger::Target::Stderr) // Use stderr to avoid interfering with LSP communication
        .init();

    log::info!("Starting PhotonDrift LSP server...");

    // Create the LSP service
    let (service, socket) = LspService::new(|client| {
        log::info!("Client connected to PhotonDrift LSP server");
        PhotonDriftLspServer::new(client)
    });

    // Start the server
    log::info!("PhotonDrift LSP server listening on stdin/stdout");
    Server::new(stdin(), stdout(), socket).serve(service).await;
    
    log::info!("PhotonDrift LSP server shutting down");
}