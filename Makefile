# PhotonDrift Build Automation Makefile
# Enhanced developer experience with organized menu system
# For help with any target: make help-<target> (e.g., make help-build)

.DEFAULT_GOAL := menu
.PHONY: menu help help-categories build test clean dev rust check lint docs docs-build docs-serve docs-clean docs-install version setup status
.PHONY: quick-start full-workflow container-build container-test security-scan validate benchmark wasm-build
.PHONY: test-rust test-docs test-all docs-dev docs-sync docs-validate fix dev-full clean-all

# Configuration
RUST_VERSION ?= 1.75
CARGO_FEATURES ?= default

## ═══════════════════════════════════════════════════════════════════════════════
## 🦀 RUST DEVELOPMENT COMMANDS
## ═══════════════════════════════════════════════════════════════════════════════

check: ## 🔍 Check Rust code without building
	@echo "🔍 Checking Rust code..."
	cargo check --all-features
	@echo "✅ Check complete"

build: ## 🏗️ Build Rust CLI
	@echo "🏗️ Building Rust CLI..."
	cargo build
	@echo "✅ Build complete"

build-release: ## 🚀 Build optimized release version
	@echo "🏗️ Building release version..."
	cargo build --release
	@echo "✅ Release build complete"

test: ## 🧪 Run Rust tests
	@echo "🧪 Running Rust tests..."
	cargo test --all-features
	@echo "✅ Tests complete"

lint: ## 🔧 Run linting and formatting checks
	@echo "🔧 Running linting checks..."
	cargo clippy --all-features -- -D warnings
	cargo fmt --check
	@echo "✅ Linting complete"

fix: ## 🔨 Fix linting issues automatically
	@echo "🔧 Fixing code issues..."
	cargo clippy --all-features --fix --allow-dirty
	cargo fmt
	@echo "✅ Fixes applied"

## ═══════════════════════════════════════════════════════════════════════════════
## 📖 DOCUMENTATION COMMANDS
## ═══════════════════════════════════════════════════════════════════════════════

docs-install: ## 📦 Install documentation dependencies
	@echo "📦 Installing documentation dependencies..."
	cd docs-site && npm ci
	@echo "✅ Dependencies installed"

docs-build: ## 📚 Build documentation site
	@echo "📚 Building documentation..."
	./scripts/build-docs.sh
	@echo "✅ Documentation built"

docs-build-fast: ## ⚡ Build documentation quickly (skip CLI build and validation)
	@echo "📚 Building documentation (fast mode)..."
	./scripts/build-docs.sh --fast
	@echo "✅ Fast documentation build complete"

docs-serve: ## 🌐 Build and serve documentation locally
	@echo "📚 Building and serving documentation..."
	./scripts/build-docs.sh --serve
	@echo "🌐 Documentation serving at http://localhost:3000"

docs-dev: ## 🔄 Start documentation development server
	@echo "📚 Starting docs development server..."
	cd docs-site && npm run start
	@echo "🌐 Development server started"

docs-clean: ## 🧹 Clean documentation build files
	@echo "🧹 Cleaning documentation..."
	./scripts/build-docs.sh --clean
	@echo "✅ Documentation cleaned"

docs-sync: ## 🔄 Sync documentation content only
	@echo "🔄 Syncing documentation content..."
	node scripts/docs-sync.js --verbose
	@echo "✅ Content synchronized"

docs-validate: ## 🔍 Validate documentation links
	@echo "🔍 Validating documentation links..."
	node scripts/link-validator.js --external
	@echo "✅ Link validation complete"

## ═══════════════════════════════════════════════════════════════════════════════
## 🐳 CONTAINER OPERATIONS
## ═══════════════════════════════════════════════════════════════════════════════

test-versioning: ## 🧪 Test container versioning implementation
	@echo "🧪 Testing container versioning implementation..."
	@if [ -f "./scripts/test-versioning.sh" ]; then \
		./scripts/test-versioning.sh; \
	else \
		echo "⚠️ test-versioning.sh not found, skipping versioning tests"; \
	fi
	@echo "✅ Versioning tests complete"

monitor: ## 📊 Monitor container health
	@echo "📊 Monitoring container health..."
	@if [ -f "./scripts/container-health-monitor.sh" ]; then \
		./scripts/container-health-monitor.sh monitor; \
	else \
		echo "⚠️ container-health-monitor.sh not found, skipping monitoring"; \
	fi

container-build: ## 🐳 Build container image using build script
	@echo "🐳 Building container image..."
	@if [ -f "./scripts/build-automation.sh" ]; then \
		./scripts/build-automation.sh -e dev build; \
	else \
		echo "❌ build-automation.sh not found"; \
		exit 1; \
	fi

