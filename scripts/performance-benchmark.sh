#!/usr/bin/env bash
# Performance benchmarking for PhotonDrift containers and builds
# Measures build times, container performance, and resource usage

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"

# Configuration
BENCHMARK_RESULTS_DIR="${PROJECT_ROOT}/benchmark-results"
BENCHMARK_FILE="${BENCHMARK_RESULTS_DIR}/benchmark-$(date +%Y%m%d-%H%M%S).json"
BASELINE_FILE="${BENCHMARK_RESULTS_DIR}/baseline.json"

# Test configurations
BUILD_ITERATIONS=3
PERFORMANCE_ITERATIONS=5
CONTAINER_TEST_DURATION=30

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

Performance benchmarking for PhotonDrift build system.

COMMANDS:
    build       Benchmark build times
    container   Benchmark container performance
    full        Run complete benchmark suite
    compare     Compare with baseline
    baseline    Set current results as baseline
    report      Generate performance report

OPTIONS:
    -i, --iterations N    Number of test iterations (default: 3)
    -d, --duration N      Container test duration in seconds (default: 30)
    -o, --output FILE     Output file for results
    -v, --verbose         Verbose output
    -h, --help            Show this help

EXAMPLES:
    $0 build                    # Benchmark build times
    $0 container               # Benchmark container performance
    $0 full                    # Complete benchmark suite
    $0 compare                 # Compare with baseline
    $0 baseline                # Set baseline from latest results

EOF
}

ensure_benchmark_dir() {
    mkdir -p "$BENCHMARK_RESULTS_DIR"
}

measure_build_time() {
    local build_type="$1"
    local iterations="$2"
    local results=()
    
    log_info "Benchmarking $build_type build ($iterations iterations)..."
    
    for i in $(seq 1 "$iterations"); do
        log_debug "Build iteration $i/$iterations"
        
        # Clean previous build
        docker system prune -f >/dev/null 2>&1 || true
        
        local start_time=$(date +%s.%3N)
        
        case "$build_type" in
            "cold")
                # Cold build - no cache
                ./scripts/build-automation.sh --no-cache -e dev build >/dev/null 2>&1
                ;;
            "warm")
                # Warm build - with cache
                ./scripts/build-automation.sh --cache -e dev build >/dev/null 2>&1
                ;;
            "optimized")
                # Optimized Dockerfile
                docker build -f Dockerfile.optimized -t photondrift:benchmark . >/dev/null 2>&1
                ;;
        esac
        
        local end_time=$(date +%s.%3N)
        local duration=$(echo "$end_time - $start_time" | bc -l)
        
        results+=("$duration")
        log_debug "Iteration $i: ${duration}s"
    done
    
    # Calculate statistics
    local total=0
    local min=${results[0]}
    local max=${results[0]}
    
    for time in "${results[@]}"; do
        total=$(echo "$total + $time" | bc -l)
        if (( $(echo "$time < $min" | bc -l) )); then
            min=$time
        fi
        if (( $(echo "$time > $max" | bc -l) )); then
            max=$time
        fi
    done
    
    local avg=$(echo "scale=3; $total / ${#results[@]}" | bc -l)
    
    cat << EOF
{
  "type": "$build_type",
  "iterations": ${#results[@]},
  "times": [$(IFS=','; echo "${results[*]}")],
  "average": $avg,
  "min": $min,
  "max": $max,
  "total": $total
}
EOF
}

