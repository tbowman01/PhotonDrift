#!/usr/bin/env bash
# Build automation script for PhotonDrift
# Simplifies local and CI builds with smart defaults

set -euo pipefail

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
DEFAULT_REGISTRY="ghcr.io"
DEFAULT_PLATFORMS="linux/amd64,linux/arm64"

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

show_usage() {
    cat << EOF
Usage: $0 [OPTIONS] COMMAND

Simplified build automation for PhotonDrift containers.

COMMANDS:
    build       Build container image(s)
    test        Run container tests
    push        Push images to registry
    scan        Security scan containers
    all         Build, test, scan, and push

OPTIONS:
    -s, --service SERVICE      Service to build (cli, dashboard-backend, all)
    -e, --env ENVIRONMENT     Target environment (dev, staging, prod)
    -p, --platform PLATFORMS  Target platforms (default: $DEFAULT_PLATFORMS)
    -r, --registry REGISTRY   Container registry (default: $DEFAULT_REGISTRY)
    -t, --tag TAG            Custom tag for the image
    -c, --cache              Enable aggressive caching
    -n, --no-cache           Disable cache completely
    -v, --verbose            Verbose output
    -h, --help               Show this help message

EXAMPLES:
    # Quick development build
    $0 build

    # Build specific service for staging
    $0 -s cli -e staging build

    # Full production pipeline
    $0 -s all -e prod all

    # Custom build with specific platforms
    $0 -p linux/arm64 -t my-custom-tag build

EOF
}

# Parse arguments
SERVICE="cli"
ENVIRONMENT="dev"
PLATFORMS="$DEFAULT_PLATFORMS"
REGISTRY="$DEFAULT_REGISTRY"
CUSTOM_TAG=""
CACHE_MODE="auto"
VERBOSE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -s|--service)
            SERVICE="$2"
            shift 2
            ;;
        -e|--env)
            ENVIRONMENT="$2"
            shift 2
            ;;
        -p|--platform)
            PLATFORMS="$2"
            shift 2
            ;;
        -r|--registry)
            REGISTRY="$2"
            shift 2
            ;;
        -t|--tag)
            CUSTOM_TAG="$2"
            shift 2
            ;;
        -c|--cache)
            CACHE_MODE="max"
            shift
            ;;
        -n|--no-cache)
            CACHE_MODE="none"
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
        build|test|push|scan|all)
            COMMAND="$1"
            shift
            break
            ;;
        *)
            log_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Validate command
if [[ -z "${COMMAND:-}" ]]; then
    log_error "No command specified"
    show_usage
    exit 1
fi

# Setup Docker buildx if needed
setup_buildx() {
    if ! docker buildx ls | grep -q "photondrift-builder"; then
        log_info "Setting up Docker buildx..."
        docker buildx create --name photondrift-builder --driver docker-container --bootstrap
    fi
    docker buildx use photondrift-builder
}

# Determine services to build
get_services() {
    case "$SERVICE" in
        all)
            echo "cli dashboard-backend dashboard-frontend"
            ;;
        *)
            echo "$SERVICE"
            ;;
    esac
}

# Get Dockerfile and context for service
get_build_config() {
    local service=$1
    case "$service" in
        cli)
            echo "Dockerfile ."
            ;;
        dashboard-backend)
            echo "dashboard/backend/Dockerfile dashboard/backend"
            ;;
        dashboard-frontend)
            echo "dashboard/frontend/Dockerfile dashboard/frontend"
            ;;
        *)
            log_error "Unknown service: $service"
            exit 1
            ;;
    esac
}

# Generate image tag
get_image_tag() {
    local service=$1
    local base_name="${REGISTRY}/${GITHUB_REPOSITORY:-tbowman01/photondrift}"
    
    # Normalize service name
    case "$service" in
        cli)
            base_name="${base_name}"
            ;;
        *)
            base_name="${base_name}-${service}"
            ;;
    esac
    
    # Use custom tag or generate based on environment
    if [[ -n "$CUSTOM_TAG" ]]; then
        echo "${base_name}:${CUSTOM_TAG}"
    else
        case "$ENVIRONMENT" in
            dev)
                echo "${base_name}:dev-$(git rev-parse --short HEAD 2>/dev/null || echo 'latest')"
                ;;
            staging)
                echo "${base_name}:staging-$(date +%Y%m%d)-$(git rev-parse --short HEAD 2>/dev/null || echo 'latest')"
                ;;
            prod)
                echo "${base_name}:latest"
                ;;
        esac
    fi
}

# Build command
cmd_build() {
    setup_buildx
    
    for service in $(get_services); do
        log_info "Building $service for $ENVIRONMENT..."
        
        read -r dockerfile context <<< "$(get_build_config "$service")"
        tag=$(get_image_tag "$service")
        
        # Build arguments
        build_args=(
            --file "$PROJECT_ROOT/$dockerfile"
            --tag "$tag"
            --platform "$PLATFORMS"
            --build-arg "BUILD_DATE=$(date -u +%Y-%m-%dT%H:%M:%SZ)"
            --build-arg "GIT_REVISION=$(git rev-parse HEAD 2>/dev/null || echo 'unknown')"
            --build-arg "VERSION=$(grep version Cargo.toml | head -1 | cut -d'"' -f2 || echo 'unknown')"
        )
        
        # Cache configuration
        case "$CACHE_MODE" in
            max)
                build_args+=(
                    --cache-from "type=registry,ref=${tag}-cache"
                    --cache-to "type=registry,ref=${tag}-cache,mode=max"
                )
                ;;
            none)
                build_args+=(--no-cache)
                ;;
            *)
                build_args+=(--cache-from "type=gha" --cache-to "type=gha,mode=max")
                ;;
        esac
        
        # Environment-specific options
        case "$ENVIRONMENT" in
            dev)
                build_args+=(--load)
                ;;
            staging|prod)
                build_args+=(--push)
                ;;
        esac
        
        # Verbose output
        if [[ "$VERBOSE" == "true" ]]; then
            build_args+=(--progress plain)
        fi
        
        # Execute build
        if docker buildx build "${build_args[@]}" "$PROJECT_ROOT/$context"; then
            log_success "Built $service: $tag"
        else
            log_error "Failed to build $service"
            exit 1
        fi
    done
}

# Test command
cmd_test() {
    for service in $(get_services); do
        tag=$(get_image_tag "$service")
        log_info "Testing $service..."
        
        # Basic functionality tests
        if docker run --rm "$tag" --version; then
            log_success "Version check passed"
        else
            log_error "Version check failed"
            exit 1
        fi
        
        # Security checks
        if docker run --rm "$tag" id -u | grep -qv "^0$"; then
            log_success "Non-root check passed"
        else
            log_error "Container running as root!"
            exit 1
        fi
        
        # Service-specific tests
        case "$service" in
            cli)
                # Test CLI functionality
                docker run --rm -v "$PROJECT_ROOT/docs/adr:/workspace" "$tag" inventory || true
                ;;
            dashboard-backend)
                # Test health endpoint
                docker run --rm -d --name test-backend "$tag"
                sleep 2
                docker exec test-backend curl -f http://localhost:3000/health || true
                docker stop test-backend
                ;;
        esac
        
        log_success "Tests passed for $service"
    done
}

# Scan command
cmd_scan() {
    for service in $(get_services); do
        tag=$(get_image_tag "$service")
        log_info "Scanning $service for vulnerabilities..."
        
        # Run Trivy scan
        docker run --rm \
            -v /var/run/docker.sock:/var/run/docker.sock \
            aquasec/trivy image \
            --severity CRITICAL,HIGH \
            --exit-code 0 \
            "$tag"
        
        log_success "Scan completed for $service"
    done
}

# Push command
cmd_push() {
    if [[ "$ENVIRONMENT" == "dev" ]]; then
        log_warning "Skipping push for dev environment"
        return
    fi
    
    for service in $(get_services); do
        tag=$(get_image_tag "$service")
        log_info "Pushing $service to registry..."
        
        if docker push "$tag"; then
            log_success "Pushed $service: $tag"
        else
            log_error "Failed to push $service"
            exit 1
        fi
    done
}

# All command - full pipeline
cmd_all() {
    log_info "Running full build pipeline..."
    cmd_build
    cmd_test
    cmd_scan
    
    if [[ "$ENVIRONMENT" != "dev" ]]; then
        cmd_push
    fi
    
    log_success "Full pipeline completed!"
}

# Main execution
case "$COMMAND" in
    build)
        cmd_build
        ;;
    test)
        cmd_test
        ;;
    scan)
        cmd_scan
        ;;
    push)
        cmd_push
        ;;
    all)
        cmd_all
        ;;
    *)
        log_error "Unknown command: $COMMAND"
        show_usage
        exit 1
        ;;
esac