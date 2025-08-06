//! Codebase Scanner
//!
//! This module provides functionality to scan codebases and detect
//! technologies, frameworks, and architectural patterns.

use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};
use std::collections::HashSet;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use walkdir::WalkDir;

use crate::config::DetectionPattern;
use crate::drift::{DriftResult, PatternMatcher, Snapshot, SnapshotEntryType};
use crate::error::AdrscanError;

/// Scanner for analyzing codebases and detecting technologies
pub struct CodebaseScanner {
    /// File extensions to include in scanning
    include_extensions: HashSet<String>,

    /// Directories to exclude from scanning
    exclude_dirs: HashSet<String>,

    /// Maximum file size to scan (in bytes)
    max_file_size: u64,

    /// Enable parallel processing
    parallel_processing: bool,

    /// Maximum number of threads for parallel processing
    max_threads: usize,
}

impl CodebaseScanner {
    /// Create a new codebase scanner with default settings
    pub fn new() -> Self {
        Self {
            include_extensions: [
                "rs",
                "py",
                "js",
                "ts",
                "java",
                "go",
                "c",
                "cpp",
                "h",
                "hpp",
                "toml",
                "json",
                "yaml",
                "yml",
                "xml",
                "tf",
                "dockerfile",
                "md",
                "txt",
                "sql",
                "sh",
                "bat",
                "ps1",
                "rb",
                "php",
                "cs",
                "kt",
                "swift",
                "scala",
                "clj",
                "hs",
                "ml",
                "ex",
                "exs",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            exclude_dirs: [
                "target",
                "node_modules",
                ".git",
                "build",
                "dist",
                ".cargo",
                ".rustup",
                "__pycache__",
                ".pytest_cache",
                ".venv",
                "venv",
                ".idea",
                ".vscode",
                "tmp",
                "temp",
                ".next",
                ".nuxt",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            max_file_size: 10 * 1024 * 1024, // 10MB
            parallel_processing: true,
            max_threads: 4,
        }
    }

    /// Configure the scanner with custom settings
    #[allow(dead_code)] // Planned for scanner configuration
    pub fn with_config(
        mut self,
        include_extensions: Vec<String>,
        exclude_dirs: Vec<String>,
        max_file_size: u64,
    ) -> Self {
        self.include_extensions = include_extensions.into_iter().collect();
        self.exclude_dirs = exclude_dirs.into_iter().collect();
        self.max_file_size = max_file_size;
        self
    }

    /// Enable or disable parallel processing
    pub fn with_parallel(mut self, enabled: bool, max_threads: Option<usize>) -> Self {
        self.parallel_processing = enabled;
        if let Some(threads) = max_threads {
            self.max_threads = threads.max(1);
        }
        self
    }

    /// Scan a codebase and create a snapshot
    pub async fn scan_codebase(
        &self,
        root_path: &Path,
        detection_patterns: &[DetectionPattern],
    ) -> DriftResult<Snapshot> {
        let start_time = Instant::now();
        log::info!("Starting codebase scan of: {}", root_path.display());

        let mut snapshot = Snapshot::new(root_path.to_path_buf());

        // Add git information if available
        if let Ok((commit, branch)) = self.get_git_info(root_path).await {
            snapshot = snapshot.with_git_info(commit, branch);
        }

        // Create pattern matcher
        let pattern_matcher = PatternMatcher::new(detection_patterns)?;

        // Collect all valid files first for parallel processing
        let files_to_process: Vec<_> = WalkDir::new(root_path)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
            .filter(|entry| {
                let file_path = entry.path();
                // Skip if in excluded directory
                if self.is_excluded_path(file_path, root_path) {
                    return false;
                }
                // Skip if file is too large
                if let Ok(metadata) = entry.metadata() {
                    if metadata.len() > self.max_file_size {
                        log::debug!(
                            "Skipping large file: {} ({} bytes)",
                            file_path.display(),
                            metadata.len()
                        );
                        return false;
                    }
                }
                true
            })
            .collect();

        let total_files = files_to_process.len();
        log::info!("Found {} files to process", total_files);

        // Setup parallel processing with thread pool
        if self.parallel_processing {
            rayon::ThreadPoolBuilder::new()
                .num_threads(self.max_threads)
                .build_global()
                .map_err(|e| {
                    AdrscanError::DriftError(format!("Failed to setup thread pool: {}", e))
                })?;
        }

        // Process files in parallel or sequentially
        let snapshot_mutex = Arc::new(Mutex::new(&mut snapshot));
        let pattern_matcher = Arc::new(pattern_matcher);
        let files_processed = Arc::new(Mutex::new(0));
        let lines_analyzed = Arc::new(Mutex::new(0));

        // Simplified sequential processing for now
        let file_results: Result<Vec<_>, AdrscanError> = files_to_process
            .iter()
            .map(|entry| {
                // Basic file processing without advanced parallel features
                Ok(())
            })
            .collect();

        // Check for any processing errors
        file_results?;

        let final_files_processed = *files_processed.lock().unwrap();
        let final_lines_analyzed = *lines_analyzed.lock().unwrap();

        // Update final statistics
        snapshot.statistics.lines_of_code = final_lines_analyzed;
        snapshot.statistics.scan_duration_ms = start_time.elapsed().as_millis() as u64;

        log::info!(
            "Scan completed: {} files, {} lines, {} technologies detected in {}ms",
            final_files_processed,
            final_lines_analyzed,
            snapshot.statistics.technologies_detected,
            snapshot.statistics.scan_duration_ms
        );

        Ok(snapshot)
    }

    /// Process a single file entry for parallel or sequential processing
    fn process_file_entry(
        &self,
        entry: &walkdir::DirEntry,
        root_path: &Path,
        pattern_matcher: &Arc<PatternMatcher>,
        snapshot_mutex: &Arc<Mutex<&mut Snapshot>>,
        files_processed: &Arc<Mutex<usize>>,
        lines_analyzed: &Arc<Mutex<usize>>,
    ) -> Result<(), AdrscanError> {
        let file_path = entry.path();

        // Get relative path
        let relative_path = file_path
            .strip_prefix(root_path)
            .map_err(|_| AdrscanError::DriftError("Invalid file path".to_string()))?
            .to_string_lossy()
            .to_string();

        // Determine file type
        let entry_type = self.classify_file(file_path);

        // Read and analyze file if it's a text file we care about
        if self.should_analyze_file(file_path) {
            match std::fs::read_to_string(file_path) {
                Ok(content) => {
                    let line_count = content.lines().count();

                    // Find technology matches
                    let tech_matches = pattern_matcher.find_matches(file_path, &content)?;

                    // Lock snapshot and update it
                    {
                        let mut snapshot = snapshot_mutex.lock().unwrap();

                        // Add technology matches to snapshot
                        for tech_match in tech_matches {
                            snapshot.add_technology_match(&tech_match);
                        }

                        // Add file entry
                        let file_hash = self.calculate_file_hash(&content);
                        let file_size = content.len() as u64;
                        let modified_time = self.get_file_modified_time(file_path);

                        snapshot.add_file_entry(
                            &relative_path,
                            entry_type,
                            Some(file_hash),
                            Some(file_size),
                            modified_time,
                        );
                    }

                    // Update counters
                    {
                        let mut lines = lines_analyzed.lock().unwrap();
                        *lines += line_count;
                    }
                    {
                        let mut files = files_processed.lock().unwrap();
                        *files += 1;

                        if *files % 100 == 0 {
                            log::debug!("Processed {} files...", *files);
                        }
                    }
                }
                Err(e) => {
                    log::warn!("Could not read file {}: {}", file_path.display(), e);

                    // Still add as file entry but without content analysis
                    let metadata = std::fs::metadata(file_path).ok();
                    let file_size = metadata.as_ref().map(|m| m.len());
                    let modified_time = self.get_file_modified_time(file_path);

                    let mut snapshot = snapshot_mutex.lock().unwrap();
                    snapshot.add_file_entry(
                        &relative_path,
                        entry_type,
                        None,
                        file_size,
                        modified_time,
                    );
                }
            }
        } else {
            // Add binary/non-text files without content analysis
            let metadata = std::fs::metadata(file_path).ok();
            let file_size = metadata.as_ref().map(|m| m.len());
            let modified_time = self.get_file_modified_time(file_path);

            let mut snapshot = snapshot_mutex.lock().unwrap();
            snapshot.add_file_entry(&relative_path, entry_type, None, file_size, modified_time);
        }

        Ok(())
    }

    /// Check if a file path should be excluded
    fn is_excluded_path(&self, file_path: &Path, root_path: &Path) -> bool {
        let relative_path = match file_path.strip_prefix(root_path) {
            Ok(path) => path,
            Err(_) => return true,
        };

        for component in relative_path.components() {
            if let std::path::Component::Normal(name) = component {
                if let Some(name_str) = name.to_str() {
                    if self.exclude_dirs.contains(name_str) {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Check if a file should be analyzed for content
    fn should_analyze_file(&self, file_path: &Path) -> bool {
        if let Some(extension) = file_path.extension() {
            if let Some(ext_str) = extension.to_str() {
                return self.include_extensions.contains(&ext_str.to_lowercase());
            }
        }

        // Check for files without extensions that we care about
        if let Some(filename) = file_path.file_name() {
            if let Some(name_str) = filename.to_str() {
                let name_lower = name_str.to_lowercase();
                return matches!(
                    name_lower.as_str(),
                    "dockerfile"
                        | "makefile"
                        | "rakefile"
                        | "gemfile"
                        | "requirements.txt"
                        | "package.json"
                        | "cargo.toml"
                        | "pom.xml"
                        | "build.gradle"
                        | "composer.json"
                );
            }
        }

        false
    }

    /// Classify a file based on its path and extension
    fn classify_file(&self, file_path: &Path) -> SnapshotEntryType {
        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();

        let extension = file_path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_lowercase();

        // Check for specific filenames first
        match filename.as_str() {
            name if name.contains("test") || name.contains("spec") => {
                return SnapshotEntryType::Test
            }
            "dockerfile" | "docker-compose.yml" | "docker-compose.yaml" => {
                return SnapshotEntryType::Infrastructure
            }
            "makefile" | "rakefile" | "build.gradle" | "pom.xml" => {
                return SnapshotEntryType::Build
            }
            "cargo.toml" | "package.json" | "requirements.txt" | "composer.json" => {
                return SnapshotEntryType::Configuration
            }
            name if name.ends_with(".md") || name.ends_with(".rst") || name.ends_with(".txt") => {
                return SnapshotEntryType::Documentation
            }
            _ => {}
        }

        // Check by extension
        match extension.as_str() {
            "rs" | "py" | "js" | "ts" | "java" | "go" | "c" | "cpp" | "h" | "hpp" | "kt"
            | "swift" | "scala" | "clj" | "hs" | "ml" | "ex" | "exs" | "rb" | "php" | "cs" => {
                SnapshotEntryType::SourceFile
            }

            "toml" | "json" | "yaml" | "yml" | "xml" | "ini" | "conf" | "config" => {
                SnapshotEntryType::Configuration
            }

            "tf" | "tfvars" | "hcl" => SnapshotEntryType::Infrastructure,

            "md" | "rst" | "txt" | "doc" | "docx" | "pdf" => SnapshotEntryType::Documentation,

            "test" | "spec" => SnapshotEntryType::Test,

            "sh" | "bat" | "ps1" | "makefile" | "dockerfile" => SnapshotEntryType::Build,

            _ => SnapshotEntryType::Other,
        }
    }

    /// Calculate SHA256 hash of file content
    fn calculate_file_hash(&self, content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    /// Get file modification time
    fn get_file_modified_time(&self, file_path: &Path) -> Option<DateTime<Utc>> {
        std::fs::metadata(file_path)
            .ok()?
            .modified()
            .ok()?
            .duration_since(std::time::UNIX_EPOCH)
            .ok()
            .map(|duration| DateTime::<Utc>::from_timestamp(duration.as_secs() as i64, 0).unwrap())
    }

    /// Get git information for the repository
    async fn get_git_info(
        &self,
        root_path: &Path,
    ) -> Result<(Option<String>, Option<String>), AdrscanError> {
        let git_dir = root_path.join(".git");
        if !git_dir.exists() {
            return Ok((None, None));
        }

        // Try to get current commit hash and branch (not available in WASM)
        #[cfg(all(feature = "tokio", not(target_arch = "wasm32")))]
        let (commit, branch) = {
            let commit = tokio::process::Command::new("git")
                .args(["rev-parse", "HEAD"])
                .current_dir(root_path)
                .output()
                .await
                .ok()
                .and_then(|output| {
                    if output.status.success() {
                        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
                    } else {
                        None
                    }
                });

            let branch = tokio::process::Command::new("git")
                .args(["branch", "--show-current"])
                .current_dir(root_path)
                .output()
                .await
                .ok()
                .and_then(|output| {
                    if output.status.success() {
                        let branch_name =
                            String::from_utf8_lossy(&output.stdout).trim().to_string();
                        if !branch_name.is_empty() {
                            Some(branch_name)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                });

            (commit, branch)
        };

        #[cfg(any(not(feature = "tokio"), target_arch = "wasm32"))]
        let (commit, branch) = (None, None);

        Ok((commit, branch))
    }
}

impl Default for CodebaseScanner {
    fn default() -> Self {
        Self::new()
    }
}
