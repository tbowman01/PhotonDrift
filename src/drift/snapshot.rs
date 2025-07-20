//! Codebase Snapshot Management
//! 
//! This module handles creating, storing, and loading snapshots of codebase
//! state for drift detection and comparison.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use crate::drift::{DriftResult, TechnologyMatch};
use crate::error::AdrscanError;

/// A snapshot of codebase state at a specific point in time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Snapshot {
    /// Timestamp when snapshot was created
    pub timestamp: DateTime<Utc>,
    
    /// Version of the snapshot format
    pub version: String,
    
    /// Root directory that was scanned
    pub root_directory: PathBuf,
    
    /// Git commit hash (if available)
    pub git_commit: Option<String>,
    
    /// Git branch (if available)
    pub git_branch: Option<String>,
    
    /// Individual entries for each file/technology detected
    pub entries: Vec<SnapshotEntry>,
    
    /// Summary statistics
    pub statistics: SnapshotStatistics,
    
    /// Metadata about the scan
    pub metadata: HashMap<String, String>,
}

/// Individual entry in a snapshot representing a detected technology or file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotEntry {
    /// Unique identifier for this entry
    pub id: String,
    
    /// Type of entry (technology, file, configuration, etc.)
    pub entry_type: SnapshotEntryType,
    
    /// File path relative to root directory
    pub file_path: String,
    
    /// Technology or pattern that was detected
    pub technology: Option<String>,
    
    /// Category of the detection
    pub category: String,
    
    /// Line number where detected (if applicable)
    pub line_number: Option<usize>,
    
    /// The actual content that was matched
    pub matched_content: Option<String>,
    
    /// File hash for change detection
    pub file_hash: Option<String>,
    
    /// File size in bytes
    pub file_size: Option<u64>,
    
    /// File modification time
    pub modified_time: Option<DateTime<Utc>>,
    
    /// Additional metadata for this entry
    pub metadata: HashMap<String, String>,
}

/// Types of entries that can be stored in a snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SnapshotEntryType {
    /// A detected technology (library, framework, etc.)
    Technology,
    /// A configuration file
    Configuration,
    /// A source code file
    SourceFile,
    /// Infrastructure as Code file
    Infrastructure,
    /// Documentation file
    Documentation,
    /// Test file
    Test,
    /// Build/deployment file
    Build,
    /// Other type of file
    Other,
}

/// Statistics about the snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SnapshotStatistics {
    /// Total number of files scanned
    pub files_scanned: usize,
    
    /// Total number of technologies detected
    pub technologies_detected: usize,
    
    /// Breakdown by file type
    pub file_types: HashMap<String, usize>,
    
    /// Breakdown by technology category
    pub technology_categories: HashMap<String, usize>,
    
    /// Total lines of code analyzed
    pub lines_of_code: usize,
    
    /// Scan duration in milliseconds
    pub scan_duration_ms: u64,
}

impl Snapshot {
    /// Create a new empty snapshot
    pub fn new(root_directory: PathBuf) -> Self {
        Self {
            timestamp: Utc::now(),
            version: "1.0".to_string(),
            root_directory,
            git_commit: None,
            git_branch: None,
            entries: Vec::new(),
            statistics: SnapshotStatistics::default(),
            metadata: HashMap::new(),
        }
    }
    
    /// Add a technology match to the snapshot
    pub fn add_technology_match(&mut self, tech_match: &TechnologyMatch) {
        let id = format!("tech_{}_{}", tech_match.file_path.replace('/', "_"), tech_match.line_number);
        
        let entry = SnapshotEntry {
            id,
            entry_type: SnapshotEntryType::Technology,
            file_path: tech_match.file_path.clone(),
            technology: Some(tech_match.pattern.name.clone()),
            category: tech_match.pattern.category.clone(),
            line_number: Some(tech_match.line_number),
            matched_content: Some(tech_match.matched_text.clone()),
            file_hash: None,
            file_size: None,
            modified_time: None,
            metadata: HashMap::new(),
        };
        
        self.entries.push(entry);
        self.update_statistics();
    }
    
    /// Add a file entry to the snapshot
    pub fn add_file_entry(
        &mut self,
        file_path: &str,
        entry_type: SnapshotEntryType,
        file_hash: Option<String>,
        file_size: Option<u64>,
        modified_time: Option<DateTime<Utc>>,
    ) {
        let id = format!("file_{}", file_path.replace('/', "_"));
        
        let entry = SnapshotEntry {
            id,
            entry_type,
            file_path: file_path.to_string(),
            technology: None,
            category: "file".to_string(),
            line_number: None,
            matched_content: None,
            file_hash,
            file_size,
            modified_time,
            metadata: HashMap::new(),
        };
        
        self.entries.push(entry);
        self.update_statistics();
    }
    
    /// Update statistics based on current entries
    fn update_statistics(&mut self) {
        let mut file_types = HashMap::new();
        let mut tech_categories = HashMap::new();
        let mut tech_count = 0;
        
        for entry in &self.entries {
            match entry.entry_type {
                SnapshotEntryType::Technology => {
                    tech_count += 1;
                    *tech_categories.entry(entry.category.clone()).or_insert(0) += 1;
                }
                _ => {
                    let ext = Path::new(&entry.file_path)
                        .extension()
                        .and_then(|s| s.to_str())
                        .unwrap_or("unknown")
                        .to_string();
                    *file_types.entry(ext).or_insert(0) += 1;
                }
            }
        }
        
        self.statistics.files_scanned = self.entries.iter()
            .filter(|e| !matches!(e.entry_type, SnapshotEntryType::Technology))
            .count();
        self.statistics.technologies_detected = tech_count;
        self.statistics.file_types = file_types;
        self.statistics.technology_categories = tech_categories;
    }
    
    /// Set git information
    pub fn with_git_info(mut self, commit: Option<String>, branch: Option<String>) -> Self {
        self.git_commit = commit;
        self.git_branch = branch;
        self
    }
    
    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
    
    /// Save snapshot to file
    pub fn save(&self, path: &Path) -> DriftResult<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| AdrscanError::SerializationError(e.to_string()))?;
        
        std::fs::write(path, json)
            .map_err(|e| AdrscanError::Io(e))?;
        
        log::info!("Snapshot saved to: {}", path.display());
        Ok(())
    }
    
    /// Load snapshot from file
    pub fn load(path: &Path) -> DriftResult<Self> {
        let content = std::fs::read_to_string(path)
            .map_err(|e| AdrscanError::FileNotFound(
                format!("Cannot read snapshot file {}: {}", path.display(), e)
            ))?;
        
        let snapshot: Self = serde_json::from_str(&content)
            .map_err(|e| AdrscanError::SerializationError(
                format!("Invalid snapshot format: {}", e)
            ))?;
        
        log::info!("Snapshot loaded from: {}", path.display());
        Ok(snapshot)
    }
    
    /// Compare this snapshot with another to find differences
    pub fn compare_with(&self, other: &Snapshot) -> SnapshotComparison {
        let mut added_entries = Vec::new();
        let mut removed_entries = Vec::new();
        let mut modified_entries = Vec::new();
        
        // Create lookup maps
        let self_entries: HashMap<&str, &SnapshotEntry> = 
            self.entries.iter().map(|e| (e.id.as_str(), e)).collect();
        let other_entries: HashMap<&str, &SnapshotEntry> = 
            other.entries.iter().map(|e| (e.id.as_str(), e)).collect();
        
        // Find added entries (in self but not in other)
        for entry in &self.entries {
            if !other_entries.contains_key(entry.id.as_str()) {
                added_entries.push(entry.clone());
            }
        }
        
        // Find removed entries (in other but not in self)
        for entry in &other.entries {
            if !self_entries.contains_key(entry.id.as_str()) {
                removed_entries.push(entry.clone());
            }
        }
        
        // Find modified entries (different content for same ID)
        for entry in &self.entries {
            if let Some(other_entry) = other_entries.get(entry.id.as_str()) {
                if entry.file_hash != other_entry.file_hash ||
                   entry.matched_content != other_entry.matched_content ||
                   entry.modified_time != other_entry.modified_time {
                    modified_entries.push((entry.clone(), (*other_entry).clone()));
                }
            }
        }
        
        SnapshotComparison {
            baseline: other.clone(),
            current: self.clone(),
            added_entries,
            removed_entries,
            modified_entries,
        }
    }
    
    /// Get entries by category
    pub fn entries_by_category(&self, category: &str) -> Vec<&SnapshotEntry> {
        self.entries.iter()
            .filter(|e| e.category == category)
            .collect()
    }
    
    /// Get technology entries only
    pub fn technology_entries(&self) -> Vec<&SnapshotEntry> {
        self.entries.iter()
            .filter(|e| matches!(e.entry_type, SnapshotEntryType::Technology))
            .collect()
    }
}

