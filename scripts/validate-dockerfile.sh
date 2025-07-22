#!/usr/bin/env bash
# Dockerfile validation script
# Validates Dockerfile best practices and security configurations

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $*"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*" >&2; }

validate_dockerfile() {
    local dockerfile="$1"
    local errors=0
    
    log_info "Validating $dockerfile..."
    
    # Check if file exists
    if [[ ! -f "$dockerfile" ]]; then
        log_error "Dockerfile not found: $dockerfile"
        return 1
    fi
    
    # Security checks
    log_info "Running security checks..."
    
    # Check for non-root user
    if ! grep -q "USER.*[^0]" "$dockerfile"; then
        log_error "❌ Dockerfile should specify non-root USER"
        ((errors++))
    else
        log_info "✅ Non-root USER found"
    fi
    
    # Check for HEALTHCHECK
    if ! grep -q "HEALTHCHECK" "$dockerfile"; then
        log_warn "⚠️  Consider adding HEALTHCHECK instruction"
    else
        log_info "✅ HEALTHCHECK found"
    fi
    
    # Check for pinned base image versions
    if grep -q "FROM.*:latest" "$dockerfile"; then
        log_error "❌ Avoid using ':latest' tag for base images"
        ((errors++))
    else
        log_info "✅ Base image versions are pinned"
    fi
    
    # Check for security labels
    if ! grep -q "org.opencontainers.image" "$dockerfile"; then
        log_warn "⚠️  Consider adding OCI-compliant labels"
    else
        log_info "✅ OCI labels found"
    fi
    
    # Multi-stage build check
    if grep -q "FROM.*AS" "$dockerfile"; then
        log_info "✅ Multi-stage build detected"
    else
        log_warn "⚠️  Consider using multi-stage builds for optimization"
    fi
    
    # Check for secrets exposure
    if grep -qE "(PASSWORD|SECRET|KEY|TOKEN).*=" "$dockerfile"; then
        log_error "❌ Potential secrets in Dockerfile"
        ((errors++))
    else
        log_info "✅ No obvious secrets found"
    fi
    
    # Performance checks
    log_info "Running performance checks..."
    
    # Check for layer optimization
    local copy_count=$(grep -c "^COPY\|^ADD" "$dockerfile" || true)
    if [[ $copy_count -gt 5 ]]; then
        log_warn "⚠️  Consider consolidating COPY/ADD instructions ($copy_count found)"
    fi
    
    # Check for cache-friendly ordering
    if grep -n "COPY.*\." "$dockerfile" | head -1 | grep -q "^[1-5]:"; then
        log_warn "⚠️  Consider copying dependency files before source code for better caching"
    else
        log_info "✅ Cache-friendly layer ordering"
    fi
    
    return $errors
}

validate_docker_compose() {
    local compose_file="$1"
    local errors=0
    
    if [[ ! -f "$compose_file" ]]; then
        return 0  # Not required
    fi
    
    log_info "Validating $compose_file..."
    
    # Check for version specification
    if ! grep -q "^version:" "$compose_file"; then
        log_warn "⚠️  Consider specifying compose file version"
    fi
    
    # Check for health checks in services
    if ! grep -q "healthcheck:" "$compose_file"; then
        log_warn "⚠️  Consider adding health checks to services"
    fi
    
    return $errors
}

main() {
    local total_errors=0
    
    log_info "Starting Dockerfile validation..."
    
    # Validate all Dockerfiles
    for dockerfile in "$PROJECT_ROOT"/Dockerfile*; do
        if [[ -f "$dockerfile" ]]; then
            validate_dockerfile "$dockerfile" || ((total_errors += $?))
        fi
    done
    
    # Validate dashboard Dockerfiles
    find "$PROJECT_ROOT" -name "Dockerfile*" -not -path "$PROJECT_ROOT/Dockerfile*" | while read -r dockerfile; do
        validate_dockerfile "$dockerfile" || ((total_errors += $?))
    done
    
    # Validate docker-compose files if they exist
    for compose in "$PROJECT_ROOT"/docker-compose*.yml "$PROJECT_ROOT"/docker-compose*.yaml; do
        if [[ -f "$compose" ]]; then
            validate_docker_compose "$compose" || ((total_errors += $?))
        fi
    done
    
    # Summary
    if [[ $total_errors -eq 0 ]]; then
        log_info "🎉 All Dockerfile validations passed!"
        exit 0
    else
        log_error "❌ Dockerfile validation failed with $total_errors errors"
        exit 1
    fi
}

# Run only if called directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi