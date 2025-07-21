//! Drift Report Generation and Data Structures
//! 
//! This module defines the structures for representing and generating
//! drift detection reports in various formats.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Comprehensive drift detection report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftReport {
    /// Timestamp when the report was generated
    pub timestamp: DateTime<Utc>,
    
    /// Directory that was scanned
    pub scanned_directory: PathBuf,
    
    /// Baseline snapshot file used (if any)
    pub baseline_snapshot: Option<PathBuf>,
    
    /// Total number of drift items found
    pub total_items: usize,
    
    /// Summary by drift category
    pub category_summary: HashMap<DriftCategory, usize>,
    
    /// Summary by severity level
    pub severity_summary: HashMap<DriftSeverity, usize>,
    
    /// Individual drift items
    pub items: Vec<DriftItem>,
    
    /// Statistics about the scan
    pub scan_stats: ScanStatistics,
}

/// Individual drift detection item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftItem {
    /// Unique identifier for this drift item
    pub id: String,
    
    /// Severity level of this drift
    pub severity: DriftSeverity,
    
    /// Category of drift
    pub category: DriftCategory,
    
    /// Human-readable title/summary
    pub title: String,
    
    /// Detailed description of the drift
    pub description: String,
    
    /// File or location where drift was detected
    pub location: DriftLocation,
    
    /// Technology/pattern that was detected
    pub detected_technology: Option<String>,
    
    /// Related ADR (if any)
    pub related_adr: Option<String>,
    
    /// Suggested action to resolve the drift
    pub suggested_action: Option<String>,
    
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

/// Severity levels for drift items
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DriftSeverity {
    /// Critical drift that likely violates architectural decisions
    Critical,
    /// High-priority drift that should be addressed soon
    High,
    /// Medium-priority drift that should be reviewed
    Medium,
    /// Low-priority drift or informational
    Low,
    /// Informational only, no action required
    Info,
}

/// Categories of architectural drift
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DriftCategory {
    /// New technology introduced without ADR
    NewTechnology,
    /// Technology usage conflicts with existing ADR
    ConflictingTechnology,
    /// Deprecated technology still in use
    DeprecatedTechnology,
    /// Architecture pattern violation
    PatternViolation,
    /// Missing required architecture component
    MissingComponent,
    /// Security-related architectural drift
    Security,
    /// Performance-related architectural drift
    Performance,
    /// Database/persistence changes
    Database,
    /// Infrastructure changes
    Infrastructure,
    /// Framework or library changes
    Framework,
    /// Configuration drift
    Configuration,
    /// Other/uncategorized drift
    Other,
}

/// Location information for drift items
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriftLocation {
    /// File path where drift was detected
    pub file_path: PathBuf,
    
    /// Line number (if applicable)
    pub line_number: Option<usize>,
    
    /// Column number (if applicable)
    pub column_number: Option<usize>,
    
    /// Code snippet or context
    pub snippet: Option<String>,
}

/// Statistics about the drift detection scan
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanStatistics {
    /// Total number of files scanned
    pub files_scanned: usize,
    
    /// Total number of lines of code analyzed
    pub lines_analyzed: usize,
    
    /// Time taken for the scan in milliseconds
    pub scan_duration_ms: u64,
    
    /// Number of patterns matched
    pub patterns_matched: usize,
    
    /// Number of ADRs analyzed
    pub adrs_analyzed: usize,
    
    /// File type breakdown
    pub file_types: HashMap<String, usize>,
}

impl DriftReport {
    /// Create a new empty drift report
    pub fn new(scanned_directory: PathBuf, baseline_snapshot: Option<PathBuf>) -> Self {
        Self {
            timestamp: Utc::now(),
            scanned_directory,
            baseline_snapshot,
            total_items: 0,
            category_summary: HashMap::new(),
            severity_summary: HashMap::new(),
            items: Vec::new(),
            scan_stats: ScanStatistics::default(),
        }
    }
    
    /// Add a drift item to the report
    pub fn add_item(&mut self, item: DriftItem) {
        *self.category_summary.entry(item.category.clone()).or_insert(0) += 1;
        *self.severity_summary.entry(item.severity.clone()).or_insert(0) += 1;
        self.items.push(item);
        self.total_items = self.items.len();
    }
    
    /// Get items by severity level
    pub fn items_by_severity(&self, severity: &DriftSeverity) -> Vec<&DriftItem> {
        self.items.iter().filter(|item| &item.severity == severity).collect()
    }
    
    /// Get items by category
    #[allow(dead_code)] // Planned for enhanced filtering UI
    pub fn items_by_category(&self, category: &DriftCategory) -> Vec<&DriftItem> {
        self.items.iter().filter(|item| &item.category == category).collect()
    }
    
