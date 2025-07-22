//! Intelligent result caching system for real-time analysis
//!
//! Provides high-performance caching with TTL, memory management,
//! and intelligent cache policies for ML analysis results.

use crate::error::AdrscanError;
use crate::realtime::{RealtimeConfig, RealtimeResult};

use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
use tokio::sync::RwLock;
use tokio::time::interval;

/// Cache entry with TTL and metadata
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    value: T,
    created_at: Instant,
    last_accessed: Instant,
    access_count: u64,
    ttl: Duration,
    size_bytes: usize,
}

impl<T> CacheEntry<T> {
    fn new(value: T, ttl: Duration, size_bytes: usize) -> Self {
        let now = Instant::now();
        Self {
            value,
            created_at: now,
            last_accessed: now,
            access_count: 1,
            ttl,
            size_bytes,
        }
    }

    fn is_expired(&self) -> bool {
        self.created_at.elapsed() > self.ttl
    }

    fn access(&mut self) -> &T {
        self.last_accessed = Instant::now();
        self.access_count += 1;
        &self.value
    }
}

/// Cache configuration and policies
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum memory usage in bytes
    pub max_memory_bytes: usize,
    /// Default TTL for cache entries
    pub default_ttl: Duration,
    /// Cleanup interval for expired entries
    pub cleanup_interval: Duration,
    /// Enable LRU eviction when memory limit is reached
    pub enable_lru_eviction: bool,
    /// Maximum number of entries
    pub max_entries: usize,
    /// Enable cache statistics
    pub enable_statistics: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_memory_bytes: 100 * 1024 * 1024, // 100MB
            default_ttl: Duration::from_secs(3600), // 1 hour
            cleanup_interval: Duration::from_secs(60), // 1 minute
            enable_lru_eviction: true,
            max_entries: 10000,
            enable_statistics: true,
        }
    }
}

/// Cache statistics and metrics
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub entries: usize,
    pub memory_usage_bytes: usize,
    pub memory_usage_percent: f64,
    pub hit_ratio: f64,
    pub average_access_time_ms: f64,
    pub evictions: u64,
    pub expirations: u64,
    pub uptime_seconds: u64,
}

impl CacheStats {
    fn calculate_hit_ratio(&mut self) {
        let total = self.hits + self.misses;
        self.hit_ratio = if total > 0 {
            self.hits as f64 / total as f64
        } else {
            0.0
        };
    }

    fn calculate_memory_usage_percent(&mut self, max_memory: usize) {
        self.memory_usage_percent = if max_memory > 0 {
            (self.memory_usage_bytes as f64 / max_memory as f64) * 100.0
        } else {
            0.0
        };
    }
}

/// High-performance intelligent cache with TTL and LRU eviction
pub struct IntelligentCache {
    config: CacheConfig,
    storage: Arc<DashMap<String, CacheEntry<Vec<u8>>>>,
    stats: Arc<RwLock<CacheStats>>,
    start_time: Instant,
}

impl IntelligentCache {
    /// Create a new intelligent cache
    pub fn new(realtime_config: &RealtimeConfig) -> RealtimeResult<Self> {
        let config = CacheConfig {
            default_ttl: Duration::from_secs(realtime_config.cache_ttl_seconds),
            ..CacheConfig::default()
        };

        let cache = Self {
            config: config.clone(),
            storage: Arc::new(DashMap::new()),
            stats: Arc::new(RwLock::new(CacheStats::default())),
            start_time: Instant::now(),
        };

        // Start background cleanup task
        cache.start_cleanup_task();

        log::info!("Initialized intelligent cache with {}MB limit", 
                  config.max_memory_bytes / (1024 * 1024));
        Ok(cache)
    }

