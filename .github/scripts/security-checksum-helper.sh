#!/bin/bash
# Security Checksum Verification Helper Script
# Provides secure download and verification functionality for CI/CD
set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to log with timestamp
log() {
    echo -e "$(date '+%Y-%m-%d %H:%M:%S') - $1"
}

# Function to verify checksum
verify_checksum() {
    local file="$1"
    local expected_checksum="$2"
    local algorithm="${3:-sha256}"
    
    log "${YELLOW}Verifying ${algorithm} checksum for ${file}...${NC}"
    
    case "$algorithm" in
        sha256)
            echo "${expected_checksum}  ${file}" | sha256sum -c - || {
                log "${RED}❌ SECURITY: ${algorithm} checksum verification failed for ${file}!${NC}"
                log "${RED}Expected: ${expected_checksum}${NC}"
                log "${RED}Actual:   $(sha256sum "${file}" | cut -d' ' -f1)${NC}"
                return 1
            }
            ;;
        sha512)
            echo "${expected_checksum}  ${file}" | sha512sum -c - || {
                log "${RED}❌ SECURITY: ${algorithm} checksum verification failed for ${file}!${NC}"
                return 1
            }
            ;;
        md5)
            log "${YELLOW}⚠️  WARNING: MD5 is cryptographically weak, consider using SHA256+${NC}"
            echo "${expected_checksum}  ${file}" | md5sum -c - || {
                log "${RED}❌ SECURITY: ${algorithm} checksum verification failed for ${file}!${NC}"
                return 1
            }
            ;;
        *)
            log "${RED}❌ Unsupported checksum algorithm: ${algorithm}${NC}"
            return 1
            ;;
    esac
    
    log "${GREEN}✅ ${algorithm} checksum verification passed for ${file}${NC}"
    return 0
}

# Function to secure download with checksum verification
secure_download() {
    local url="$1"
    local output_file="$2"
    local expected_checksum="$3"
    local algorithm="${4:-sha256}"
    local max_retries="${5:-3}"
    
    log "${YELLOW}Securely downloading ${url} to ${output_file}...${NC}"
    
    # Check if output file already exists
    if [[ -f "${output_file}" ]]; then
        log "${YELLOW}File ${output_file} already exists, verifying checksum...${NC}"
        if verify_checksum "${output_file}" "${expected_checksum}" "${algorithm}"; then
            log "${GREEN}✅ File already exists and checksum is valid${NC}"
            return 0
        else
            log "${YELLOW}Existing file checksum invalid, re-downloading...${NC}"
            rm -f "${output_file}"
        fi
    fi
    
    local retry=0
    while [[ ${retry} -lt ${max_retries} ]]; do
        retry=$((retry + 1))
        log "${YELLOW}Download attempt ${retry}/${max_retries}...${NC}"
        
        # Use wget with security options
        if wget --quiet \
                --timeout=30 \
                --tries=3 \
                --no-check-certificate=false \
                --ca-certificate=/etc/ssl/certs/ca-certificates.crt \
                --output-document="${output_file}" \
                "${url}"; then
            
            # Verify checksum
            if verify_checksum "${output_file}" "${expected_checksum}" "${algorithm}"; then
                log "${GREEN}✅ Secure download completed successfully${NC}"
                return 0
            else
                log "${RED}❌ Downloaded file failed checksum verification${NC}"
                rm -f "${output_file}"
            fi
        else
            log "${RED}❌ Download failed (attempt ${retry}/${max_retries})${NC}"
        fi
        
        if [[ ${retry} -lt ${max_retries} ]]; then
            sleep $((retry * 2))  # Exponential backoff
        fi
    done
    
    log "${RED}❌ SECURITY: Failed to securely download ${url} after ${max_retries} attempts${NC}"
    return 1
}

# Function to get latest checksum from GitHub releases
get_github_release_checksum() {
    local repo="$1"
    local version="$2"
    local filename="$3"
    
    log "${YELLOW}Fetching checksum for ${filename} from ${repo} ${version}...${NC}"
    
    # Download checksums file
    local checksums_url="https://github.com/${repo}/releases/download/${version}/checksums.txt"
    local checksums_file="/tmp/checksums_${version}.txt"
    
    if wget --quiet --output-document="${checksums_file}" "${checksums_url}"; then
        # Extract checksum for specific file
        local checksum
        checksum=$(grep "${filename}" "${checksums_file}" | cut -d' ' -f1)
        
        if [[ -n "${checksum}" ]]; then
            log "${GREEN}✅ Found checksum: ${checksum}${NC}"
            echo "${checksum}"
            return 0
        else
            log "${RED}❌ Checksum not found for ${filename}${NC}"
            return 1
        fi
    else
        log "${RED}❌ Failed to download checksums file${NC}"
        return 1
    fi
}

# Main function for CLI usage
main() {
    case "${1:-}" in
        verify)
            verify_checksum "$2" "$3" "${4:-sha256}"
            ;;
        download)
            secure_download "$2" "$3" "$4" "${5:-sha256}" "${6:-3}"
            ;;
        github-checksum)
            get_github_release_checksum "$2" "$3" "$4"
            ;;
        *)
            echo "Security Checksum Helper"
            echo "Usage:"
            echo "  $0 verify <file> <checksum> [algorithm]"
            echo "  $0 download <url> <output> <checksum> [algorithm] [retries]"
            echo "  $0 github-checksum <repo> <version> <filename>"
            echo ""
            echo "Examples:"
            echo "  $0 verify grype.tar.gz abc123def456..."
            echo "  $0 download https://example.com/file.tar.gz file.tar.gz abc123def456..."
            echo "  $0 github-checksum anchore/grype v0.79.4 grype_0.79.4_linux_amd64.tar.gz"
            exit 1
            ;;
    esac
}

# Run main function if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi