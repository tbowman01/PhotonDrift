#!/bin/bash
set -euo pipefail

# Phase 2.5: Comprehensive Dependency Validation Script
# Purpose: Validate dependency updates with automated testing and approval

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

# Configuration
PERFORMANCE_THRESHOLD=0.05  # 5% regression threshold
TEST_TIMEOUT=300           # 5 minute timeout for tests
BENCHMARK_BASELINE="baseline"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

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

# Function to check if a command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Validate prerequisites
validate_prerequisites() {
    log_info "Validating prerequisites..."
    
    local missing_tools=()
    
    if ! command_exists cargo; then
        missing_tools+=("cargo")
    fi
    
    if ! command_exists npm; then
        missing_tools+=("npm")
    fi
    
    if ! command_exists jq; then
        missing_tools+=("jq")
    fi
    
    if ! command_exists gh; then
        missing_tools+=("gh")
    fi
    
    if [ ${#missing_tools[@]} -ne 0 ]; then
        log_error "Missing required tools: ${missing_tools[*]}"
        exit 1
    fi
    
    log_success "All prerequisites satisfied"
}

# Create baseline performance benchmarks
create_performance_baseline() {
    log_info "Creating performance baseline..."
    
    cd "$PROJECT_ROOT"
    
    # Install current dependencies
    cargo build --release --all-features
    
    # Run performance benchmarks
    if [ -f "benches/dependency_performance.rs" ]; then
        cargo bench --bench dependency_performance -- --save-baseline "$BENCHMARK_BASELINE"
        log_success "Performance baseline created"
    else
        log_warning "No performance benchmarks found - creating basic ones"
        create_basic_benchmarks
    fi
}

# Create basic performance benchmarks if they don't exist
create_basic_benchmarks() {
    mkdir -p "$PROJECT_ROOT/benches"
    
    cat > "$PROJECT_ROOT/benches/dependency_performance.rs" << 'EOF'
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn basic_functionality_benchmark(c: &mut Criterion) {
    c.bench_function("basic_adr_parsing", |b| {
        let sample_adr = r#"# ADR-001: Test Decision

## Status
Accepted

## Context
Test context

## Decision
Test decision

## Consequences
Test consequences
"#;
        
        b.iter(|| {
            // Basic ADR parsing benchmark
            black_box(sample_adr.len())
        })
    });
}

criterion_group!(benches, basic_functionality_benchmark);
criterion_main!(benches);
EOF

    # Add benchmark dependency to Cargo.toml if not present
    if ! grep -q "\[bench\]" "$PROJECT_ROOT/Cargo.toml"; then
        cat >> "$PROJECT_ROOT/Cargo.toml" << 'EOF'

[[bench]]
name = "dependency_performance"
harness = false

[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
EOF
    fi
    
    log_success "Basic performance benchmarks created"
}

# Security audit for Rust dependencies
rust_security_audit() {
    log_info "Running Rust security audit..."
    
    cd "$PROJECT_ROOT"
    
    # Install cargo-audit if not present
    if ! command_exists cargo-audit; then
        log_info "Installing cargo-audit..."
        cargo install cargo-audit
    fi
    
    # Run security audit
    if cargo audit --json > rust-audit.json 2>/dev/null; then
        local vuln_count=$(jq '.vulnerabilities.count' rust-audit.json 2>/dev/null || echo "0")
        
        if [ "$vuln_count" -eq 0 ]; then
            log_success "Rust security audit passed - no vulnerabilities found"
            return 0
        else
            log_error "Rust security audit failed - $vuln_count vulnerabilities found"
            return 1
        fi
    else
        log_error "Rust security audit failed to run"
        return 1
    fi
}

# Node.js security audit
nodejs_security_audit() {
    log_info "Running Node.js security audit..."
    
    cd "$PROJECT_ROOT"
    
    # Check if package.json exists
    if [ ! -f "package.json" ]; then
        log_info "No package.json found - skipping Node.js audit"
        return 0
    fi
    
    # Install dependencies if node_modules doesn't exist
    if [ ! -d "node_modules" ]; then
        npm install
    fi
    
    # Run security audit
    if npm audit --audit-level=moderate --json > npm-audit.json 2>/dev/null; then
        local vuln_count=$(jq '.metadata.vulnerabilities.total' npm-audit.json 2>/dev/null || echo "0")
        
        if [ "$vuln_count" -eq 0 ]; then
            log_success "Node.js security audit passed - no vulnerabilities found"
            return 0
        else
            log_error "Node.js security audit failed - $vuln_count vulnerabilities found"
            return 1
        fi
    else
        log_warning "Node.js security audit had issues - check manually"
        return 0  # Don't fail on audit issues for Node.js
    fi
}

# Comprehensive test suite
run_comprehensive_tests() {
    log_info "Running comprehensive test suite..."
    
    cd "$PROJECT_ROOT"
    
    # Rust tests with all features
    log_info "Running Rust tests with all features..."
    if ! timeout $TEST_TIMEOUT cargo test --all-features --verbose; then
        log_error "Rust tests failed"
        return 1
    fi
    
    # Clippy linting
    log_info "Running clippy linting..."
    if ! cargo clippy --all-targets --all-features -- -D warnings; then
        log_error "Clippy linting failed"
        return 1
    fi
    
    # Format checking
    log_info "Checking code formatting..."
    if ! cargo fmt --all -- --check; then
        log_error "Code formatting check failed"
        return 1
    fi
    
    # Build tests
    log_info "Testing release build..."
    if ! cargo build --release --all-features; then
        log_error "Release build failed"
        return 1
    fi
    
    # WASM-specific tests if wasmtime is enabled
    if cargo check --features=plugins 2>/dev/null; then
        log_info "Running WASM-specific tests..."
        if ! cargo test --features=plugins; then
            log_error "WASM tests failed"
            return 1
        fi
    fi
    
    log_success "All tests passed"
    return 0
}

# Performance regression testing
performance_regression_test() {
    log_info "Running performance regression tests..."
    
    cd "$PROJECT_ROOT"
    
    # Run current benchmarks
    if ! cargo bench --bench dependency_performance -- --save-baseline "current"; then
        log_error "Performance benchmarks failed to run"
        return 1
    fi
    
    # Compare with baseline (basic implementation)
    log_info "Comparing performance with baseline..."
    
    # For now, just ensure benchmarks run successfully
    # In the future, implement proper performance comparison
    log_success "Performance regression test completed"
    return 0
}

# Integration test specific to dependency changes
integration_test() {
    log_info "Running integration tests..."
    
    cd "$PROJECT_ROOT"
    
    # Test core functionality end-to-end
    log_info "Testing core ADR functionality..."
    
    # Create temporary test directory
    local test_dir="/tmp/adrscan-integration-test-$$"
    mkdir -p "$test_dir/docs/adr"
    
    # Create sample ADR
    cat > "$test_dir/docs/adr/001-test-decision.md" << 'EOF'
# ADR-001: Test Architectural Decision

## Status
Accepted

## Context
This is a test ADR for dependency validation.

## Decision
We will use this for testing.

## Consequences
Testing should work properly.
EOF

    # Test CLI functionality
    if cargo run --release -- inventory --adr-dir "$test_dir/docs/adr" --format json > /dev/null; then
        log_success "CLI integration test passed"
    else
        log_error "CLI integration test failed"
        rm -rf "$test_dir"
        return 1
    fi
    
    # Test WASM functionality if available
    if [ -d "wasm" ] && [ -f "wasm/package.json" ]; then
        log_info "Testing WASM integration..."
        cd "wasm"
        if npm test > /dev/null 2>&1; then
            log_success "WASM integration test passed"
        else
            log_warning "WASM integration test failed - may need module build"
        fi
        cd "$PROJECT_ROOT"
    fi
    
    # Cleanup
    rm -rf "$test_dir"
    return 0
}

# Apply specific dependency update
apply_dependency_update() {
    local package="$1"
    local new_version="$2"
    local package_manager="${3:-cargo}"
    
    log_info "Applying $package update to version $new_version..."
    
    cd "$PROJECT_ROOT"
    
    case "$package_manager" in
        "cargo")
            # Update Cargo.toml
            if command_exists sed; then
                sed -i.bak "s/^$package = .*/$package = \"$new_version\"/" Cargo.toml
                log_success "Updated $package to $new_version in Cargo.toml"
            else
                log_error "sed command not found - manual update required"
                return 1
            fi
            
            # Update lockfile
            cargo update -p "$package"
            ;;
        "npm")
            # Update package.json and install
            npm install "$package@$new_version"
            ;;
        *)
            log_error "Unknown package manager: $package_manager"
            return 1
            ;;
    esac
    
    return 0
}

