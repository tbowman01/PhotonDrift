#!/bin/bash
# Setup pre-commit hooks for PhotonDrift

set -e

echo "Setting up pre-commit hooks for PhotonDrift..."

# Check if .git directory exists
if [ ! -d ".git" ]; then
    echo "Error: This script must be run from the root of the git repository"
    exit 1
fi

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Create pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/bash
# PhotonDrift pre-commit hook

echo "Running pre-commit checks..."

# Run cargo fmt check
echo "Checking code formatting..."
if ! cargo fmt -- --check; then
    echo "❌ Code formatting check failed. Run 'cargo fmt' to fix."
    exit 1
fi

# Run clippy
echo "Running clippy..."
if ! cargo clippy -- -D warnings 2>/dev/null; then
    echo "❌ Clippy check failed. Fix the warnings above."
    exit 1
fi

# Run tests
echo "Running tests..."
if ! cargo test --quiet; then
    echo "❌ Tests failed. Fix the failing tests."
    exit 1
fi

# Check for secrets (if detect-secrets is installed)
if command -v detect-secrets &> /dev/null; then
    echo "Scanning for secrets..."
    if ! detect-secrets scan --baseline .secrets.baseline 2>/dev/null; then
        echo "⚠️  Warning: Could not scan for secrets"
    fi
fi

echo "✅ All pre-commit checks passed!"
EOF

# Make hook executable
chmod +x .git/hooks/pre-commit

echo "✅ Pre-commit hooks installed successfully!"
echo ""
echo "The following checks will run before each commit:"
echo "  - Code formatting (cargo fmt)"
echo "  - Linting (cargo clippy)"
echo "  - Tests (cargo test)"
echo "  - Secret detection (if detect-secrets is installed)"
echo ""
echo "To skip hooks temporarily, use: git commit --no-verify"