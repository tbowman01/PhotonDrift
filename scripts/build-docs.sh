#!/bin/bash
# PhotonDrift Documentation Build Script
# Builds the complete documentation site with content sync and validation

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Script directory and project root
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DOCS_SITE_DIR="$PROJECT_ROOT/docs-site"

# Configuration
BUILD_CLI=${BUILD_CLI:-true}
SYNC_CONTENT=${SYNC_CONTENT:-true}
VALIDATE_LINKS=${VALIDATE_LINKS:-true}
SERVE_AFTER_BUILD=${SERVE_AFTER_BUILD:-false}
CLEAN_BUILD=${CLEAN_BUILD:-false}

# Functions
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

show_usage() {
    cat << EOF
PhotonDrift Documentation Build Script

Usage: $0 [OPTIONS]

Options:
    -h, --help              Show this help message
    -c, --clean             Clean build (remove node_modules and build dirs)
    -s, --serve             Serve the site after building
    -f, --fast              Fast build (skip CLI build and link validation)
    --no-sync              Skip content synchronization
    --no-validate          Skip link validation
    --no-cli               Skip CLI documentation generation

Environment Variables:
    BUILD_CLI=true|false           Build Rust CLI for docs generation (default: true)
    SYNC_CONTENT=true|false        Sync content from docs/ (default: true)
    VALIDATE_LINKS=true|false      Validate links (default: true)
    SERVE_AFTER_BUILD=true|false   Serve after build (default: false)
    CLEAN_BUILD=true|false         Clean build directories (default: false)

Examples:
    $0                             # Full build
    $0 --clean --serve            # Clean build and serve
    $0 --fast                     # Quick build for development
    $0 --no-validate --serve      # Build without validation and serve

EOF
}

check_dependencies() {
    log_info "Checking dependencies..."
    
    # Check Node.js
    if ! command -v node &> /dev/null; then
        log_error "Node.js is not installed. Please install Node.js 18 or later."
        exit 1
    fi
    
    local node_version=$(node --version | sed 's/v//')
    local required_version="18.0.0"
    if ! printf '%s\n%s\n' "$required_version" "$node_version" | sort -V -C; then
        log_error "Node.js version $node_version is too old. Please install Node.js 18 or later."
        exit 1
    fi
    
    # Check npm
    if ! command -v npm &> /dev/null; then
        log_error "npm is not installed. Please install npm."
        exit 1
    fi
    
    # Check if docs-site directory exists
    if [ ! -d "$DOCS_SITE_DIR" ]; then
        log_error "Documentation site directory not found: $DOCS_SITE_DIR"
        exit 1
    fi
    
    log_success "Dependencies check passed"
}

build_cli() {
    if [ "$BUILD_CLI" = "true" ]; then
        log_info "Building Rust CLI for documentation generation..."
        cd "$PROJECT_ROOT"
        
        if command -v cargo &> /dev/null; then
            cargo build --bin adrscan --release
            if [ -f "target/release/adrscan" ]; then
                cp target/release/adrscan "$DOCS_SITE_DIR/" || true
                log_success "CLI built and copied to docs-site directory"
            else
                log_warning "CLI binary not found after build, CLI docs generation may fail"
            fi
        else
            log_warning "Cargo not found, skipping CLI build. CLI docs generation may fail."
        fi
    else
        log_info "Skipping CLI build (BUILD_CLI=false)"
    fi
}

install_dependencies() {
    log_info "Installing Node.js dependencies..."
    cd "$DOCS_SITE_DIR"
    
    if [ "$CLEAN_BUILD" = "true" ] && [ -d "node_modules" ]; then
        log_info "Removing existing node_modules..."
        rm -rf node_modules package-lock.json
    fi
    
    npm ci
    log_success "Dependencies installed"
}

sync_content() {
    if [ "$SYNC_CONTENT" = "true" ]; then
        log_info "Synchronizing documentation content..."
        cd "$DOCS_SITE_DIR"
        
        npm run sync-docs
        log_success "Content synchronization completed"
    else
        log_info "Skipping content sync (SYNC_CONTENT=false)"
    fi
}

