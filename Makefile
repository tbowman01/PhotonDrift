# PhotonDrift Build Automation Makefile
# Provides convenient shortcuts for common build operations

.DEFAULT_GOAL := help
.PHONY: help build test scan push clean dev staging prod all

# Configuration
SCRIPT := ./scripts/build-automation.sh
SERVICE ?= cli
ENVIRONMENT ?= dev
PLATFORMS ?= linux/amd64,linux/arm64

## Development shortcuts
dev: ## Quick development build (AMD64 only)
	$(SCRIPT) -e dev build

dev-test: ## Development build with tests
	$(SCRIPT) -e dev build test

## Staging builds
staging: ## Staging build with multi-platform
	$(SCRIPT) -e staging -p $(PLATFORMS) build

staging-all: ## Complete staging pipeline
	$(SCRIPT) -e staging -s all all

## Production builds
prod: ## Production build with full security
	$(SCRIPT) -e prod build

prod-all: ## Complete production pipeline for all services
	$(SCRIPT) -e prod -s all all

## Individual commands
build: ## Build container image
	$(SCRIPT) -s $(SERVICE) -e $(ENVIRONMENT) build

test: ## Run container tests
	$(SCRIPT) -s $(SERVICE) -e $(ENVIRONMENT) test

scan: ## Security scan containers
	$(SCRIPT) -s $(SERVICE) -e $(ENVIRONMENT) scan

push: ## Push images to registry
	$(SCRIPT) -s $(SERVICE) -e $(ENVIRONMENT) push

all: ## Complete pipeline (build + test + scan + push)
	$(SCRIPT) -s $(SERVICE) -e $(ENVIRONMENT) all

## Multi-service builds
build-all: ## Build all services
	$(SCRIPT) -s all -e $(ENVIRONMENT) build

test-all: ## Test all services
	$(SCRIPT) -s all -e $(ENVIRONMENT) test

## Cache management
clean-cache: ## Clear Docker build cache
	$(SCRIPT) --no-cache build

cache-build: ## Build with aggressive caching
	$(SCRIPT) --cache build

## Advanced tools
validate: ## Run pre-commit validation hooks
	@echo "Running validation checks..."
	@./scripts/validate-dockerfile.sh
	@./scripts/validate-build-scripts.sh
	@./scripts/security-check.sh
	@echo "✅ Validation complete"

test-versioning: ## Test container versioning implementation
	@echo "Testing container versioning implementation..."
	@./scripts/test-versioning.sh
	@echo "✅ Versioning tests complete"

monitor: ## Monitor container health
	@./scripts/container-health-monitor.sh monitor

benchmark: ## Run performance benchmarks
	@./scripts/performance-benchmark.sh full

debug: ## Run diagnostic tools
	@./scripts/debug-toolkit.sh diagnose

## Utility commands
setup: ## Setup build environment
	@echo "Setting up PhotonDrift build environment..."
	@chmod +x $(SCRIPT)
	@chmod +x scripts/*.sh
	@if ! docker buildx ls | grep -q "photondrift-builder"; then \
		docker buildx create --name photondrift-builder --driver docker-container --bootstrap; \
	fi
	@echo "✅ Build environment ready"

clean: ## Clean up Docker resources
	@echo "Cleaning up Docker resources..."
	@docker system prune -f
	@docker buildx prune -f
	@echo "✅ Cleanup complete"

version: ## Show version information
	@echo "PhotonDrift Build System"
	@echo "========================"
	@grep -E '^version = ' Cargo.toml | head -1 | cut -d'"' -f2 || echo "Version not found"
	@echo "Docker: $$(docker version --format '{{.Client.Version}}' 2>/dev/null || echo 'Not installed')"
	@echo "Buildx: $$(docker buildx version 2>/dev/null | head -1 | cut -d' ' -f2 || echo 'Not installed')"

## GitHub Actions simulation
act-build: ## Simulate GitHub Actions build locally
	act -j build

act-matrix: ## Simulate matrix build locally
	act workflow_dispatch -j build --input services=cli --input environments=dev

## Kubernetes/Helm shortcuts
k8s-deploy: ## Deploy to Kubernetes
	kubectl apply -f k8s/deployment.yaml

k8s-logs: ## View Kubernetes logs
	kubectl logs -n photondrift -l app.kubernetes.io/name=photondrift --tail=100

helm-install: ## Install via Helm
	helm install photondrift helm/photondrift

helm-upgrade: ## Upgrade Helm release
	helm upgrade photondrift helm/photondrift

## Docker Compose shortcuts
compose-build: ## Build all services with Docker Compose
	$(SCRIPT) -s all -e dev build
	docker-compose build

compose-up: ## Start the full development stack
	$(SCRIPT) -s all -e dev build
	docker-compose up -d

compose-down: ## Stop the development stack
	docker-compose down

compose-logs: ## View logs from all services
	docker-compose logs -f

## Quick workflows
quick: dev-test ## Quick development workflow (build + test)

local: ## Complete local development setup
	$(MAKE) setup
	$(MAKE) dev-test
	@echo "✅ Local development environment ready"

ci: ## Simulate CI/CD pipeline locally
	$(MAKE) setup
	$(SCRIPT) -e staging all
	@echo "✅ CI simulation complete"

## Help target
help: ## Show this help message
	@echo "PhotonDrift Build Automation"
	@echo "============================="
	@echo ""
	@echo "Usage: make [target] [SERVICE=service] [ENVIRONMENT=env] [PLATFORMS=platforms]"
	@echo ""
	@echo "Examples:"
	@echo "  make dev                          # Quick development build"
	@echo "  make staging SERVICE=all          # Build all services for staging"
	@echo "  make prod ENVIRONMENT=prod        # Production build with security"
	@echo "  make build SERVICE=cli            # Build specific service"
	@echo ""
	@echo "Available targets:"
	@echo ""
	@awk 'BEGIN {FS = ":.*##"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 }' $(MAKEFILE_LIST)
	@echo ""
	@echo "Configuration:"
	@echo "  SERVICE     Service to build (cli, dashboard-backend, dashboard-frontend, all)"
	@echo "  ENVIRONMENT Target environment (dev, staging, prod)"
	@echo "  PLATFORMS   Build platforms (default: linux/amd64,linux/arm64)"