measure_container_performance() {
    local duration="$1"
    
    log_info "Benchmarking container performance (${duration}s test)..."
    
    # Start container
    local container_name="photondrift-benchmark-$$"
    docker run -d --name "$container_name" photondrift:benchmark tail -f /dev/null >/dev/null
    
    # Wait for container to stabilize
    sleep 2
    
    local start_time=$(date +%s)
    local end_time=$((start_time + duration))
    local samples=()
    
    log_debug "Collecting performance samples for ${duration}s..."
    
    while [[ $(date +%s) -lt $end_time ]]; do
        # Get container stats
        local stats=$(docker stats --no-stream --format "{{.CPUPerc}},{{.MemUsage}},{{.NetIO}},{{.BlockIO}}" "$container_name" 2>/dev/null || echo "0.00%,0B / 0B,0B / 0B,0B / 0B")
        
        local cpu_percent=$(echo "$stats" | cut -d',' -f1 | sed 's/%//')
        local mem_usage=$(echo "$stats" | cut -d',' -f2 | awk '{print $1}' | sed 's/[^0-9.]//g')
        local net_io=$(echo "$stats" | cut -d',' -f3)
        local block_io=$(echo "$stats" | cut -d',' -f4)
        
        samples+=("{\"timestamp\": $(date +%s), \"cpu_percent\": $cpu_percent, \"memory_mb\": ${mem_usage:-0}, \"net_io\": \"$net_io\", \"block_io\": \"$block_io\"}")
        
        sleep 1
    done
    
    # Cleanup
    docker stop "$container_name" >/dev/null 2>&1
    docker rm "$container_name" >/dev/null 2>&1
    
    # Calculate averages
    local avg_cpu=0
    local avg_memory=0
    local sample_count=${#samples[@]}
    
    if [[ $sample_count -gt 0 ]]; then
        for sample in "${samples[@]}"; do
            local cpu=$(echo "$sample" | jq -r '.cpu_percent')
            local mem=$(echo "$sample" | jq -r '.memory_mb')
            avg_cpu=$(echo "$avg_cpu + $cpu" | bc -l)
            avg_memory=$(echo "$avg_memory + $mem" | bc -l)
        done
        
        avg_cpu=$(echo "scale=2; $avg_cpu / $sample_count" | bc -l)
        avg_memory=$(echo "scale=2; $avg_memory / $sample_count" | bc -l)
    fi
    
    cat << EOF
{
  "duration": $duration,
  "samples": $sample_count,
  "average_cpu_percent": $avg_cpu,
  "average_memory_mb": $avg_memory,
  "detailed_samples": [$(IFS=','; echo "${samples[*]}")]
}
EOF
}

benchmark_resource_usage() {
    log_info "Benchmarking resource usage during build..."
    
    # Start monitoring
    local monitor_pid=""
    local resource_log="/tmp/resource-usage-$$.log"
    
    # Monitor system resources during build
    (
        while true; do
            local timestamp=$(date +%s)
            local cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | sed 's/%us,//')
            local mem_usage=$(free | grep Mem | awk '{printf "%.1f", $3/$2 * 100.0}')
            local disk_usage=$(df "$PROJECT_ROOT" | tail -1 | awk '{print $5}' | sed 's/%//')
            
            echo "$timestamp,$cpu_usage,$mem_usage,$disk_usage" >> "$resource_log"
            sleep 2
        done
    ) &
    monitor_pid=$!
    
    # Run build while monitoring
    local build_start=$(date +%s.%3N)
    ./scripts/build-automation.sh -e dev build >/dev/null 2>&1
    local build_end=$(date +%s.%3N)
    local build_duration=$(echo "$build_end - $build_start" | bc -l)
    
    # Stop monitoring
    kill $monitor_pid 2>/dev/null || true
    wait $monitor_pid 2>/dev/null || true
    
    # Analyze resource usage
    local max_cpu=0
    local max_memory=0
    local avg_cpu=0
    local avg_memory=0
    local sample_count=0
    
    if [[ -f "$resource_log" ]]; then
        while IFS=',' read -r timestamp cpu mem disk; do
            if [[ -n "$cpu" && -n "$mem" ]]; then
                if (( $(echo "$cpu > $max_cpu" | bc -l) )); then
                    max_cpu=$cpu
                fi
                if (( $(echo "$mem > $max_memory" | bc -l) )); then
                    max_memory=$mem
                fi
                avg_cpu=$(echo "$avg_cpu + $cpu" | bc -l)
                avg_memory=$(echo "$avg_memory + $mem" | bc -l)
                ((sample_count++))
            fi
        done < "$resource_log"
        
        if [[ $sample_count -gt 0 ]]; then
            avg_cpu=$(echo "scale=2; $avg_cpu / $sample_count" | bc -l)
            avg_memory=$(echo "scale=2; $avg_memory / $sample_count" | bc -l)
        fi
    fi
    
    rm -f "$resource_log"
    
    cat << EOF
{
  "build_duration": $build_duration,
  "max_cpu_percent": $max_cpu,
  "max_memory_percent": $max_memory,
  "average_cpu_percent": $avg_cpu,
  "average_memory_percent": $avg_memory,
  "samples": $sample_count
}
EOF
}

run_build_benchmarks() {
    local iterations="$1"
    log_info "Running build benchmarks..."
    
    local cold_build=$(measure_build_time "cold" "$iterations")
    local warm_build=$(measure_build_time "warm" "$iterations")
    local optimized_build=$(measure_build_time "optimized" "$iterations")
    local resource_usage=$(benchmark_resource_usage)
    
    cat << EOF
{
  "cold_build": $cold_build,
  "warm_build": $warm_build,
  "optimized_build": $optimized_build,
  "resource_usage": $resource_usage
}
EOF
}

run_container_benchmarks() {
    local duration="$1"
    log_info "Running container benchmarks..."
    
    # Ensure we have a test image
    if ! docker images | grep -q "photondrift:benchmark"; then
        log_info "Building benchmark image..."
        ./scripts/build-automation.sh -e dev build >/dev/null 2>&1
        docker tag photondrift:dev photondrift:benchmark
    fi
    
    local container_performance=$(measure_container_performance "$duration")
    
    cat << EOF
{
  "container_performance": $container_performance
}
EOF
}

generate_benchmark_report() {
    local results_file="$1"
    local report_file="${BENCHMARK_RESULTS_DIR}/performance-report-$(date +%Y%m%d-%H%M%S).md"
    
    if [[ ! -f "$results_file" ]]; then
        log_error "Results file not found: $results_file"
        return 1
    fi
    
    local results=$(cat "$results_file")
    
    cat > "$report_file" << EOF
# PhotonDrift Performance Benchmark Report

Generated: $(date -u +"%Y-%m-%d %H:%M:%S UTC")  
Results file: $(basename "$results_file")

## Build Performance

### Cold Build (No Cache)
$(if echo "$results" | jq -e '.build_benchmarks.cold_build' >/dev/null 2>&1; then
    local cold=$(echo "$results" | jq '.build_benchmarks.cold_build')
    echo "- **Average**: $(echo "$cold" | jq -r '.average')s"
    echo "- **Best**: $(echo "$cold" | jq -r '.min')s"
    echo "- **Worst**: $(echo "$cold" | jq -r '.max')s"
    echo "- **Iterations**: $(echo "$cold" | jq -r '.iterations')"
else
    echo "Not tested"
fi)

### Warm Build (With Cache)
$(if echo "$results" | jq -e '.build_benchmarks.warm_build' >/dev/null 2>&1; then
    local warm=$(echo "$results" | jq '.build_benchmarks.warm_build')
    echo "- **Average**: $(echo "$warm" | jq -r '.average')s"
    echo "- **Best**: $(echo "$warm" | jq -r '.min')s"
    echo "- **Worst**: $(echo "$warm" | jq -r '.max')s"
    echo "- **Iterations**: $(echo "$warm" | jq -r '.iterations')"
    
    # Calculate improvement
    if echo "$results" | jq -e '.build_benchmarks.cold_build' >/dev/null 2>&1; then
        local cold_avg=$(echo "$results" | jq -r '.build_benchmarks.cold_build.average')
        local warm_avg=$(echo "$warm" | jq -r '.average')
        local improvement=$(echo "scale=1; ($cold_avg - $warm_avg) / $cold_avg * 100" | bc -l)
        echo "- **Cache Improvement**: ${improvement}% faster"
    fi
else
    echo "Not tested"
fi)

### Optimized Build
$(if echo "$results" | jq -e '.build_benchmarks.optimized_build' >/dev/null 2>&1; then
    local opt=$(echo "$results" | jq '.build_benchmarks.optimized_build')
    echo "- **Average**: $(echo "$opt" | jq -r '.average')s"
    echo "- **Best**: $(echo "$opt" | jq -r '.min')s"
    echo "- **Worst**: $(echo "$opt" | jq -r '.max')s"
    echo "- **Iterations**: $(echo "$opt" | jq -r '.iterations')"
else
    echo "Not tested"
fi)

## Resource Usage During Build

$(if echo "$results" | jq -e '.build_benchmarks.resource_usage' >/dev/null 2>&1; then
    local resource=$(echo "$results" | jq '.build_benchmarks.resource_usage')
    echo "- **Build Duration**: $(echo "$resource" | jq -r '.build_duration')s"
    echo "- **Peak CPU**: $(echo "$resource" | jq -r '.max_cpu_percent')%"
    echo "- **Peak Memory**: $(echo "$resource" | jq -r '.max_memory_percent')%"
    echo "- **Average CPU**: $(echo "$resource" | jq -r '.average_cpu_percent')%"
    echo "- **Average Memory**: $(echo "$resource" | jq -r '.average_memory_percent')%"
else
    echo "Not measured"
fi)

## Container Performance

$(if echo "$results" | jq -e '.container_benchmarks.container_performance' >/dev/null 2>&1; then
    local container=$(echo "$results" | jq '.container_benchmarks.container_performance')
    echo "- **Test Duration**: $(echo "$container" | jq -r '.duration')s"
    echo "- **Average CPU**: $(echo "$container" | jq -r '.average_cpu_percent')%"
    echo "- **Average Memory**: $(echo "$container" | jq -r '.average_memory_mb')MB"
    echo "- **Samples**: $(echo "$container" | jq -r '.samples')"
else
    echo "Not tested"
fi)

## Environment

- **OS**: $(uname -s)
- **Architecture**: $(uname -m)
- **Docker Version**: $(docker --version 2>/dev/null || echo "Not available")
- **BuildKit**: $(docker buildx version 2>/dev/null | head -1 || echo "Not available")

## Recommendations

### Performance Optimization
$(if echo "$results" | jq -e '.build_benchmarks.warm_build.average' >/dev/null 2>&1; then
    local warm_time=$(echo "$results" | jq -r '.build_benchmarks.warm_build.average')
    if (( $(echo "$warm_time > 300" | bc -l) )); then
        echo "- ⚠️  Build time > 5 minutes - consider optimizing Dockerfile layers"
    else
        echo "- ✅ Build times are acceptable"
    fi
fi)

$(if echo "$results" | jq -e '.build_benchmarks.resource_usage.max_memory_percent' >/dev/null 2>&1; then
    local max_mem=$(echo "$results" | jq -r '.build_benchmarks.resource_usage.max_memory_percent')
    if (( $(echo "$max_mem > 80" | bc -l) )); then
        echo "- ⚠️  High memory usage during build - consider reducing parallel jobs"
    else
        echo "- ✅ Memory usage is acceptable"
    fi
fi)

### Container Optimization
$(if echo "$results" | jq -e '.container_benchmarks.container_performance.average_cpu_percent' >/dev/null 2>&1; then
    local avg_cpu=$(echo "$results" | jq -r '.container_benchmarks.container_performance.average_cpu_percent')
    if (( $(echo "$avg_cpu > 10" | bc -l) )); then
        echo "- ⚠️  High idle CPU usage - investigate background processes"
    else
        echo "- ✅ Container CPU usage is efficient"
    fi
fi)

---
*Generated by performance-benchmark.sh*
EOF

    log_info "Performance report generated: $report_file"
    echo "$report_file"
}

compare_with_baseline() {
    local current_file="$1"
    
    if [[ ! -f "$BASELINE_FILE" ]]; then
        log_warn "No baseline file found. Run 'baseline' command first."
        return 1
    fi
    
    if [[ ! -f "$current_file" ]]; then
        log_error "Current results file not found: $current_file"
        return 1
    fi
    
    log_info "Comparing with baseline..."
    
    local baseline=$(cat "$BASELINE_FILE")
    local current=$(cat "$current_file")
    
    # Compare build times
    if echo "$baseline" | jq -e '.build_benchmarks.warm_build.average' >/dev/null 2>&1 && \
       echo "$current" | jq -e '.build_benchmarks.warm_build.average' >/dev/null 2>&1; then
        
        local baseline_time=$(echo "$baseline" | jq -r '.build_benchmarks.warm_build.average')
        local current_time=$(echo "$current" | jq -r '.build_benchmarks.warm_build.average')
        local change=$(echo "scale=1; ($current_time - $baseline_time) / $baseline_time * 100" | bc -l)
        
        echo "🏗️  Build Time Change: ${change}%"
        if (( $(echo "$change > 10" | bc -l) )); then
            echo "   ⚠️  Significant regression detected"
        elif (( $(echo "$change < -10" | bc -l) )); then
            echo "   ✅ Significant improvement detected"
        else
            echo "   ➡️  Within normal variance"
        fi
    fi
    
    # Add more comparisons as needed
    
    log_info "Comparison completed"
}

