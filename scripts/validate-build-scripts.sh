#!/usr/bin/env bash
# Build script validation
# Validates shell scripts and GitHub Actions workflows

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

validate_shell_script() {
    local script="$1"
    local errors=0
    
    log_info "Validating shell script: $script"
    
    # Check shebang
    if ! head -1 "$script" | grep -q "^#!/"; then
        log_error "❌ Missing shebang in $script"
        ((errors++))
    else
        log_info "✅ Shebang found"
    fi
    
    # Check for set -e (exit on error)
    if ! grep -q "set -e" "$script"; then
        log_warn "⚠️  Consider adding 'set -e' for error handling"
    else
        log_info "✅ Error handling (set -e) found"
    fi
    
    # Check for set -u (exit on undefined variable)
    if ! grep -q "set -u" "$script"; then
        log_warn "⚠️  Consider adding 'set -u' for undefined variable protection"
    else
        log_info "✅ Undefined variable protection (set -u) found"
    fi
    
    # Check for proper quoting
    if grep -qE '\$[A-Za-z_][A-Za-z0-9_]*[^}]' "$script" && ! grep -q 'shellcheck disable' "$script"; then
        log_warn "⚠️  Check variable quoting - consider using \"\$VAR\" instead of \$VAR"
    fi
    
    # Check for executable permission
    if [[ ! -x "$script" ]]; then
        log_error "❌ Script is not executable: $script"
        ((errors++))
    else
        log_info "✅ Script is executable"
    fi
    
    # Run shellcheck if available
    if command -v shellcheck >/dev/null 2>&1; then
        log_info "Running shellcheck..."
        if shellcheck "$script"; then
            log_info "✅ Shellcheck passed"
        else
            log_error "❌ Shellcheck failed"
            ((errors++))
        fi
    else
        log_warn "⚠️  Shellcheck not available - consider installing for better validation"
    fi
    
    return $errors
}

validate_github_workflow() {
    local workflow="$1"
    local errors=0
    
    log_info "Validating GitHub workflow: $workflow"
    
    # Check YAML syntax
    if command -v yq >/dev/null 2>&1; then
        if yq eval '.' "$workflow" >/dev/null 2>&1; then
            log_info "✅ YAML syntax is valid"
        else
            log_error "❌ Invalid YAML syntax"
            ((errors++))
        fi
    elif command -v python3 >/dev/null 2>&1; then
        if python3 -c "import yaml; yaml.safe_load(open('$workflow'))" 2>/dev/null; then
            log_info "✅ YAML syntax is valid"
        else
            log_error "❌ Invalid YAML syntax"
            ((errors++))
        fi
    fi
    
    # Check for required workflow elements
    if ! grep -q "^name:" "$workflow"; then
        log_warn "⚠️  Consider adding workflow name"
    fi
    
    if ! grep -q "^on:" "$workflow"; then
        log_error "❌ Missing workflow triggers (on:)"
        ((errors++))
    fi
    
    if ! grep -q "^jobs:" "$workflow"; then
        log_error "❌ Missing jobs section"
        ((errors++))
    fi
    
    # Security checks
    if grep -q "secrets\." "$workflow" && ! grep -q "secrets: inherit"; then
        log_info "✅ Secrets usage detected - ensure proper handling"
    fi
    
    # Check for pinned action versions
    local unpinned_actions=$(grep -E "uses:.*@[^v]" "$workflow" | grep -v "@main\|@master" || true)
    if [[ -n "$unpinned_actions" ]]; then
        log_warn "⚠️  Consider pinning action versions to specific tags"
        echo "$unpinned_actions"
    fi
    
    # Check for permissions
    if ! grep -q "permissions:" "$workflow"; then
        log_warn "⚠️  Consider specifying explicit permissions"
    else
        log_info "✅ Permissions specified"
    fi
    
    # Container workflow specific checks
    if [[ "$workflow" == *"container"* ]]; then
        if ! grep -q "docker/build-push-action" "$workflow"; then
            log_warn "⚠️  Container workflow should use docker/build-push-action"
        fi
        
        if ! grep -q "platforms:" "$workflow"; then
            log_warn "⚠️  Consider specifying build platforms"
        fi
        
        if ! grep -q "cache-from\|cache-to" "$workflow"; then
            log_warn "⚠️  Consider adding build cache for performance"
        fi
    fi
    
    return $errors
}

validate_makefile() {
    local makefile="$1"
    local errors=0
    
    if [[ ! -f "$makefile" ]]; then
        return 0
    fi
    
    log_info "Validating Makefile: $makefile"
    
    # Check for .PHONY declarations
    if ! grep -q "^\.PHONY:" "$makefile"; then
        log_warn "⚠️  Consider adding .PHONY declarations for non-file targets"
    else
        log_info "✅ .PHONY declarations found"
    fi
    
    # Check for help target
    if ! grep -q "^help:" "$makefile"; then
        log_warn "⚠️  Consider adding a help target"
    else
        log_info "✅ Help target found"
    fi
    
    # Check for proper variable usage
    if grep -qE '\$[A-Za-z_][A-Za-z0-9_]*[^})]' "$makefile"; then
        log_warn "⚠️  Consider using \$(VAR) instead of \$VAR in Makefiles"
    fi
    
    return $errors
}

main() {
    local total_errors=0
    
    log_info "Starting build script validation..."
    
    # Validate shell scripts
    find "$PROJECT_ROOT/scripts" -name "*.sh" -type f 2>/dev/null | while read -r script; do
        validate_shell_script "$script" || ((total_errors += $?))
    done
    
    # Validate GitHub workflows
    find "$PROJECT_ROOT/.github/workflows" -name "*.yml" -o -name "*.yaml" 2>/dev/null | while read -r workflow; do
        validate_github_workflow "$workflow" || ((total_errors += $?))
    done
    
    # Validate Makefile
    if [[ -f "$PROJECT_ROOT/Makefile" ]]; then
        validate_makefile "$PROJECT_ROOT/Makefile" || ((total_errors += $?))
    fi
    
    # Validate build automation script specifically
    if [[ -f "$PROJECT_ROOT/scripts/build-automation.sh" ]]; then
        log_info "Validating build automation script..."
        
        # Check for required functions
        if ! grep -q "show_usage" "$PROJECT_ROOT/scripts/build-automation.sh"; then
            log_warn "⚠️  Build script should have usage documentation"
        fi
        
        # Check for error handling
        if ! grep -q "set -euo pipefail" "$PROJECT_ROOT/scripts/build-automation.sh"; then
            log_warn "⚠️  Build script should use strict error handling"
        fi
        
        # Check for configuration validation
        if ! grep -q "ENVIRONMENT.*dev\|staging\|prod" "$PROJECT_ROOT/scripts/build-automation.sh"; then
            log_warn "⚠️  Build script should validate environment values"
        fi
    fi
    
    # Summary
    if [[ $total_errors -eq 0 ]]; then
        log_info "🎉 All build script validations passed!"
        exit 0
    else
        log_error "❌ Build script validation failed with $total_errors errors"
        exit 1
    fi
}

# Run only if called directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi