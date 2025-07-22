//! ML processing pipeline for real-time analysis
//!
//! Provides event-driven machine learning analysis with efficient
//! incremental processing and result caching.

use crate::error::AdrscanError;
use crate::realtime::{RealtimeConfig, RealtimeResult};
use crate::realtime::events::{EventBus, PipelineEvent};
use crate::realtime::watcher::FileChangeEvent;
use crate::realtime::cache::IntelligentCache;

#[cfg(feature = "ml")]
use crate::ml::{MLDetector, MLFeatures};

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, Mutex, RwLock};
use tokio::time::timeout;

/// ML analysis result for a file
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MLAnalysisResult {
    pub file_path: PathBuf,
    pub analysis_timestamp: std::time::SystemTime,
    pub processing_time_ms: u64,
    pub drift_probability: f64,
    pub feature_vector: Vec<f64>,
    pub anomaly_score: f64,
    pub recommendations: Vec<String>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Pipeline processing statistics
#[derive(Debug, Default)]
pub struct PipelineStats {
    pub files_processed: u64,
    pub total_processing_time_ms: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub errors: u64,
    pub average_latency_ms: f64,
    pub peak_latency_ms: u64,
}

/// Configuration for ML pipeline processing
#[derive(Debug, Clone)]
pub struct PipelineConfig {
    pub max_concurrent_jobs: usize,
    pub processing_timeout_ms: u64,
    pub enable_caching: bool,
    pub feature_extraction_enabled: bool,
    pub anomaly_detection_enabled: bool,
    pub drift_detection_enabled: bool,
}

impl Default for PipelineConfig {
    fn default() -> Self {
        Self {
            max_concurrent_jobs: 4,
            processing_timeout_ms: 5000,
            enable_caching: true,
            feature_extraction_enabled: true,
            anomaly_detection_enabled: true,
            drift_detection_enabled: true,
        }
    }
}

/// Event-driven ML processing pipeline
pub struct MLPipeline {
    config: RealtimeConfig,
    pipeline_config: PipelineConfig,
    event_bus: Arc<EventBus>,
    cache: Arc<IntelligentCache>,
    stats: Arc<RwLock<PipelineStats>>,
    processing_queue: Arc<Mutex<mpsc::UnboundedSender<FileChangeEvent>>>,
    active_jobs: Arc<Mutex<HashMap<PathBuf, Instant>>>,
    
    #[cfg(feature = "ml")]
    ml_detector: Arc<Mutex<MLDetector>>,
}

impl MLPipeline {
    /// Create a new ML processing pipeline
    pub fn new(config: &RealtimeConfig) -> RealtimeResult<Self> {
        let pipeline_config = PipelineConfig::default();
        let event_bus = Arc::new(EventBus::new());
        let cache = Arc::new(IntelligentCache::new(config)?);
        let stats = Arc::new(RwLock::new(PipelineStats::default()));
        let (tx, rx) = mpsc::unbounded_channel();
        let processing_queue = Arc::new(Mutex::new(tx));
        let active_jobs = Arc::new(Mutex::new(HashMap::new()));
        
        #[cfg(feature = "ml")]
        let ml_detector = Arc::new(Mutex::new(MLDetector::new()?));
        
        let pipeline = Self {
            config: config.clone(),
            pipeline_config,
            event_bus,
            cache,
            stats,
            processing_queue,
            active_jobs,
            
            #[cfg(feature = "ml")]
            ml_detector,
        };
        
        // Start processing workers
        pipeline.start_processing_workers(rx);
        
        Ok(pipeline)
    }

    /// Process a file change event through the ML pipeline
    pub async fn process_file_change(&self, event: FileChangeEvent) -> RealtimeResult<Option<MLAnalysisResult>> {
        let start_time = Instant::now();
        
        // Check if file is already being processed
        if self.is_file_being_processed(&event.path).await {
            log::debug!("File {:?} is already being processed, skipping", event.path);
            return Ok(None);
        }
        
        // Check cache first if enabled
        if self.pipeline_config.enable_caching {
            if let Some(cached_result) = self.get_cached_result(&event.path).await? {
                self.update_cache_hit_stats().await;
                return Ok(Some(cached_result));
            }
        }
        
        self.update_cache_miss_stats().await;
        
        // Mark file as being processed
        self.mark_file_processing(&event.path, start_time).await;
        
        // Queue for processing
        let processing_queue = self.processing_queue.lock().await;
        processing_queue.send(event)
            .map_err(|e| AdrscanError::RealtimeError(format!("Failed to queue file for processing: {}", e)))?;
        
        Ok(None) // Actual result will be available via event bus
    }

    /// Get current pipeline statistics
    pub async fn get_stats(&self) -> PipelineStats {
        self.stats.read().await.clone()
    }

    /// Subscribe to pipeline events
    pub fn subscribe_to_events(&self) -> tokio::sync::broadcast::Receiver<PipelineEvent> {
        self.event_bus.subscribe()
    }

    /// Perform ML analysis on a file
    #[cfg(feature = "ml")]
    async fn perform_ml_analysis(&self, event: &FileChangeEvent) -> RealtimeResult<MLAnalysisResult> {
        let start_time = Instant::now();
        
        // Extract features from the file
        let features = self.extract_features(&event.path).await?;
        
        // Perform drift detection
        let drift_probability = if self.pipeline_config.drift_detection_enabled {
            self.detect_drift(&features).await?
        } else {
            0.0
        };
        
        // Perform anomaly detection
        let anomaly_score = if self.pipeline_config.anomaly_detection_enabled {
            self.detect_anomaly(&features).await?
        } else {
            0.0
        };
        
        // Generate recommendations
        let recommendations = self.generate_recommendations(drift_probability, anomaly_score).await;
        
        let processing_time = start_time.elapsed();
        
        let result = MLAnalysisResult {
            file_path: event.path.clone(),
            analysis_timestamp: std::time::SystemTime::now(),
            processing_time_ms: processing_time.as_millis() as u64,
            drift_probability,
            feature_vector: features.to_vec(),
            anomaly_score,
            recommendations,
            metadata: self.create_metadata(&event).await,
        };
        
        // Update statistics
        self.update_processing_stats(processing_time).await;
        
        Ok(result)
    }

    /// Fallback analysis when ML features are not available
    #[cfg(not(feature = "ml"))]
    async fn perform_ml_analysis(&self, event: &FileChangeEvent) -> RealtimeResult<MLAnalysisResult> {
        let start_time = Instant::now();
        
        // Basic analysis without ML features
        let basic_features = self.extract_basic_features(&event.path).await?;
        let processing_time = start_time.elapsed();
        
        let result = MLAnalysisResult {
            file_path: event.path.clone(),
            analysis_timestamp: std::time::SystemTime::now(),
            processing_time_ms: processing_time.as_millis() as u64,
            drift_probability: 0.0, // No ML analysis available
            feature_vector: basic_features,
            anomaly_score: 0.0,
            recommendations: vec!["ML features not available - enable 'ml' feature for advanced analysis".to_string()],
            metadata: self.create_metadata(&event).await,
        };
        
        self.update_processing_stats(processing_time).await;
        Ok(result)
    }

    fn start_processing_workers(&self, mut rx: mpsc::UnboundedReceiver<FileChangeEvent>) {
        let pipeline = Arc::new(self.clone_for_worker());
        
        for worker_id in 0..self.pipeline_config.max_concurrent_jobs {
            let pipeline_worker = Arc::clone(&pipeline);
            let mut rx_worker = rx; // Move receiver to first worker
            
            tokio::spawn(async move {
                log::debug!("Starting ML pipeline worker {}", worker_id);
                
                while let Some(event) = rx_worker.recv().await {
                    let processing_timeout = Duration::from_millis(pipeline_worker.pipeline_config.processing_timeout_ms);
                    
                    match timeout(processing_timeout, pipeline_worker.perform_ml_analysis(&event)).await {
                        Ok(Ok(result)) => {
                            // Cache the result if caching is enabled
                            if pipeline_worker.pipeline_config.enable_caching {
                                if let Err(e) = pipeline_worker.cache_result(&result).await {
                                    log::error!("Failed to cache analysis result: {}", e);
                                }
                            }
                            
                            // Publish result via event bus
                            let pipeline_event = PipelineEvent::AnalysisCompleted {
                                file_path: result.file_path.clone(),
                                result: result.clone(),
                            };
                            
                            if let Err(e) = pipeline_worker.event_bus.publish(pipeline_event).await {
                                log::error!("Failed to publish analysis result: {}", e);
                            }
                            
                            log::debug!("Completed ML analysis for {:?} in {}ms", 
                                       result.file_path, result.processing_time_ms);
                        }
                        Ok(Err(e)) => {
                            log::error!("ML analysis failed for {:?}: {}", event.path, e);
                            pipeline_worker.update_error_stats().await;
                            
                            let error_event = PipelineEvent::AnalysisError {
                                file_path: event.path.clone(),
                                error: format!("Analysis failed: {}", e),
                            };
                            
                            let _ = pipeline_worker.event_bus.publish(error_event).await;
                        }
                        Err(_) => {
                            log::error!("ML analysis timed out for {:?}", event.path);
                            pipeline_worker.update_error_stats().await;
                            
                            let timeout_event = PipelineEvent::AnalysisError {
                                file_path: event.path.clone(),
                                error: "Analysis timed out".to_string(),
                            };
                            
                            let _ = pipeline_worker.event_bus.publish(timeout_event).await;
                        }
                    }
                    
                    // Remove from active jobs
                    pipeline_worker.unmark_file_processing(&event.path).await;
                }
                
                log::debug!("ML pipeline worker {} stopped", worker_id);
            });
            
            // Only create receiver for first worker
            break;
        }
    }

    fn clone_for_worker(&self) -> MLPipeline {
        MLPipeline {
            config: self.config.clone(),
            pipeline_config: self.pipeline_config.clone(),
            event_bus: Arc::clone(&self.event_bus),
            cache: Arc::clone(&self.cache),
            stats: Arc::clone(&self.stats),
            processing_queue: Arc::clone(&self.processing_queue),
            active_jobs: Arc::clone(&self.active_jobs),
            
            #[cfg(feature = "ml")]
            ml_detector: Arc::clone(&self.ml_detector),
        }
    }

    async fn is_file_being_processed(&self, path: &PathBuf) -> bool {
        let active_jobs = self.active_jobs.lock().await;
        active_jobs.contains_key(path)
    }

    async fn mark_file_processing(&self, path: &PathBuf, start_time: Instant) {
        let mut active_jobs = self.active_jobs.lock().await;
        active_jobs.insert(path.clone(), start_time);
    }

    async fn unmark_file_processing(&self, path: &PathBuf) {
        let mut active_jobs = self.active_jobs.lock().await;
        active_jobs.remove(path);
    }

    async fn get_cached_result(&self, path: &PathBuf) -> RealtimeResult<Option<MLAnalysisResult>> {
        let cache_key = format!("ml_analysis:{}", path.display());
        self.cache.get(&cache_key).await
    }

    async fn cache_result(&self, result: &MLAnalysisResult) -> RealtimeResult<()> {
        let cache_key = format!("ml_analysis:{}", result.file_path.display());
        self.cache.set(&cache_key, result).await
    }

    #[cfg(feature = "ml")]
    async fn extract_features(&self, _path: &PathBuf) -> RealtimeResult<Vec<f64>> {
        // Placeholder for ML feature extraction
        // In real implementation, this would extract various features from the file
        Ok(vec![0.1, 0.2, 0.3, 0.4, 0.5]) // Dummy features
    }

    async fn extract_basic_features(&self, path: &PathBuf) -> RealtimeResult<Vec<f64>> {
        // Basic features without ML: file size, modification time, etc.
        let metadata = std::fs::metadata(path)
            .map_err(|e| AdrscanError::RealtimeError(format!("Failed to read file metadata: {}", e)))?;
        
        let file_size = metadata.len() as f64;
        let modified_time = metadata.modified()
            .map_err(|e| AdrscanError::RealtimeError(format!("Failed to get modification time: {}", e)))?
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| AdrscanError::RealtimeError(format!("Invalid modification time: {}", e)))?
            .as_secs() as f64;
        
        Ok(vec![file_size, modified_time])
    }

    #[cfg(feature = "ml")]
    async fn detect_drift(&self, features: &[f64]) -> RealtimeResult<f64> {
        // Placeholder for drift detection
        // In real implementation, this would use the ML detector
        Ok(features.iter().sum::<f64>() / features.len() as f64)
    }

    #[cfg(feature = "ml")]
    async fn detect_anomaly(&self, features: &[f64]) -> RealtimeResult<f64> {
        // Placeholder for anomaly detection
        // In real implementation, this would use statistical methods or ML models
        let mean = features.iter().sum::<f64>() / features.len() as f64;
        let variance = features.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / features.len() as f64;
        Ok(variance.sqrt())
    }

    async fn generate_recommendations(&self, drift_prob: f64, anomaly_score: f64) -> Vec<String> {
        let mut recommendations = Vec::new();
        
        if drift_prob > 0.7 {
            recommendations.push("High drift detected - consider updating ADRs".to_string());
        } else if drift_prob > 0.5 {
            recommendations.push("Moderate drift detected - monitor for changes".to_string());
        }
        
        if anomaly_score > 0.8 {
            recommendations.push("Unusual file patterns detected - manual review recommended".to_string());
        }
        
        if recommendations.is_empty() {
            recommendations.push("File appears normal - no action needed".to_string());
        }
        
        recommendations
    }

    async fn create_metadata(&self, event: &FileChangeEvent) -> HashMap<String, serde_json::Value> {
        let mut metadata = HashMap::new();
        
        metadata.insert("event_kind".to_string(), 
                        serde_json::Value::String(format!("{:?}", event.kind)));
        metadata.insert("timestamp".to_string(), 
                        serde_json::Value::String(format!("{:?}", event.timestamp)));
        
        if let Some(size) = event.size {
            metadata.insert("file_size".to_string(), 
                           serde_json::Value::Number(serde_json::Number::from(size)));
        }
        
        metadata
    }

    async fn update_processing_stats(&self, processing_time: Duration) {
        let mut stats = self.stats.write().await;
        stats.files_processed += 1;
        
        let processing_time_ms = processing_time.as_millis() as u64;
        stats.total_processing_time_ms += processing_time_ms;
        stats.average_latency_ms = stats.total_processing_time_ms as f64 / stats.files_processed as f64;
        
        if processing_time_ms > stats.peak_latency_ms {
            stats.peak_latency_ms = processing_time_ms;
        }
    }

    async fn update_cache_hit_stats(&self) {
        let mut stats = self.stats.write().await;
        stats.cache_hits += 1;
    }

    async fn update_cache_miss_stats(&self) {
        let mut stats = self.stats.write().await;
        stats.cache_misses += 1;
    }

    async fn update_error_stats(&self) {
        let mut stats = self.stats.write().await;
        stats.errors += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use crate::realtime::watcher::FileChangeKind;
    
    #[tokio::test]
    async fn test_pipeline_creation() {
        let config = RealtimeConfig::default();
        let pipeline = MLPipeline::new(&config).unwrap();
        
        let stats = pipeline.get_stats().await;
        assert_eq!(stats.files_processed, 0);
    }

    #[tokio::test]
    async fn test_basic_feature_extraction() {
        let config = RealtimeConfig::default();
        let pipeline = MLPipeline::new(&config).unwrap();
        let temp_dir = tempdir().unwrap();
        let temp_file = temp_dir.path().join("test.txt");
        std::fs::write(&temp_file, "test content").unwrap();
        
        let features = pipeline.extract_basic_features(&temp_file).await.unwrap();
        assert_eq!(features.len(), 2); // file size and modification time
        assert!(features[0] > 0.0); // file size should be positive
    }
}