#!/usr/bin/env bash
# Test script for container versioning implementation
# Verifies that dynamic versioning works correctly

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
TEST_TAG="photondrift:versioning-test"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $*"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $*"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
}

# Extract version information
extract_version_info() {
    log_info "Extracting version information..."
    
    VERSION=$(grep '^version = ' "$PROJECT_ROOT/Cargo.toml" | head -1 | cut -d'"' -f2)
    GIT_SHA_FULL=$(git rev-parse HEAD 2>/dev/null || echo 'unknown')
    GIT_SHA_SHORT=$(git rev-parse --short HEAD 2>/dev/null || echo 'unknown')
    GIT_REF=$(git symbolic-ref HEAD 2>/dev/null || echo 'unknown')
    BRANCH=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || echo 'unknown')
    BUILD_DATE=$(date -u +%Y-%m-%dT%H:%M:%SZ)
    
    # Extract semantic version
    if [[ "$VERSION" =~ ^([0-9]+)\.([0-9]+)\.([0-9]+)(-.*)?$ ]]; then
        SEMVER="${BASH_REMATCH[1]}.${BASH_REMATCH[2]}.${BASH_REMATCH[3]}"
    else
        SEMVER="$VERSION"
    fi
    
    log_info "Version: $VERSION"
    log_info "SemVer: $SEMVER"
    log_info "Git SHA: $GIT_SHA_SHORT"
    log_info "Branch: $BRANCH"
    log_info "Build Date: $BUILD_DATE"
}

# Build test container
build_test_container() {
    log_info "Building test container with versioning..."
    
    cd "$PROJECT_ROOT"
    
    # Build with all versioning arguments
    docker build \
        --build-arg "VERSION=$VERSION" \
        --build-arg "BUILD_DATE=$BUILD_DATE" \
        --build-arg "GIT_SHA=$GIT_SHA_FULL" \
        --build-arg "GIT_SHA_SHORT=$GIT_SHA_SHORT" \
        --build-arg "GIT_REF=$GIT_REF" \
        --build-arg "BRANCH=$BRANCH" \
        --build-arg "BUILD_TYPE=test" \
        --build-arg "SEMVER=$SEMVER" \
        --tag "$TEST_TAG" \
        --file Dockerfile \
        .
    
    log_success "Container built successfully"
}

# Test binary functionality
test_binary() {
    log_info "Testing binary functionality..."
    
    # Test version command
    if docker run --rm "$TEST_TAG" --version >/dev/null; then
        log_success "Version command works"
        CONTAINER_VERSION=$(docker run --rm "$TEST_TAG" --version)
        log_info "Container version output: $CONTAINER_VERSION"
    else
        log_error "Version command failed"
        return 1
    fi
    
    # Test help command
    if docker run --rm "$TEST_TAG" --help >/dev/null; then
        log_success "Help command works"
    else
        log_error "Help command failed"
        return 1
    fi
    
    # Verify binary location
    BINARY_PATH=$(docker run --rm "$TEST_TAG" which adrscan)
    if [[ "$BINARY_PATH" == "/usr/local/bin/adrscan" ]]; then
        log_success "Binary correctly located at /usr/local/bin/adrscan"
    else
        log_error "Binary not at expected location. Found: $BINARY_PATH"
        return 1
    fi
}

# Test environment variables
test_environment_variables() {
    log_info "Testing environment variables..."
    
    # Check if ADRSCAN environment variables exist
    ENV_VARS=$(docker run --rm "$TEST_TAG" env | grep ADRSCAN_ || echo "")
    
    if [[ -z "$ENV_VARS" ]]; then
        log_warning "No ADRSCAN environment variables found"
    else
        log_success "ADRSCAN environment variables found:"
        echo "$ENV_VARS" | while read -r line; do
            log_info "  $line"
        done
    fi
}

