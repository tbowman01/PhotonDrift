#!/bin/bash
# Setup script for ADRScan git hooks and pre-commit configuration

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

print_status() {
    echo -e "${BLUE}[Setup]${NC} $1"
}

print_success() {
    echo -e "${GREEN}✅${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠️${NC} $1"
}

print_error() {
    echo -e "${RED}❌${NC} $1"
}

print_status "Setting up ADRScan development environment..."

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    print_error "Not in a git repository. Please run this script from the project root."
    exit 1
fi

# Check if we're in the right project
if [ ! -f "Cargo.toml" ]; then
    print_error "Cargo.toml not found. Please run this script from the ADRScan project root."
    exit 1
fi

# 1. Install pre-commit if not available
print_status "Checking for pre-commit..."
if ! command -v pre-commit &> /dev/null; then
    print_warning "pre-commit not found. Installing..."
    
    # Try different installation methods
    if command -v pip &> /dev/null; then
        pip install pre-commit
    elif command -v pip3 &> /dev/null; then
        pip3 install pre-commit
    elif command -v brew &> /dev/null; then
        brew install pre-commit
    elif command -v conda &> /dev/null; then
        conda install -c conda-forge pre-commit
    else
        print_error "Could not install pre-commit. Please install it manually:"
        echo "  pip install pre-commit"
        echo "  # or"
        echo "  brew install pre-commit"
        echo "  # or"
        echo "  conda install -c conda-forge pre-commit"
        exit 1
    fi
fi
print_success "pre-commit is available"

# 2. Configure git hooks directory
print_status "Configuring git hooks directory..."
git config core.hooksPath .githooks
print_success "Git hooks directory set to .githooks"

# 3. Install pre-commit hooks
print_status "Installing pre-commit hooks..."
if [ -f ".pre-commit-config.yaml" ]; then
    pre-commit install
    print_success "Pre-commit hooks installed"
else
    print_warning ".pre-commit-config.yaml not found, skipping pre-commit installation"
fi

# 4. Make sure hook scripts are executable
print_status "Setting hook permissions..."
if [ -f ".githooks/pre-commit" ]; then
    chmod +x .githooks/pre-commit
    print_success "Pre-commit hook made executable"
fi

# 5. Test Rust toolchain
print_status "Checking Rust toolchain..."
if ! command -v rustfmt &> /dev/null; then
    print_warning "rustfmt not found. Installing..."
    rustup component add rustfmt
fi

if ! command -v cargo-clippy &> /dev/null; then
    print_warning "clippy not found. Installing..."
    rustup component add clippy
fi
print_success "Rust toolchain is ready"

# 6. Run initial format check
print_status "Running initial format check..."
if ! cargo fmt --all -- --check; then
    print_warning "Code needs formatting. Running cargo fmt..."
    cargo fmt --all
    print_success "Code formatted"
else
    print_success "Code is already properly formatted"
fi

# 7. Run initial clippy check
print_status "Running initial clippy check..."
if cargo clippy --all-targets --all-features -- -D warnings; then
    print_success "Clippy checks passed"
else
    print_warning "Clippy found issues. Please review and fix them."
fi

# 8. Test pre-commit hooks
print_status "Testing pre-commit hooks..."
if pre-commit run --all-files; then
    print_success "All pre-commit hooks passed"
else
    print_warning "Some pre-commit hooks failed. This is normal for initial setup."
    print_status "Run 'pre-commit run --all-files' to see detailed results"
fi

print_success "Development environment setup complete!"
echo
print_status "Available commands:"
echo "  cargo fmt --all              # Format all Rust code"
echo "  cargo clippy --all-features  # Run linting checks"
echo "  cargo test                   # Run test suite"
echo "  pre-commit run --all-files   # Run all pre-commit hooks"
echo "  pre-commit run <hook-id>     # Run specific hook"
echo
print_status "Environment variables for pre-commit hook:"
echo "  SKIP_TESTS=1                 # Skip test suite in pre-commit"
echo "  ALLOW_TODOS=1                # Allow TODO/FIXME comments"
echo "  SKIP_SECURITY_CHECK=1        # Skip basic security checks"
echo
print_success "Happy coding! 🦀"