//! Real-time file watching and incremental ML analysis system
//!
//! This module provides real-time monitoring of file system changes and
//! performs incremental machine learning analysis with debounced processing.

#[cfg(feature = "realtime")]
pub mod watcher;

#[cfg(feature = "realtime")]
pub mod pipeline;

#[cfg(feature = "realtime")]
pub mod events;

#[cfg(feature = "realtime")]
pub mod websocket;

#[cfg(feature = "realtime")]
pub mod cache;

#[cfg(feature = "realtime")]
pub use watcher::FileWatcher;

#[cfg(feature = "realtime")]
pub use pipeline::MLPipeline;

#[cfg(feature = "realtime")]
pub use events::EventBus;

#[cfg(feature = "realtime")]
pub use websocket::WebSocketServer;

#[cfg(feature = "realtime")]
pub use cache::IntelligentCache;

use crate::error::AdrscanError;

/// Result type for real-time operations
pub type RealtimeResult<T> = Result<T, AdrscanError>;

/// Configuration for real-time analysis
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RealtimeConfig {
    /// Debounce delay in milliseconds for file change processing
    pub debounce_delay_ms: u64,
    /// Maximum number of files to watch simultaneously
    pub max_watched_files: usize,
    /// Cache TTL in seconds
    pub cache_ttl_seconds: u64,
    /// WebSocket server port
    pub websocket_port: u16,
    /// Enable ML analysis pipeline
    pub enable_ml_analysis: bool,
    /// Maximum latency target in milliseconds
    pub max_latency_ms: u64,
}

impl Default for RealtimeConfig {
    fn default() -> Self {
        Self {
            debounce_delay_ms: 300,
            max_watched_files: 1000,
            cache_ttl_seconds: 3600, // 1 hour
            websocket_port: 8080,
            enable_ml_analysis: true,
            max_latency_ms: 50,
        }
    }
}

#[cfg(feature = "realtime")]
pub struct RealtimeSystem {
    watcher: FileWatcher,
    pipeline: MLPipeline,
    event_bus: EventBus,
    websocket_server: Option<WebSocketServer>,
    cache: IntelligentCache,
    config: RealtimeConfig,
}

#[cfg(feature = "realtime")]
impl RealtimeSystem {
    /// Create a new real-time analysis system
    pub fn new(config: RealtimeConfig) -> RealtimeResult<Self> {
        let watcher = FileWatcher::new(&config)?;
        let pipeline = MLPipeline::new(&config)?;
        let event_bus = EventBus::new();
        let cache = IntelligentCache::new(&config)?;

        let websocket_server = if config.websocket_port > 0 {
            Some(WebSocketServer::new(config.websocket_port)?)
        } else {
            None
        };

        Ok(Self {
            watcher,
            pipeline,
            event_bus,
            websocket_server,
            cache,
            config,
        })
    }

    /// Start the real-time monitoring system
    pub async fn start(&mut self) -> RealtimeResult<()> {
        log::info!("Starting real-time analysis system");

        // Start WebSocket server if configured
        if let Some(websocket_server) = &mut self.websocket_server {
            websocket_server.start().await?;
        }

        // Start file watcher
        self.watcher.start().await?;

        log::info!("Real-time analysis system started successfully");
        Ok(())
    }

    /// Stop the real-time monitoring system
    pub async fn stop(&mut self) -> RealtimeResult<()> {
        log::info!("Stopping real-time analysis system");

        // Stop file watcher
        self.watcher.stop().await?;

        // Stop WebSocket server
        if let Some(websocket_server) = &mut self.websocket_server {
            websocket_server.stop().await?;
        }

        log::info!("Real-time analysis system stopped");
        Ok(())
    }
}