    /// Generate a summary string
    #[allow(dead_code)] // Planned for enhanced reporting
    pub fn summary(&self) -> String {
        format!(
            "Drift Report Summary:\n\
             - Total Items: {}\n\
             - Critical: {}\n\
             - High: {}\n\
             - Medium: {}\n\
             - Low: {}\n\
             - Files Scanned: {}\n\
             - Scan Duration: {}ms",
            self.total_items,
            self.severity_summary.get(&DriftSeverity::Critical).unwrap_or(&0),
            self.severity_summary.get(&DriftSeverity::High).unwrap_or(&0),
            self.severity_summary.get(&DriftSeverity::Medium).unwrap_or(&0),
            self.severity_summary.get(&DriftSeverity::Low).unwrap_or(&0),
            self.scan_stats.files_scanned,
            self.scan_stats.scan_duration_ms
        )
    }
    
    /// Export report to JSON
    pub fn to_json(&self) -> serde_json::Result<String> {
        serde_json::to_string_pretty(self)
    }
    
    /// Export report to YAML
    pub fn to_yaml(&self) -> serde_yaml::Result<String> {
        serde_yaml::to_string(self)
    }
}

impl DriftItem {
    /// Create a new drift item
    pub fn new(
        id: String,
        severity: DriftSeverity,
        category: DriftCategory,
        title: String,
        description: String,
        location: DriftLocation,
    ) -> Self {
        Self {
            id,
            severity,
            category,
            title,
            description,
            location,
            detected_technology: None,
            related_adr: None,
            suggested_action: None,
            metadata: HashMap::new(),
        }
    }
    
    /// Set detected technology
    pub fn with_technology(mut self, technology: String) -> Self {
        self.detected_technology = Some(technology);
        self
    }
    
    /// Set related ADR
    pub fn with_related_adr(mut self, adr: String) -> Self {
        self.related_adr = Some(adr);
        self
    }
    
    /// Set suggested action
    pub fn with_suggested_action(mut self, action: String) -> Self {
        self.suggested_action = Some(action);
        self
    }
    
    /// Add metadata
    pub fn with_metadata(mut self, key: String, value: String) -> Self {
        self.metadata.insert(key, value);
        self
    }
}

impl DriftLocation {
    /// Create a new drift location
    pub fn new(file_path: PathBuf) -> Self {
        Self {
            file_path,
            line_number: None,
            column_number: None,
            snippet: None,
        }
    }
    
    /// Set line number
    pub fn with_line(mut self, line: usize) -> Self {
        self.line_number = Some(line);
        self
    }
    
    /// Set column number
    #[allow(dead_code)] // Planned for enhanced location tracking
    pub fn with_column(mut self, column: usize) -> Self {
        self.column_number = Some(column);
        self
    }
    
    /// Set code snippet
    #[allow(dead_code)] // Planned for enhanced location tracking
    pub fn with_snippet(mut self, snippet: String) -> Self {
        self.snippet = Some(snippet);
        self
    }
}

impl Default for ScanStatistics {
    fn default() -> Self {
        Self {
            files_scanned: 0,
            lines_analyzed: 0,
            scan_duration_ms: 0,
            patterns_matched: 0,
            adrs_analyzed: 0,
            file_types: HashMap::new(),
        }
    }
}

impl std::fmt::Display for DriftSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriftSeverity::Critical => write!(f, "CRITICAL"),
            DriftSeverity::High => write!(f, "HIGH"),
            DriftSeverity::Medium => write!(f, "MEDIUM"),
            DriftSeverity::Low => write!(f, "LOW"),
            DriftSeverity::Info => write!(f, "INFO"),
        }
    }
}

impl std::fmt::Display for DriftCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DriftCategory::NewTechnology => write!(f, "New Technology"),
            DriftCategory::ConflictingTechnology => write!(f, "Conflicting Technology"),
            DriftCategory::DeprecatedTechnology => write!(f, "Deprecated Technology"),
            DriftCategory::PatternViolation => write!(f, "Pattern Violation"),
            DriftCategory::MissingComponent => write!(f, "Missing Component"),
            DriftCategory::Security => write!(f, "Security"),
            DriftCategory::Performance => write!(f, "Performance"),
            DriftCategory::Database => write!(f, "Database"),
            DriftCategory::Infrastructure => write!(f, "Infrastructure"),
            DriftCategory::Framework => write!(f, "Framework"),
            DriftCategory::Configuration => write!(f, "Configuration"),
            DriftCategory::Other => write!(f, "Other"),
        }
    }
}