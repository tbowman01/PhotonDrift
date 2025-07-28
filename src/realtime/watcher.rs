//! File system event monitoring with high-performance debounced processing
//!
//! Provides cross-platform file system watching with intelligent debouncing
//! to handle rapid file changes efficiently.

use crate::error::AdrscanError;
use crate::realtime::{RealtimeConfig, RealtimeResult};
use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::broadcast;
use tokio::time::{interval, sleep};

/// File change event with metadata
#[derive(Debug, Clone)]
pub struct FileChangeEvent {
    pub path: PathBuf,
    pub kind: FileChangeKind,
    pub timestamp: Instant,
    pub size: Option<u64>,
    pub checksum: Option<String>,
}

/// Types of file change events
#[derive(Debug, Clone, PartialEq)]
pub enum FileChangeKind {
    Created,
    Modified,
    Deleted,
    Renamed { from: PathBuf },
}

/// Debounced file change tracker
struct DebounceTracker {
    pending_changes: HashMap<PathBuf, FileChangeEvent>,
    last_change: HashMap<PathBuf, Instant>,
    debounce_duration: Duration,
}

impl DebounceTracker {
    fn new(debounce_duration: Duration) -> Self {
        Self {
            pending_changes: HashMap::new(),
            last_change: HashMap::new(),
            debounce_duration,
        }
    }

    fn record_change(&mut self, event: FileChangeEvent) {
        let path = event.path.clone();
        let now = Instant::now();

        self.pending_changes.insert(path.clone(), event);
        self.last_change.insert(path, now);
    }

    fn get_ready_changes(&mut self) -> Vec<FileChangeEvent> {
        let now = Instant::now();
        let mut ready_changes = Vec::new();

        let ready_paths: Vec<PathBuf> = self
            .last_change
            .iter()
            .filter(|(_, &last_time)| now.duration_since(last_time) >= self.debounce_duration)
            .map(|(path, _)| path.clone())
            .collect();

        for path in ready_paths {
            if let Some(event) = self.pending_changes.remove(&path) {
                self.last_change.remove(&path);
                ready_changes.push(event);
            }
        }

        ready_changes
    }

    fn has_pending_changes(&self) -> bool {
        !self.pending_changes.is_empty()
    }
}

/// High-performance file system watcher with intelligent debouncing
pub struct FileWatcher {
    config: RealtimeConfig,
    watcher: Option<RecommendedWatcher>,
    event_sender: broadcast::Sender<FileChangeEvent>,
    debounce_tracker: Arc<Mutex<DebounceTracker>>,
    watched_paths: Vec<PathBuf>,
    running: Arc<Mutex<bool>>,
}

impl FileWatcher {
    /// Create a new file watcher with the specified configuration
    pub fn new(config: &RealtimeConfig) -> RealtimeResult<Self> {
        let (event_sender, _) = broadcast::channel(1024);
        let debounce_duration = Duration::from_millis(config.debounce_delay_ms);
        let debounce_tracker = Arc::new(Mutex::new(DebounceTracker::new(debounce_duration)));

        Ok(Self {
            config: config.clone(),
            watcher: None,
            event_sender,
            debounce_tracker,
            watched_paths: Vec::new(),
            running: Arc::new(Mutex::new(false)),
        })
    }

    /// Add a path to watch for changes
    pub fn add_watch_path<P: AsRef<Path>>(&mut self, path: P) -> RealtimeResult<()> {
        let path = path.as_ref().to_path_buf();

        if self.watched_paths.len() >= self.config.max_watched_files {
            return Err(AdrscanError::RealtimeError(format!(
                "Maximum number of watched files ({}) exceeded",
                self.config.max_watched_files
            )));
        }

        self.watched_paths.push(path);
        log::debug!("Added watch path: {:?}", self.watched_paths.last().unwrap());
        Ok(())
    }

    /// Start the file watcher
    pub async fn start(&mut self) -> RealtimeResult<()> {
        {
            let mut running = self.running.lock().unwrap();
            if *running {
                return Err(AdrscanError::RealtimeError(
                    "File watcher is already running".to_string(),
                ));
            }
            *running = true;
        }

        let (tx, rx) = mpsc::channel();

        // Configure the watcher
        let config = Config::default()
            .with_poll_interval(Duration::from_millis(self.config.max_latency_ms / 2))
            .with_compare_contents(true);

        let mut watcher = RecommendedWatcher::new(tx, config)
            .map_err(|e| AdrscanError::RealtimeError(format!("Failed to create watcher: {}", e)))?;

        // Watch all configured paths
        for path in &self.watched_paths {
            watcher.watch(path, RecursiveMode::Recursive).map_err(|e| {
                AdrscanError::RealtimeError(format!("Failed to watch path {:?}: {}", path, e))
            })?;
            log::info!("Watching path: {:?}", path);
        }

        self.watcher = Some(watcher);

        // Start event processing tasks
        self.start_event_processor(rx).await;
        self.start_debounce_processor().await;

        log::info!(
            "File watcher started with {} paths",
            self.watched_paths.len()
        );
        Ok(())
    }

