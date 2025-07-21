#!/bin/bash
# Intelligent Issue Triage Script for PhotonDrift
# This script analyzes issues and applies labels automatically

set -e

# Configuration
REPO="tbowman01/PhotonDrift"
AUTO_LABEL=${AUTO_LABEL:-false}
AUTO_ASSIGN=${AUTO_ASSIGN:-false}

# Color codes for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🤖 PhotonDrift Intelligent Issue Triage System${NC}"
echo "================================================"

# Function to classify issue type based on title and body
classify_issue_type() {
    local title="$1"
    local body="$2"
    
    if [[ "$title" =~ \[BUG\] ]] || [[ "$body" =~ (bug|error|broken|fix) ]]; then
        echo "bug"
    elif [[ "$title" =~ (Security|security|vulnerability) ]] || [[ "$body" =~ (security|vulnerability|CVE) ]]; then
        echo "security"
    elif [[ "$title" =~ (Performance|performance) ]] || [[ "$body" =~ (performance|slow|optimization) ]]; then
        echo "performance"
    elif [[ "$title" =~ (Dependency|dependency) ]] || [[ "$body" =~ (dependency|package|update) ]]; then
        echo "dependencies"
    elif [[ "$title" =~ \[PHASE ]] || [[ "$title" =~ (Feature|feature) ]]; then
        echo "feature"
    elif [[ "$title" =~ (Report|report|Analysis) ]]; then
        echo "documentation"
    else
        echo "needs-triage"
    fi
}

# Function to determine priority based on content
determine_priority() {
    local title="$1"
    local body="$2"
    local labels="$3"
    
    # Check if already has priority label
    if [[ "$labels" =~ priority- ]]; then
        return
    fi
    
    # Security issues are always high priority
    if [[ "$body" =~ (security|vulnerability|CVE) ]]; then
        echo "high"
    # Phase 3 features are high priority
    elif [[ "$title" =~ \[PHASE\ 3\] ]]; then
        echo "high"
    # Bugs affecting core functionality
    elif [[ "$title" =~ \[BUG\] ]] && [[ "$body" =~ (critical|blocking|urgent) ]]; then
        echo "high"
    # Performance issues
    elif [[ "$body" =~ (performance|slow) ]]; then
        echo "medium"
    # Dependencies and roadmap items
    elif [[ "$title" =~ \[Roadmap\] ]] || [[ "$body" =~ (future|enhancement) ]]; then
        echo "low"
    else
        echo "medium"
    fi
}

# Function to suggest assignee based on issue type
suggest_assignee() {
    local type="$1"
    local title="$2"
    
    case "$type" in
        "security")
            echo "security-team"
            ;;
        "bug"|"dependencies")
            echo "rust-team"
            ;;
        "performance")
            echo "performance-team"
            ;;
        "feature")
            if [[ "$title" =~ (WASM|wasm) ]]; then
                echo "wasm-specialist"
            elif [[ "$title" =~ (CI|CD|pipeline) ]]; then
                echo "devops-team"
            else
                echo "dev-team"
            fi
            ;;
        *)
            echo "triage-team"
            ;;
    esac
}

# Get all open issues
echo -e "${YELLOW}📋 Fetching open issues...${NC}"
issues=$(gh issue list --repo "$REPO" --state open --json number,title,body,labels,assignees --limit 100)

# Process each issue
echo "$issues" | jq -c '.[]' | while read -r issue; do
    number=$(echo "$issue" | jq -r '.number')
    title=$(echo "$issue" | jq -r '.title')
    body=$(echo "$issue" | jq -r '.body // ""')
    current_labels=$(echo "$issue" | jq -r '.labels[].name' | tr '\n' ' ')
    assignees=$(echo "$issue" | jq -r '.assignees')
    
    echo -e "\n${BLUE}Processing Issue #$number: ${title:0:50}...${NC}"
    
    # Classify issue
    issue_type=$(classify_issue_type "$title" "$body")
    priority=$(determine_priority "$title" "$body" "$current_labels")
    suggested_assignee=$(suggest_assignee "$issue_type" "$title")
    
    # Prepare labels to add
    labels_to_add=()
    
    # Add type label if not present
    case "$issue_type" in
        "bug")
            [[ ! "$current_labels" =~ bug ]] && labels_to_add+=("bug")
            ;;
        "security")
            [[ ! "$current_labels" =~ security ]] && labels_to_add+=("security")
            ;;
        "performance")
            [[ ! "$current_labels" =~ performance ]] && labels_to_add+=("performance")
            ;;
        "dependencies")
            [[ ! "$current_labels" =~ dependencies ]] && labels_to_add+=("dependencies")
            ;;
        "feature")
            [[ ! "$current_labels" =~ type-feature ]] && labels_to_add+=("type-feature")
            ;;
        "documentation")
            [[ ! "$current_labels" =~ documentation ]] && labels_to_add+=("documentation")
            ;;
    esac
    
    # Add priority label if not present
    if [[ ! "$current_labels" =~ priority- ]]; then
        labels_to_add+=("priority-$priority")
    fi
    
    # Remove needs-triage if we classified it
    if [[ "$current_labels" =~ needs-triage ]] && [[ "$issue_type" != "needs-triage" ]]; then
        labels_to_remove=("needs-triage")
    fi
    
    # Display recommendations
    echo -e "  📊 Type: ${GREEN}$issue_type${NC}"
    echo -e "  🎯 Priority: ${YELLOW}$priority${NC}"
    echo -e "  👤 Suggested Assignee: ${BLUE}$suggested_assignee${NC}"
    
    if [ ${#labels_to_add[@]} -gt 0 ]; then
        echo -e "  🏷️  Labels to add: ${GREEN}${labels_to_add[*]}${NC}"
    fi
    
    # Apply labels if auto-label is enabled
    if [ "$AUTO_LABEL" = true ] && [ ${#labels_to_add[@]} -gt 0 ]; then
        echo -e "  ${YELLOW}Applying labels...${NC}"
        for label in "${labels_to_add[@]}"; do
            gh issue edit "$number" --repo "$REPO" --add-label "$label" 2>/dev/null || \
                echo -e "  ${RED}Warning: Label '$label' might not exist${NC}"
        done
        
        # Remove needs-triage if applicable
        if [ ${#labels_to_remove[@]} -gt 0 ]; then
            for label in "${labels_to_remove[@]}"; do
                gh issue edit "$number" --repo "$REPO" --remove-label "$label" 2>/dev/null
            done
        fi
    fi
    
    # Auto-assign if enabled and no assignees
    if [ "$AUTO_ASSIGN" = true ] && [ "$assignees" = "[]" ]; then
        echo -e "  ${YELLOW}Note: Auto-assignment would assign to: $suggested_assignee${NC}"
        # In a real implementation, you would map team names to actual GitHub usernames
        # gh issue edit "$number" --repo "$REPO" --add-assignee "$github_username"
    fi
done

echo -e "\n${GREEN}✅ Triage complete!${NC}"

# Generate summary
total_issues=$(echo "$issues" | jq '. | length')
echo -e "\n${BLUE}📊 Summary:${NC}"
echo "  Total open issues: $total_issues"

# Count by type
echo -e "\n  By Type:"
for type in bug security performance dependencies feature documentation; do
    count=$(echo "$issues" | jq -r ".[].title + \" \" + (.[].body // \"\")" | \
        grep -c -iE "$type" || echo "0")
    [ "$count" -gt 0 ] && echo "    - $type: $count"
done

echo -e "\n${YELLOW}💡 To apply labels automatically, run:${NC}"
echo "  AUTO_LABEL=true $0"
echo -e "\n${YELLOW}💡 To enable auto-assignment, run:${NC}"
echo "  AUTO_ASSIGN=true AUTO_LABEL=true $0"