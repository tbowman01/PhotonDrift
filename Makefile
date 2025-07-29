# PhotonDrift Build Automation Makefile
# Provides convenient shortcuts for development and build operations

.DEFAULT_GOAL := help
.PHONY: help build test clean dev rust check lint docs docs-build docs-serve docs-clean docs-install version

# Configuration
RUST_VERSION ?= 1.75
CARGO_FEATURES ?= default

## Rust Development Commands
check: ## Check Rust code without building
	@echo "🔍 Checking Rust code..."
	cargo check --all-features
	@echo "✅ Check complete"

build: ## Build Rust CLI
	@echo "🏗️ Building Rust CLI..."
	cargo build
	@echo "✅ Build complete"

build-release: ## Build optimized release version
	@echo "🏗️ Building release version..."
	cargo build --release
	@echo "✅ Release build complete"

test: ## Run Rust tests
	@echo "🧪 Running Rust tests..."
	cargo test --all-features
	@echo "✅ Tests complete"

lint: ## Run linting and formatting checks
	@echo "🔧 Running linting checks..."
	cargo clippy --all-features -- -D warnings
	cargo fmt --check
	@echo "✅ Linting complete"

fix: ## Fix linting issues automatically
	@echo "🔧 Fixing code issues..."
	cargo clippy --all-features --fix --allow-dirty
	cargo fmt
	@echo "✅ Fixes applied"

## Documentation Commands
docs-install: ## Install documentation dependencies
	@echo "📦 Installing documentation dependencies..."
	cd docs-site && npm ci
	@echo "✅ Dependencies installed"

docs-build: ## Build documentation site
	@echo "📚 Building documentation..."
	./scripts/build-docs.sh
	@echo "✅ Documentation built"

docs-build-fast: ## Build documentation quickly (skip CLI build and validation)
	@echo "📚 Building documentation (fast mode)..."
	./scripts/build-docs.sh --fast
	@echo "✅ Fast documentation build complete"

docs-serve: ## Build and serve documentation locally
	@echo "📚 Building and serving documentation..."
	./scripts/build-docs.sh --serve
	@echo "🌐 Documentation serving at http://localhost:3000"

docs-dev: ## Start documentation development server
	@echo "📚 Starting docs development server..."
	cd docs-site && npm run start
	@echo "🌐 Development server started"

docs-clean: ## Clean documentation build files
	@echo "🧹 Cleaning documentation..."
	./scripts/build-docs.sh --clean
	@echo "✅ Documentation cleaned"

docs-sync: ## Sync documentation content only
	@echo "🔄 Syncing documentation content..."
	node scripts/docs-sync.js --verbose
	@echo "✅ Content synchronized"

docs-validate: ## Validate documentation links
	@echo "🔍 Validating documentation links..."
	node scripts/link-validator.js --external
	@echo "✅ Link validation complete"

## Container Commands (using existing build script)
container-build: ## Build container image using build script
	@echo "🐳 Building container image..."
	@if [ -f "./scripts/build-automation.sh" ]; then \
		./scripts/build-automation.sh -e dev build; \
	else \
		echo "❌ build-automation.sh not found"; \
		exit 1; \
	fi

container-test: ## Test container using build script
	@echo "🧪 Testing container..."
	@if [ -f "./scripts/build-automation.sh" ]; then \
		./scripts/build-automation.sh -e dev test; \
	else \
		echo "❌ build-automation.sh not found"; \
		exit 1; \
	fi

## Development Workflows
dev: ## Quick development build (Rust + docs)
	$(MAKE) check
	$(MAKE) docs-build-fast
	@echo "✅ Development build complete"

dev-full: ## Full development build with tests
	$(MAKE) test
	$(MAKE) docs-build
	@echo "✅ Full development build complete"

## Security and Validation
security-scan: ## Run security checks
	@echo "🔒 Running security scan..."
	@if [ -f "./scripts/security-check.sh" ]; then \
		./scripts/security-check.sh; \
	else \
		echo "⚠️ Security check script not found, running cargo audit instead"; \
		cargo audit; \
	fi

validate: ## Run all validation checks
	@echo "🔍 Running validation checks..."
	$(MAKE) check
	$(MAKE) lint
	$(MAKE) docs-validate
	@echo "✅ All validations passed"

## Utility Commands
clean: ## Clean all build artifacts
	@echo "🧹 Cleaning build artifacts..."
	cargo clean
	rm -rf docs-site/build/
	rm -rf docs-site/.docusaurus/
	rm -rf docs-site/node_modules/
	rm -f link-validation-report.json
	@echo "✅ Cleanup complete"

setup: ## Setup development environment
	@echo "🔧 Setting up development environment..."
	@chmod +x scripts/*.sh scripts/*.js
	rustup default $(RUST_VERSION)
	rustup component add clippy rustfmt
	@if ! command -v node >/dev/null 2>&1; then \
		echo "⚠️ Node.js not found. Please install Node.js 18+"; \
	fi
	@echo "✅ Development environment ready"

version: ## Show version information
	@echo "PhotonDrift Build System"
	@echo "========================"
	@grep -E '^version = ' Cargo.toml | head -1 | cut -d'"' -f2 || echo "Version not found"
	@echo ""
	@echo "Environment:"
	@echo "  Rust: $$(rustc --version 2>/dev/null || echo 'Not installed')"
	@echo "  Cargo: $$(cargo --version 2>/dev/null || echo 'Not installed')"
	@echo "  Node.js: $$(node --version 2>/dev/null || echo 'Not installed')"
	@echo "  Docker: $$(docker --version 2>/dev/null | cut -d' ' -f3 | tr -d ',' || echo 'Not installed')"

status: ## Show project status
	@echo "📊 Project Status"
	@echo "================="
	@echo ""
	@echo "🦀 Rust Status:"
	@if cargo check --quiet 2>/dev/null; then \
		echo "  ✅ Code compiles cleanly"; \
	else \
		echo "  ❌ Compilation issues found"; \
	fi
	@echo ""
	@echo "📚 Documentation Status:"
	@if [ -d "docs-site/node_modules" ]; then \
		echo "  ✅ Dependencies installed"; \
	else \
		echo "  ❌ Dependencies not installed (run 'make docs-install')"; \
	fi
	@if [ -d "docs-site/build" ]; then \
		echo "  ✅ Documentation built"; \
	else \
		echo "  ⚠️ Documentation not built (run 'make docs-build')"; \
	fi

## Test individual components
test-rust: ## Test only Rust code
	cargo test --all-features

test-docs: ## Test documentation build
	$(MAKE) docs-build-fast

test-all: ## Run all tests
	$(MAKE) test-rust
	$(MAKE) test-docs
	@echo "✅ All tests completed"

## Advanced Development Commands
wasm-build: ## Build WebAssembly version
	@echo "🕸️ Building WebAssembly..."
	@if [ -f "./scripts/build-wasm-optimized.sh" ]; then \
		./scripts/build-wasm-optimized.sh; \
	else \
		echo "❌ WASM build script not found"; \
		exit 1; \
	fi

benchmark: ## Run performance benchmarks
	@echo "⚡ Running benchmarks..."
	@if [ -f "./scripts/performance-benchmark.sh" ]; then \
		./scripts/performance-benchmark.sh; \
	else \
		echo "❌ Benchmark script not found"; \
		exit 1; \
	fi

## Help target
help: ## Show this help message
	@echo "PhotonDrift Development Commands"
	@echo "==============================="
	@echo ""
	@echo "🚀 Quick Start:"
	@echo "  make setup          # Setup development environment"
	@echo "  make dev            # Quick development build"
	@echo "  make docs-serve     # Build and serve documentation"
	@echo ""
	@echo "📚 Documentation:"
	@echo "  make docs-build     # Build documentation"
	@echo "  make docs-serve     # Build and serve locally"
	@echo "  make docs-dev       # Start development server"
	@echo ""
	@echo "🦀 Rust Development:"
	@echo "  make check          # Check code without building"
	@echo "  make build          # Build the CLI"
	@echo "  make test           # Run tests"
	@echo "  make lint           # Run linting checks"
	@echo ""
	@echo "Available targets:"
	@echo ""
	@awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
	@echo ""
	@echo "For more info: make status"