    /// Stop the file watcher
    pub async fn stop(&mut self) -> RealtimeResult<()> {
        {
            let mut running = self.running.lock().unwrap();
            if !*running {
                return Ok(());
            }
            *running = false;
        }

        self.watcher = None;
        log::info!("File watcher stopped");
        Ok(())
    }

    /// Subscribe to file change events
    pub fn subscribe(&self) -> broadcast::Receiver<FileChangeEvent> {
        self.event_sender.subscribe()
    }

    /// Get current statistics
    pub fn get_stats(&self) -> WatcherStats {
        let debounce_tracker = self.debounce_tracker.lock().unwrap();
        WatcherStats {
            watched_paths: self.watched_paths.len(),
            pending_changes: debounce_tracker.pending_changes.len(),
            is_running: *self.running.lock().unwrap(),
        }
    }

    async fn start_event_processor(&self, rx: mpsc::Receiver<Result<Event, notify::Error>>) {
        let debounce_tracker = Arc::clone(&self.debounce_tracker);
        let running = Arc::clone(&self.running);

        tokio::spawn(async move {
            while *running.lock().unwrap() {
                match rx.recv() {
                    Ok(Ok(event)) => {
                        if let Some(file_event) = Self::convert_notify_event(event) {
                            let mut tracker = debounce_tracker.lock().unwrap();
                            tracker.record_change(file_event);
                        }
                    }
                    Ok(Err(e)) => {
                        log::error!("File watcher error: {}", e);
                    }
                    Err(_) => {
                        // Channel closed
                        break;
                    }
                }
            }
        });
    }

    async fn start_debounce_processor(&self) {
        let debounce_tracker = Arc::clone(&self.debounce_tracker);
        let event_sender = self.event_sender.clone();
        let running = Arc::clone(&self.running);
        let check_interval = Duration::from_millis(self.config.max_latency_ms / 4);

        tokio::spawn(async move {
            let mut interval = interval(check_interval);

            while *running.lock().unwrap() {
                interval.tick().await;

                let ready_changes = {
                    let mut tracker = debounce_tracker.lock().unwrap();
                    tracker.get_ready_changes()
                };

                for change in ready_changes {
                    if let Err(e) = event_sender.send(change) {
                        log::debug!("No subscribers for file change event: {}", e);
                    }
                }

                // Sleep if no pending changes to reduce CPU usage
                if !debounce_tracker.lock().unwrap().has_pending_changes() {
                    sleep(Duration::from_millis(100)).await;
                }
            }
        });
    }

    fn convert_notify_event(event: Event) -> Option<FileChangeEvent> {
        let timestamp = Instant::now();

        match event.kind {
            EventKind::Create(_) => {
                if let Some(path) = event.paths.first() {
                    Some(FileChangeEvent {
                        path: path.clone(),
                        kind: FileChangeKind::Created,
                        timestamp,
                        size: std::fs::metadata(path).ok().map(|m| m.len()),
                        checksum: None, // Could be computed later for ML analysis
                    })
                } else {
                    None
                }
            }
            EventKind::Modify(_) => {
                if let Some(path) = event.paths.first() {
                    Some(FileChangeEvent {
                        path: path.clone(),
                        kind: FileChangeKind::Modified,
                        timestamp,
                        size: std::fs::metadata(path).ok().map(|m| m.len()),
                        checksum: None,
                    })
                } else {
                    None
                }
            }
            EventKind::Remove(_) => {
                if let Some(path) = event.paths.first() {
                    Some(FileChangeEvent {
                        path: path.clone(),
                        kind: FileChangeKind::Deleted,
                        timestamp,
                        size: None,
                        checksum: None,
                    })
                } else {
                    None
                }
            }
            _ => None, // Handle other event types as needed
        }
    }
}

/// Statistics for the file watcher
#[derive(Debug)]
pub struct WatcherStats {
    pub watched_paths: usize,
    pub pending_changes: usize,
    pub is_running: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use tokio::time::timeout;

    #[tokio::test]
    async fn test_file_watcher_creation() {
        let config = RealtimeConfig::default();
        let watcher = FileWatcher::new(&config).unwrap();
        assert!(!watcher.get_stats().is_running);
    }

    #[tokio::test]
    async fn test_add_watch_path() {
        let config = RealtimeConfig::default();
        let mut watcher = FileWatcher::new(&config).unwrap();
        let temp_dir = tempdir().unwrap();

        watcher.add_watch_path(temp_dir.path()).unwrap();
        assert_eq!(watcher.get_stats().watched_paths, 1);
    }

    #[tokio::test]
    async fn test_max_watched_files_limit() {
        let mut config = RealtimeConfig::default();
        config.max_watched_files = 1;

        let mut watcher = FileWatcher::new(&config).unwrap();
        let temp_dir1 = tempdir().unwrap();
        let temp_dir2 = tempdir().unwrap();

        watcher.add_watch_path(temp_dir1.path()).unwrap();
        assert!(watcher.add_watch_path(temp_dir2.path()).is_err());
    }
}
