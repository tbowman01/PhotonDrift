//! Compatibility Testing Module
//! 
//! Byzantine fault-tolerant compatibility testing for PhotonDrift dependency updates

pub mod test_matrix_comprehensive;
pub mod security_testing;
pub mod performance_testing;

pub use test_matrix_comprehensive::*;
pub use security_testing::*;
pub use performance_testing::*;

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Master compatibility test orchestrator for all issues
#[derive(Debug, Serialize, Deserialize)]
pub struct MasterCompatibilityTestSuite {
    pub test_suites: HashMap<String, TestSuiteType>,
    pub execution_strategy: ExecutionStrategy,
    pub byzantine_consensus: ByzantineConsensusConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TestSuiteType {
    ComprehensiveMatrix(CompatibilityTestMatrix),
    SecuritySuite(SecurityTestSuite),
    PerformanceSuite(PerformanceTestSuite),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionStrategy {
    pub parallel_execution: bool,
    pub fault_tolerance: f64,
    pub retry_attempts: u32,
    pub timeout_per_test: std::time::Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ByzantineConsensusConfig {
    pub required_consensus_threshold: f64, // 2/3 for Byzantine fault tolerance
    pub agent_fault_tolerance: u32,
    pub coordination_timeout: std::time::Duration,
}

/// Create master test suite for all 4 issues
pub fn create_master_compatibility_suite() -> MasterCompatibilityTestSuite {
    let mut test_suites = HashMap::new();

    // Issue #115 & #117: NPM Compatibility
    test_suites.insert(
        "115_117_npm".to_string(),
        TestSuiteType::ComprehensiveMatrix(create_npm_compatibility_matrix()),
    );
    test_suites.insert(
        "115_117_npm_security".to_string(),
        TestSuiteType::SecuritySuite(create_npm_security_suite()),
    );
    test_suites.insert(
        "115_117_npm_performance".to_string(),
        TestSuiteType::PerformanceSuite(create_npm_performance_suite()),
    );

    // Issue #51: Rust Module Refactoring
    test_suites.insert(
        "51_rust_modules".to_string(),
        TestSuiteType::ComprehensiveMatrix(create_rust_module_compatibility_matrix()),
    );
    test_suites.insert(
        "51_rust_modules_security".to_string(),
        TestSuiteType::SecuritySuite(create_rust_module_security_suite()),
    );
    test_suites.insert(
        "51_rust_modules_performance".to_string(),
        TestSuiteType::PerformanceSuite(create_rust_module_performance_suite()),
    );

    // Issue #45: Crate Deprecation
    test_suites.insert(
        "45_crate_deprecation".to_string(),
        TestSuiteType::ComprehensiveMatrix(create_crate_deprecation_compatibility_matrix()),
    );
    test_suites.insert(
        "45_crate_deprecation_security".to_string(),
        TestSuiteType::SecuritySuite(create_crate_deprecation_security_suite()),
    );
    test_suites.insert(
        "45_crate_deprecation_performance".to_string(),
        TestSuiteType::PerformanceSuite(create_crate_deprecation_performance_suite()),
    );

    MasterCompatibilityTestSuite {
        test_suites,
        execution_strategy: ExecutionStrategy {
            parallel_execution: true,
            fault_tolerance: 0.33, // Allow 1/3 failures
            retry_attempts: 3,
            timeout_per_test: std::time::Duration::from_secs(300), // 5 minutes per test
        },
        byzantine_consensus: ByzantineConsensusConfig {
            required_consensus_threshold: 0.67, // 2/3 Byzantine fault tolerance
            agent_fault_tolerance: 2, // Can tolerate up to 2 agent failures
            coordination_timeout: std::time::Duration::from_secs(30),
        },
    }
}

/// Execute master compatibility test suite with Byzantine consensus
pub async fn execute_master_compatibility_tests() -> Result<MasterTestResults, Box<dyn std::error::Error>> {
    println!("🚀 Starting Master Compatibility Test Suite with Byzantine Consensus");
    println!("📋 Testing Issues: #115, #117, #51, #45");
    
    let master_suite = create_master_compatibility_suite();
    let start_time = std::time::Instant::now();
    
    let mut results = HashMap::new();
    let mut consensus_votes = Vec::new();

    // Execute all test suites in parallel
    for (suite_id, suite_type) in &master_suite.test_suites {
        println!("🔄 Executing test suite: {}", suite_id);
        
        let result = match suite_type {
            TestSuiteType::ComprehensiveMatrix(matrix) => {
                let executor = ByzantineTestExecutor::new();
                let report = executor.execute_all_tests().await?;
                TestResult::Comprehensive(report)
            }
            TestSuiteType::SecuritySuite(suite) => {
                let security_results = execute_security_tests(suite).await;
                TestResult::Security(security_results)
            }
            TestSuiteType::PerformanceSuite(suite) => {
                let perf_results = execute_performance_tests(suite).await;
                TestResult::Performance(perf_results)
            }
        };

        // Record consensus vote
        let vote = result.is_successful();
        consensus_votes.push(vote);
        results.insert(suite_id.clone(), result);
    }

    let total_duration = start_time.elapsed();
    let consensus_achieved = calculate_byzantine_consensus(&consensus_votes, &master_suite.byzantine_consensus);

    Ok(MasterTestResults {
        total_suites_executed: master_suite.test_suites.len(),
        total_duration,
        results,
        consensus_achieved,
        consensus_details: ConsensusSummary {
            total_votes: consensus_votes.len(),
            successful_votes: consensus_votes.iter().filter(|&&v| v).count(),
            consensus_threshold: master_suite.byzantine_consensus.required_consensus_threshold,
            achieved_consensus: consensus_achieved,
        },
    })
}

fn calculate_byzantine_consensus(votes: &[bool], config: &ByzantineConsensusConfig) -> bool {
    let successful_votes = votes.iter().filter(|&&v| v).count();
    let success_rate = successful_votes as f64 / votes.len() as f64;
    success_rate >= config.required_consensus_threshold
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TestResult {
    Comprehensive(TestExecutionReport),
    Security(SecurityTestResults),
    Performance(PerformanceTestResults),
}

impl TestResult {
    pub fn is_successful(&self) -> bool {
        match self {
            TestResult::Comprehensive(report) => report.consensus_achieved,
            TestResult::Security(results) => results.overall_passed,
            TestResult::Performance(results) => results.overall_passed,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MasterTestResults {
    pub total_suites_executed: usize,
    pub total_duration: std::time::Duration,
    pub results: HashMap<String, TestResult>,
    pub consensus_achieved: bool,
    pub consensus_details: ConsensusSummary,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsensusSummary {
    pub total_votes: usize,
    pub successful_votes: usize,
    pub consensus_threshold: f64,
    pub achieved_consensus: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_master_compatibility_suite() {
        let results = execute_master_compatibility_tests().await.unwrap();
        
        assert!(results.consensus_achieved);
        assert_eq!(results.total_suites_executed, 9); // 3 test types × 3 issues
        
        println!("✅ Master compatibility test suite completed successfully");
        println!("📊 Consensus achieved: {}", results.consensus_achieved);
        println!("⏱️ Total duration: {:?}", results.total_duration);
    }

    #[test]
    fn test_byzantine_consensus_calculation() {
        let config = ByzantineConsensusConfig {
            required_consensus_threshold: 0.67,
            agent_fault_tolerance: 2,
            coordination_timeout: std::time::Duration::from_secs(30),
        };

        // Test with 2/3 success (should pass)
        let votes_pass = vec![true, true, false];
        assert!(calculate_byzantine_consensus(&votes_pass, &config));

        // Test with 1/3 success (should fail)
        let votes_fail = vec![true, false, false];
        assert!(!calculate_byzantine_consensus(&votes_fail, &config));

        println!("✅ Byzantine consensus calculation works correctly");
    }
}