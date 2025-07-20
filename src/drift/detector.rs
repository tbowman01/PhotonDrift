//! Core Drift Detection Logic
//! 
//! This module implements the main drift detection algorithms that compare
//! current codebase state against ADRs and baseline snapshots.

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

use crate::drift::{
    DriftResult,
    DriftReport,
    DriftItem,
    DriftSeverity,
    DriftCategory,
    DriftLocation,
    ScanStatistics,
    Snapshot,
};
use crate::config::DetectionPattern;
use crate::parser::AdrParser;

/// Core drift detection engine
pub struct DriftDetector {
    /// Threshold for determining technology significance
    #[allow(dead_code)] // Planned for advanced drift scoring
    significance_threshold: f64,
    
    /// Maximum number of drift items to report per category
    #[allow(dead_code)] // Planned for result limiting
    max_items_per_category: usize,
}

/// Parsed ADR decision information
#[derive(Debug, Clone)]
pub struct AdrDecision {
    /// File path of the ADR
    pub file_path: String,
    
    /// ADR title
    pub title: String,
    
    /// ADR status
    pub status: String,
    
    /// Technologies mentioned in the ADR
    pub mentioned_technologies: Vec<String>,
    
    /// Categories this ADR covers
    #[allow(dead_code)] // Planned for enhanced filtering and analysis
    pub categories: Vec<String>,
    
    /// Decision type (accepts, rejects, supersedes, etc.)
    pub decision_type: DecisionType,
}

/// Types of architectural decisions
#[derive(Debug, Clone, PartialEq)]
pub enum DecisionType {
    /// Accepts a specific technology or pattern
    Accepts,
    /// Rejects a specific technology or pattern
    Rejects,
    /// Supersedes a previous decision
    Supersedes,
    /// Proposes a new approach (not yet accepted)
    Proposes,
    /// Documents current state without decision
    Documents,
}

impl DriftDetector {
    /// Create a new drift detector with default settings
    pub fn new() -> Self {
        Self {
            significance_threshold: 0.7,
            max_items_per_category: 50,
        }
    }
    
    /// Configure the drift detector
    #[allow(dead_code)] // Planned for advanced configuration
    pub fn with_config(mut self, significance_threshold: f64, max_items_per_category: usize) -> Self {
        self.significance_threshold = significance_threshold;
        self.max_items_per_category = max_items_per_category;
        self
    }
    
    /// Parse ADR files to extract architectural decisions
    pub async fn parse_adr_decisions(&self, adr_dir: &Path) -> DriftResult<Vec<AdrDecision>> {
        let mut decisions = Vec::new();
        
        if !adr_dir.exists() {
            log::warn!("ADR directory does not exist: {}", adr_dir.display());
            return Ok(decisions);
        }
        
        for entry in walkdir::WalkDir::new(adr_dir)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(extension) = path.extension() {
                    if extension == "md" || extension == "markdown" {
                        match self.parse_single_adr(path).await {
                            Ok(decision) => decisions.push(decision),
                            Err(e) => {
                                log::warn!("Failed to parse ADR {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            }
        }
        
        log::info!("Parsed {} ADR decisions", decisions.len());
        Ok(decisions)
    }
    
    /// Parse a single ADR file
    async fn parse_single_adr(&self, file_path: &Path) -> DriftResult<AdrDecision> {
        let adr_doc = AdrParser::parse_file(file_path)?;
        
        let relative_path = file_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        // Extract technologies mentioned in the ADR content
        let mentioned_technologies = self.extract_technologies_from_content(&adr_doc.content);
        
        // Determine decision type based on status and content
        let decision_type = self.determine_decision_type(&adr_doc.metadata.status, &adr_doc.content);
        
        // Extract categories from tags or infer from content
        let categories = if !adr_doc.metadata.tags.is_empty() {
            adr_doc.metadata.tags
        } else {
            self.infer_categories_from_content(&adr_doc.content)
        };
        
        Ok(AdrDecision {
            file_path: relative_path,
            title: adr_doc.metadata.title,
            status: adr_doc.metadata.status,
            mentioned_technologies,
            categories,
            decision_type,
        })
    }
    
    /// Extract technology names from ADR content
    fn extract_technologies_from_content(&self, content: &str) -> Vec<String> {
        let mut technologies = HashSet::new();
        let content_lower = content.to_lowercase();
        
        // Common technology keywords to look for
        let tech_keywords = [
            "rust", "python", "javascript", "typescript", "java", "go", "kotlin",
            "react", "vue", "angular", "express", "django", "flask", "spring",
            "postgresql", "mysql", "sqlite", "mongodb", "redis",
            "docker", "kubernetes", "terraform", "aws", "azure", "gcp",
            "kafka", "rabbitmq", "elasticsearch", "prometheus", "grafana",
            "nginx", "apache", "microservices", "monolith", "api", "rest", "graphql",
        ];
        
        for keyword in &tech_keywords {
            if content_lower.contains(keyword) {
                technologies.insert(keyword.to_string());
            }
        }
        
        technologies.into_iter().collect()
    }
    
    /// Determine the type of decision from status and content
    fn determine_decision_type(&self, status: &str, content: &str) -> DecisionType {
        let status_lower = status.to_lowercase();
        let content_lower = content.to_lowercase();
        
        match status_lower.as_str() {
            "accepted" => {
                if content_lower.contains("reject") || content_lower.contains("deprecate") ||
                   content_lower.contains("we will not") || content_lower.contains("avoid") ||
                   content_lower.contains("not use") {
                    DecisionType::Rejects
                } else if content_lower.contains("we will use") || content_lower.contains("adopt") ||
                         content_lower.contains("use") {
                    DecisionType::Accepts
                } else {
                    DecisionType::Documents
                }
            }
            "rejected" => DecisionType::Rejects,
            "proposed" => DecisionType::Proposes,
            "superseded" => DecisionType::Supersedes,
            _ => {
                if content_lower.contains("we will use") || content_lower.contains("adopt") {
                    DecisionType::Accepts
                } else if content_lower.contains("we will not") || content_lower.contains("avoid") {
                    DecisionType::Rejects
                } else {
                    DecisionType::Documents
                }
            }
        }
    }
    
    /// Infer categories from ADR content
    fn infer_categories_from_content(&self, content: &str) -> Vec<String> {
        let mut categories = HashSet::new();
        let content_lower = content.to_lowercase();
        
        if content_lower.contains("database") || content_lower.contains("sql") || 
           content_lower.contains("storage") || content_lower.contains("persistence") {
            categories.insert("database".to_string());
        }
        
        if content_lower.contains("framework") || content_lower.contains("library") ||
           content_lower.contains("dependency") {
            categories.insert("framework".to_string());
        }
        
        if content_lower.contains("cloud") || content_lower.contains("aws") ||
           content_lower.contains("azure") || content_lower.contains("gcp") {
            categories.insert("cloud".to_string());
        }
        
        if content_lower.contains("security") || content_lower.contains("authentication") ||
           content_lower.contains("authorization") || content_lower.contains("crypto") {
            categories.insert("security".to_string());
        }
        
        if content_lower.contains("performance") || content_lower.contains("optimization") ||
           content_lower.contains("scaling") || content_lower.contains("latency") {
            categories.insert("performance".to_string());
        }
        
        if content_lower.contains("infrastructure") || content_lower.contains("deployment") ||
           content_lower.contains("container") || content_lower.contains("kubernetes") {
            categories.insert("infrastructure".to_string());
        }
        
        if categories.is_empty() {
            categories.insert("other".to_string());
        }
        
        categories.into_iter().collect()
    }
    
    /// Main drift detection method
    pub async fn detect_drift(
        &self,
        current_snapshot: &Snapshot,
        baseline_snapshot: Option<&Snapshot>,
        adr_decisions: &[AdrDecision],
        _detection_patterns: &[DetectionPattern],
    ) -> DriftResult<DriftReport> {
        let mut report = DriftReport::new(
            current_snapshot.root_directory.clone(),
            baseline_snapshot.map(|_s| PathBuf::from("baseline.json")),
        );
        
        // 1. Detect drift from baseline snapshot (if provided)
        if let Some(baseline) = baseline_snapshot {
            self.detect_snapshot_drift(current_snapshot, baseline, &mut report).await?;
        }
        
        // 2. Detect drift from ADR decisions
        self.detect_adr_drift(current_snapshot, adr_decisions, &mut report).await?;
        
        // 3. Detect new technologies not covered by ADRs
        self.detect_uncovered_technologies(current_snapshot, adr_decisions, &mut report).await?;
        
        // 4. Set final statistics
        report.scan_stats = ScanStatistics {
            files_scanned: current_snapshot.statistics.files_scanned,
            lines_analyzed: current_snapshot.statistics.lines_of_code,
            scan_duration_ms: current_snapshot.statistics.scan_duration_ms,
            patterns_matched: current_snapshot.statistics.technologies_detected,
            adrs_analyzed: adr_decisions.len(),
            file_types: current_snapshot.statistics.file_types.clone(),
        };
        
        Ok(report)
    }
    
    /// Detect drift between current and baseline snapshots
    async fn detect_snapshot_drift(
        &self,
        current: &Snapshot,
        baseline: &Snapshot,
        report: &mut DriftReport,
    ) -> DriftResult<()> {
        let comparison = current.compare_with(baseline);
        
        // Process added technologies
        for added_entry in &comparison.added_entries {
            if let Some(ref technology) = added_entry.technology {
                let drift_item = DriftItem::new(
                    format!("added_tech_{}", added_entry.id),
                    DriftSeverity::Medium,
                    DriftCategory::NewTechnology,
                    format!("New technology detected: {}", technology),
                    format!("Technology '{}' was introduced in {} since the baseline snapshot", 
                            technology, added_entry.file_path),
                    DriftLocation::new(PathBuf::from(&added_entry.file_path))
                        .with_line(added_entry.line_number.unwrap_or(1)),
                )
                .with_technology(technology.clone())
                .with_suggested_action(format!("Consider creating an ADR to document the decision to use {}", technology));
                
                report.add_item(drift_item);
            }
        }
        
        // Process removed technologies
        for removed_entry in &comparison.removed_entries {
            if let Some(ref technology) = removed_entry.technology {
                let drift_item = DriftItem::new(
                    format!("removed_tech_{}", removed_entry.id),
                    DriftSeverity::Low,
                    DriftCategory::DeprecatedTechnology,
                    format!("Technology removed: {}", technology),
                    format!("Technology '{}' was removed from {} since the baseline snapshot", 
                            technology, removed_entry.file_path),
                    DriftLocation::new(PathBuf::from(&removed_entry.file_path)),
                )
                .with_technology(technology.clone())
                .with_suggested_action("Consider updating related ADRs to reflect this change".to_string());
                
                report.add_item(drift_item);
            }
        }
        
        Ok(())
    }
    
    /// Detect drift from ADR decisions
    async fn detect_adr_drift(
        &self,
        current_snapshot: &Snapshot,
        adr_decisions: &[AdrDecision],
        report: &mut DriftReport,
    ) -> DriftResult<()> {
        // Group ADR decisions by technology
        let mut technology_decisions: HashMap<String, Vec<&AdrDecision>> = HashMap::new();
        
        for decision in adr_decisions {
            for technology in &decision.mentioned_technologies {
                technology_decisions.entry(technology.clone())
                    .or_insert_with(Vec::new)
                    .push(decision);
            }
        }
        
        // Check each technology in the current snapshot against ADRs
        for tech_entry in current_snapshot.technology_entries() {
            if let Some(ref detected_tech) = tech_entry.technology {
                let tech_lower = detected_tech.to_lowercase();
                
                // Find matching ADR decisions
                let matching_decisions: Vec<&AdrDecision> = technology_decisions
                    .iter()
                    .filter(|(adr_tech, _)| tech_lower.contains(&adr_tech.to_lowercase()))
                    .flat_map(|(_, decisions)| decisions.iter().copied())
                    .collect();
                
                if matching_decisions.is_empty() {
                    // No ADR covers this technology
                    let drift_item = DriftItem::new(
                        format!("uncovered_tech_{}", tech_entry.id),
                        DriftSeverity::High,
                        DriftCategory::NewTechnology,
                        format!("Uncovered technology: {}", detected_tech),
                        format!("Technology '{}' is in use but not covered by any ADR", detected_tech),
                        DriftLocation::new(PathBuf::from(&tech_entry.file_path))
                            .with_line(tech_entry.line_number.unwrap_or(1)),
                    )
                    .with_technology(detected_tech.clone())
                    .with_suggested_action(format!("Create an ADR to document the decision to use {}", detected_tech));
                    
                    report.add_item(drift_item);
                } else {
                    // Check for conflicts with ADR decisions
                    for decision in matching_decisions {
                        if decision.decision_type == DecisionType::Rejects && decision.status == "accepted" {
                            let drift_item = DriftItem::new(
                                format!("conflicting_tech_{}_{}", tech_entry.id, decision.file_path.replace('.', "_")),
                                DriftSeverity::Critical,
                                DriftCategory::ConflictingTechnology,
                                format!("Rejected technology in use: {}", detected_tech),
                                format!("Technology '{}' is being used but was rejected in ADR '{}'", 
                                        detected_tech, decision.title),
                                DriftLocation::new(PathBuf::from(&tech_entry.file_path))
                                    .with_line(tech_entry.line_number.unwrap_or(1)),
                            )
                            .with_technology(detected_tech.clone())
                            .with_related_adr(decision.title.clone())
                            .with_suggested_action(format!("Remove {} or update the ADR to accept its use", detected_tech));
                            
                            report.add_item(drift_item);
                        }
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Detect technologies that are not covered by any ADRs
    async fn detect_uncovered_technologies(
        &self,
        current_snapshot: &Snapshot,
        adr_decisions: &[AdrDecision],
        report: &mut DriftReport,
    ) -> DriftResult<()> {
        // Collect all technologies mentioned in ADRs
        let mut covered_technologies = HashSet::new();
        for decision in adr_decisions {
            for tech in &decision.mentioned_technologies {
                covered_technologies.insert(tech.to_lowercase());
            }
        }
        
        // Find technologies in snapshot that aren't covered
        let mut uncovered_count: HashMap<String, usize> = HashMap::new();
        
        for tech_entry in current_snapshot.technology_entries() {
            if let Some(ref technology) = tech_entry.technology {
                let tech_lower = technology.to_lowercase();
                
                // Check if this technology is covered by any ADR
                let is_covered = covered_technologies.iter()
                    .any(|covered_tech| tech_lower.contains(covered_tech) || covered_tech.contains(&tech_lower));
                
                if !is_covered {
                    *uncovered_count.entry(technology.clone()).or_insert(0) += 1;
                }
            }
        }
        
        // Report significant uncovered technologies
        for (technology, count) in uncovered_count {
            if count >= 3 { // Only report if technology appears multiple times
                let drift_item = DriftItem::new(
                    format!("uncovered_significant_{}", technology.replace(' ', "_")),
                    DriftSeverity::Medium,
                    DriftCategory::NewTechnology,
                    format!("Significant uncovered technology: {}", technology),
                    format!("Technology '{}' appears {} times but is not documented in any ADR", 
                            technology, count),
                    DriftLocation::new(PathBuf::from("multiple_files")),
                )
                .with_technology(technology.clone())
                .with_metadata("occurrence_count".to_string(), count.to_string())
                .with_suggested_action(format!("Consider creating an ADR to document the decision to use {}", technology));
                
                report.add_item(drift_item);
            }
        }
        
        Ok(())
    }
}

impl Default for DriftDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;
    use crate::drift::snapshot::{Snapshot, SnapshotEntry, SnapshotEntryType};

    fn create_test_adr(dir: &Path, filename: &str, content: &str) -> PathBuf {
        let file_path = dir.join(filename);
        fs::write(&file_path, content).unwrap();
        file_path
    }

    fn create_test_detector() -> DriftDetector {
        DriftDetector::new().with_config(0.5, 10)
    }

    #[tokio::test]
    async fn test_parse_adr_decisions() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path();
        
        // Create test ADR files
        create_test_adr(adr_dir, "0001-use-rust.md", r#"---
title: "Use Rust for Backend Development"
status: accepted
date: 2023-01-15
---

# Use Rust for Backend Development

We will use Rust for all backend services due to its performance and safety.
This decision covers rust, actix-web, and postgresql technologies.
"#);

        create_test_adr(adr_dir, "0002-reject-mongodb.md", r#"---
title: "Reject MongoDB for Primary Database"
status: accepted
date: 2023-02-01
---

# Reject MongoDB for Primary Database

We will not use MongoDB as our primary database.
Instead, we will use PostgreSQL for better consistency.
"#);

        let detector = create_test_detector();
        let decisions = detector.parse_adr_decisions(adr_dir).await.unwrap();
        
        assert_eq!(decisions.len(), 2);
        
        let rust_decision = decisions.iter().find(|d| d.title.contains("Rust")).unwrap();
        assert_eq!(rust_decision.decision_type, DecisionType::Accepts);
        assert!(rust_decision.mentioned_technologies.contains(&"rust".to_string()));
        assert!(rust_decision.mentioned_technologies.contains(&"postgresql".to_string()));
        
        let mongo_decision = decisions.iter().find(|d| d.title.contains("MongoDB")).unwrap();
        assert_eq!(mongo_decision.decision_type, DecisionType::Rejects);
        assert!(mongo_decision.mentioned_technologies.contains(&"mongodb".to_string()));
    }

    #[test]
    fn test_extract_technologies_from_content() {
        let detector = create_test_detector();
        
        let content = r#"
        We will use Rust for the backend with PostgreSQL database.
        The frontend will use React and TypeScript.
        We'll deploy on AWS using Docker containers.
        "#;
        
        let technologies = detector.extract_technologies_from_content(content);
        
        assert!(technologies.contains(&"rust".to_string()));
        assert!(technologies.contains(&"postgresql".to_string()));
        assert!(technologies.contains(&"react".to_string()));
        assert!(technologies.contains(&"typescript".to_string()));
        assert!(technologies.contains(&"aws".to_string()));
        assert!(technologies.contains(&"docker".to_string()));
    }

    #[test]
    fn test_determine_decision_type() {
        let detector = create_test_detector();
        
        // Test accepted decisions
        assert_eq!(
            detector.determine_decision_type("accepted", "We will use Rust"),
            DecisionType::Accepts
        );
        
        // Test rejected decisions
        assert_eq!(
            detector.determine_decision_type("accepted", "We will reject MongoDB"),
            DecisionType::Rejects
        );
        
        assert_eq!(
            detector.determine_decision_type("rejected", "MongoDB was considered"),
            DecisionType::Rejects
        );
        
        // Test proposed decisions
        assert_eq!(
            detector.determine_decision_type("proposed", "We should consider Rust"),
            DecisionType::Proposes
        );
        
        // Test superseded decisions
        assert_eq!(
            detector.determine_decision_type("superseded", "This replaces ADR-001"),
            DecisionType::Supersedes
        );
    }

    #[test]
    fn test_infer_categories_from_content() {
        let detector = create_test_detector();
        
        let database_content = "We will use PostgreSQL as our primary database for data persistence.";
        let categories = detector.infer_categories_from_content(database_content);
        assert!(categories.contains(&"database".to_string()));
        
        let framework_content = "React will be our frontend framework of choice.";
        let categories = detector.infer_categories_from_content(framework_content);
        assert!(categories.contains(&"framework".to_string()));
        
        let cloud_content = "Deploy the application on AWS using EC2 instances.";
        let categories = detector.infer_categories_from_content(cloud_content);
        assert!(categories.contains(&"cloud".to_string()));
        
        let security_content = "All API endpoints will require JWT authentication.";
        let categories = detector.infer_categories_from_content(security_content);
        assert!(categories.contains(&"security".to_string()));
    }

    #[tokio::test]
    async fn test_detect_adr_drift() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path();
        
        // Create ADR that rejects MongoDB
        create_test_adr(adr_dir, "reject-mongo.md", r#"---
title: "Reject MongoDB"
status: accepted
---

We will not use MongoDB. Use PostgreSQL instead.
"#);

        let detector = create_test_detector();
        let decisions = detector.parse_adr_decisions(adr_dir).await.unwrap();
        
        // Create snapshot with MongoDB usage
        let mut snapshot = Snapshot::new(temp_dir.path().to_path_buf());
        snapshot.add_file_entry(
            "src/database.rs",
            SnapshotEntryType::Technology,
            None,
            None,
            None,
        );
        
        // Add MongoDB technology entry to the snapshot
        let mongo_entry = SnapshotEntry {
            id: "tech_mongo_1".to_string(),
            entry_type: SnapshotEntryType::Technology,
            file_path: "src/database.rs".to_string(),
            technology: Some("mongodb".to_string()),  // Use lowercase to match extraction
            category: "database".to_string(),
            line_number: Some(10),
            matched_content: Some("use mongodb::Client;".to_string()),
            file_hash: None,
            file_size: None,
            modified_time: None,
            metadata: HashMap::new(),
        };
        snapshot.entries.push(mongo_entry);
        
        let mut report = DriftReport::new(temp_dir.path().to_path_buf(), None);
        detector.detect_adr_drift(&snapshot, &decisions, &mut report).await.unwrap();
        
        // Should detect conflict between MongoDB usage and ADR rejection
        assert!(report.total_items > 0);
        let critical_items = report.items_by_severity(&DriftSeverity::Critical);
        assert!(!critical_items.is_empty());
        
        let conflict_item = critical_items.iter()
            .find(|item| item.category == DriftCategory::ConflictingTechnology)
            .unwrap();
        assert!(conflict_item.title.contains("Rejected technology"));
        assert!(conflict_item.description.contains("MongoDB"));
    }

    #[tokio::test]
    async fn test_detect_uncovered_technologies() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path();
        
        // Create ADR that only mentions PostgreSQL
        create_test_adr(adr_dir, "use-postgres.md", r#"---
title: "Use PostgreSQL"
status: accepted
---

We will use PostgreSQL as our database.
"#);

        let detector = create_test_detector();
        let decisions = detector.parse_adr_decisions(adr_dir).await.unwrap();
        
        // Create snapshot with Redis usage (not covered by ADRs)
        let mut snapshot = Snapshot::new(temp_dir.path().to_path_buf());
        
        // Add multiple Redis entries to make it "significant"
        for i in 1..=5 {
            let redis_entry = SnapshotEntry {
                id: format!("tech_redis_{}", i),
                entry_type: SnapshotEntryType::Technology,
                file_path: format!("src/cache_{}.rs", i),
                technology: Some("Redis".to_string()),
                category: "database".to_string(),
                line_number: Some(i),
                matched_content: Some("use redis::Client;".to_string()),
                file_hash: None,
                file_size: None,
                modified_time: None,
                metadata: HashMap::new(),
            };
            snapshot.entries.push(redis_entry);
        }
        
        let mut report = DriftReport::new(temp_dir.path().to_path_buf(), None);
        detector.detect_uncovered_technologies(&snapshot, &decisions, &mut report).await.unwrap();
        
        // Should detect Redis as significant uncovered technology
        assert!(report.total_items > 0);
        let items = report.items_by_category(&DriftCategory::NewTechnology);
        assert!(!items.is_empty());
        
        let redis_item = items.iter()
            .find(|item| item.title.contains("Redis"))
            .unwrap();
        assert!(redis_item.description.contains("appears 5 times"));
    }

    #[tokio::test]
    async fn test_detect_snapshot_drift() {
        let temp_dir = TempDir::new().unwrap();
        
        // Create baseline snapshot
        let mut baseline = Snapshot::new(temp_dir.path().to_path_buf());
        let old_tech_entry = SnapshotEntry {
            id: "tech_old_1".to_string(),
            entry_type: SnapshotEntryType::Technology,
            file_path: "src/old.rs".to_string(),
            technology: Some("Old Library".to_string()),
            category: "framework".to_string(),
            line_number: Some(1),
            matched_content: Some("use old_lib;".to_string()),
            file_hash: Some("old_hash".to_string()),
            file_size: None,
            modified_time: None,
            metadata: HashMap::new(),
        };
        baseline.entries.push(old_tech_entry);
        
        // Create current snapshot with new technology
        let mut current = Snapshot::new(temp_dir.path().to_path_buf());
        let new_tech_entry = SnapshotEntry {
            id: "tech_new_1".to_string(),
            entry_type: SnapshotEntryType::Technology,
            file_path: "src/new.rs".to_string(),
            technology: Some("New Library".to_string()),
            category: "framework".to_string(),
            line_number: Some(1),
            matched_content: Some("use new_lib;".to_string()),
            file_hash: Some("new_hash".to_string()),
            file_size: None,
            modified_time: None,
            metadata: HashMap::new(),
        };
        current.entries.push(new_tech_entry);
        
        let detector = create_test_detector();
        let mut report = DriftReport::new(temp_dir.path().to_path_buf(), None);
        
        detector.detect_snapshot_drift(&current, &baseline, &mut report).await.unwrap();
        
        // Should detect both added and removed technologies
        assert!(report.total_items >= 2);
        
        let added_items = report.items_by_category(&DriftCategory::NewTechnology);
        assert!(!added_items.is_empty());
        assert!(added_items.iter().any(|item| item.title.contains("New Library")));
        
        let removed_items = report.items_by_category(&DriftCategory::DeprecatedTechnology);
        assert!(!removed_items.is_empty());
        assert!(removed_items.iter().any(|item| item.title.contains("Old Library")));
    }

    #[tokio::test]
    async fn test_full_drift_detection() {
        let temp_dir = TempDir::new().unwrap();
        let adr_dir = temp_dir.path().join("adr");
        fs::create_dir(&adr_dir).unwrap();
        
        // Create comprehensive ADR
        create_test_adr(&adr_dir, "tech-stack.md", r#"---
title: "Technology Stack Decisions"
status: accepted
date: 2023-01-01
---

# Technology Stack

We will use:
- Rust for backend
- PostgreSQL for database
- React for frontend

We will NOT use:
- MongoDB (consistency issues)
- Python (performance concerns)
"#);

        let detector = create_test_detector();
        let decisions = detector.parse_adr_decisions(&adr_dir).await.unwrap();
        
        // Create current snapshot with mixed compliance
        let mut current = Snapshot::new(temp_dir.path().to_path_buf());
        
        // Add compliant technology (Rust)
        let rust_entry = SnapshotEntry {
            id: "tech_rust_1".to_string(),
            entry_type: SnapshotEntryType::Technology,
            file_path: "src/main.rs".to_string(),
            technology: Some("Rust".to_string()),
            category: "framework".to_string(),
            line_number: Some(1),
            matched_content: Some("fn main()".to_string()),
            file_hash: None,
            file_size: None,
            modified_time: None,
            metadata: HashMap::new(),
        };
        current.entries.push(rust_entry);
        
        // Add rejected technology (Python)
        let python_entry = SnapshotEntry {
            id: "tech_python_1".to_string(),
            entry_type: SnapshotEntryType::Technology,
            file_path: "scripts/deploy.py".to_string(),
            technology: Some("Python".to_string()),
            category: "framework".to_string(),
            line_number: Some(1),
            matched_content: Some("#!/usr/bin/env python".to_string()),
            file_hash: None,
            file_size: None,
            modified_time: None,
            metadata: HashMap::new(),
        };
        current.entries.push(python_entry);
        
        // Add uncovered technology (Redis - appears multiple times)
        for i in 1..=4 {
            let redis_entry = SnapshotEntry {
                id: format!("tech_redis_{}", i),
                entry_type: SnapshotEntryType::Technology,
                file_path: format!("src/cache_{}.rs", i),
                technology: Some("Redis".to_string()),
                category: "database".to_string(),
                line_number: Some(i),
                matched_content: Some("use redis::Client;".to_string()),
                file_hash: None,
                file_size: None,
                modified_time: None,
                metadata: HashMap::new(),
            };
            current.entries.push(redis_entry);
        }
        
        // Perform full drift detection
        let patterns = vec![]; // Empty patterns for this test
        let report = detector.detect_drift(&current, None, &decisions, &patterns).await.unwrap();
        
        // Verify results
        assert!(report.total_items >= 2); // At least Python conflict + Redis uncovered
        
        // Check for critical Python conflict
        let critical_items = report.items_by_severity(&DriftSeverity::Critical);
        assert!(critical_items.iter().any(|item| 
            item.category == DriftCategory::ConflictingTechnology && 
            item.description.contains("Python")
        ));
        
        // Check for uncovered Redis
        let new_tech_items = report.items_by_category(&DriftCategory::NewTechnology);
        assert!(new_tech_items.iter().any(|item| 
            item.title.contains("Redis")
        ));
        
        // Verify summary
        assert!(report.severity_summary.contains_key(&DriftSeverity::Critical));
        assert!(report.category_summary.contains_key(&DriftCategory::ConflictingTechnology));
    }
}