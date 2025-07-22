//! LSP integration tests for PhotonDrift
//! 
//! This module contains comprehensive tests for the Language Server Protocol
//! implementation, including protocol compliance, performance, and integration tests.

#[cfg(feature = "lsp")]
mod integration;
#[cfg(feature = "lsp")]
mod protocol_compliance;
#[cfg(feature = "lsp")]
mod performance;
#[cfg(feature = "lsp")]
mod mock_client;