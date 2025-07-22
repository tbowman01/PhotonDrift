#!/usr/bin/env bash
# Debug toolkit for PhotonDrift build system
# Comprehensive debugging and troubleshooting tools

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Configuration
DEBUG_LOG_DIR="${PROJECT_ROOT}/debug-logs"
DEBUG_REPORT_FILE="${DEBUG_LOG_DIR}/debug-report-$(date +%Y%m%d-%H%M%S).md"
CONTAINER_LOGS_DIR="${DEBUG_LOG_DIR}/container-logs"

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[$(date +'%H:%M:%S')]${NC} $*"; }
log_warn() { echo -e "${YELLOW}[$(date +'%H:%M:%S')]${NC} $*"; }
log_error() { echo -e "${RED}[$(date +'%H:%M:%S')]${NC} $*" >&2; }
log_debug() { echo -e "${BLUE}[$(date +'%H:%M:%S')]${NC} $*"; }
log_section() { echo -e "${CYAN}[$(date +'%H:%M:%S')] === $* ===${NC}"; }

show_usage() {
    cat << EOF
Usage: $0 [OPTIONS] COMMAND

Debug toolkit for PhotonDrift build system troubleshooting.

COMMANDS:
    diagnose        Run comprehensive diagnostics
    logs            Collect and analyze logs
    container       Debug container issues
    build           Debug build problems
    network         Network connectivity checks
    permissions     Check file permissions
    environment     Validate environment setup
    report          Generate debug report
    clean           Clean debug artifacts

OPTIONS:
    -v, --verbose       Verbose output
    -o, --output DIR    Output directory for logs
    -c, --container ID  Specific container to debug
    -h, --help          Show this help

EXAMPLES:
    $0 diagnose                    # Full system diagnostics
    $0 build                       # Debug build issues
    $0 container photondrift       # Debug specific container
    $0 logs --output ./debug       # Collect logs to custom dir
    $0 report                      # Generate comprehensive report

EOF
}

ensure_debug_dirs() {
    mkdir -p "$DEBUG_LOG_DIR" "$CONTAINER_LOGS_DIR"
}

check_system_requirements() {
    log_section "System Requirements Check"
    
    local issues=0
    
    # Check Docker
    if command -v docker >/dev/null 2>&1; then
        local docker_version=$(docker --version)
        log_info "✅ Docker: $docker_version"
        
        # Check Docker daemon
        if docker info >/dev/null 2>&1; then
            log_info "✅ Docker daemon is running"
        else
            log_error "❌ Docker daemon is not accessible"
            ((issues++))
        fi
        
        # Check Docker Buildx
        if docker buildx version >/dev/null 2>&1; then
            local buildx_version=$(docker buildx version)
            log_info "✅ Docker Buildx: $buildx_version"
        else
            log_warn "⚠️  Docker Buildx not available"
        fi
        
    else
        log_error "❌ Docker is not installed"
        ((issues++))
    fi
    
    # Check required tools
    for tool in jq bc curl; do
        if command -v "$tool" >/dev/null 2>&1; then
            log_info "✅ $tool is available"
        else
            log_warn "⚠️  $tool is not installed (recommended)"
        fi
    done
    
    # Check disk space
    local disk_usage=$(df "$PROJECT_ROOT" | tail -1 | awk '{print $5}' | sed 's/%//')
    if [[ $disk_usage -gt 90 ]]; then
        log_error "❌ Disk space critical: ${disk_usage}% used"
        ((issues++))
    elif [[ $disk_usage -gt 80 ]]; then
        log_warn "⚠️  Disk space low: ${disk_usage}% used"
    else
        log_info "✅ Disk space: ${disk_usage}% used"
    fi
    
    # Check memory
    if command -v free >/dev/null 2>&1; then
        local mem_usage=$(free | grep Mem | awk '{printf "%.0f", $3/$2 * 100.0}')
        if [[ $mem_usage -gt 90 ]]; then
            log_error "❌ Memory usage critical: ${mem_usage}%"
            ((issues++))
        elif [[ $mem_usage -gt 80 ]]; then
            log_warn "⚠️  Memory usage high: ${mem_usage}%"
        else
            log_info "✅ Memory usage: ${mem_usage}%"
        fi
    fi
    
    return $issues
}

