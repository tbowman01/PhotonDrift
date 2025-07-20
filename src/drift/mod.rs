//! Drift Detection Engine
//! 
//! This module provides the core functionality for detecting architectural drift
//! between Architecture Decision Records (ADRs) and the actual codebase state.

pub mod detector;
pub mod patterns;
pub mod report;
pub mod scanner;
pub mod snapshot;

pub use detector::DriftDetector;
pub use patterns::{TechnologyMatch, PatternMatcher};
pub use report::{DriftReport, DriftItem, DriftSeverity, DriftCategory, DriftLocation, ScanStatistics};
pub use scanner::CodebaseScanner;
pub use snapshot::{Snapshot, SnapshotEntry, SnapshotEntryType, SnapshotComparison};

use crate::error::AdrscanError;
use crate::config::DetectionPattern;
use std::path::Path;

/// Result type for drift detection operations
pub type DriftResult<T> = std::result::Result<T, AdrscanError>;

/// Main entry point for drift detection functionality
pub struct DriftEngine {
    detector: DriftDetector,
    scanner: CodebaseScanner,
}

impl DriftEngine {
    /// Create a new drift detection engine
    pub fn new() -> Self {
        Self {
            detector: DriftDetector::new(),
            scanner: CodebaseScanner::new(),
        }
    }

    /// Perform comprehensive drift detection
    pub async fn detect_drift(
        &self,
        adr_dir: &Path,
        codebase_dir: &Path,
        baseline_snapshot: Option<&Path>,
        detection_patterns: &[DetectionPattern],
    ) -> DriftResult<DriftReport> {
        log::info!("Starting drift detection...");
        
        // 1. Scan current codebase state
        let current_snapshot = self.scanner.scan_codebase(codebase_dir, detection_patterns).await?;
        
        // 2. Load baseline snapshot if provided
        let baseline = if let Some(baseline_path) = baseline_snapshot {
            Some(Snapshot::load(baseline_path)?)
        } else {
            None
        };
        
        // 3. Parse ADRs for architectural decisions
        let adr_decisions = self.detector.parse_adr_decisions(adr_dir).await?;
        
        // 4. Detect drift between current state, baseline, and ADRs
        let drift_report = self.detector.detect_drift(
            &current_snapshot,
            baseline.as_ref(),
            &adr_decisions,
            detection_patterns,
        ).await?;
        
        log::info!("Drift detection completed. Found {} drift items", drift_report.items.len());
        Ok(drift_report)
    }
}

impl Default for DriftEngine {
    fn default() -> Self {
        Self::new()
    }
}