main() {
    local command=""
    local iterations="$BUILD_ITERATIONS"
    local duration="$CONTAINER_TEST_DURATION"
    local output_file="$BENCHMARK_FILE"
    local verbose=false
    
    # Parse arguments
    while [[ $# -gt 0 ]]; do
        case $1 in
            -i|--iterations)
                iterations="$2"
                shift 2
                ;;
            -d|--duration)
                duration="$2"
                shift 2
                ;;
            -o|--output)
                output_file="$2"
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
            build|container|full|compare|baseline|report)
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
    for cmd in docker bc jq; do
        if ! command -v "$cmd" >/dev/null 2>&1; then
            log_error "$cmd is required but not installed"
            exit 1
        fi
    done
    
    ensure_benchmark_dir
    
    # Execute command
    case "$command" in
        build)
            local results=$(run_build_benchmarks "$iterations")
            echo "{\"timestamp\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\", \"build_benchmarks\": $results}" > "$output_file"
            log_info "Build benchmarks completed: $output_file"
            ;;
        container)
            local results=$(run_container_benchmarks "$duration")
            echo "{\"timestamp\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\", \"container_benchmarks\": $results}" > "$output_file"
            log_info "Container benchmarks completed: $output_file"
            ;;
        full)
            local build_results=$(run_build_benchmarks "$iterations")
            local container_results=$(run_container_benchmarks "$duration")
            echo "{\"timestamp\": \"$(date -u +%Y-%m-%dT%H:%M:%SZ)\", \"build_benchmarks\": $build_results, \"container_benchmarks\": $container_results}" > "$output_file"
            log_info "Full benchmark suite completed: $output_file"
            ;;
        compare)
            local latest_file=$(ls -t "$BENCHMARK_RESULTS_DIR"/benchmark-*.json 2>/dev/null | head -1)
            if [[ -n "$latest_file" ]]; then
                compare_with_baseline "$latest_file"
            else
                log_error "No benchmark results found"
                exit 1
            fi
            ;;
        baseline)
            local latest_file=$(ls -t "$BENCHMARK_RESULTS_DIR"/benchmark-*.json 2>/dev/null | head -1)
            if [[ -n "$latest_file" ]]; then
                cp "$latest_file" "$BASELINE_FILE"
                log_info "Baseline set from: $latest_file"
            else
                log_error "No benchmark results found to set as baseline"
                exit 1
            fi
            ;;
        report)
            local latest_file=$(ls -t "$BENCHMARK_RESULTS_DIR"/benchmark-*.json 2>/dev/null | head -1)
            if [[ -n "$latest_file" ]]; then
                generate_benchmark_report "$latest_file"
            else
                log_error "No benchmark results found"
                exit 1
            fi
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