container-test: ## 🧪 Test container using build script
	@echo "🧪 Testing container..."
	@if [ -f "./scripts/build-automation.sh" ]; then \
		./scripts/build-automation.sh -e dev test; \
	else \
		echo "❌ build-automation.sh not found"; \
		exit 1; \
	fi

## ═══════════════════════════════════════════════════════════════════════════════
## 🚀 QUICK START & DEVELOPMENT WORKFLOWS
## ═══════════════════════════════════════════════════════════════════════════════

quick-start: ## 🚀 Complete setup for new contributors (recommended first command)
	@echo "🚀 PhotonDrift Quick Start for New Contributors"
	@echo "==============================================="
	@echo ""
	@echo "Step 1/4: Setting up development environment..."
	$(MAKE) setup
	@echo ""
	@echo "Step 2/4: Checking Rust code..."
	$(MAKE) check
	@echo ""
	@echo "Step 3/4: Installing documentation dependencies..."
	$(MAKE) docs-install
	@echo ""
	@echo "Step 4/4: Building documentation..."
	$(MAKE) docs-build-fast
	@echo ""
	@echo "🎉 Quick start complete! Try these next steps:"
	@echo "   make docs-serve    # View documentation at http://localhost:3000"
	@echo "   make test          # Run tests to ensure everything works"
	@echo "   make dev           # Quick development build"
	@echo ""

full-workflow: ## 🔄 Complete development workflow with all checks (CI equivalent)
	@echo "🔄 Running Full Development Workflow"
	@echo "===================================="
	@echo ""
	@echo "Phase 1: Environment setup..."
	$(MAKE) setup
	@echo ""
	@echo "Phase 2: Code validation..."
	$(MAKE) validate
	@echo ""
	@echo "Phase 3: Complete build and test..."
	$(MAKE) test
	$(MAKE) build-release
	@echo ""
	@echo "Phase 4: Documentation..."
	$(MAKE) docs-build
	@echo ""
	@echo "Phase 5: Security check..."
	$(MAKE) security-scan
	@echo ""
	@echo "✅ Full workflow complete - ready for production!"

dev: ## ⚡ Quick development build (Rust check + fast docs)
	@echo "⚡ Quick Development Build"
	@echo "========================="
	$(MAKE) check
	$(MAKE) docs-build-fast
	@echo "✅ Quick development build complete"

dev-full: ## 🔍 Full development build with tests and complete docs
	@echo "🔍 Full Development Build"
	@echo "========================"
	$(MAKE) test
	$(MAKE) docs-build
	@echo "✅ Full development build complete"

## ═══════════════════════════════════════════════════════════════════════════════
## 🔒 SECURITY & VALIDATION
## ═══════════════════════════════════════════════════════════════════════════════

security-scan: ## 🔒 Run security checks
	@echo "🔒 Running security scan..."
	@if [ -f "./scripts/security-check.sh" ]; then \
		./scripts/security-check.sh; \
	else \
		echo "⚠️ Security check script not found, running cargo audit instead"; \
		cargo audit; \
	fi

validate: ## ✅ Run all validation checks
	@echo "🔍 Running validation checks..."
	$(MAKE) check
	$(MAKE) lint
	$(MAKE) docs-validate
	@echo "✅ All validations passed"

## ═══════════════════════════════════════════════════════════════════════════════
## 🧹 CLEANUP & UTILITY COMMANDS  
## ═══════════════════════════════════════════════════════════════════════════════

clean: ## 🧹 Clean build artifacts (keeps dependencies)
	@echo "🧹 Cleaning build artifacts..."
	@echo "  • Rust build cache..."
	cargo clean
	@echo "  • Documentation build files..."
	rm -rf docs-site/build/
	rm -rf docs-site/.docusaurus/
	@echo "  • Temporary files..."
	rm -f link-validation-report.json
	@echo "✅ Build artifacts cleaned (dependencies preserved)"

clean-all: ## 🗑️ Deep clean everything (including dependencies)
	@echo "🗑️ Deep cleaning everything..."
	@echo "  • Rust build cache..."
	cargo clean
	@echo "  • Documentation files..."
	rm -rf docs-site/build/
	rm -rf docs-site/.docusaurus/
	rm -rf docs-site/node_modules/
	@echo "  • Temporary and cache files..."
	rm -f link-validation-report.json
	rm -rf target/
	rm -rf .swarm/
	@echo "  • Git cleanup..."
	git clean -fd
	@echo "⚠️  Deep clean complete - you'll need to run 'make setup' next"

