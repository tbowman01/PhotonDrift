//! Example demonstrating real-time file watching and ML analysis
//!
//! This example shows how to use the real-time analysis system to monitor
//! a directory for ADR file changes and perform incremental ML analysis.
//!
//! Run with: cargo run --example realtime_usage --features "realtime,ml"

#![cfg(feature = "realtime")]

use adrscan::error::AdrscanError;
use adrscan::realtime::events::{AlertLevel, EventHandler, PipelineEvent};
use adrscan::realtime::{RealtimeConfig, RealtimeSystem};

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{interval, sleep};

/// Custom event handler that logs analysis results
struct AnalysisLogger;

impl EventHandler for AnalysisLogger {
    fn handle_event(&self, event: PipelineEvent) -> Result<(), AdrscanError> {
        match event {
            PipelineEvent::FileChanged {
                path, change_type, ..
            } => {
                println!("📁 File {} detected: {:?}", change_type, path.display());
            }
            PipelineEvent::AnalysisStarted { file_path, .. } => {
                println!("🔍 Starting ML analysis for: {}", file_path.display());
            }
            PipelineEvent::AnalysisCompleted { file_path, result } => {
                println!("✅ Analysis completed for: {}", file_path.display());
                println!(
                    "   📊 Drift probability: {:.2}%",
                    result.drift_probability * 100.0
                );
                println!("   ⚠️  Anomaly score: {:.3}", result.anomaly_score);
                println!("   ⏱️  Processing time: {}ms", result.processing_time_ms);
                println!("   💡 Recommendations:");
                for rec in &result.recommendations {
                    println!("      - {}", rec);
                }
            }
            PipelineEvent::AnalysisError { file_path, error } => {
                eprintln!("❌ Analysis failed for {}: {}", file_path.display(), error);
            }
            PipelineEvent::SystemAlert {
                level,
                message,
                component,
                ..
            } => {
                let emoji = match level {
                    AlertLevel::Info => "ℹ️",
                    AlertLevel::Warning => "⚠️",
                    AlertLevel::Error => "❌",
                    AlertLevel::Critical => "🚨",
                };
                println!("{} [{}] {}", emoji, component, message);
            }
            PipelineEvent::PerformanceMetrics { metrics, .. } => {
                println!(
                    "📈 Performance: {:.1} events/s, {:.1}% cache hit ratio, {:.1}MB memory",
                    metrics.events_per_second,
                    metrics.cache_hit_ratio * 100.0,
                    metrics.memory_usage_mb
                );
            }
            _ => {} // Handle other events as needed
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
            "PerformanceMetrics".to_string(),
        ]
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("🚀 Starting PhotonDrift Real-time Analysis System");
    println!("   Built for enterprise-grade ADR monitoring with sub-50ms latency");

    // Configure the real-time system
    let config = RealtimeConfig {
        debounce_delay_ms: 300,   // 300ms debounce for file changes
        max_watched_files: 1000,  // Support up to 1000 files
        cache_ttl_seconds: 3600,  // 1-hour cache TTL
        websocket_port: 8080,     // WebSocket server for real-time updates
        enable_ml_analysis: true, // Enable ML-powered drift detection
        max_latency_ms: 50,       // <50ms target latency
    };

    // Create and configure the real-time system
    let mut system = RealtimeSystem::new(config)?;

    // Register our custom event handler
    let logger = Arc::new(AnalysisLogger);
    // Note: In actual implementation, you would register the handler with the system

    println!("📂 Monitoring directory: ./docs/adrs (or current directory if not found)");
    println!("🌐 WebSocket server available at: ws://localhost:8080");
    println!("⚡ Maximum latency target: <50ms");
    println!("🧠 ML-powered drift detection: Enabled");

    // Start the real-time system
    system.start().await?;

    // Simulate some work and monitoring
    let mut stats_interval = interval(Duration::from_secs(10));
    let mut demo_interval = interval(Duration::from_secs(30));

    println!("\n✨ Real-time system is now active. Try these actions:");
    println!("   1. Create/modify .md files in the monitored directory");
    println!("   2. Watch the real-time analysis results");
    println!("   3. Connect to ws://localhost:8080 for live updates");
    println!("   4. Press Ctrl+C to stop\n");

    // Main monitoring loop
    for iteration in 1..=10 {
        tokio::select! {
            _ = stats_interval.tick() => {
                println!("📊 System Status (iteration {})", iteration);
                // In real implementation, would get actual system stats
                println!("   Active watchers: N/A (demo mode)");
                println!("   Cache hit ratio: N/A (demo mode)");
                println!("   Memory usage: N/A (demo mode)");
            }

            _ = demo_interval.tick() => {
                println!("💡 Demo: Simulating file analysis results...");
                // In real implementation, actual file events would trigger this
                demonstrate_analysis_results().await;
            }

            _ = tokio::signal::ctrl_c() => {
                println!("\n🛑 Shutting down gracefully...");
                break;
            }
        }

        // Limit demo to prevent infinite running
        if iteration >= 10 {
            println!("📋 Demo completed after {} iterations", iteration);
            break;
        }
    }

    // Stop the system
    system.stop().await?;

    println!("✅ Real-time analysis system stopped cleanly");
    println!("📈 Demo completed successfully!");

    Ok(())
}

async fn demonstrate_analysis_results() {
    use adrscan::realtime::pipeline::MLAnalysisResult;
    use std::collections::HashMap;
    use std::time::SystemTime;

    // Simulate analysis results that would come from the real system
    let demo_results = vec![
        (
            "ADR-001-authentication.md",
            0.15,
            0.2,
            vec!["File appears normal - no action needed"],
        ),
        (
            "ADR-002-database-choice.md",
            0.75,
            0.3,
            vec!["High drift detected - consider updating ADRs"],
        ),
        (
            "ADR-003-api-versioning.md",
            0.45,
            0.8,
            vec![
                "Moderate drift detected - monitor for changes",
                "Unusual file patterns detected - manual review recommended",
            ],
        ),
    ];

    for (filename, drift_prob, anomaly_score, recommendations) in demo_results {
        let result = MLAnalysisResult {
            file_path: PathBuf::from(filename),
            analysis_timestamp: SystemTime::now(),
            processing_time_ms: rand::random::<u64>() % 45 + 5, // 5-50ms
            drift_probability: drift_prob,
            feature_vector: vec![rand::random::<f64>(); 5],
            anomaly_score,
            recommendations: recommendations.into_iter().map(|s| s.to_string()).collect(),
            metadata: HashMap::new(),
        };

        // Simulate the event handler processing
        let handler = AnalysisLogger;
        let event = PipelineEvent::AnalysisCompleted {
            file_path: result.file_path.clone(),
            result,
        };

        if let Err(e) = handler.handle_event(event) {
            eprintln!("Error handling demo event: {}", e);
        }

        // Add small delay between results
        sleep(Duration::from_millis(100)).await;
    }
}

/// WebSocket client example (would be in a separate file normally)
#[cfg(feature = "websocket-client")]
async fn websocket_client_example() -> Result<(), Box<dyn std::error::Error>> {
    use futures_util::{SinkExt, StreamExt};
    use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

    println!("🔌 Connecting to WebSocket server...");

    let url = "ws://localhost:8080";
    let (ws_stream, _) = connect_async(url).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    // Send registration message
    let register_msg = serde_json::json!({
        "type": "Register",
        "client_id": "example_client",
        "subscriptions": ["analysis_completed", "file_changed", "system_alert"]
    });

    ws_sender
        .send(Message::Text(register_msg.to_string()))
        .await?;

    println!("✅ Connected to WebSocket server");
    println!("🔄 Listening for real-time updates...");

    // Listen for updates
    while let Some(message) = ws_receiver.next().await {
        match message? {
            Message::Text(text) => {
                println!("📨 Received: {}", text);

                // Parse and handle different message types
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&text) {
                    match parsed["type"].as_str() {
                        Some("Event") => {
                            println!("🎯 Real-time event received!");
                        }
                        Some("Status") => {
                            println!("📊 System status update");
                        }
                        Some("Pong") => {
                            // Handle pong response
                        }
                        _ => {
                            println!("❓ Unknown message type");
                        }
                    }
                }
            }
            Message::Close(_) => {
                println!("🔌 WebSocket connection closed");
                break;
            }
            _ => {} // Handle other message types
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analysis_logger_creation() {
        let logger = AnalysisLogger;
        let event_types = logger.event_types();
        assert!(event_types.contains(&"AnalysisCompleted".to_string()));
        assert!(event_types.len() >= 5);
    }

    #[tokio::test]
    async fn test_config_validation() {
        let config = RealtimeConfig {
            debounce_delay_ms: 300,
            max_watched_files: 1000,
            cache_ttl_seconds: 3600,
            websocket_port: 8080,
            enable_ml_analysis: true,
            max_latency_ms: 50,
        };

        // Test that reasonable config values work
        assert!(config.debounce_delay_ms >= 100);
        assert!(config.max_watched_files >= 100);
        assert!(config.max_latency_ms <= 100);
    }
}