    /// Store a value in the cache with custom TTL
    pub async fn set_with_ttl<T>(&self, key: &str, value: &T, ttl: Duration) -> RealtimeResult<()>
    where
        T: Serialize,
    {
        let serialized = serde_json::to_vec(value)
            .map_err(|e| AdrscanError::RealtimeError(format!("Serialization failed: {}", e)))?;

        let size_bytes = serialized.len();
        
        // Check memory limits before insertion
        if self.should_evict_for_memory(size_bytes).await {
            self.evict_lru_entries(size_bytes).await;
        }

        // Check entry count limits
        if self.storage.len() >= self.config.max_entries {
            self.evict_lru_entries(0).await; // Evict at least one entry
        }

        let entry = CacheEntry::new(serialized, ttl, size_bytes);
        self.storage.insert(key.to_string(), entry);

        // Update statistics
        if self.config.enable_statistics {
            let mut stats = self.stats.write().await;
            stats.entries = self.storage.len();
            stats.memory_usage_bytes += size_bytes;
            stats.calculate_memory_usage_percent(self.config.max_memory_bytes);
        }

        log::debug!("Cached entry '{}' with TTL {:?} ({} bytes)", key, ttl, size_bytes);
        Ok(())
    }

    /// Store a value in the cache with default TTL
    pub async fn set<T>(&self, key: &str, value: &T) -> RealtimeResult<()>
    where
        T: Serialize,
    {
        self.set_with_ttl(key, value, self.config.default_ttl).await
    }

    /// Retrieve a value from the cache
    pub async fn get<T>(&self, key: &str) -> RealtimeResult<Option<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        let start_time = Instant::now();
        
        let result = if let Some(mut entry) = self.storage.get_mut(key) {
            if entry.is_expired() {
                // Remove expired entry
                drop(entry);
                self.storage.remove(key);
                
                if self.config.enable_statistics {
                    let mut stats = self.stats.write().await;
                    stats.misses += 1;
                    stats.expirations += 1;
                    stats.entries = self.storage.len();
                    stats.memory_usage_bytes = self.calculate_memory_usage();
                    stats.calculate_hit_ratio();
                    stats.calculate_memory_usage_percent(self.config.max_memory_bytes);
                }
                
                log::debug!("Cache entry '{}' expired", key);
                None
            } else {
                // Access the entry (updates access time and count)
                let data = entry.access().clone();
                
                if self.config.enable_statistics {
                    let mut stats = self.stats.write().await;
                    stats.hits += 1;
                    stats.calculate_hit_ratio();
                    
                    let access_time_ms = start_time.elapsed().as_millis() as f64;
                    stats.average_access_time_ms = if stats.hits == 1 {
                        access_time_ms
                    } else {
                        (stats.average_access_time_ms * (stats.hits - 1) as f64 + access_time_ms) / stats.hits as f64
                    };
                }
                
                // Deserialize
                match serde_json::from_slice(&data) {
                    Ok(value) => Some(value),
                    Err(e) => {
                        log::error!("Deserialization failed for cache entry '{}': {}", key, e);
                        // Remove corrupted entry
                        drop(entry);
                        self.storage.remove(key);
                        None
                    }
                }
            }
        } else {
            // Cache miss
            if self.config.enable_statistics {
                let mut stats = self.stats.write().await;
                stats.misses += 1;
                stats.calculate_hit_ratio();
            }
            None
        };

        if result.is_some() {
            log::debug!("Cache hit for '{}'", key);
        } else {
            log::debug!("Cache miss for '{}'", key);
        }