setup: ## 🛠️ Setup development environment
	@echo "🔧 Setting up development environment..."
	@chmod +x scripts/*.sh scripts/*.js
	rustup default $(RUST_VERSION)
	rustup component add clippy rustfmt
	@if ! command -v node >/dev/null 2>&1; then \
		echo "⚠️ Node.js not found. Please install Node.js 18+"; \
	fi
	@echo "✅ Development environment ready"

version: ## ℹ️ Show version information
	@echo "PhotonDrift Build System"
	@echo "========================"
	@grep -E '^version = ' Cargo.toml | head -1 | cut -d'"' -f2 || echo "Version not found"
	@echo ""
	@echo "Environment:"
	@echo "  Rust: $$(rustc --version 2>/dev/null || echo 'Not installed')"
	@echo "  Cargo: $$(cargo --version 2>/dev/null || echo 'Not installed')"
	@echo "  Node.js: $$(node --version 2>/dev/null || echo 'Not installed')"
	@echo "  Docker: $$(docker --version 2>/dev/null | cut -d' ' -f3 | tr -d ',' || echo 'Not installed')"

status: ## 📊 Show project status
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

## ═══════════════════════════════════════════════════════════════════════════════
## 🧪 INDIVIDUAL TESTING COMMANDS
## ═══════════════════════════════════════════════════════════════════════════════

test-rust: ## 🦀 Test only Rust code
	cargo test --all-features

test-docs: ## 📚 Test documentation build
	$(MAKE) docs-build-fast

test-all: ## 🎯 Run all tests
	$(MAKE) test-rust
	$(MAKE) test-docs
	@echo "✅ All tests completed"

## ═══════════════════════════════════════════════════════════════════════════════
## ⚡ ADVANCED DEVELOPMENT COMMANDS
## ═══════════════════════════════════════════════════════════════════════════════

wasm-build: ## 🕸️ Build WebAssembly version
	@echo "🕸️ Building WebAssembly..."
	@if [ -f "./scripts/build-wasm-optimized.sh" ]; then \
		./scripts/build-wasm-optimized.sh; \
	else \
		echo "❌ WASM build script not found"; \
		exit 1; \
	fi

benchmark: ## ⚡ Run performance benchmarks
	@echo "⚡ Running benchmarks..."
	@if [ -f "./scripts/performance-benchmark.sh" ]; then \
		./scripts/performance-benchmark.sh; \
	else \
		echo "❌ Benchmark script not found"; \
		exit 1; \
	fi

## ═══════════════════════════════════════════════════════════════════════════════
## 📋 MAIN MENU & HELP SYSTEM
## ═══════════════════════════════════════════════════════════════════════════════

menu: ## 🏠 Show the main development menu (default target)
	@echo ""
	@echo "╔══════════════════════════════════════════════════════════════════════════════╗"
	@echo "║                          📋 PhotonDrift Developer Menu                      ║"
	@echo "╚══════════════════════════════════════════════════════════════════════════════╝"
	@echo ""
	@echo "🚀 QUICK START (New Contributors)"
	@echo "   make quick-start      # Complete setup + first build (recommended for new devs)"
	@echo "   make full-workflow    # Full development workflow with all checks"
	@echo ""
	@echo "⚡ COMMON WORKFLOWS"
	@echo "   make dev             # Quick development build (Rust check + fast docs)"
	@echo "   make dev-full        # Full development build (tests + complete docs)"
	@echo "   make docs-serve      # Build and serve documentation locally"
	@echo "   make validate        # Run all validation checks (lint, tests, docs)"
	@echo ""
	@echo "📚 DETAILED CATEGORIES"
	@echo "   make help-rust       # 🦀 Rust development commands"
	@echo "   make help-docs       # 📖 Documentation commands"
	@echo "   make help-container  # 🐳 Container operations"
	@echo "   make help-test       # 🧪 Testing commands"
	@echo "   make help-tools      # 🔧 Utility and development tools"
	@echo "   make help-advanced   # ⚡ Advanced features (WASM, benchmarks, etc.)"
	@echo ""
	@echo "ℹ️  OTHER COMMANDS"
	@echo "   make help            # Show traditional help with all targets"
	@echo "   make status          # Show current project status"
	@echo "   make version         # Show version and environment info"
	@echo ""
	@echo "💡 TIP: Run 'make help-<category>' for detailed command info"
	@echo "💡 TIP: For specific help on any target: make help-<target-name>"
	@echo ""

help: ## 📜 Show traditional help with all available targets
	@echo ""
	@echo "╔══════════════════════════════════════════════════════════════════════════════╗"
	@echo "║                      📜 PhotonDrift - All Available Targets                 ║"
	@echo "╚══════════════════════════════════════════════════════════════════════════════╝"
	@echo ""
	@awk 'BEGIN {FS = ":.*##"; printf "%-25s %s\n", "TARGET", "DESCRIPTION"} /^[a-zA-Z_-]+:.*?##/ { printf "\033[36m%-25s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST) | sort
	@echo ""
	@echo "💡 Use 'make menu' for organized categories"
	@echo "💡 Use 'make help-<category>' for focused help"

help-categories: ## 📂 Show available help categories
	@echo ""
	@echo "📂 Available Help Categories:"
	@echo "   🦀 help-rust       - Rust development commands"
	@echo "   📖 help-docs       - Documentation commands"  
	@echo "   🐳 help-container  - Container operations"
	@echo "   🧪 help-test       - Testing commands"
	@echo "   🔧 help-tools      - Utility and development tools"
	@echo "   ⚡ help-advanced   - Advanced features"
	@echo ""

help-rust: ## 🦀 Show Rust development commands
	@echo ""
	@echo "🦀 RUST DEVELOPMENT COMMANDS"
	@echo "══════════════════════════════"
	@echo ""
	@echo "🔍 Code Analysis:"
	@echo "   make check           # Check Rust code without building (fast)"
	@echo "   make lint            # Run linting and formatting checks"
	@echo "   make fix             # Auto-fix linting issues"
	@echo ""
	@echo "🏗️ Building:"
	@echo "   make build           # Build Rust CLI (debug mode)"
	@echo "   make build-release   # Build optimized release version"
	@echo ""
	@echo "🧪 Testing:"
	@echo "   make test            # Run all Rust tests with features"
	@echo "   make test-rust       # Run only Rust tests (alias)"
	@echo ""
	@echo "💡 Recommended workflow: check → lint → test → build"

help-docs: ## 📖 Show documentation commands
	@echo ""
	@echo "📖 DOCUMENTATION COMMANDS"
	@echo "═══════════════════════════"
	@echo ""
	@echo "🚀 Quick Actions:"
	@echo "   make docs-serve      # Build and serve docs locally (http://localhost:3000)"
	@echo "   make docs-dev        # Start development server (live reload)"
	@echo ""
	@echo "🏗️ Building:"
	@echo "   make docs-install    # Install documentation dependencies"
	@echo "   make docs-build      # Build complete documentation site"
	@echo "   make docs-build-fast # Build docs quickly (skip validation)"
	@echo ""
	@echo "🔧 Maintenance:"
	@echo "   make docs-clean      # Clean documentation build files"
	@echo "   make docs-sync       # Sync documentation content only"
	@echo "   make docs-validate   # Validate documentation links"
	@echo ""
	@echo "💡 First time? Run: docs-install → docs-serve"

help-container: ## 🐳 Show container operation commands
	@echo ""
	@echo "🐳 CONTAINER OPERATIONS"
	@echo "════════════════════════"
	@echo ""
	@echo "   make container-build # Build container image using build script"
	@echo "   make container-test  # Test container functionality"
	@echo ""
	@echo "💡 Requires: Docker installed and ./scripts/build-automation.sh"

help-test: ## 🧪 Show testing commands
	@echo ""
	@echo "🧪 TESTING COMMANDS"
	@echo "════════════════════"
	@echo ""
	@echo "🎯 Individual Tests:"
	@echo "   make test-rust       # Test only Rust code"
	@echo "   make test-docs       # Test documentation build"
	@echo ""
	@echo "🔍 Comprehensive Testing:"
	@echo "   make test            # Run all Rust tests (primary)"
	@echo "   make test-all        # Run all tests (Rust + docs)"
	@echo "   make validate        # Run all validation (tests + lint + docs)"
	@echo ""
	@echo "💡 Quick check: test-rust → validate for full confidence"

help-tools: ## 🔧 Show utility and development tool commands
	@echo ""
	@echo "🔧 UTILITY & DEVELOPMENT TOOLS"
	@echo "════════════════════════════════"
	@echo ""
	@echo "🛠️ Environment:"
	@echo "   make setup           # Setup complete development environment"
	@echo "   make clean           # Clean all build artifacts"
	@echo "   make clean-all       # Deep clean (includes node_modules)"
	@echo ""
	@echo "📊 Information:"
	@echo "   make status          # Show current project status"
	@echo "   make version         # Show version and environment info"
	@echo ""
	@echo "🔒 Security:"  
	@echo "   make security-scan   # Run security checks and audit"
	@echo ""
	@echo "💡 New contributors: start with 'make setup'"

help-advanced: ## ⚡ Show advanced feature commands
	@echo ""
	@echo "⚡ ADVANCED FEATURES"
	@echo "═════════════════════"
	@echo ""
	@echo "🕸️ WebAssembly:"
	@echo "   make wasm-build      # Build WebAssembly version"
	@echo ""
	@echo "⚡ Performance:"
	@echo "   make benchmark       # Run performance benchmarks"
	@echo ""
	@echo "💡 These require additional scripts in ./scripts/ directory"