check_project_structure() {
    log_section "Project Structure Validation"
    
    local issues=0
    local required_files=(
        "Cargo.toml"
        "src/main.rs"
        "src/lib.rs"
        "Dockerfile"
        "scripts/build-automation.sh"
        ".github/workflows/container-build.yml"
    )
    
    for file in "${required_files[@]}"; do
        if [[ -f "$PROJECT_ROOT/$file" ]]; then
            log_info "✅ $file exists"
        else
            log_error "❌ Missing required file: $file"
            ((issues++))
        fi
    done
    
    # Check script permissions
    for script in "$PROJECT_ROOT"/scripts/*.sh; do
        if [[ -f "$script" ]]; then
            if [[ -x "$script" ]]; then
                log_info "✅ $(basename "$script") is executable"
            else
                log_warn "⚠️  $(basename "$script") is not executable"
            fi
        fi
    done
    
    return $issues
}

check_docker_configuration() {
    log_section "Docker Configuration Check"
    
    local issues=0
    
    # Check Docker context
    local context=$(docker context show 2>/dev/null || echo "default")
    log_info "Docker context: $context"
    
    # Check available builders
    if docker buildx ls >/dev/null 2>&1; then
        log_info "Available builders:"
        docker buildx ls | while read -r line; do
            log_debug "  $line"
        done
    fi
    
    # Check Docker storage
    if docker system df >/dev/null 2>&1; then
        log_info "Docker storage usage:"
        docker system df | while read -r line; do
            log_debug "  $line"
        done
    fi
    
    # Check for running containers
    local running_containers=$(docker ps --format "{{.Names}}" | wc -l)
    log_info "Running containers: $running_containers"
    
    # Check for PhotonDrift images
    local photondrift_images=$(docker images | grep -E "(photondrift|adrscan)" | wc -l || echo "0")
    log_info "PhotonDrift images: $photondrift_images"
    
    return $issues
}

analyze_build_logs() {
    log_section "Build Log Analysis"
    
    local build_log="${DEBUG_LOG_DIR}/build-debug.log"
    
    # Run a test build with verbose output
    log_info "Running test build with verbose logging..."
    
    if ./scripts/build-automation.sh --verbose -e dev build > "$build_log" 2>&1; then
        log_info "✅ Test build completed successfully"
    else
        log_error "❌ Test build failed"
        
        # Analyze common build failures
        log_info "Analyzing build failure patterns..."
        
        if grep -q "permission denied" "$build_log"; then
            log_error "🔍 Permission denied error detected"
        fi
        
        if grep -q "No space left on device" "$build_log"; then
            log_error "🔍 Disk space issue detected"
        fi
        
        if grep -q "network timeout\|connection refused" "$build_log"; then
            log_error "🔍 Network connectivity issue detected"
        fi
        
        if grep -q "manifest unknown\|pull access denied" "$build_log"; then
            log_error "🔍 Registry access issue detected"
        fi
        
        # Show last 20 lines of build log
        log_error "Last 20 lines of build log:"
        tail -20 "$build_log" | while read -r line; do
            log_debug "  $line"
        done
    fi
}

debug_container_issues() {
    local container_name="$1"
    
    log_section "Container Debug: $container_name"
    
    # Check if container exists
    if docker ps -a --format "{{.Names}}" | grep -q "^${container_name}$"; then
        log_info "✅ Container '$container_name' exists"
        
        # Get container status
        local status=$(docker inspect --format='{{.State.Status}}' "$container_name")
        log_info "Container status: $status"
        
        # Get container logs
        local container_log="${CONTAINER_LOGS_DIR}/${container_name}.log"
        docker logs "$container_name" > "$container_log" 2>&1
        log_info "Container logs saved to: $container_log"
        
        # Check health status
        local health=$(docker inspect --format='{{.State.Health.Status}}' "$container_name" 2>/dev/null || echo "none")
        log_info "Health status: $health"
        
        # Get resource usage
        if [[ "$status" == "running" ]]; then
            log_info "Current resource usage:"
            docker stats --no-stream "$container_name" | while read -r line; do
                log_debug "  $line"
            done
        fi
        
        # Check port bindings
        local ports=$(docker port "$container_name" 2>/dev/null || echo "none")
        log_info "Port bindings: $ports"
        
        # Inspect container configuration
        docker inspect "$container_name" > "${CONTAINER_LOGS_DIR}/${container_name}-inspect.json"
        log_info "Container inspection saved to: ${CONTAINER_LOGS_DIR}/${container_name}-inspect.json"
        
    else
        log_error "❌ Container '$container_name' not found"
        
        # List available containers
        log_info "Available containers:"
        docker ps -a --format "table {{.Names}}\t{{.Image}}\t{{.Status}}" | while read -r line; do
            log_debug "  $line"
        done
    fi
}

check_network_connectivity() {
    log_section "Network Connectivity Check"
    
    local test_urls=(
        "https://ghcr.io"
        "https://registry-1.docker.io"
        "https://github.com"
        "https://api.github.com"
    )
    
    for url in "${test_urls[@]}"; do
        if curl -s --max-time 10 "$url" >/dev/null 2>&1; then
            log_info "✅ $url is reachable"
        else
            log_error "❌ $url is not reachable"
        fi
    done
    
    # Check DNS resolution
    if nslookup github.com >/dev/null 2>&1; then
        log_info "✅ DNS resolution working"
    else
        log_error "❌ DNS resolution issues"
    fi
    
    # Check Docker registry connectivity
    if docker pull hello-world >/dev/null 2>&1; then
        log_info "✅ Docker registry accessible"
        docker rmi hello-world >/dev/null 2>&1 || true
    else
        log_error "❌ Docker registry not accessible"
    fi
}

validate_environment() {
    log_section "Environment Validation"
    
    # Check environment variables
    local env_vars=(
        "PATH"
        "HOME"
        "USER"
        "DOCKER_BUILDKIT"
        "GITHUB_REPOSITORY"
        "GITHUB_TOKEN"
    )
    
    for var in "${env_vars[@]}"; do
        if [[ -n "${!var:-}" ]]; then
            if [[ "$var" == "GITHUB_TOKEN" ]]; then
                log_info "✅ $var is set (hidden)"
            else
                log_info "✅ $var: ${!var}"
            fi
        else
            if [[ "$var" == "GITHUB_TOKEN" || "$var" == "GITHUB_REPOSITORY" ]]; then
                log_debug "ℹ️  $var not set (optional for local development)"
            else
                log_warn "⚠️  $var not set"
            fi
        fi
    done
    
    # Check shell configuration
    log_info "Shell: $SHELL"
    log_info "PWD: $PWD"
    log_info "OS: $(uname -s)"
    log_info "Architecture: $(uname -m)"
}

check_file_permissions() {
    log_section "File Permissions Check"
    
    local files_to_check=(
        "scripts/build-automation.sh"
        "scripts/validate-dockerfile.sh"
        "scripts/validate-build-scripts.sh"
        "scripts/security-check.sh"
        "scripts/container-health-monitor.sh"
        "scripts/performance-benchmark.sh"
        "scripts/debug-toolkit.sh"
    )
    
    for file in "${files_to_check[@]}"; do
        local full_path="$PROJECT_ROOT/$file"
        if [[ -f "$full_path" ]]; then
            local perms=$(stat -c "%a" "$full_path" 2>/dev/null || stat -f "%A" "$full_path" 2>/dev/null || echo "unknown")
            if [[ -x "$full_path" ]]; then
                log_info "✅ $file ($perms) - executable"
            else
                log_warn "⚠️  $file ($perms) - not executable"
            fi
        else
            log_error "❌ $file - not found"
        fi
    done
}

collect_system_logs() {
    log_section "System Log Collection"
    
    local system_log="${DEBUG_LOG_DIR}/system-info.log"
    
    {
        echo "=== System Information ==="
        uname -a
        echo ""
        
        echo "=== Docker Information ==="
        docker info 2>/dev/null || echo "Docker not accessible"
        echo ""
        
        echo "=== Docker Version ==="
        docker version 2>/dev/null || echo "Docker not accessible"
        echo ""
        
        echo "=== Disk Usage ==="
        df -h
        echo ""
        
        echo "=== Memory Usage ==="
        free -h 2>/dev/null || echo "free command not available"
        echo ""
        
        echo "=== Process List ==="
        ps aux | head -20
        echo ""
        
        echo "=== Network Interfaces ==="
        ip addr show 2>/dev/null || ifconfig 2>/dev/null || echo "Network info not available"
        echo ""
        
    } > "$system_log"
    
    log_info "System information saved to: $system_log"
}

generate_debug_report() {
    log_section "Generating Debug Report"
    
    cat > "$DEBUG_REPORT_FILE" << EOF
# PhotonDrift Debug Report

Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")
Host: $(hostname)
User: $(whoami)

## Summary

This debug report contains comprehensive diagnostics for the PhotonDrift build system.

## System Information

- **OS**: $(uname -s) $(uname -r)
- **Architecture**: $(uname -m)
- **Shell**: $SHELL
- **Working Directory**: $PWD

## Docker Information

\`\`\`
$(docker version 2>/dev/null || echo "Docker not available")
\`\`\`

## Project Structure

\`\`\`
$(find "$PROJECT_ROOT" -maxdepth 2 -type f -name "*.toml" -o -name "*.yml" -o -name "*.yaml" -o -name "*.sh" -o -name "Dockerfile*" | sort)
\`\`\`

## Resource Usage

\`\`\`
$(df -h "$PROJECT_ROOT" 2>/dev/null || echo "Disk usage unavailable")
\`\`\`

\`\`\`
$(free -h 2>/dev/null || echo "Memory usage unavailable")
\`\`\`

## Docker Storage

\`\`\`
$(docker system df 2>/dev/null || echo "Docker storage info unavailable")
\`\`\`

## Available Images

\`\`\`
$(docker images --format "table {{.Repository}}\t{{.Tag}}\t{{.Size}}" | head -20 2>/dev/null || echo "Docker images unavailable")
\`\`\`

## Running Containers

\`\`\`
$(docker ps --format "table {{.Names}}\t{{.Image}}\t{{.Status}}\t{{.Ports}}" 2>/dev/null || echo "Docker containers unavailable")
\`\`\`

## Environment Variables

\`\`\`
PATH=$PATH
HOME=$HOME
DOCKER_BUILDKIT=${DOCKER_BUILDKIT:-not set}
\`\`\`

## File Permissions

| File | Permissions | Executable |
|------|-------------|------------|
$(for script in "$PROJECT_ROOT"/scripts/*.sh; do
    if [[ -f "$script" ]]; then
        local perms=$(stat -c "%a" "$script" 2>/dev/null || stat -f "%A" "$script" 2>/dev/null || echo "unknown")
        local executable=$([[ -x "$script" ]] && echo "✅" || echo "❌")
        echo "| $(basename "$script") | $perms | $executable |"
    fi
done)

## Recent Logs

$(if [[ -f "${DEBUG_LOG_DIR}/build-debug.log" ]]; then
    echo "### Build Log (last 30 lines)"
    echo "\`\`\`"
    tail -30 "${DEBUG_LOG_DIR}/build-debug.log" 2>/dev/null || echo "Build log unavailable"
    echo "\`\`\`"
fi)

## Recommendations

### Immediate Actions
- Review any ❌ items in the diagnostics above
- Ensure all required scripts are executable
- Verify Docker daemon is running and accessible
- Check available disk space and memory

### Performance Optimization
- Clean Docker cache if disk space is low: \`docker system prune\`
- Consider using Docker BuildKit for better performance
- Monitor resource usage during builds

### Security
- Verify file permissions are appropriate
- Ensure no secrets are exposed in logs
- Keep Docker and system packages updated

## Debug Files Generated

- Debug Report: $DEBUG_REPORT_FILE
- System Info: ${DEBUG_LOG_DIR}/system-info.log
- Build Log: ${DEBUG_LOG_DIR}/build-debug.log
- Container Logs: $CONTAINER_LOGS_DIR/

---
*Generated by debug-toolkit.sh*
EOF

    log_info "Debug report generated: $DEBUG_REPORT_FILE"
}

clean_debug_artifacts() {
    log_section "Cleaning Debug Artifacts"
    
    if [[ -d "$DEBUG_LOG_DIR" ]]; then
        local file_count=$(find "$DEBUG_LOG_DIR" -type f | wc -l)
        log_info "Removing $file_count debug files..."
        rm -rf "$DEBUG_LOG_DIR"
        log_info "✅ Debug artifacts cleaned"
    else
        log_info "No debug artifacts to clean"
    fi
}

main() {
    local command=""
    local verbose=false
    local output_dir="$DEBUG_LOG_DIR"
    local container_name=""
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -v|--verbose)
                verbose=true
                shift
                ;;
            -o|--output)
                output_dir="$2"
                DEBUG_LOG_DIR="$output_dir"
                DEBUG_REPORT_FILE="${output_dir}/debug-report-$(date +%Y%m%d-%H%M%S).md"
                CONTAINER_LOGS_DIR="${output_dir}/container-logs"
                shift 2
                ;;
            -c|--container)
                container_name="$2"
                shift 2
                ;;
            -h|--help)
                show_usage
                exit 0
                ;;
            diagnose|logs|container|build|network|permissions|environment|report|clean)
                command="$1"
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
    
    if [[ -z "$command" ]]; then
        log_error "No command specified"
        show_usage
        exit 1
    fi
    
    ensure_debug_dirs
    
    # Execute command
    case "$command" in
        diagnose)
            log_info "Running comprehensive diagnostics..."
            local total_issues=0
            
            check_system_requirements || ((total_issues += $?))
            check_project_structure || ((total_issues += $?))
            check_docker_configuration || ((total_issues += $?))
            validate_environment
            check_file_permissions
            
            if [[ $total_issues -eq 0 ]]; then
                log_info "🎉 No critical issues found!"
            else
                log_warn "⚠️  Found $total_issues critical issues that need attention"
            fi
            ;;
        logs)
            collect_system_logs
            analyze_build_logs
            ;;
        container)
            if [[ -n "$container_name" ]]; then
                debug_container_issues "$container_name"
            else
                log_error "Container name required. Use -c/--container option"
                exit 1
            fi
            ;;
        build)
            analyze_build_logs
            ;;
        network)
            check_network_connectivity
            ;;
        permissions)
            check_file_permissions
            ;;
        environment)
            validate_environment
            ;;
        report)
            check_system_requirements >/dev/null 2>&1 || true
            check_project_structure >/dev/null 2>&1 || true
            check_docker_configuration >/dev/null 2>&1 || true
            collect_system_logs
            generate_debug_report
            ;;
        clean)
            clean_debug_artifacts
            ;;
        *)
            log_error "Unknown command: $command"
            show_usage
            exit 1
            ;;
    esac
}

# Run only if called directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi