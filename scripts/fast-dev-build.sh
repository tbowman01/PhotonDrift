#!/usr/bin/env bash
# Fast development build script for PhotonDrift
# Optimized for maximum compilation speed during development

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

log_info() { echo -e "${BLUE}[$(date +'%H:%M:%S')]${NC} $*"; }
log_success() { echo -e "${GREEN}[$(date +'%H:%M:%S')]${NC} $*"; }
log_warn() { echo -e "${YELLOW}[$(date +'%H:%M:%S')]${NC} $*"; }
log_error() { echo -e "${RED}[$(date +'%H:%M:%S')]${NC} $*" >&2; }

show_usage() {
    cat << EOF
Usage: $0 [OPTIONS] [COMPONENT]

Fast development build script optimized for compilation speed.

COMPONENTS:
    rust        Build Rust CLI only (default)
    vscode      Build VSCode extension only
    docs        Build documentation site only
    all         Build all components

OPTIONS:
    --profile PROFILE   Rust build profile (dev|dev-opt|release) [default: dev]
    --parallel          Enable parallel builds where possible
    --skip-tests        Skip running tests
    --watch             Watch for changes and rebuild
    --clean             Clean before building
    -v, --verbose       Verbose output
    -h, --help          Show this help

EXAMPLES:
    $0                          # Fast Rust build
    $0 --parallel all           # Build all components in parallel
    $0 --watch rust             # Watch and rebuild Rust code
    $0 --profile dev-opt        # Build with dev-opt profile

EOF
}

# Default values
COMPONENT="rust"
PROFILE="dev"
PARALLEL=false
SKIP_TESTS=false
WATCH=false
CLEAN=false
VERBOSE=false

# Parse arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        --profile)
            PROFILE="$2"
            shift 2
            ;;
        --parallel)
            PARALLEL=true
            shift
            ;;
        --skip-tests)
            SKIP_TESTS=true
            shift
            ;;
        --watch)
            WATCH=true
            shift
            ;;
        --clean)
            CLEAN=true
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -h|--help)
            show_usage
            exit 0
            ;;
        rust|vscode|docs|all)
            COMPONENT="$1"
            shift
            ;;
        *)
            log_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Setup environment for fast builds
setup_environment() {
    log_info "Setting up fast build environment..."
    
    # Set Rust environment variables for faster compilation
    export CARGO_INCREMENTAL=1
    export CARGO_TARGET_DIR="$PROJECT_ROOT/target"
    
    # Use more parallel jobs if available
    local cpu_count=$(nproc 2>/dev/null || sysctl -n hw.ncpu 2>/dev/null || echo 4)
    local parallel_jobs=$((cpu_count + 1))
    export CARGO_BUILD_JOBS="$parallel_jobs"
    
    # Enable faster linker if available
    if command -v lld >/dev/null 2>&1; then
        export RUSTFLAGS="-C link-arg=-fuse-ld=lld"
    elif command -v mold >/dev/null 2>&1; then
        export RUSTFLAGS="-C link-arg=-fuse-ld=mold"
    fi
    
    log_info "Using $parallel_jobs parallel jobs"
    
    if [[ "$VERBOSE" == "true" ]]; then
        export CARGO_LOG=info
    fi
}

# Clean build artifacts
clean_builds() {
    if [[ "$CLEAN" == "true" ]]; then
        log_info "Cleaning build artifacts..."
        cargo clean
        rm -rf "$PROJECT_ROOT/extensions/vscode/out"
        rm -rf "$PROJECT_ROOT/docs-site/build"
        rm -f "$PROJECT_ROOT/extensions/vscode/.tsbuildinfo"
    fi
}

# Fast Rust build
build_rust() {
    log_info "Building Rust CLI (profile: $PROFILE)..."
    
    local start_time=$(date +%s.%3N)
    local cargo_args=("build")
    
    case "$PROFILE" in
        dev)
            # Default dev build - fastest
            ;;
        dev-opt)
            cargo_args+=("--profile" "dev-opt")
            ;;
        release)
            cargo_args+=("--release")
            ;;
        *)
            log_error "Unknown profile: $PROFILE"
            return 1
            ;;
    esac
    
    # Add verbose output if requested
    if [[ "$VERBOSE" == "true" ]]; then
        cargo_args+=("--verbose")
    fi
    
    # Enable watch mode if requested
    if [[ "$WATCH" == "true" ]]; then
        if command -v cargo-watch >/dev/null 2>&1; then
            log_info "Starting watch mode..."
            cargo watch -x "${cargo_args[*]}"
            return $?
        else
            log_warn "cargo-watch not found, install with: cargo install cargo-watch"
            log_info "Falling back to single build..."
        fi
    fi
    
    # Execute build
    if cargo "${cargo_args[@]}"; then
        local end_time=$(date +%s.%3N)
        local duration=$(echo "$end_time - $start_time" | bc -l)
        log_success "Rust build completed in ${duration}s"
        
        # Run tests if not skipped
        if [[ "$SKIP_TESTS" == "false" ]]; then
            log_info "Running fast tests..."
            cargo test --lib --bins -- --test-threads="$(nproc 2>/dev/null || echo 4)"
        fi
    else
        log_error "Rust build failed"
        return 1
    fi
}

