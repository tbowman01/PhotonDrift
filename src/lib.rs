//! ADRScan - Architecture Decision Record management and drift detection
//! 
//! This library provides the core functionality for scanning, parsing, and analyzing
//! Architecture Decision Records (ADRs) in software projects.

pub mod commands;
pub mod config;
pub mod drift;
pub mod error;
pub mod parser;

// WebAssembly module (only compiled for wasm32 target)
#[cfg(all(target_arch = "wasm32", feature = "wasm"))]
pub mod wasm_simple;

pub use config::Config;
pub use error::AdrscanError;

/// Result type used throughout the library
pub type Result<T> = std::result::Result<T, AdrscanError>;