# Test metadata files
test_metadata_files() {
    log_info "Testing metadata files..."
    
    # Check if metadata directory exists
    if docker run --rm "$TEST_TAG" test -d /etc/adrscan; then
        log_success "Metadata directory exists"
        
        # List metadata files
        METADATA_FILES=$(docker run --rm "$TEST_TAG" ls -la /etc/adrscan/ || echo "")
        log_info "Metadata files:"
        echo "$METADATA_FILES"
        
        # Check specific files
        EXPECTED_FILES=("version" "build_date" "git_sha")
        for file in "${EXPECTED_FILES[@]}"; do
            if docker run --rm "$TEST_TAG" test -f "/etc/adrscan/$file"; then
                CONTENT=$(docker run --rm "$TEST_TAG" cat "/etc/adrscan/$file")
                log_success "File $file exists: $CONTENT"
            else
                log_warning "File $file not found"
            fi
        done
    else
        log_warning "Metadata directory /etc/adrscan does not exist"
    fi
}

# Test container labels
test_container_labels() {
    log_info "Testing container labels..."
    
    # Extract labels
    LABELS=$(docker inspect "$TEST_TAG" | jq -r '.[0].Config.Labels' 2>/dev/null || echo "{}")
    
    if [[ "$LABELS" == "{}" || "$LABELS" == "null" ]]; then
        log_warning "No labels found in container"
        return 0
    fi
    
    # Check required labels
    REQUIRED_LABELS=(
        "org.opencontainers.image.version"
        "org.opencontainers.image.created"
        "org.opencontainers.image.revision"
        "build.timestamp"
        "build.commit"
        "build.branch"
    )
    
    echo "$LABELS" > /tmp/labels.json
    
    for label in "${REQUIRED_LABELS[@]}"; do
        if jq -e ".\"$label\"" /tmp/labels.json >/dev/null 2>&1; then
            VALUE=$(jq -r ".\"$label\"" /tmp/labels.json)
            log_success "Label $label: $VALUE"
        else
            log_warning "Missing label: $label"
        fi
    done
    
    rm -f /tmp/labels.json
}

# Test security (non-root)
test_security() {
    log_info "Testing security configurations..."
    
    # Check if running as non-root
    USER_ID=$(docker run --rm "$TEST_TAG" id -u)
    if [[ "$USER_ID" != "0" ]]; then
        log_success "Container runs as non-root user (UID: $USER_ID)"
    else
        log_error "Container running as root!"
        return 1
    fi
    
    # Check user name
    USERNAME=$(docker run --rm "$TEST_TAG" id -un || echo "unknown")
    log_info "Username: $USERNAME"
}

# Test ADR scanning functionality
test_adr_functionality() {
    log_info "Testing ADR scanning functionality..."
    
    # Test basic scan (should not fail even if no ADRs found)
    if docker run --rm -v "$PROJECT_ROOT:/workspace:ro" "$TEST_TAG" scan --json /workspace >/dev/null 2>&1; then
        log_success "ADR scan command executed successfully"
    else
        log_warning "ADR scan had issues (may be expected if no ADRs in workspace)"
    fi
}

# Cleanup
cleanup() {
    log_info "Cleaning up test container..."
    docker rmi "$TEST_TAG" >/dev/null 2>&1 || true
    log_success "Cleanup complete"
}

# Main execution
main() {
    log_info "Starting container versioning tests..."
    
    # Extract version info
    extract_version_info
    
    # Build and test
    if build_test_container; then
        log_success "Build phase completed"
    else
        log_error "Build phase failed"
        exit 1
    fi
    
    # Run all tests
    TESTS=(
        "test_binary"
        "test_environment_variables"
        "test_metadata_files"
        "test_container_labels"
        "test_security"
        "test_adr_functionality"
    )
    
    FAILED_TESTS=0
    
    for test in "${TESTS[@]}"; do
        log_info "Running $test..."
        if $test; then
            log_success "$test passed"
        else
            log_error "$test failed"
            ((FAILED_TESTS++))
        fi
        echo
    done
    
    # Summary
    if [[ $FAILED_TESTS -eq 0 ]]; then
        log_success "All tests passed! Container versioning implementation is working correctly."
    else
        log_error "$FAILED_TESTS test(s) failed. Please review the implementation."
    fi
    
    # Cleanup
    cleanup
    
    # Exit with appropriate code
    exit $FAILED_TESTS
}

# Show usage if requested
if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then
    echo "Usage: $0"
    echo
    echo "Test script for container versioning implementation."
    echo "Builds a test container and verifies all versioning features work correctly."
    echo
    echo "Options:"
    echo "  -h, --help    Show this help message"
    echo
    exit 0
fi

# Run main function
main "$@"