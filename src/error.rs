use thiserror::Error;

/// ADRScan error types
#[derive(Error, Debug)]
pub enum AdrscanError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("ADR validation error: {0}")]
    ValidationError(String),

    #[error("Drift detection error: {0}")]
    DriftError(String),

    #[error("Feature not implemented: {0}")]
    #[allow(dead_code)] // Planned for unimplemented features
    NotImplemented(String),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Directory not found: {0}")]
    DirectoryNotFound(String),

    #[error("Permission denied: {0}")]
    #[allow(dead_code)] // Planned for permission checking
    PermissionDenied(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Real-time analysis error: {0}")]
    RealtimeError(String),

    #[error("ML processing error: {0}")]
    MLError(String),

    #[error("Cache error: {0}")]
    CacheError(String),

    #[error("WebSocket error: {0}")]
    WebSocketError(String),

    #[error("Event bus error: {0}")]
    EventBusError(String),

    #[error("File watcher error: {0}")]
    FileWatcherError(String),
}