        Ok(result)
    }

    /// Remove a specific key from the cache
    pub async fn remove(&self, key: &str) -> RealtimeResult<bool> {
        let removed = self.storage.remove(key).is_some();
        
        if removed && self.config.enable_statistics {
            let mut stats = self.stats.write().await;
            stats.entries = self.storage.len();
            stats.memory_usage_bytes = self.calculate_memory_usage();
            stats.calculate_memory_usage_percent(self.config.max_memory_bytes);
        }
        
        log::debug!("Removed cache entry '{}': {}", key, removed);
        Ok(removed)
    }

    /// Clear all cache entries
    pub async fn clear(&self) -> RealtimeResult<()> {
        let count = self.storage.len();
        self.storage.clear();
        
        if self.config.enable_statistics {
            let mut stats = self.stats.write().await;
            stats.entries = 0;
            stats.memory_usage_bytes = 0;
            stats.memory_usage_percent = 0.0;
        }
        
        log::info!("Cleared cache ({} entries removed)", count);
        Ok(())
    }

    /// Get cache statistics
    pub async fn get_stats(&self) -> CacheStats {
        if !self.config.enable_statistics {
            return CacheStats::default();
        }

        let mut stats = self.stats.read().await.clone();
        stats.entries = self.storage.len();
        stats.memory_usage_bytes = self.calculate_memory_usage();
        stats.uptime_seconds = self.start_time.elapsed().as_secs();
        stats.calculate_memory_usage_percent(self.config.max_memory_bytes);
        stats
    }

    /// Get all cache keys (for debugging)
    pub fn get_keys(&self) -> Vec<String> {
        self.storage.iter().map(|entry| entry.key().clone()).collect()
    }

    /// Check if cache contains a key
    pub fn contains_key(&self, key: &str) -> bool {
        if let Some(entry) = self.storage.get(key) {
            !entry.is_expired()
        } else {
            false
        }
    }

    fn calculate_memory_usage(&self) -> usize {
        self.storage
            .iter()
            .map(|entry| entry.value().size_bytes)
            .sum()
    }

    async fn should_evict_for_memory(&self, additional_bytes: usize) -> bool {
        let current_memory = self.calculate_memory_usage();
        current_memory + additional_bytes > self.config.max_memory_bytes
    }

    async fn evict_lru_entries(&self, target_bytes: usize) {
        if !self.config.enable_lru_eviction {
            return;
        }

        let mut eviction_candidates: Vec<(String, Instant, usize)> = self.storage
            .iter()
            .map(|entry| {
                let key = entry.key().clone();
                let last_accessed = entry.value().last_accessed;
                let size = entry.value().size_bytes;
                (key, last_accessed, size)
            })
            .collect();

        // Sort by last accessed time (oldest first)
        eviction_candidates.sort_by_key(|(_, last_accessed, _)| *last_accessed);

        let mut evicted_bytes = 0;
        let mut evicted_count = 0;

        for (key, _, size) in eviction_candidates {
            if evicted_bytes >= target_bytes && evicted_count > 0 {
                break;
            }

            if self.storage.remove(&key).is_some() {
                evicted_bytes += size;
                evicted_count += 1;
                log::debug!("Evicted cache entry '{}' ({} bytes)", key, size);
            }
        }

        if evicted_count > 0 && self.config.enable_statistics {
            let mut stats = self.stats.write().await;
            stats.evictions += evicted_count;
            stats.entries = self.storage.len();
            stats.memory_usage_bytes = self.calculate_memory_usage();
            stats.calculate_memory_usage_percent(self.config.max_memory_bytes);
        }

        if evicted_count > 0 {
            log::info!("Evicted {} cache entries ({} bytes)", evicted_count, evicted_bytes);
        }
    }

    fn start_cleanup_task(&self) {
        let storage = Arc::clone(&self.storage);
        let stats = Arc::clone(&self.stats);
        let cleanup_interval = self.config.cleanup_interval;
        let enable_statistics = self.config.enable_statistics;
        let max_memory = self.config.max_memory_bytes;

        tokio::spawn(async move {
            let mut interval = interval(cleanup_interval);
            
            loop {
                interval.tick().await;
                
                let mut expired_keys = Vec::new();
                let mut expired_bytes = 0;
                
                // Find expired entries
                for entry in storage.iter() {
                    if entry.value().is_expired() {
                        expired_keys.push(entry.key().clone());
                        expired_bytes += entry.value().size_bytes;
                    }
                }
                
                // Remove expired entries
                for key in &expired_keys {
                    storage.remove(key);
                }
                
                if !expired_keys.is_empty() {
                    log::debug!("Cleaned up {} expired cache entries ({} bytes)", 
                               expired_keys.len(), expired_bytes);
                    
                    if enable_statistics {
                        let mut stats = stats.write().await;
                        stats.expirations += expired_keys.len() as u64;
                        stats.entries = storage.len();
                        let current_memory: usize = storage.iter()
                            .map(|entry| entry.value().size_bytes)
                            .sum();
                        stats.memory_usage_bytes = current_memory;
                        stats.calculate_memory_usage_percent(max_memory);
                    }
                }
            }
        });
    }
}

impl Default for IntelligentCache {
    fn default() -> Self {
        let config = RealtimeConfig::default();
        Self::new(&config).expect("Failed to create default cache")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use tokio::time::sleep;
    
    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestData {
        id: u32,
        name: String,
        values: Vec<f64>,
    }

    #[tokio::test]
    async fn test_cache_creation() {
        let config = RealtimeConfig::default();
        let cache = IntelligentCache::new(&config).unwrap();
        let stats = cache.get_stats().await;
        assert_eq!(stats.entries, 0);
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 0);
    }

    #[tokio::test]
    async fn test_cache_set_and_get() {
        let config = RealtimeConfig::default();
        let cache = IntelligentCache::new(&config).unwrap();
        
        let test_data = TestData {
            id: 1,
            name: "test".to_string(),
            values: vec![1.0, 2.0, 3.0],
        };
        
        // Set value
        cache.set("test_key", &test_data).await.unwrap();
        
        // Get value
        let retrieved: Option<TestData> = cache.get("test_key").await.unwrap();
        assert_eq!(retrieved, Some(test_data));
        
        let stats = cache.get_stats().await;
        assert_eq!(stats.entries, 1);
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 0);
    }

    #[tokio::test]
    async fn test_cache_miss() {
        let config = RealtimeConfig::default();
        let cache = IntelligentCache::new(&config).unwrap();
        
        let result: Option<TestData> = cache.get("nonexistent").await.unwrap();
        assert_eq!(result, None);
        
        let stats = cache.get_stats().await;
        assert_eq!(stats.hits, 0);
        assert_eq!(stats.misses, 1);
    }

    #[tokio::test]
    async fn test_cache_expiration() {
        let config = RealtimeConfig::default();
        let cache = IntelligentCache::new(&config).unwrap();
        
        let test_data = TestData {
            id: 1,
            name: "test".to_string(),
            values: vec![1.0, 2.0, 3.0],
        };
        
        // Set with short TTL
        let short_ttl = Duration::from_millis(50);
        cache.set_with_ttl("test_key", &test_data, short_ttl).await.unwrap();
        
        // Should exist immediately
        let retrieved: Option<TestData> = cache.get("test_key").await.unwrap();
        assert_eq!(retrieved, Some(test_data));
        
        // Wait for expiration
        sleep(Duration::from_millis(100)).await;
        
        // Should be expired
        let retrieved: Option<TestData> = cache.get("test_key").await.unwrap();
        assert_eq!(retrieved, None);
        
        let stats = cache.get_stats().await;
        assert_eq!(stats.expirations, 1);
    }

    #[tokio::test]
    async fn test_cache_remove() {
        let config = RealtimeConfig::default();
        let cache = IntelligentCache::new(&config).unwrap();
        
        let test_data = TestData {
            id: 1,
            name: "test".to_string(),
            values: vec![1.0, 2.0, 3.0],
        };
        
        cache.set("test_key", &test_data).await.unwrap();
        assert!(cache.contains_key("test_key"));
        
        let removed = cache.remove("test_key").await.unwrap();
        assert!(removed);
        assert!(!cache.contains_key("test_key"));
    }

    #[tokio::test]
    async fn test_cache_clear() {
        let config = RealtimeConfig::default();
        let cache = IntelligentCache::new(&config).unwrap();
        
        let test_data = TestData {
            id: 1,
            name: "test".to_string(),
            values: vec![1.0, 2.0, 3.0],
        };
        
        cache.set("key1", &test_data).await.unwrap();
        cache.set("key2", &test_data).await.unwrap();
        
        assert_eq!(cache.get_keys().len(), 2);
        
        cache.clear().await.unwrap();
        assert_eq!(cache.get_keys().len(), 0);
        
        let stats = cache.get_stats().await;
        assert_eq!(stats.entries, 0);
    }

    #[tokio::test]
    async fn test_hit_ratio_calculation() {
        let config = RealtimeConfig::default();
        let cache = IntelligentCache::new(&config).unwrap();
        
        let test_data = TestData {
            id: 1,
            name: "test".to_string(),
            values: vec![1.0, 2.0, 3.0],
        };
        
        cache.set("test_key", &test_data).await.unwrap();
        
        // 2 hits
        let _: Option<TestData> = cache.get("test_key").await.unwrap();
        let _: Option<TestData> = cache.get("test_key").await.unwrap();
        
        // 1 miss
        let _: Option<TestData> = cache.get("nonexistent").await.unwrap();
        
        let stats = cache.get_stats().await;
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 1);
        assert!((stats.hit_ratio - 0.6666666666666666).abs() < 0.0001);
    }
}