/// Result of comparing two snapshots
#[derive(Debug, Clone)]
pub struct SnapshotComparison {
    /// The baseline snapshot (older)
    pub baseline: Snapshot,
    
    /// The current snapshot (newer)
    pub current: Snapshot,
    
    /// Entries added since baseline
    pub added_entries: Vec<SnapshotEntry>,
    
    /// Entries removed since baseline
    pub removed_entries: Vec<SnapshotEntry>,
    
    /// Entries modified since baseline (current, baseline)
    pub modified_entries: Vec<(SnapshotEntry, SnapshotEntry)>,
}

impl SnapshotComparison {
    /// Get summary of changes
    pub fn summary(&self) -> String {
        format!(
            "Snapshot Comparison:\n\
             - Added: {} entries\n\
             - Removed: {} entries\n\
             - Modified: {} entries\n\
             - Total changes: {}",
            self.added_entries.len(),
            self.removed_entries.len(),
            self.modified_entries.len(),
            self.added_entries.len() + self.removed_entries.len() + self.modified_entries.len()
        )
    }
    
    /// Get technology-specific changes
    pub fn technology_changes(&self) -> (Vec<&SnapshotEntry>, Vec<&SnapshotEntry>, Vec<&SnapshotEntry>) {
        let added_tech: Vec<&SnapshotEntry> = self.added_entries.iter()
            .filter(|e| matches!(e.entry_type, SnapshotEntryType::Technology))
            .collect();
        
        let removed_tech: Vec<&SnapshotEntry> = self.removed_entries.iter()
            .filter(|e| matches!(e.entry_type, SnapshotEntryType::Technology))
            .collect();
        
        let modified_tech: Vec<&SnapshotEntry> = self.modified_entries.iter()
            .map(|(current, _)| current)
            .filter(|e| matches!(e.entry_type, SnapshotEntryType::Technology))
            .collect();
        
        (added_tech, removed_tech, modified_tech)
    }
}

impl Default for SnapshotStatistics {
    fn default() -> Self {
        Self {
            files_scanned: 0,
            technologies_detected: 0,
            file_types: HashMap::new(),
            technology_categories: HashMap::new(),
            lines_of_code: 0,
            scan_duration_ms: 0,
        }
    }
}

impl std::fmt::Display for SnapshotEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SnapshotEntryType::Technology => write!(f, "Technology"),
            SnapshotEntryType::Configuration => write!(f, "Configuration"),
            SnapshotEntryType::SourceFile => write!(f, "Source File"),
            SnapshotEntryType::Infrastructure => write!(f, "Infrastructure"),
            SnapshotEntryType::Documentation => write!(f, "Documentation"),
            SnapshotEntryType::Test => write!(f, "Test"),
            SnapshotEntryType::Build => write!(f, "Build"),
            SnapshotEntryType::Other => write!(f, "Other"),
        }
    }
}