# Fast VSCode extension build
build_vscode() {
    log_info "Building VSCode extension..."
    
    cd "$PROJECT_ROOT/extensions/vscode"
    
    local start_time=$(date +%s.%3N)
    
    # Install dependencies if needed
    if [[ ! -d "node_modules" ]]; then
        log_info "Installing dependencies..."
        npm ci
    fi
    
    # Use TypeScript incremental compilation
    local tsc_args=("--build")
    if [[ "$VERBOSE" == "true" ]]; then
        tsc_args+=("--verbose")
    fi
    
    if [[ "$WATCH" == "true" ]]; then
        log_info "Starting watch mode..."
        npx tsc --watch --preserveWatchOutput
        return $?
    fi
    
    # Fast compilation
    if npx tsc "${tsc_args[@]}"; then
        local end_time=$(date +%s.%3N)
        local duration=$(echo "$end_time - $start_time" | bc -l)
        log_success "VSCode extension build completed in ${duration}s"
    else
        log_error "VSCode extension build failed"
        return 1
    fi
    
    cd "$PROJECT_ROOT"
}

# Fast docs build
build_docs() {
    log_info "Building documentation site..."
    
    cd "$PROJECT_ROOT/docs-site"
    
    local start_time=$(date +%s.%3N)
    
    # Install dependencies if needed
    if [[ ! -d "node_modules" ]]; then
        log_info "Installing dependencies..."
        npm ci
    fi
    
    if [[ "$WATCH" == "true" ]]; then
        log_info "Starting development server..."
        npm run dev:fast
        return $?
    fi
    
    # Fast development build
    if npm run build:dev; then
        local end_time=$(date +%s.%3N)
        local duration=$(echo "$end_time - $start_time" | bc -l)
        log_success "Documentation build completed in ${duration}s"
    else
        log_error "Documentation build failed"
        return 1
    fi
    
    cd "$PROJECT_ROOT"
}

# Parallel build execution
build_parallel() {
    log_info "Starting parallel builds..."
    
    local pids=()
    local components=()
    
    case "$COMPONENT" in
        all)
            components=("rust" "vscode" "docs")
            ;;
        *)
            log_error "Parallel mode only supported for 'all' component"
            return 1
            ;;
    esac
    
    # Start all builds in background
    for comp in "${components[@]}"; do
        (
            case "$comp" in
                rust) build_rust ;;
                vscode) build_vscode ;;
                docs) build_docs ;;
            esac
        ) &
        pids+=($!)
        log_info "Started $comp build (PID: $!)"
    done
    
    # Wait for all builds to complete
    local failed=0
    for i in "${!pids[@]}"; do
        local pid=${pids[$i]}
        local comp=${components[$i]}
        
        if wait "$pid"; then
            log_success "$comp build completed successfully"
        else
            log_error "$comp build failed"
            ((failed++))
        fi
    done
    
    if [[ $failed -eq 0 ]]; then
        log_success "All parallel builds completed successfully"
        return 0
    else
        log_error "$failed build(s) failed"
        return 1
    fi
}

# Main execution
main() {
    log_info "Fast development build starting..."
    log_info "Component: $COMPONENT, Profile: $PROFILE, Parallel: $PARALLEL"
    
    setup_environment
    clean_builds
    
    local start_time=$(date +%s.%3N)
    
    if [[ "$PARALLEL" == "true" ]]; then
        build_parallel
    else
        case "$COMPONENT" in
            rust)
                build_rust
                ;;
            vscode)
                build_vscode
                ;;
            docs)
                build_docs
                ;;
            all)
                build_rust && build_vscode && build_docs
                ;;
            *)
                log_error "Unknown component: $COMPONENT"
                show_usage
                exit 1
                ;;
        esac
    fi
    
    local exit_code=$?
    local end_time=$(date +%s.%3N)
    local total_duration=$(echo "$end_time - $start_time" | bc -l)
    
    if [[ $exit_code -eq 0 ]]; then
        log_success "✨ Fast build completed in ${total_duration}s"
    else
        log_error "❌ Build failed after ${total_duration}s"
    fi
    
    return $exit_code
}

# Run only if called directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi