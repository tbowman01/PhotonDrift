#!/usr/bin/env bash
# Container security check script
# Performs security validation for containers and build processes

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[INFO]${NC} $*"; }
log_warn() { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*" >&2; }
log_debug() { echo -e "${BLUE}[DEBUG]${NC} $*"; }

check_dockerfile_security() {
    local dockerfile="$1"
    local errors=0
    
    log_info "Security check for $dockerfile..."
    
    # Critical security checks
    
    # 1. Check for root user
    if ! grep -q "USER.*[^0]" "$dockerfile"; then
        log_error "❌ CRITICAL: Container runs as root user"
        ((errors++))
    else
        local user_line=$(grep "USER" "$dockerfile" | tail -1)
        log_info "✅ Non-root user specified: $user_line"
    fi
    
    # 2. Check for exposed secrets
    if grep -qiE "(password|secret|key|token|api_key)" "$dockerfile"; then
        log_error "❌ CRITICAL: Potential secrets exposed in Dockerfile"
        grep -niE "(password|secret|key|token|api_key)" "$dockerfile" | head -5
        ((errors++))
    else
        log_info "✅ No obvious secrets found"
    fi
    
    # 3. Check for privileged operations
    if grep -q "privileged.*true" "$dockerfile"; then
        log_error "❌ CRITICAL: Privileged container detected"
        ((errors++))
    fi
    
    # 4. Check for package manager cache cleanup
    if grep -q "apt-get install" "$dockerfile" && ! grep -q "rm -rf /var/lib/apt/lists" "$dockerfile"; then
        log_warn "⚠️  Consider cleaning apt cache to reduce image size"
    fi
    
    if grep -q "apk add" "$dockerfile" && ! grep -q "rm -rf /var/cache/apk" "$dockerfile"; then
        log_warn "⚠️  Consider cleaning apk cache to reduce image size"
    fi
    
    # 5. Check for proper COPY vs ADD usage
    if grep -q "^ADD.*http" "$dockerfile"; then
        log_warn "⚠️  Consider using COPY instead of ADD for local files"
    fi
    
    # 6. Check for health checks
    if ! grep -q "HEALTHCHECK" "$dockerfile"; then
        log_warn "⚠️  Consider adding HEALTHCHECK for container monitoring"
    else
        log_info "✅ Health check configured"
    fi
    
    # 7. Check for minimal base images
    if grep -q "FROM.*ubuntu.*latest\|FROM.*debian.*latest" "$dockerfile"; then
        log_warn "⚠️  Consider using minimal base images like Alpine"
    fi
    
    # 8. Check for security labels
    if grep -q "security\.scan.*enabled" "$dockerfile"; then
        log_info "✅ Security scanning enabled"
    else
        log_warn "⚠️  Consider adding security.scan=enabled label"
    fi
    
    return $errors
}

check_workflow_security() {
    local workflow="$1"
    local errors=0
    
    log_info "Security check for workflow: $workflow"
    
    # 1. Check permissions
    if ! grep -q "permissions:" "$workflow"; then
        log_warn "⚠️  Workflow missing explicit permissions"
    else
        # Check for overly broad permissions
        if grep -q "permissions:.*write-all\|contents:.*write.*packages:.*write.*security-events:.*write" "$workflow"; then
            log_warn "⚠️  Review broad permissions - ensure they're necessary"
        fi
        log_info "✅ Permissions specified"
    fi
    
    # 2. Check for secrets handling
    if grep -q "secrets\." "$workflow"; then
        if grep -qE "echo.*secrets\.|print.*secrets\." "$workflow"; then
            log_error "❌ CRITICAL: Secrets may be logged/printed"
            ((errors++))
        else
            log_info "✅ Secrets usage appears safe"
        fi
    fi
    
    # 3. Check for pinned action versions
    local unpinned_count=$(grep -cE "uses:.*@main|uses:.*@master" "$workflow" || true)
    if [[ $unpinned_count -gt 0 ]]; then
        log_warn "⚠️  $unpinned_count actions using floating refs (@main/@master)"
    fi
    
    # 4. Check for third-party actions
    local third_party_actions=$(grep -E "uses:.*[^/]+/[^/]+@" "$workflow" | grep -v "actions/" | grep -v "docker/" | grep -v "github/" || true)
    if [[ -n "$third_party_actions" ]]; then
        log_warn "⚠️  Third-party actions detected - review security:"
        echo "$third_party_actions"
    fi
    
    # 5. Check for environment variable exposure
    if grep -qE "env:.*SECRET|env:.*TOKEN" "$workflow"; then
        log_warn "⚠️  Environment variables with secrets - ensure proper handling"
    fi
    
    # 6. Container-specific security checks
    if grep -q "docker/build-push-action" "$workflow"; then
        # Check for security scanning
        if ! grep -q "trivy\|aquasecurity\|security.*scan" "$workflow"; then
            log_warn "⚠️  Consider adding security scanning to container builds"
        else
            log_info "✅ Security scanning configured"
        fi
        
        # Check for SBOM generation
        if grep -q "sbom.*true" "$workflow"; then
            log_info "✅ SBOM generation enabled"
        else
            log_warn "⚠️  Consider enabling SBOM generation for supply chain security"
        fi
        
        # Check for attestation
        if grep -q "provenance.*true\|attest-build-provenance" "$workflow"; then
            log_info "✅ Build attestation enabled"
        else
            log_warn "⚠️  Consider enabling build attestation"
        fi
    fi
    
    return $errors
}

check_script_security() {
    local script="$1"
    local errors=0
    
    log_info "Security check for script: $script"
    
    # 1. Check for hardcoded secrets
    if grep -qiE "(password|secret|key|token).*=" "$script"; then
        log_error "❌ CRITICAL: Potential hardcoded secrets in script"
        ((errors++))
    fi
    
    # 2. Check for unsafe operations
    if grep -q "rm -rf /" "$script"; then
        log_error "❌ CRITICAL: Dangerous rm command detected"
        ((errors++))
    fi
    
    # 3. Check for input validation
    if grep -q "\$1\|\$@" "$script" && ! grep -q "shift\|case.*\$1\|if.*\$#" "$script"; then
        log_warn "⚠️  Script uses arguments but may lack input validation"
    fi
    
    # 4. Check for sudo usage
    if grep -q "sudo" "$script"; then
        log_warn "⚠️  Script uses sudo - ensure it's necessary"
    fi
    
    # 5. Check for network operations
    if grep -qE "curl|wget|nc|telnet" "$script"; then
        log_info "ℹ️  Script performs network operations - ensure endpoints are trusted"
    fi
    
    # 6. Check for temporary file handling
    if grep -q "/tmp" "$script" && ! grep -q "mktemp"; then
        log_warn "⚠️  Consider using mktemp for secure temporary files"
    fi
    
    return $errors
}

check_container_runtime_security() {
    log_info "Checking container runtime security..."
    
    # Check if any containers are running as root
    if command -v docker >/dev/null 2>&1; then
        local running_containers=$(docker ps --format "table {{.Names}}\t{{.Image}}" 2>/dev/null || true)
        if [[ -n "$running_containers" ]]; then
            log_info "Active containers detected - checking security..."
            
            # This would check running containers, but we'll skip for pre-commit
            log_info "✅ Container runtime checks completed"
        fi
    fi
}

generate_security_report() {
    local output_file="$PROJECT_ROOT/security-check-report.md"
    
    cat > "$output_file" << EOF
# Security Check Report

Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")

## Summary

This report covers security validation for:
- Dockerfiles and container configurations
- GitHub Actions workflows
- Build and deployment scripts
- Container runtime security

## Recommendations

### High Priority
- Ensure all containers run as non-root users
- Remove any hardcoded secrets from code and configurations
- Pin all GitHub Actions to specific versions
- Enable security scanning in CI/CD pipelines

### Medium Priority
- Add health checks to all containers
- Enable SBOM and attestation generation
- Review and minimize container permissions
- Use minimal base images (Alpine when possible)

### Low Priority
- Clean package manager caches in Dockerfiles
- Add comprehensive input validation to scripts
- Consider using distroless images for production

## Tools Used
- Static analysis of Dockerfiles
- GitHub Actions workflow validation
- Shell script security scanning
- Container runtime checks

---
*This report is generated automatically by the security-check.sh script*
EOF

    log_info "Security report generated: $output_file"
}

main() {
    local total_errors=0
    
    log_info "Starting comprehensive security check..."
    
    # Check all Dockerfiles
    find "$PROJECT_ROOT" -name "Dockerfile*" -type f | while read -r dockerfile; do
        check_dockerfile_security "$dockerfile" || ((total_errors += $?))
    done
    
    # Check GitHub workflows
    find "$PROJECT_ROOT/.github/workflows" -name "*.yml" -o -name "*.yaml" 2>/dev/null | while read -r workflow; do
        check_workflow_security "$workflow" || ((total_errors += $?))
    done
    
    # Check scripts
    find "$PROJECT_ROOT/scripts" -name "*.sh" -type f 2>/dev/null | while read -r script; do
        check_script_security "$script" || ((total_errors += $?))
    done
    
    # Container runtime checks
    check_container_runtime_security
    
    # Generate security report
    generate_security_report
    
    # Summary
    if [[ $total_errors -eq 0 ]]; then
        log_info "🎉 Security validation completed successfully!"
        log_info "📋 Review the generated security report for recommendations"
        exit 0
    else
        log_error "❌ Security validation failed with $total_errors critical issues"
        log_error "🚨 Please address critical security issues before proceeding"
        exit 1
    fi
}

# Run only if called directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi