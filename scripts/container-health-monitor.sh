#!/usr/bin/env bash
# Container health monitoring and metrics collection
# Provides comprehensive monitoring for PhotonDrift containers

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Configuration
MONITOR_INTERVAL=${MONITOR_INTERVAL:-30}
METRICS_FILE="${PROJECT_ROOT}/container-metrics.json"
ALERT_THRESHOLD_CPU=80
ALERT_THRESHOLD_MEMORY=85
ALERT_THRESHOLD_DISK=90

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info() { echo -e "${GREEN}[$(date +'%H:%M:%S')]${NC} $*"; }
log_warn() { echo -e "${YELLOW}[$(date +'%H:%M:%S')]${NC} $*"; }
log_error() { echo -e "${RED}[$(date +'%H:%M:%S')]${NC} $*" >&2; }
log_debug() { echo -e "${BLUE}[$(date +'%H:%M:%S')]${NC} $*"; }

show_usage() {
    cat << EOF
Usage: $0 [OPTIONS] COMMAND

Container health monitoring and metrics for PhotonDrift.

COMMANDS:
    monitor     Start continuous monitoring
    check       One-time health check
    metrics     Show current metrics
    alerts      Check for active alerts
    report      Generate health report
    cleanup     Clean up old metrics

OPTIONS:
    -i, --interval SECONDS    Monitoring interval (default: $MONITOR_INTERVAL)
    -f, --file FILE          Metrics output file (default: $METRICS_FILE)
    -v, --verbose            Verbose output
    -h, --help               Show this help

EXAMPLES:
    $0 monitor                # Start continuous monitoring
    $0 check                  # Quick health check
    $0 metrics --verbose      # Detailed metrics
    $0 report                 # Generate health report

EOF
}

get_container_stats() {
    local container_name="$1"
    
    if ! docker ps --format "{{.Names}}" | grep -q "^${container_name}$"; then
        echo "null"
        return
    fi
    
    # Get container stats in JSON format
    local stats=$(docker stats --no-stream --format "table {{.Container}}\t{{.CPUPerc}}\t{{.MemUsage}}\t{{.MemPerc}}\t{{.NetIO}}\t{{.BlockIO}}" "$container_name" 2>/dev/null || echo "")
    
    if [[ -z "$stats" ]]; then
        echo "null"
        return
    fi
    
    # Parse stats (this is a simplified version - in production, you'd want more robust parsing)
    local cpu_percent=$(echo "$stats" | tail -1 | awk '{print $2}' | sed 's/%//')
    local mem_usage=$(echo "$stats" | tail -1 | awk '{print $3}')
    local mem_percent=$(echo "$stats" | tail -1 | awk '{print $4}' | sed 's/%//')
    local net_io=$(echo "$stats" | tail -1 | awk '{print $5}')
    local block_io=$(echo "$stats" | tail -1 | awk '{print $6}')
    
    # Health check status
    local health_status=$(docker inspect --format='{{.State.Health.Status}}' "$container_name" 2>/dev/null || echo "none")
    
    # Container uptime
    local started_at=$(docker inspect --format='{{.State.StartedAt}}' "$container_name" 2>/dev/null || echo "unknown")
    local uptime="unknown"
    if [[ "$started_at" != "unknown" ]]; then
        uptime=$(( $(date +%s) - $(date -d "$started_at" +%s 2>/dev/null || echo 0) ))
    fi
    
    # Generate JSON
    cat << EOF
{
  "container": "$container_name",
  "timestamp": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
  "status": "$(docker inspect --format='{{.State.Status}}' "$container_name" 2>/dev/null || echo "unknown")",
  "health": "$health_status",
  "uptime_seconds": $uptime,
  "cpu_percent": ${cpu_percent:-0},
  "memory_usage": "$mem_usage",
  "memory_percent": ${mem_percent:-0},
  "network_io": "$net_io",
  "block_io": "$block_io",
  "alerts": []
}
EOF
}

check_container_health() {
    local container_name="$1"
    local stats_json="$2"
    local alerts=()
    
    # Parse JSON stats
    local cpu_percent=$(echo "$stats_json" | jq -r '.cpu_percent // 0')
    local mem_percent=$(echo "$stats_json" | jq -r '.memory_percent // 0')
    local health_status=$(echo "$stats_json" | jq -r '.health // "none"')
    local status=$(echo "$stats_json" | jq -r '.status // "unknown"')
    
    # Check CPU usage
    if (( $(echo "$cpu_percent > $ALERT_THRESHOLD_CPU" | bc -l 2>/dev/null || echo 0) )); then
        alerts+=("High CPU usage: ${cpu_percent}%")
    fi
    
    # Check memory usage
    if (( $(echo "$mem_percent > $ALERT_THRESHOLD_MEMORY" | bc -l 2>/dev/null || echo 0) )); then
        alerts+=("High memory usage: ${mem_percent}%")
    fi
    
    # Check health status
    if [[ "$health_status" == "unhealthy" ]]; then
        alerts+=("Container health check failed")
    fi
    
    # Check if container is running
    if [[ "$status" != "running" ]]; then
        alerts+=("Container not running: $status")
    fi
    
    # Update alerts in JSON
    local alerts_json=$(printf '%s\n' "${alerts[@]}" | jq -R . | jq -s .)
    echo "$stats_json" | jq ".alerts = $alerts_json"
}

discover_photondrift_containers() {
    # Find PhotonDrift-related containers
    docker ps --format "{{.Names}}" | grep -E "(photondrift|adrscan)" || true
    
    # Also check for containers from our images
    docker ps --filter "ancestor=ghcr.io/tbowman01/photondrift" --format "{{.Names}}" || true
}

collect_metrics() {
    local containers=$(discover_photondrift_containers)
    local all_metrics=()
    
    if [[ -z "$containers" ]]; then
        log_warn "No PhotonDrift containers found"
        echo "[]" > "$METRICS_FILE"
        return
    fi
    
    log_info "Collecting metrics for containers: $(echo "$containers" | tr '\n' ' ')"
    
    while IFS= read -r container; do
        [[ -z "$container" ]] && continue
        
        log_debug "Getting stats for $container..."
        local stats=$(get_container_stats "$container")
        
        if [[ "$stats" != "null" ]]; then
            local health_check=$(check_container_health "$container" "$stats")
            all_metrics+=("$health_check")
        fi
    done <<< "$containers"
    
    # Save metrics to file
    printf '%s\n' "${all_metrics[@]}" | jq -s . > "$METRICS_FILE"
    
    log_info "Metrics saved to $METRICS_FILE"
}

show_metrics() {
    if [[ ! -f "$METRICS_FILE" ]]; then
        log_warn "No metrics file found. Run 'monitor' or 'check' first."
        return 1
    fi
    
    local metrics=$(cat "$METRICS_FILE")
    local container_count=$(echo "$metrics" | jq 'length')
    
    echo "📊 PhotonDrift Container Metrics"
    echo "================================="
    echo "Containers monitored: $container_count"
    echo "Last updated: $(echo "$metrics" | jq -r '.[0].timestamp // "unknown"')"
    echo ""
    
    echo "$metrics" | jq -r '.[] | 
        "🐳 \(.container)
        Status: \(.status) | Health: \(.health)
        CPU: \(.cpu_percent)% | Memory: \(.memory_percent)%
        Uptime: \(.uptime_seconds)s | Network: \(.network_io)
        Alerts: \(if .alerts | length > 0 then (.alerts | join(", ")) else "None" end)
        "'
}

check_alerts() {
    if [[ ! -f "$METRICS_FILE" ]]; then
        log_warn "No metrics file found. Run 'monitor' or 'check' first."
        return 1
    fi
    
    local metrics=$(cat "$METRICS_FILE")
    local active_alerts=$(echo "$metrics" | jq -r '.[] | select(.alerts | length > 0) | .container + ": " + (.alerts | join(", "))')
    
    if [[ -n "$active_alerts" ]]; then
        log_error "🚨 Active Alerts:"
        echo "$active_alerts"
        return 1
    else
        log_info "✅ No active alerts"
        return 0
    fi
}

generate_health_report() {
    local report_file="${PROJECT_ROOT}/container-health-report.md"
    
    if [[ ! -f "$METRICS_FILE" ]]; then
        log_warn "No metrics file found. Run 'monitor' or 'check' first."
        return 1
    fi
    
    local metrics=$(cat "$METRICS_FILE")
    
    cat > "$report_file" << EOF
# PhotonDrift Container Health Report

Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")

## Summary

$(echo "$metrics" | jq -r 'length') containers monitored

### Container Status
$(echo "$metrics" | jq -r '.[] | "- **\(.container)**: \(.status) (Health: \(.health))"')

### Resource Usage
| Container | CPU % | Memory % | Uptime | Alerts |
|-----------|-------|----------|---------|---------|
$(echo "$metrics" | jq -r '.[] | "| \(.container) | \(.cpu_percent)% | \(.memory_percent)% | \(.uptime_seconds)s | \(if .alerts | length > 0 then (.alerts | length | tostring) else "0" end) |"')

### Active Alerts
$(if echo "$metrics" | jq -e '.[] | select(.alerts | length > 0)' >/dev/null 2>&1; then
    echo "$metrics" | jq -r '.[] | select(.alerts | length > 0) | "#### \(.container)\n" + (.alerts | map("- " + .) | join("\n"))'
else
    echo "✅ No active alerts"
fi)

## Recommendations

### Performance Optimization
- Monitor containers with CPU usage > 70%
- Consider scaling containers with memory usage > 80%
- Check network I/O patterns for optimization opportunities

### Security
- Ensure all containers are running with health checks
- Verify non-root user execution
- Regular security updates for base images

### Monitoring
- Set up automated alerting for critical thresholds
- Implement log aggregation for better debugging
- Consider APM tools for application-level metrics

---
*Generated by container-health-monitor.sh*
EOF

    log_info "Health report generated: $report_file"
}

start_monitoring() {
    log_info "Starting continuous monitoring (interval: ${MONITOR_INTERVAL}s)"
    log_info "Press Ctrl+C to stop"
    
    local iteration=0
    
    while true; do
        ((iteration++))
        log_info "Monitoring iteration $iteration"
        
        collect_metrics
        
        # Check for alerts and log them
        if ! check_alerts >/dev/null 2>&1; then
            log_warn "Alerts detected - check 'alerts' command for details"
        fi
        
        sleep "$MONITOR_INTERVAL"
    done
}

cleanup_old_metrics() {
    local days=${1:-7}
    log_info "Cleaning up metrics older than $days days..."
    
    # In a real implementation, you'd clean up timestamped files
    # For now, just archive the current metrics
    if [[ -f "$METRICS_FILE" ]]; then
        local archive_name="container-metrics-$(date +%Y%m%d).json"
        mv "$METRICS_FILE" "${PROJECT_ROOT}/metrics-archive-$archive_name"
        log_info "Archived current metrics to metrics-archive-$archive_name"
    fi
    
    log_info "Cleanup completed"
}

main() {
    local command=""
    local verbose=false
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -i|--interval)
                MONITOR_INTERVAL="$2"
                shift 2
                ;;
            -f|--file)
                METRICS_FILE="$2"
                shift 2
                ;;
            -v|--verbose)
                verbose=true
                shift
                ;;
            -h|--help)
                show_usage
                exit 0
                ;;
            monitor|check|metrics|alerts|report|cleanup)
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
    
    # Check dependencies
    if ! command -v docker >/dev/null 2>&1; then
        log_error "Docker is required but not installed"
        exit 1
    fi
    
    if ! command -v jq >/dev/null 2>&1; then
        log_error "jq is required but not installed"
        exit 1
    fi
    
    # Execute command
    case "$command" in
        monitor)
            start_monitoring
            ;;
        check)
            collect_metrics
            show_metrics
            ;;
        metrics)
            show_metrics
            ;;
        alerts)
            check_alerts
            ;;
        report)
            generate_health_report
            ;;
        cleanup)
            cleanup_old_metrics "${1:-7}"
            ;;
        *)
            log_error "Unknown command: $command"
            show_usage
            exit 1
            ;;
    esac
}

# Handle Ctrl+C gracefully
trap 'log_info "Monitoring stopped"; exit 0' INT

# Run only if called directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi