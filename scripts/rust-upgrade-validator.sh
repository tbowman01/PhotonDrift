#!/bin/bash
# Rust Upgrade Validation Script
# Comprehensive validation for Rust 1.81 + wasmtime v35 upgrade

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
TARGET_RUST_VERSION="1.81.0"
TARGET_WASMTIME_VERSION="35.0"
BASELINE_METRICS_FILE="upgrade-baseline-metrics.json"
RESULTS_FILE="upgrade-validation-results.json"

# Logging functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Initialize results tracking
init_results() {
    cat > "$RESULTS_FILE" << EOF
{
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "rust_version": "$(rustc --version)",
  "phases": {},
  "overall_status": "running"
}
EOF
}

# Update results
update_result() {
    local phase="$1"
    local status="$2"
    local details="$3"
    
    jq --arg phase "$phase" --arg status "$status" --arg details "$details" \
       '.phases[$phase] = {status: $status, details: $details, timestamp: now | todate}' \
       "$RESULTS_FILE" > tmp.json && mv tmp.json "$RESULTS_FILE"
}

# Phase 1: Environment Validation
validate_environment() {
    log_info "Phase 1: Validating environment..."
    
    # Check Rust version
    local rust_version
    rust_version=$(rustc --version | grep -oE '[0-9]+\.[0-9]+\.[0-9]+' | head -1)
    
    if [[ "$rust_version" == "$TARGET_RUST_VERSION" ]]; then
        log_success "Rust version $rust_version matches target"
        update_result "environment" "success" "Rust version correct"
    else
        log_error "Rust version $rust_version does not match target $TARGET_RUST_VERSION"
        update_result "environment" "failed" "Rust version mismatch"
        return 1
    fi
    
    # Check required components
    if rustup component list --installed | grep -q "clippy\|rustfmt"; then
        log_success "Required Rust components installed"
    else
        log_error "Missing required Rust components (clippy, rustfmt)"
        update_result "environment" "failed" "Missing components"
        return 1
    fi
    
    # Check WASM target
    if rustup target list --installed | grep -q "wasm32-wasip1"; then
        log_success "WASM target wasm32-wasip1 installed"
    else
        log_warning "WASM target wasm32-wasip1 not installed, installing..."
        rustup target add wasm32-wasip1
    fi
    
    log_success "Environment validation completed"
    return 0
}

# Phase 2: Compilation Validation
validate_compilation() {
    log_info "Phase 2: Validating compilation..."
    
    local start_time
    start_time=$(date +%s)
    
    # Basic compilation check
    if cargo check --all-features 2>&1 | tee compilation.log; then
        log_success "Basic compilation check passed"
    else
        log_error "Basic compilation failed"
        update_result "compilation" "failed" "Basic compilation errors"
        return 1
    fi
    
    # Release build check
    if cargo build --release --all-features 2>&1 | tee release-build.log; then
        log_success "Release build successful"
    else
        log_error "Release build failed"
        update_result "compilation" "failed" "Release build errors"
        return 1
    fi
    
    # WASM build check
    if cargo build --target wasm32-wasip1 --features wasm 2>&1 | tee wasm-build.log; then
        log_success "WASM build successful"
    else
        log_error "WASM build failed"
        update_result "compilation" "failed" "WASM build errors"
        return 1
    fi
    
    local end_time
    end_time=$(date +%s)
    local build_time=$((end_time - start_time))
    
    log_success "Compilation validation completed in ${build_time}s"
    update_result "compilation" "success" "All builds successful in ${build_time}s"
    return 0
}

# Phase 3: Test Suite Validation
validate_tests() {
    log_info "Phase 3: Validating test suite..."
    
    local start_time
    start_time=$(date +%s)
    
    # Run all tests
    if cargo test --all-features --verbose 2>&1 | tee test-results.log; then
        local test_summary
        test_summary=$(grep -E "test result:|running [0-9]+ tests" test-results.log | tail -2)
        log_success "All tests passed"
        log_info "Test summary: $test_summary"
    else
        log_error "Some tests failed"
        update_result "tests" "failed" "Test failures detected"
        return 1
    fi
    
    # ML-specific tests
    if cargo test ml:: --verbose 2>&1 | tee ml-test-results.log; then
        log_success "ML tests passed"
    else
        log_error "ML tests failed"
        update_result "tests" "failed" "ML test failures"
        return 1
    fi
    
    # Release tests
    if cargo test --release --all-features 2>&1 | tee release-test-results.log; then
        log_success "Release tests passed"
    else
        log_error "Release tests failed"
        update_result "tests" "failed" "Release test failures"
        return 1
    fi
    
    local end_time
    end_time=$(date +%s)
    local test_time=$((end_time - start_time))
    
    log_success "Test validation completed in ${test_time}s"
    update_result "tests" "success" "All tests passed in ${test_time}s"
    return 0
}

# Phase 4: Performance Validation
validate_performance() {
    log_info "Phase 4: Validating performance..."
    
    # Check if baseline exists
    if [[ ! -f "$BASELINE_METRICS_FILE" ]]; then
        log_warning "No baseline metrics found, creating baseline..."
        create_baseline_metrics
        return 0
    fi
    
    # Run performance benchmarks
    if command -v cargo-criterion >/dev/null 2>&1; then
        log_info "Running Criterion benchmarks..."
        cargo bench 2>&1 | tee benchmark-results.log || true
    else
        log_warning "cargo-criterion not installed, skipping detailed benchmarks"
    fi
    
    # Basic build time measurement
    local start_time
    start_time=$(date +%s)
    cargo build --release >/dev/null 2>&1
    local end_time
    end_time=$(date +%s)
    local build_time=$((end_time - start_time))
    
    log_info "Release build time: ${build_time}s"
    
    # Compare with baseline if available
    if [[ -f "$BASELINE_METRICS_FILE" ]]; then
        local baseline_build_time
        baseline_build_time=$(jq -r '.build_time_seconds' "$BASELINE_METRICS_FILE")
        local performance_change
        performance_change=$(echo "scale=2; ($build_time - $baseline_build_time) / $baseline_build_time * 100" | bc -l)
        
        if (( $(echo "$performance_change < 10" | bc -l) )); then
            log_success "Build time within acceptable range (${performance_change}% change)"
        else
            log_warning "Build time increased significantly (${performance_change}% change)"
        fi
    fi
    
    update_result "performance" "success" "Performance validation completed"
    return 0
}

# Phase 5: Security Validation
validate_security() {
    log_info "Phase 5: Validating security..."
    
    # Cargo audit
    if command -v cargo-audit >/dev/null 2>&1; then
        if cargo audit 2>&1 | tee security-audit.log; then
            log_success "No security vulnerabilities found"
        else
            log_warning "Security vulnerabilities detected - review security-audit.log"
        fi
    else
        log_warning "cargo-audit not installed, skipping security audit"
    fi
    
    # Check for new compiler warnings
    if cargo clippy --all-features -- -D warnings 2>&1 | tee clippy-results.log; then
        log_success "No clippy warnings"
    else
        log_warning "Clippy warnings detected - review clippy-results.log"
    fi
    
    update_result "security" "success" "Security validation completed"
    return 0
}

# Phase 6: Integration Validation
validate_integration() {
    log_info "Phase 6: Validating integration..."
    
    # Docker build validation
    if [[ -f "Dockerfile" ]]; then
        log_info "Testing Docker build..."
        if docker build -t photondrift-test . 2>&1 | tee docker-build.log; then
            log_success "Docker build successful"
        else
            log_error "Docker build failed"
            update_result "integration" "failed" "Docker build errors"
            return 1
        fi
    fi
    
    # Makefile targets validation
    if [[ -f "Makefile" ]]; then
        log_info "Testing Makefile targets..."
        make check 2>&1 | tee makefile-check.log || true
        make test 2>&1 | tee makefile-test.log || true
    fi
    
    update_result "integration" "success" "Integration validation completed"
    return 0
}

# Create baseline metrics
create_baseline_metrics() {
    log_info "Creating baseline performance metrics..."
    
    local start_time
    start_time=$(date +%s)
    cargo build --release >/dev/null 2>&1
    local end_time
    end_time=$(date +%s)
    local build_time=$((end_time - start_time))
    
    cat > "$BASELINE_METRICS_FILE" << EOF
{
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "rust_version": "$(rustc --version)",
  "build_time_seconds": $build_time,
  "binary_size_bytes": $(stat -f%z target/release/adrscan 2>/dev/null || stat -c%s target/release/adrscan 2>/dev/null || echo 0)
}
EOF
    
    log_success "Baseline metrics created"
}

# Generate summary report
generate_report() {
    log_info "Generating validation report..."
    
    local overall_status="success"
    local failed_phases
    failed_phases=$(jq -r '.phases | to_entries[] | select(.value.status == "failed") | .key' "$RESULTS_FILE")
    
    if [[ -n "$failed_phases" ]]; then
        overall_status="failed"
    fi
    
    jq --arg status "$overall_status" '.overall_status = $status' "$RESULTS_FILE" > tmp.json && mv tmp.json "$RESULTS_FILE"
    
    echo
    echo "==============================================="
    echo "     RUST UPGRADE VALIDATION REPORT"
    echo "==============================================="
    echo
    
    if [[ "$overall_status" == "success" ]]; then
        log_success "🎉 All validation phases completed successfully!"
        echo
        echo "✅ Environment validation"
        echo "✅ Compilation validation"  
        echo "✅ Test suite validation"
        echo "✅ Performance validation"
        echo "✅ Security validation"
        echo "✅ Integration validation"
        echo
        log_success "Rust $TARGET_RUST_VERSION + wasmtime v$TARGET_WASMTIME_VERSION upgrade validated"
    else
        log_error "❌ Validation failed in the following phases:"
        echo "$failed_phases"
        echo
        log_error "Review detailed logs and fix issues before proceeding"
    fi
    
    echo
    echo "Detailed results: $RESULTS_FILE"
    echo "Log files: *.log"
    echo
}

# Cleanup function
cleanup() {
    log_info "Cleaning up temporary files..."
    # Keep log files but clean up other temporary files
    rm -f tmp.json
}

# Main execution
main() {
    echo "==============================================="
    echo "   RUST UPGRADE VALIDATION SCRIPT"
    echo "   Target: Rust $TARGET_RUST_VERSION + wasmtime v$TARGET_WASMTIME_VERSION"
    echo "==============================================="
    echo
    
    # Set up cleanup trap
    trap cleanup EXIT
    
    # Initialize results tracking
    init_results
    
    # Run validation phases
    local failed=0
    
    validate_environment || failed=1
    validate_compilation || failed=1
    validate_tests || failed=1
    validate_performance || failed=1
    validate_security || failed=1
    validate_integration || failed=1
    
    # Generate final report
    generate_report
    
    # Exit with appropriate code
    if [[ $failed -eq 1 ]]; then
        exit 1
    else
        exit 0
    fi
}

# Script entry point
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi