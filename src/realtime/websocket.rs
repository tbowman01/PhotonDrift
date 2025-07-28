//! WebSocket server for real-time communication
//!
//! Provides WebSocket connectivity for real-time updates and monitoring
//! of the file watching and ML analysis system.

use crate::error::AdrscanError;
use crate::realtime::events::{EventBus, PipelineEvent};
use crate::realtime::{RealtimeConfig, RealtimeResult};

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{broadcast, RwLock};
use tokio_tungstenite::{accept_async, tungstenite::protocol::Message, WebSocketStream};

/// WebSocket message types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    /// Client registration
    Register {
        client_id: String,
        subscriptions: Vec<String>,
    },
    /// Real-time event from the system
    Event {
        event: PipelineEvent,
        timestamp: u64,
    },
    /// System status update
    Status {
        active_watchers: usize,
        pending_analyses: usize,
        cache_hit_ratio: f64,
        uptime_seconds: u64,
    },
    /// Client heartbeat/ping
    Ping { client_id: String },
    /// Server heartbeat/pong response
    Pong { timestamp: u64 },
    /// Error message
    Error { message: String },
    /// Client subscription update
    Subscribe {
        client_id: String,
        event_types: Vec<String>,
    },
    /// Client unsubscribe
    Unsubscribe {
        client_id: String,
        event_types: Vec<String>,
    },
}

/// WebSocket client information
#[derive(Debug, Clone)]
struct WebSocketClient {
    id: String,
    addr: SocketAddr,
    subscriptions: Vec<String>,
    connected_at: std::time::SystemTime,
    last_ping: std::time::SystemTime,
}

/// WebSocket server statistics
#[derive(Debug, Default)]
pub struct WebSocketStats {
    pub connected_clients: usize,
    pub total_connections: u64,
    pub messages_sent: u64,
    pub messages_received: u64,
    pub errors: u64,
    pub uptime_seconds: u64,
}

/// High-performance WebSocket server for real-time updates
pub struct WebSocketServer {
    port: u16,
    event_bus: Arc<EventBus>,
    clients: Arc<RwLock<HashMap<String, WebSocketClient>>>,
    message_sender: Arc<RwLock<Option<broadcast::Sender<WebSocketMessage>>>>,
    stats: Arc<RwLock<WebSocketStats>>,
    listener: Option<TcpListener>,
    start_time: std::time::SystemTime,
}

impl WebSocketServer {
    /// Create a new WebSocket server
    pub fn new(port: u16) -> RealtimeResult<Self> {
        let event_bus = Arc::new(EventBus::new());
        let clients = Arc::new(RwLock::new(HashMap::new()));
        let message_sender = Arc::new(RwLock::new(None));
        let stats = Arc::new(RwLock::new(WebSocketStats::default()));

        Ok(Self {
            port,
            event_bus,
            clients,
            message_sender,
            stats,
            listener: None,
            start_time: std::time::SystemTime::now(),
        })
    }

    /// Start the WebSocket server
    pub async fn start(&mut self) -> RealtimeResult<()> {
        let addr = format!("127.0.0.1:{}", self.port);
        let listener = TcpListener::bind(&addr).await.map_err(|e| {
            AdrscanError::RealtimeError(format!("Failed to bind to {}: {}", addr, e))
        })?;

        log::info!("WebSocket server listening on {}", addr);

        // Create message broadcast channel
        let (msg_sender, _) = broadcast::channel(1024);
        {
            let mut sender_guard = self.message_sender.write().await;
            *sender_guard = Some(msg_sender.clone());
        }

        self.listener = Some(listener);

        // Start event subscription
        self.start_event_subscription().await;

        // Start client connection handler
        self.start_connection_handler().await;

        // Start periodic tasks
        self.start_periodic_tasks().await;

        log::info!(
            "WebSocket server started successfully on port {}",
            self.port
        );
        Ok(())
    }

    /// Stop the WebSocket server
    pub async fn stop(&mut self) -> RealtimeResult<()> {
        self.listener = None;

        // Disconnect all clients
        let client_ids: Vec<String> = {
            let clients = self.clients.read().await;
            clients.keys().cloned().collect()
        };

        for client_id in client_ids {
            self.disconnect_client(&client_id).await;
        }

        log::info!("WebSocket server stopped");
        Ok(())
    }

    /// Get current server statistics
    pub async fn get_stats(&self) -> WebSocketStats {
        let mut stats = self.stats.read().await.clone();
        stats.connected_clients = self.clients.read().await.len();
        stats.uptime_seconds = self.start_time.elapsed().unwrap_or_default().as_secs();
        stats
    }

    /// Broadcast a message to all connected clients
    pub async fn broadcast(&self, message: WebSocketMessage) -> RealtimeResult<()> {
        let sender_guard = self.message_sender.read().await;
        if let Some(sender) = sender_guard.as_ref() {
            match sender.send(message) {
                Ok(count) => {
                    let mut stats = self.stats.write().await;
                    stats.messages_sent += count as u64;
                    log::debug!("Broadcasted message to {} clients", count);
                }
                Err(_) => {
                    log::debug!("No WebSocket clients connected for broadcast");
                }
            }
        }
        Ok(())
    }

    async fn start_event_subscription(&self) {
        let event_bus = Arc::clone(&self.event_bus);
        let message_sender = Arc::clone(&self.message_sender);

        tokio::spawn(async move {
            let mut event_receiver = event_bus.subscribe();

            while let Ok(event) = event_receiver.recv().await {
                let ws_message = WebSocketMessage::Event {
                    event,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as u64,
                };

                let sender_guard = message_sender.read().await;
                if let Some(sender) = sender_guard.as_ref() {
                    if let Err(e) = sender.send(ws_message) {
                        log::debug!("Failed to send event to WebSocket clients: {}", e);
                    }
                }
            }
        });
    }

    async fn start_connection_handler(&self) {
        if let Some(listener) = &self.listener {
            let listener = listener.try_clone().expect("Failed to clone listener");
            let clients = Arc::clone(&self.clients);
            let message_sender = Arc::clone(&self.message_sender);
            let stats = Arc::clone(&self.stats);

            tokio::spawn(async move {
                while let Ok((stream, addr)) = listener.accept().await {
                    let clients = Arc::clone(&clients);
                    let message_sender = Arc::clone(&message_sender);
                    let stats = Arc::clone(&stats);

                    tokio::spawn(async move {
                        if let Err(e) =
                            Self::handle_connection(stream, addr, clients, message_sender, stats)
                                .await
                        {
                            log::error!("WebSocket connection error: {}", e);
                        }
                    });
                }
            });
        }
    }

    async fn handle_connection(
        stream: TcpStream,
        addr: SocketAddr,
        clients: Arc<RwLock<HashMap<String, WebSocketClient>>>,
        message_sender: Arc<RwLock<Option<broadcast::Sender<WebSocketMessage>>>>,
        stats: Arc<RwLock<WebSocketStats>>,
    ) -> RealtimeResult<()> {
        log::info!("New WebSocket connection from {}", addr);

        let ws_stream = accept_async(stream).await.map_err(|e| {
            AdrscanError::RealtimeError(format!("WebSocket handshake failed: {}", e))
        })?;

        // Update connection stats
        {
            let mut stats = stats.write().await;
            stats.total_connections += 1;
        }

        let (mut ws_sender, mut ws_receiver) = ws_stream.split();
        let client_id = format!("client_{}", addr);

        // Register client
        {
            let mut clients = clients.write().await;
            clients.insert(
                client_id.clone(),
                WebSocketClient {
                    id: client_id.clone(),
                    addr,
                    subscriptions: vec!["all".to_string()], // Default subscription
                    connected_at: std::time::SystemTime::now(),
                    last_ping: std::time::SystemTime::now(),
                },
            );
        }

        // Subscribe to broadcast messages
        let mut message_receiver = {
            let sender_guard = message_sender.read().await;
            if let Some(sender) = sender_guard.as_ref() {
                sender.subscribe()
            } else {
                return Err(AdrscanError::RealtimeError(
                    "Message sender not available".to_string(),
                ));
            }
        };

        // Handle outgoing messages
        let clients_clone = Arc::clone(&clients);
        let stats_clone = Arc::clone(&stats);
        let client_id_clone = client_id.clone();

        let send_task = tokio::spawn(async move {
            while let Ok(message) = message_receiver.recv().await {
                // Check if client should receive this message
                let should_send = {
                    let clients = clients_clone.read().await;
                    if let Some(client) = clients.get(&client_id_clone) {
                        Self::should_send_to_client(client, &message)
                    } else {
                        false
                    }
                };

                if should_send {
                    let json_msg = serde_json::to_string(&message).unwrap_or_default();
                    if let Err(e) = ws_sender.send(Message::Text(json_msg)).await {
                        log::error!("Failed to send WebSocket message: {}", e);
                        break;
                    }

                    let mut stats = stats_clone.write().await;
                    stats.messages_sent += 1;
                }
            }
        });

        // Handle incoming messages
        let clients_clone = Arc::clone(&clients);
        let stats_clone = Arc::clone(&stats);

        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    let mut stats = stats_clone.write().await;
                    stats.messages_received += 1;
                    drop(stats);

                    if let Err(e) =
                        Self::handle_client_message(&text, &client_id, &clients_clone).await
                    {
                        log::error!("Error handling client message: {}", e);
                        let mut stats = stats_clone.write().await;
                        stats.errors += 1;
                    }
                }
                Ok(Message::Close(_)) => {
                    log::info!("Client {} disconnected", client_id);
                    break;
                }
                Ok(Message::Ping(_)) => {
                    // Handle ping - pong is automatic
                    let mut clients = clients_clone.write().await;
                    if let Some(client) = clients.get_mut(&client_id) {
                        client.last_ping = std::time::SystemTime::now();
                    }
                }
                Err(e) => {
                    log::error!("WebSocket error for client {}: {}", client_id, e);
                    let mut stats = stats_clone.write().await;
                    stats.errors += 1;
                    break;
                }
                _ => {
                    // Handle other message types as needed
                }
            }
        }

        // Cleanup
        send_task.abort();
        {
            let mut clients = clients.write().await;
            clients.remove(&client_id);
        }

        log::info!("Client {} disconnected", client_id);
        Ok(())
    }

    async fn handle_client_message(
        text: &str,
        client_id: &str,
        clients: &Arc<RwLock<HashMap<String, WebSocketClient>>>,
    ) -> RealtimeResult<()> {
        let message: WebSocketMessage = serde_json::from_str(text)
            .map_err(|e| AdrscanError::RealtimeError(format!("Invalid message format: {}", e)))?;

        match message {
            WebSocketMessage::Register {
                client_id: reg_id,
                subscriptions,
            } => {
                let mut clients = clients.write().await;
                if let Some(client) = clients.get_mut(client_id) {
                    client.id = reg_id;
                    client.subscriptions = subscriptions;
                    log::info!(
                        "Client {} registered with subscriptions: {:?}",
                        client.id,
                        client.subscriptions
                    );
                }
            }
            WebSocketMessage::Subscribe {
                client_id: sub_id,
                event_types,
            } => {
                let mut clients = clients.write().await;
                if let Some(client) = clients.get_mut(client_id) {
                    for event_type in event_types {
                        if !client.subscriptions.contains(&event_type) {
                            client.subscriptions.push(event_type);
                        }
                    }
                    log::debug!(
                        "Client {} updated subscriptions: {:?}",
                        sub_id,
                        client.subscriptions
                    );
                }
            }
            WebSocketMessage::Unsubscribe {
                client_id: unsub_id,
                event_types,
            } => {
                let mut clients = clients.write().await;
                if let Some(client) = clients.get_mut(client_id) {
                    client.subscriptions.retain(|s| !event_types.contains(s));
                    log::debug!(
                        "Client {} updated subscriptions: {:?}",
                        unsub_id,
                        client.subscriptions
                    );
                }
            }
            WebSocketMessage::Ping { .. } => {
                let mut clients = clients.write().await;
                if let Some(client) = clients.get_mut(client_id) {
                    client.last_ping = std::time::SystemTime::now();
                }
            }
            _ => {
                log::debug!("Received unexpected message type from client {}", client_id);
            }
        }

        Ok(())
    }

    fn should_send_to_client(client: &WebSocketClient, message: &WebSocketMessage) -> bool {
        // Check if client has "all" subscription
        if client.subscriptions.contains(&"all".to_string()) {
            return true;
        }

        // Check specific subscriptions based on message type
        match message {
            WebSocketMessage::Event { event, .. } => {
                let event_type = match event {
                    PipelineEvent::FileChanged { .. } => "file_changed",
                    PipelineEvent::AnalysisStarted { .. } => "analysis_started",
                    PipelineEvent::AnalysisCompleted { .. } => "analysis_completed",
                    PipelineEvent::AnalysisError { .. } => "analysis_error",
                    PipelineEvent::PerformanceMetrics { .. } => "performance_metrics",
                    PipelineEvent::SystemAlert { .. } => "system_alert",
                    _ => "other",
                };
                client.subscriptions.contains(&event_type.to_string())
            }
            WebSocketMessage::Status { .. } => client.subscriptions.contains(&"status".to_string()),
            _ => true, // Send system messages to all clients
        }
    }

    async fn start_periodic_tasks(&self) {
        // Start status broadcast task
        self.start_status_broadcast_task().await;

        // Start client cleanup task
        self.start_client_cleanup_task().await;
    }

    async fn start_status_broadcast_task(&self) {
        let message_sender = Arc::clone(&self.message_sender);
        let clients = Arc::clone(&self.clients);
        let start_time = self.start_time;

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(30));

            loop {
                interval.tick().await;

                let status_message = WebSocketMessage::Status {
                    active_watchers: 0,   // Would be populated by actual system
                    pending_analyses: 0,  // Would be populated by actual system
                    cache_hit_ratio: 0.0, // Would be populated by actual system
                    uptime_seconds: start_time.elapsed().unwrap_or_default().as_secs(),
                };

                let sender_guard = message_sender.read().await;
                if let Some(sender) = sender_guard.as_ref() {
                    if let Err(e) = sender.send(status_message) {
                        log::debug!("No clients for status broadcast: {}", e);
                    }
                }
            }
        });
    }

    async fn start_client_cleanup_task(&self) {
        let clients = Arc::clone(&self.clients);

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
            let timeout_duration = std::time::Duration::from_secs(300); // 5 minutes

            loop {
                interval.tick().await;

                let now = std::time::SystemTime::now();
                let mut to_remove = Vec::new();

                {
                    let clients = clients.read().await;
                    for (id, client) in clients.iter() {
                        if let Ok(elapsed) = now.duration_since(client.last_ping) {
                            if elapsed > timeout_duration {
                                to_remove.push(id.clone());
                            }
                        }
                    }
                }

                if !to_remove.is_empty() {
                    let mut clients = clients.write().await;
                    for id in to_remove {
                        clients.remove(&id);
                        log::info!("Removed inactive client: {}", id);
                    }
                }
            }
        });
    }

    async fn disconnect_client(&self, client_id: &str) {
        let mut clients = self.clients.write().await;
        clients.remove(client_id);
        log::info!("Disconnected client: {}", client_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_websocket_server_creation() {
        let server = WebSocketServer::new(8080).unwrap();
        let stats = server.get_stats().await;
        assert_eq!(stats.connected_clients, 0);
        assert_eq!(stats.total_connections, 0);
    }

    #[test]
    fn test_websocket_message_serialization() {
        let message = WebSocketMessage::Status {
            active_watchers: 5,
            pending_analyses: 3,
            cache_hit_ratio: 0.85,
            uptime_seconds: 3600,
        };

        let json = serde_json::to_string(&message).unwrap();
        let deserialized: WebSocketMessage = serde_json::from_str(&json).unwrap();

        match deserialized {
            WebSocketMessage::Status {
                active_watchers, ..
            } => {
                assert_eq!(active_watchers, 5);
            }
            _ => panic!("Unexpected message type"),
        }
    }
}