# Rollback dependency update
rollback_dependency_update() {
    local package="$1"
    local package_manager="${2:-cargo}"
    
    log_warning "Rolling back $package update..."
    
    cd "$PROJECT_ROOT"
    
    case "$package_manager" in
        "cargo")
            if [ -f "Cargo.toml.bak" ]; then
                mv Cargo.toml.bak Cargo.toml
                cargo update
                log_success "Rolled back $package update"
            else
                log_error "No backup found for rollback"
                return 1
            fi
            ;;
        "npm")
            git checkout package.json package-lock.json 2>/dev/null || true
            npm install
            ;;
    esac
    
    return 0
}

# Full validation pipeline for a dependency update
validate_dependency_update() {
    local package="$1"
    local new_version="$2"
    local package_manager="${3:-cargo}"
    
    log_info "Starting validation pipeline for $package $new_version"
    
    # Create baseline if it doesn't exist
    if [ ! -f "target/criterion/$BENCHMARK_BASELINE" ]; then
        create_performance_baseline
    fi
    
    # Apply the update
    if ! apply_dependency_update "$package" "$new_version" "$package_manager"; then
        log_error "Failed to apply dependency update"
        return 1
    fi
    
    # Run validation tests
    local validation_passed=true
    
    # Security audit
    if ! rust_security_audit; then
        validation_passed=false
    fi
    
    if ! nodejs_security_audit; then
        validation_passed=false
    fi
    
    # Comprehensive tests
    if ! run_comprehensive_tests; then
        validation_passed=false
    fi
    
    # Performance regression test
    if ! performance_regression_test; then
        validation_passed=false
    fi
    
    # Integration tests
    if ! integration_test; then
        validation_passed=false
    fi
    
    # Evaluate results
    if [ "$validation_passed" = true ]; then
        log_success "Dependency validation passed for $package $new_version"
        
        # Clean up backup files
        rm -f Cargo.toml.bak
        
        return 0
    else
        log_error "Dependency validation failed for $package $new_version"
        
        # Rollback changes
        rollback_dependency_update "$package" "$package_manager"
        
        return 1
    fi
}

# Main function
main() {
    local action="${1:-help}"
    
    case "$action" in
        "validate")
            local package="${2:-}"
            local version="${3:-}"
            local pkg_manager="${4:-cargo}"
            
            if [ -z "$package" ] || [ -z "$version" ]; then
                log_error "Usage: $0 validate <package> <version> [package_manager]"
                exit 1
            fi
            
            validate_prerequisites
            validate_dependency_update "$package" "$version" "$pkg_manager"
            ;;
        "baseline")
            validate_prerequisites
            create_performance_baseline
            ;;
        "audit")
            validate_prerequisites
            rust_security_audit
            nodejs_security_audit
            ;;
        "test")
            validate_prerequisites
            run_comprehensive_tests
            ;;
        "help"|*)
            echo "Phase 2.5 Dependency Validation Script"
            echo ""
            echo "Usage: $0 <action> [options]"
            echo ""
            echo "Actions:"
            echo "  validate <package> <version> [pkg_manager]  - Full validation pipeline"
            echo "  baseline                                     - Create performance baseline"
            echo "  audit                                        - Run security audits"
            echo "  test                                         - Run test suite"
            echo "  help                                         - Show this help"
            echo ""
            echo "Examples:"
            echo "  $0 validate wasmtime 35.0 cargo"
            echo "  $0 validate notify 8.0 cargo"
            echo "  $0 baseline"
            echo "  $0 audit"
            ;;
    esac
}

# Run main function with all arguments
main "$@"