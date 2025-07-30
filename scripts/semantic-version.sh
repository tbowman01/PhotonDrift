#!/bin/bash
# Semantic Versioning Automation for PhotonDrift
# Generates branch-based versions: develop -> alpha, main -> rc

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BASE_VERSION="0.3.0"
TIMESTAMP=$(date +%Y%m%d)
COMMIT_SHORT=$(git rev-parse --short HEAD)
BRANCH_NAME=$(git branch --show-current)
GITHUB_RUN_ID=${GITHUB_RUN_ID:-$(date +%s | tail -c 4)}

# Function to print colored output
log() {
    local color=$1
    local message=$2
    echo -e "${color}[SEMANTIC-VERSION]${NC} ${message}"
}

# Function to extract current version from Cargo.toml
get_current_version() {
    grep '^version = ' Cargo.toml | head -1 | cut -d'"' -f2
}

# Function to extract base version (remove pre-release suffixes)
get_base_version() {
    local version=$1
    echo "$version" | sed -E 's/-(alpha|rc|beta|dev).*$//'
}

# Function to increment version based on type
increment_version() {
    local version=$1
    local increment_type=$2
    
    IFS='.' read -r major minor patch <<< "$version"
    
    case $increment_type in
        major)
            echo "$((major + 1)).0.0"
            ;;
        minor)
            echo "${major}.$((minor + 1)).0"
            ;;
        patch)
            echo "${major}.${minor}.$((patch + 1))"
            ;;
        *)
            echo "$version"
            ;;
    esac
}

# Function to generate version based on branch
generate_version() {
    local branch=$1
    local base_version=$2
    local increment_type=${3:-"patch"}
    
    # Increment base version
    local new_base=$(increment_version "$base_version" "$increment_type")
    
    case $branch in
        develop)
            echo "${new_base}-alpha.${GITHUB_RUN_ID}"
            ;;
        main)
            echo "${new_base}-rc.${GITHUB_RUN_ID}"
            ;;
        release/*)
            local release_version=$(echo "$branch" | sed 's|release/||')
            echo "${release_version}"
            ;;
        hotfix/*)
            local hotfix_base=$(get_base_version "$base_version")
            local hotfix_version=$(increment_version "$hotfix_base" "patch")
            echo "${hotfix_version}-hotfix.${GITHUB_RUN_ID}"
            ;;
        *)
            echo "${new_base}-dev.${GITHUB_RUN_ID}"
            ;;
    esac
}

# Function to create git tag
create_git_tag() {
    local version=$1
    local tag_name="v${version}"
    local branch=$2
    
    log "$BLUE" "Creating git tag: $tag_name"
    
    # Create annotated tag with detailed message
    local tag_message="Release ${version}

Generated for branch: ${branch}
Commit: ${COMMIT_SHORT}
Timestamp: ${TIMESTAMP}
Build: $(date -u '+%Y-%m-%d %H:%M:%S UTC')

Branch-based versioning:
- develop → alpha releases
- main → release candidates (rc)
- release/* → stable releases
- hotfix/* → hotfix releases"

    git tag -a "$tag_name" -m "$tag_message"
    
    log "$GREEN" "Tag created successfully: $tag_name"
    
    # Push tag if in CI environment
    if [[ "${CI:-false}" == "true" ]]; then
        log "$BLUE" "Pushing tag to remote..."
        git push origin "$tag_name"
        log "$GREEN" "Tag pushed to remote"
    else
        log "$YELLOW" "Local development detected. Tag created locally only."
        log "$YELLOW" "Run 'git push origin $tag_name' to push to remote."
    fi
}

# Function to update Cargo.toml version
update_cargo_version() {
    local new_version=$1
    local cargo_file="Cargo.toml"
    
    log "$BLUE" "Updating $cargo_file with version: $new_version"
    
    # Create backup
    cp "$cargo_file" "${cargo_file}.backup"
    
    # Update version using sed
    sed -i.tmp "s/^version = \".*\"/version = \"$new_version\"/" "$cargo_file"
    rm -f "${cargo_file}.tmp"
    
    # Verify the change
    local updated_version=$(get_current_version)
    if [[ "$updated_version" == "$new_version" ]]; then
        log "$GREEN" "Version updated successfully: $updated_version"
        rm -f "${cargo_file}.backup"
    else
        log "$RED" "Failed to update version. Restoring backup..."
        mv "${cargo_file}.backup" "$cargo_file"
        exit 1
    fi
}

# Function to generate version info JSON
generate_version_info() {
    local version=$1
    local branch=$2
    
    cat > version-info.json << EOF
{
  "version": "$version",
  "branch": "$branch",
  "commit": "$COMMIT_SHORT",
  "timestamp": "$TIMESTAMP",
  "github_run_id": "$GITHUB_RUN_ID",
  "build_date": "$(date -u '+%Y-%m-%d %H:%M:%S UTC')",
  "version_type": "$(echo "$version" | grep -oE '(alpha|rc|dev|hotfix)' || echo 'stable')",
  "base_version": "$(get_base_version "$version")",
  "is_prerelease": $(if echo "$version" | grep -qE '(alpha|rc|dev|hotfix)'; then echo 'true'; else echo 'false'; fi)
}
EOF
    
    log "$GREEN" "Version info generated in version-info.json"
}

# Main execution
main() {
    local increment_type=${1:-"patch"}
    local force_tag=${2:-"false"}
    
    log "$BLUE" "=== PhotonDrift Semantic Versioning ==="
    log "$BLUE" "Current branch: $BRANCH_NAME"
    log "$BLUE" "Increment type: $increment_type"
    
    # Get current version and extract base
    local current_version=$(get_current_version)
    local base_version=$(get_base_version "$current_version")
    
    log "$BLUE" "Current version: $current_version"
    log "$BLUE" "Base version: $base_version"
    
    # Generate new version
    local new_version=$(generate_version "$BRANCH_NAME" "$base_version" "$increment_type")
    
    log "$GREEN" "Generated version: $new_version"
    
    # Update Cargo.toml
    update_cargo_version "$new_version"
    
    # Generate version info
    generate_version_info "$new_version" "$BRANCH_NAME"
    
    # Create git tag for alpha and rc builds
    if [[ "$BRANCH_NAME" == "develop" || "$BRANCH_NAME" == "main" || "$force_tag" == "true" ]]; then
        create_git_tag "$new_version" "$BRANCH_NAME"
    else
        log "$YELLOW" "Skipping tag creation for branch: $BRANCH_NAME"
        log "$YELLOW" "Tags are created for: develop (alpha), main (rc), or with --force-tag"
    fi
    
    # Output for GitHub Actions
    if [[ "${GITHUB_ACTIONS:-false}" == "true" ]]; then
        echo "version=$new_version" >> "$GITHUB_OUTPUT"
        echo "tag=v$new_version" >> "$GITHUB_OUTPUT"
        echo "is_prerelease=$(echo "$new_version" | grep -qE '(alpha|rc|dev|hotfix)' && echo 'true' || echo 'false')" >> "$GITHUB_OUTPUT"
        echo "version_type=$(echo "$new_version" | grep -oE '(alpha|rc|dev|hotfix)' || echo 'stable')" >> "$GITHUB_OUTPUT"
    fi
    
    log "$GREEN" "=== Semantic versioning complete ==="
    log "$GREEN" "New version: $new_version"
    log "$GREEN" "Ready for build and deployment!"
}

# Help function
show_help() {
    cat << EOF
PhotonDrift Semantic Versioning Tool

USAGE:
    ./semantic-version.sh [INCREMENT_TYPE] [--force-tag]

INCREMENT_TYPES:
    patch     Increment patch version (default)
    minor     Increment minor version  
    major     Increment major version

OPTIONS:
    --force-tag    Create git tag regardless of branch
    --help         Show this help message

BRANCH VERSIONING:
    develop    → X.Y.Z-alpha.RUN_ID
    main       → X.Y.Z-rc.RUN_ID  
    release/*  → X.Y.Z (stable release)
    hotfix/*   → X.Y.Z-hotfix.RUN_ID
    other      → X.Y.Z-dev.RUN_ID

EXAMPLES:
    ./semantic-version.sh                    # patch increment
    ./semantic-version.sh minor              # minor increment
    ./semantic-version.sh major --force-tag  # major increment with forced tag

EOF
}

# Parse arguments
case "${1:-}" in
    --help|-h)
        show_help
        exit 0
        ;;
    *)
        main "$@"
        ;;
esac