generate_cli_docs() {
    if [ "$BUILD_CLI" = "true" ]; then
        log_info "Generating CLI documentation..."
        cd "$DOCS_SITE_DIR"
        
        npm run generate-cli-docs || {
            log_warning "CLI documentation generation failed, continuing with existing docs"
        }
        log_success "CLI documentation generation completed"
    else
        log_info "Skipping CLI documentation generation (BUILD_CLI=false)"
    fi
}

validate_links() {
    if [ "$VALIDATE_LINKS" = "true" ]; then
        log_info "Validating documentation links..."
        cd "$DOCS_SITE_DIR"
        
        npm run validate-links || {
            log_warning "Link validation failed, continuing build"
        }
        log_success "Link validation completed"
    else
        log_info "Skipping link validation (VALIDATE_LINKS=false)"
    fi
}

build_site() {
    log_info "Building documentation site..."
    cd "$DOCS_SITE_DIR"
    
    if [ "$CLEAN_BUILD" = "true" ] && [ -d "build" ]; then
        log_info "Removing existing build directory..."
        rm -rf build
    fi
    
    npm run build
    
    # Verify build
    if [ ! -d "build" ]; then
        log_error "Build directory not created"
        exit 1
    fi
    
    if [ ! -f "build/index.html" ]; then
        log_error "Build appears to be incomplete (no index.html)"
        exit 1
    fi
    
    log_success "Documentation site built successfully"
}

show_build_stats() {
    log_info "Build statistics:"
    cd "$DOCS_SITE_DIR/build"
    
    local pages=$(find . -name "*.html" | wc -l)
    local assets=$(find . -type f | wc -l)
    local size=$(du -sh . | cut -f1)
    
    echo "  📄 Generated pages: $pages"
    echo "  📦 Total assets: $assets"
    echo "  💾 Build size: $size"
}

serve_site() {
    if [ "$SERVE_AFTER_BUILD" = "true" ]; then
        log_info "Starting development server..."
        cd "$DOCS_SITE_DIR"
        
        log_success "Documentation site available at: http://localhost:3000"
        log_info "Press Ctrl+C to stop the server"
        npm run serve
    fi
}

main() {
    # Parse command line arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -h|--help)
                show_usage
                exit 0
                ;;
            -c|--clean)
                CLEAN_BUILD=true
                shift
                ;;
            -s|--serve)
                SERVE_AFTER_BUILD=true
                shift
                ;;
            -f|--fast)
                BUILD_CLI=false
                VALIDATE_LINKS=false
                shift
                ;;
            --no-sync)
                SYNC_CONTENT=false
                shift
                ;;
            --no-validate)
                VALIDATE_LINKS=false
                shift
                ;;
            --no-cli)
                BUILD_CLI=false
                shift
                ;;
            *)
                log_error "Unknown option: $1"
                show_usage
                exit 1
                ;;
        esac
    done
    
    log_info "Starting PhotonDrift documentation build..."
    log_info "Configuration:"
    echo "  🏗️ Build CLI: $BUILD_CLI"
    echo "  🔄 Sync content: $SYNC_CONTENT"
    echo "  🔍 Validate links: $VALIDATE_LINKS"
    echo "  🧹 Clean build: $CLEAN_BUILD"
    echo "  🌐 Serve after build: $SERVE_AFTER_BUILD"
    echo ""
    
    # Build process
    check_dependencies
    build_cli
    install_dependencies
    sync_content
    generate_cli_docs
    validate_links
    build_site
    show_build_stats
    
    log_success "Documentation build completed! 🎉"
    echo ""
    echo "Next steps:"
    echo "  📁 Build output: $DOCS_SITE_DIR/build"
    echo "  🌐 Serve locally: cd docs-site && npm run serve"
    echo "  🚀 Deploy: Push to main/develop branch for auto-deployment"
    echo ""
    
    serve_site
}

# Run main function
main "$@"