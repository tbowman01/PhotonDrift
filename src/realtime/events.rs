//! Event bus architecture for real-time system coordination
//!
//! Provides a high-performance event-driven communication system
//! for coordinating between different components of the real-time analysis system.

use crate::error::AdrscanError;
use crate::realtime::pipeline::MLAnalysisResult;
use crate::realtime::{RealtimeConfig, RealtimeResult};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Instant, SystemTime};
use tokio::sync::{broadcast, RwLock};

/// Types of events that can be published through the event bus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineEvent {
    /// File change detected
    FileChanged {
        path: PathBuf,
        change_type: String,
        timestamp: SystemTime,
    },
    /// ML analysis started for a file
    AnalysisStarted {
        file_path: PathBuf,
        start_time: SystemTime,
    },
    /// ML analysis completed successfully
    AnalysisCompleted {
        file_path: PathBuf,
        result: MLAnalysisResult,
    },
    /// ML analysis failed
    AnalysisError { file_path: PathBuf, error: String },
    /// System performance metrics update
    PerformanceMetrics {
        timestamp: SystemTime,
        metrics: PerformanceSnapshot,
    },
    /// Cache operation event
    CacheEvent {
        operation: String,
        key: String,
        hit: bool,
        timestamp: SystemTime,
    },
    /// WebSocket client connected/disconnected
    WebSocketEvent {
        client_id: String,
        event_type: String,
        timestamp: SystemTime,
    },
    /// System warning or error
    SystemAlert {
        level: AlertLevel,
        message: String,
        component: String,
        timestamp: SystemTime,
    },
}

/// Alert levels for system events
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AlertLevel {
    Info,
    Warning,
    Error,
    Critical,
}

/// Performance snapshot for monitoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    pub memory_usage_mb: f64,
    pub cpu_usage_percent: f64,
    pub active_watchers: usize,
    pub pending_analyses: usize,
    pub cache_hit_ratio: f64,
    pub average_processing_time_ms: f64,
    pub events_per_second: f64,
}

/// Event subscription configuration
#[derive(Debug, Clone)]
pub struct EventSubscription {
    pub event_types: Vec<String>,
    pub filter_criteria: HashMap<String, String>,
    pub max_events_per_second: Option<u64>,
}

/// Event handler trait for processing events
pub trait EventHandler: Send + Sync {
    fn handle_event(&self, event: PipelineEvent) -> RealtimeResult<()>;
    fn event_types(&self) -> Vec<String>;
}

/// Statistics for the event bus
#[derive(Debug, Default)]
pub struct EventBusStats {
    pub events_published: u64,
    pub events_delivered: u64,
    pub active_subscribers: usize,
    pub failed_deliveries: u64,
    pub average_delivery_time_ms: f64,
}

/// High-performance event bus for real-time system coordination
pub struct EventBus {
    sender: broadcast::Sender<PipelineEvent>,
    stats: Arc<RwLock<EventBusStats>>,
    handlers: Arc<RwLock<Vec<Arc<dyn EventHandler>>>>,
    performance_tracker: Arc<RwLock<PerformanceTracker>>,
}

/// Tracks performance metrics for event processing
struct PerformanceTracker {
    last_performance_update: Instant,
    events_since_last_update: u64,
    total_delivery_time_ms: u64,
    delivery_count: u64,
}

impl EventBus {
    /// Create a new event bus
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(10000); // Large buffer for high throughput
        let stats = Arc::new(RwLock::new(EventBusStats::default()));
        let handlers = Arc::new(RwLock::new(Vec::new()));
        let performance_tracker = Arc::new(RwLock::new(PerformanceTracker {
            last_performance_update: Instant::now(),
            events_since_last_update: 0,
            total_delivery_time_ms: 0,
            delivery_count: 0,
        }));

        Self {
            sender,
            stats,
            handlers,
            performance_tracker,
        }
    }

    /// Publish an event to all subscribers
    pub async fn publish(&self, event: PipelineEvent) -> RealtimeResult<()> {
        let start_time = Instant::now();

        // Send to broadcast channel
        match self.sender.send(event.clone()) {
            Ok(subscriber_count) => {
                // Update statistics
                {
                    let mut stats = self.stats.write().await;
                    stats.events_published += 1;
                    stats.events_delivered += subscriber_count as u64;
                    stats.active_subscribers = subscriber_count;
                }

                // Send to registered handlers
                let handlers = self.handlers.read().await;
                for handler in handlers.iter() {
                    if self.should_handle_event(handler.as_ref(), &event) {
                        if let Err(e) = handler.handle_event(event.clone()) {
                            log::error!("Event handler failed: {}", e);
                            let mut stats = self.stats.write().await;
                            stats.failed_deliveries += 1;
                        }
                    }
                }

                // Update performance metrics
                self.update_performance_metrics(start_time.elapsed()).await;

                log::debug!("Published event to {} subscribers", subscriber_count);
                Ok(())
            }
            Err(_) => {
                // No active subscribers
                log::debug!("No subscribers for event: {:?}", event);
                Ok(())
            }
        }
    }

    /// Subscribe to events from the bus
    pub fn subscribe(&self) -> broadcast::Receiver<PipelineEvent> {
        self.sender.subscribe()
    }

    /// Register an event handler
    pub async fn register_handler(&self, handler: Arc<dyn EventHandler>) {
        let mut handlers = self.handlers.write().await;
        handlers.push(handler);
        log::info!("Registered new event handler");
    }

    /// Unregister an event handler
    pub async fn unregister_handler(&self, handler_id: usize) -> RealtimeResult<()> {
        let mut handlers = self.handlers.write().await;
        if handler_id < handlers.len() {
            handlers.remove(handler_id);
            log::info!("Unregistered event handler {}", handler_id);
            Ok(())
        } else {
            Err(AdrscanError::RealtimeError(format!(
                "Invalid handler ID: {}",
                handler_id
            )))
        }
    }

    /// Get current statistics
    pub async fn get_stats(&self) -> EventBusStats {
        let stats = self.stats.read().await;
        EventBusStats {
            events_published: stats.events_published,
            events_delivered: stats.events_delivered,
            active_subscribers: self.sender.receiver_count(),
            failed_deliveries: stats.failed_deliveries,
            average_delivery_time_ms: stats.average_delivery_time_ms,
        }
    }

    /// Publish performance metrics periodically
    pub async fn publish_performance_metrics(
        &self,
        metrics: PerformanceSnapshot,
    ) -> RealtimeResult<()> {
        let event = PipelineEvent::PerformanceMetrics {
            timestamp: SystemTime::now(),
            metrics,
        };
        self.publish(event).await
    }

    /// Publish system alert
    pub async fn publish_alert(
        &self,
        level: AlertLevel,
        message: String,
        component: String,
    ) -> RealtimeResult<()> {
        let event = PipelineEvent::SystemAlert {
            level,
            message,
            component,
            timestamp: SystemTime::now(),
        };
        self.publish(event).await
    }

    /// Start background tasks for the event bus
    pub fn start_background_tasks(&self) {
        // Start performance metrics collection
        self.start_performance_metrics_task();
    }

    fn start_performance_metrics_task(&self) {
        let stats = Arc::clone(&self.stats);
        let performance_tracker = Arc::clone(&self.performance_tracker);
        let sender = self.sender.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(10));

            loop {
                interval.tick().await;

                let (events_per_second, avg_delivery_time) = {
                    let mut tracker = performance_tracker.write().await;
                    let elapsed = tracker.last_performance_update.elapsed();
                    let events_per_second =
                        tracker.events_since_last_update as f64 / elapsed.as_secs_f64();
                    let avg_delivery_time = if tracker.delivery_count > 0 {
                        tracker.total_delivery_time_ms as f64 / tracker.delivery_count as f64
                    } else {
                        0.0
                    };

                    // Reset counters
                    tracker.last_performance_update = Instant::now();
                    tracker.events_since_last_update = 0;
                    tracker.total_delivery_time_ms = 0;
                    tracker.delivery_count = 0;

                    (events_per_second, avg_delivery_time)
                };

                // Update stats
                {
                    let mut stats = stats.write().await;
                    stats.average_delivery_time_ms = avg_delivery_time;
                }

                // Publish performance metrics
                let metrics = PerformanceSnapshot {
                    memory_usage_mb: get_memory_usage(),
                    cpu_usage_percent: get_cpu_usage(),
                    active_watchers: 0,   // Would be updated by actual watchers
                    pending_analyses: 0,  // Would be updated by pipeline
                    cache_hit_ratio: 0.0, // Would be updated by cache
                    average_processing_time_ms: avg_delivery_time,
                    events_per_second,
                };

                let event = PipelineEvent::PerformanceMetrics {
                    timestamp: SystemTime::now(),
                    metrics,
                };

                if let Err(e) = sender.send(event) {
                    log::debug!("No subscribers for performance metrics: {}", e);
                }
            }
        });
    }

    fn should_handle_event(&self, handler: &dyn EventHandler, event: &PipelineEvent) -> bool {
        let event_type = match event {
            PipelineEvent::FileChanged { .. } => "FileChanged",
            PipelineEvent::AnalysisStarted { .. } => "AnalysisStarted",
            PipelineEvent::AnalysisCompleted { .. } => "AnalysisCompleted",
            PipelineEvent::AnalysisError { .. } => "AnalysisError",
            PipelineEvent::PerformanceMetrics { .. } => "PerformanceMetrics",
            PipelineEvent::CacheEvent { .. } => "CacheEvent",
            PipelineEvent::WebSocketEvent { .. } => "WebSocketEvent",
            PipelineEvent::SystemAlert { .. } => "SystemAlert",
        };

        handler.event_types().contains(&event_type.to_string())
    }

    async fn update_performance_metrics(&self, delivery_time: std::time::Duration) {
        let mut tracker = self.performance_tracker.write().await;
        tracker.events_since_last_update += 1;
        tracker.total_delivery_time_ms += delivery_time.as_millis() as u64;
        tracker.delivery_count += 1;
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

/// Example event handler for logging
pub struct LoggingEventHandler {
    log_level: log::Level,
}

impl LoggingEventHandler {
    pub fn new(log_level: log::Level) -> Self {
        Self { log_level }
    }
}

impl EventHandler for LoggingEventHandler {
    fn handle_event(&self, event: PipelineEvent) -> RealtimeResult<()> {
        match self.log_level {
            log::Level::Debug => log::debug!("Event: {:?}", event),
            log::Level::Info => match &event {
                PipelineEvent::AnalysisCompleted { file_path, result } => {
                    log::info!(
                        "Analysis completed for {:?}: drift={:.2}, anomaly={:.2}",
                        file_path,
                        result.drift_probability,
                        result.anomaly_score
                    );
                }
                PipelineEvent::AnalysisError { file_path, error } => {
                    log::info!("Analysis failed for {:?}: {}", file_path, error);
                }
                PipelineEvent::SystemAlert {
                    level,
                    message,
                    component,
                    ..
                } => match level {
                    AlertLevel::Critical | AlertLevel::Error => {
                        log::error!("[{}] {}", component, message)
                    }
                    AlertLevel::Warning => log::warn!("[{}] {}", component, message),
                    AlertLevel::Info => log::info!("[{}] {}", component, message),
                },
                _ => {} // Don't log other events at info level
            },
            _ => {} // Other log levels not handled
        }
        Ok(())
    }

    fn event_types(&self) -> Vec<String> {
        vec![
            "FileChanged".to_string(),
            "AnalysisStarted".to_string(),
            "AnalysisCompleted".to_string(),
            "AnalysisError".to_string(),
            "SystemAlert".to_string(),
        ]
    }
}

// Helper functions for system metrics (placeholders)
fn get_memory_usage() -> f64 {
    // Placeholder - would use system APIs to get actual memory usage
    0.0
}

fn get_cpu_usage() -> f64 {
    // Placeholder - would use system APIs to get actual CPU usage
    0.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct TestHandler {
        event_count: AtomicUsize,
    }

    impl TestHandler {
        fn new() -> Arc<Self> {
            Arc::new(Self {
                event_count: AtomicUsize::new(0),
            })
        }

        fn get_event_count(&self) -> usize {
            self.event_count.load(Ordering::SeqCst)
        }
    }

    impl EventHandler for TestHandler {
        fn handle_event(&self, _event: PipelineEvent) -> RealtimeResult<()> {
            self.event_count.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }

        fn event_types(&self) -> Vec<String> {
            vec!["AnalysisCompleted".to_string()]
        }
    }

    #[tokio::test]
    async fn test_event_bus_creation() {
        let bus = EventBus::new();
        let stats = bus.get_stats().await;
        assert_eq!(stats.events_published, 0);
    }

    #[tokio::test]
    async fn test_event_publishing() {
        let bus = EventBus::new();

        let event = PipelineEvent::FileChanged {
            path: PathBuf::from("test.txt"),
            change_type: "modified".to_string(),
            timestamp: SystemTime::now(),
        };

        bus.publish(event).await.unwrap();

        let stats = bus.get_stats().await;
        assert_eq!(stats.events_published, 1);
    }

    #[tokio::test]
    async fn test_event_handler_registration() {
        let bus = EventBus::new();
        let handler = TestHandler::new();

        bus.register_handler(handler.clone()).await;

        let event = PipelineEvent::AnalysisCompleted {
            file_path: PathBuf::from("test.txt"),
            result: MLAnalysisResult {
                file_path: PathBuf::from("test.txt"),
                analysis_timestamp: SystemTime::now(),
                processing_time_ms: 100,
                drift_probability: 0.5,
                feature_vector: vec![1.0, 2.0, 3.0],
                anomaly_score: 0.3,
                recommendations: vec!["Test recommendation".to_string()],
                metadata: HashMap::new(),
            },
        };

        bus.publish(event).await.unwrap();

        // Small delay to allow handler to process
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;

        assert_eq!(handler.get_event_count(), 1);
    }
}
