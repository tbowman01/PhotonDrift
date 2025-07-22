//! ADRScan - Architecture Decision Record management and drift detection
//!
//! This library provides the core functionality for scanning, parsing, and analyzing
//! Architecture Decision Records (ADRs) in software projects.

pub mod commands;
pub mod config;
pub mod drift;
pub mod error;
#[cfg(feature = "lsp")]
pub mod lsp;
pub mod ml;
pub mod parser;

// Plugin system (only available with plugins feature)
#[cfg(feature = "plugins")]
pub mod plugins;

// Real-time analysis module (only available with realtime feature)
#[cfg(feature = "realtime")]
pub mod realtime;

// WebAssembly module (only compiled for wasm32 target)
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub mod wasm_simple;

pub use config::Config;
pub use error::AdrscanError;

/// Result type used throughout the library
pub type Result<T> = std::result::Result<T, AdrscanError>;
