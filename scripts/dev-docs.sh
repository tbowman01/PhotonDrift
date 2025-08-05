#!/bin/bash
# PhotonDrift Documentation Development Script
# Quick development workflow for documentation

set -euo pipefail

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Project paths
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DOCS_SITE_DIR="$PROJECT_ROOT/docs-site"

log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

show_usage() {
    cat << EOF
PhotonDrift Documentation Development Helper

Usage: $0 [COMMAND]

Commands:
    setup       Initial setup (install dependencies)
    dev         Start development server with sync
    sync        Sync content only
    clean       Clean build and node_modules
    help        Show this help

Quick Development Workflow:
    $0 setup    # Run once
    $0 dev      # Start development
    $0 sync     # Sync content changes

EOF
}

setup() {
    log_info "Setting up documentation development environment..."
    
    cd "$DOCS_SITE_DIR"
    
    # Install dependencies
    log_info "Installing Node.js dependencies..."
    npm install
    
    log_success "Development environment setup complete!"
    echo ""
    echo "Next steps:"
    echo "  🚀 Run: $0 dev"
    echo "  📝 Edit files in: docs/"
    echo "  🔄 Sync changes: $0 sync"
}

dev() {
    log_info "Starting documentation development server..."
    
    cd "$DOCS_SITE_DIR"
    
    # Check if dependencies are installed
    if [ ! -d "node_modules" ]; then
        log_warning "Dependencies not found. Running setup..."
        npm install
    fi
    
    # Sync content first
    log_info "Syncing content..."
    npm run sync-docs || {
        log_warning "Content sync failed, continuing..."
    }
    
    log_success "Development server starting..."
    log_info "📝 Edit files in: $PROJECT_ROOT/docs/"
    log_info "🔄 Run 'npm run sync-docs' in docs-site/ after changes"
    log_info "🌐 Site will be available at: http://localhost:3000"
    echo ""
    
    # Start development server
    npm start
}

sync_content() {
    log_info "Syncing documentation content..."
    
    cd "$DOCS_SITE_DIR"
    
    npm run sync-docs
    
    log_success "Content sync completed!"
    log_info "🔄 Refresh your browser to see changes"
}

clean() {
    log_info "Cleaning documentation build artifacts..."
    
    cd "$DOCS_SITE_DIR"
    
    # Remove build directories
    [ -d "build" ] && rm -rf build && log_info "Removed build/"
    [ -d "node_modules" ] && rm -rf node_modules && log_info "Removed node_modules/"
    [ -f "package-lock.json" ] && rm -f package-lock.json && log_info "Removed package-lock.json"
    
    log_success "Clean completed!"
}

main() {
    local command=${1:-help}
    
    case $command in
        setup)
            setup
            ;;
        dev)
            dev
            ;;
        sync)
            sync_content
            ;;
        clean)
            clean
            ;;
        help|--help|-h)
            show_usage
            ;;
        *)
            echo "Unknown command: $command"
            echo ""
            show_usage
            exit 1
            ;;
    esac
}

# Check if docs-site directory exists
if [ ! -d "$DOCS_SITE_DIR" ]; then
    echo "Error: Documentation site directory not found: $DOCS_SITE_DIR"
    exit 1
fi

main "$@"