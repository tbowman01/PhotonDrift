#!/bin/bash
# Optimized WASM build script for ADRScan
# This script builds WASM modules with maximum performance and minimum size

set -e

echo "🚀 Starting optimized WASM build process..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if required tools are installed
check_dependencies() {
    print_status "Checking build dependencies..."
    
    if ! command -v wasm-pack &> /dev/null; then
        print_error "wasm-pack is not installed. Installing..."
        curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
    fi
    
    if ! command -v wasm-opt &> /dev/null; then
        print_warning "wasm-opt not found. Install binaryen for further optimizations."
        print_warning "  cargo install wasm-opt"
        print_warning "  or brew install binaryen (macOS)"
        print_warning "  or apt-get install binaryen (Ubuntu)"
    fi
    
    if ! command -v twiggy &> /dev/null; then
        print_warning "twiggy not found. Install for binary size analysis:"
        print_warning "  cargo install twiggy"
    fi
}

# Build function with optimizations
build_target() {
    local target=$1
    local output_dir=$2
    local features=${3:-"wasm"}
    
    print_status "Building for target: $target"
    print_status "Output directory: $output_dir"
    print_status "Features: $features"
    
    # Clean previous builds
    rm -rf "$output_dir"
    
    # Primary wasm-pack build with optimizations
    RUSTFLAGS="-C target-feature=+simd128 -C target-feature=+bulk-memory" \
    wasm-pack build \
        --release \
        --target "$target" \
        --out-dir "$output_dir" \
        --features "$features" \
        --scope adrscan \
        -- \
        --profile wasm-release \
        -Z build-std=std,panic_abort \
        --target wasm32-unknown-unknown
    
    # Post-build optimizations with wasm-opt if available
    if command -v wasm-opt &> /dev/null; then
        print_status "Applying wasm-opt optimizations..."
        
        local wasm_file="$output_dir"/*_bg.wasm
        if [ -f $wasm_file ]; then
            # Create backup
            cp "$wasm_file" "$wasm_file.backup"
            
            # Apply aggressive optimizations
            wasm-opt -Oz \
                --enable-simd \
                --enable-bulk-memory \
                --enable-sign-ext \
                --enable-mutable-globals \
                --strip-debug \
                --strip-producers \
                --dce \
                --remove-unused-names \
                --merge-blocks \
                --optimize-instructions \
                --optimize-stack-ir \
                --reorder-functions \
                --reorder-locals \
                --vacuum \
                "$wasm_file.backup" \
                -o "$wasm_file"
            
            # Compare sizes
            local original_size=$(wc -c < "$wasm_file.backup")
            local optimized_size=$(wc -c < "$wasm_file")
            local savings=$((original_size - optimized_size))
            local percentage=$((savings * 100 / original_size))
            
            print_success "wasm-opt reduced size by $savings bytes (${percentage}%)"
            rm "$wasm_file.backup"
        fi
    fi
    
    print_success "Build completed for $target"
}

# Analyze build results
analyze_build() {
    local output_dir=$1
    
    print_status "Analyzing build results for $output_dir..."
    
    local wasm_file="$output_dir"/*_bg.wasm
    if [ -f $wasm_file ]; then
        local size=$(wc -c < "$wasm_file")
        local size_kb=$((size / 1024))
        print_success "WASM size: ${size_kb}KB (${size} bytes)"
        
        # Detailed analysis with twiggy if available
        if command -v twiggy &> /dev/null; then
            print_status "Top space-consuming functions:"
            twiggy top "$wasm_file" -n 10 || true
            
            print_status "Monomorphizations analysis:"
            twiggy monos "$wasm_file" -n 5 || true
        fi
    fi
}

# Main build process
main() {
    print_status "Starting ADRScan WASM optimized build"
    
    # Move to project root
    cd "$(dirname "$0")/.."
    
    check_dependencies
    
    # Build for different targets
    print_status "Building for Node.js target..."
    build_target "nodejs" "pkg" "wasm"
    analyze_build "pkg"
    
    print_status "Building for Web target..."
    build_target "web" "wasm-web" "wasm"
    analyze_build "wasm-web"
    
    print_status "Building for Bundler target..."
    build_target "bundler" "wasm-bundler" "wasm"
    analyze_build "wasm-bundler"
    
    # Copy files to wasm directory for npm publishing
    print_status "Copying Node.js build to wasm directory..."
    cp -r pkg/* wasm/
    
    print_success "All WASM builds completed successfully!"
    
    # Final size comparison
    print_status "Final size comparison:"
    echo "Target          | Size (KB) | Size (bytes)"
    echo "----------------|-----------|-------------"
    
    for dir in pkg wasm-web wasm-bundler; do
        if [ -d "$dir" ]; then
            local wasm_file="$dir"/*_bg.wasm
            if [ -f $wasm_file ]; then
                local size=$(wc -c < "$wasm_file")
                local size_kb=$((size / 1024))
                printf "%-15s | %9d | %11d\n" "$dir" "$size_kb" "$size"
            fi
        fi
    done
    
    print_success "🎉 Optimized WASM build process completed!"
    print_status "Next steps:"
    echo "  - Test the builds: cd wasm && npm test"
    echo "  - Publish: cd wasm && npm publish"
    echo "  - Benchmark: cd wasm && npm run benchmark"
}

# Run main function
main "$@"