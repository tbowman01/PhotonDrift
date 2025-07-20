# ADRScan GitHub Action

Automatically detect architectural drift in your repositories by comparing your codebase against Architecture Decision Records (ADRs).

## Features

- 🔍 **Automated Drift Detection** - Scans your codebase for changes that deviate from documented architectural decisions
- 📋 **ADR Management** - Automatically inventories and validates your Architecture Decision Records
- 🐛 **Issue Creation** - Optionally creates GitHub issues for detected architectural drift
- 💬 **PR Comments** - Adds detailed drift reports to pull request comments
- 📊 **Multiple Formats** - Supports console, JSON, and YAML output formats
- ⚡ **Fast & Efficient** - Rust-based implementation with intelligent caching

## Usage

### Basic Usage

Add the following step to your GitHub Actions workflow:

```yaml
steps:
  - uses: actions/checkout@v4
  
  - name: ADRScan Drift Detection
    uses: tbowman01/PhotonDrift@main
    with:
      adr-directory: './docs/adr'
      target-directory: '.'
      fail-on-drift: true
```

### Complete Example

```yaml
name: Architecture Compliance Check

on:
  pull_request:
    branches: [ main ]
  push:
    branches: [ main ]

jobs:
  adrscan:
    runs-on: ubuntu-latest
    name: Check Architectural Drift
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      with:
        fetch-depth: 0  # Full history for better analysis
    
    - name: Run ADRScan
      uses: tbowman01/PhotonDrift@main
      with:
        adr-directory: './docs/adr'
        target-directory: '.'
        severity-threshold: 'medium'
        fail-on-drift: true
        create-issues: true
        output-format: 'json'
        github-token: ${{ secrets.GITHUB_TOKEN }}
    
    - name: Upload drift report
      if: always()
      uses: actions/upload-artifact@v3
      with:
        name: drift-report
        path: drift-report.json
```

### Scheduled Drift Checks

```yaml
name: Weekly Architecture Review

on:
  schedule:
    - cron: '0 9 * * 1'  # Every Monday at 9 AM UTC

jobs:
  architecture-review:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Weekly Drift Check
      uses: tbowman01/PhotonDrift@main
      with:
        adr-directory: './docs/adr'
        create-issues: true
        fail-on-drift: false  # Don't fail scheduled checks
        severity-threshold: 'high'
```

## Inputs

| Input | Description | Required | Default |
|-------|-------------|----------|---------|
| `adr-directory` | Directory containing Architecture Decision Records | No | `./docs/adr` |
| `target-directory` | Directory to scan for architectural drift | No | `.` |
| `severity-threshold` | Minimum severity to report (`low`, `medium`, `high`, `critical`) | No | `medium` |
| `fail-on-drift` | Fail the workflow if drift is detected | No | `true` |
| `create-issues` | Create GitHub issues for detected drift | No | `false` |
| `output-format` | Output format (`console`, `json`, `yaml`) | No | `console` |
| `github-token` | GitHub token for API operations | No | `${{ github.token }}` |

## Outputs

| Output | Description |
|--------|-------------|
| `drift-detected` | Boolean indicating if architectural drift was detected |
| `drift-count` | Number of drift items found |
| `report-path` | Path to the generated drift report file |
| `summary` | Human-readable summary of results |

## What Gets Detected

ADRScan detects various types of architectural drift:

### 🔧 **Technology Drift**
- New dependencies not documented in ADRs
- Rejected technologies being introduced
- Framework changes without decisions

### 📁 **Structural Drift**
- New architectural patterns
- Package organization changes
- Module boundary violations

### ⚙️ **Configuration Drift**
- Infrastructure changes
- Build system modifications
- Environment configuration updates

### 🏗️ **Design Pattern Drift**
- Inconsistent coding patterns
- Architecture principle violations
- Interface contract changes

## ADR Directory Structure

ADRScan expects your ADRs to follow this structure:

```
docs/adr/
├── README.md
├── 0001-record-architecture-decisions.md
├── 0002-choose-database-technology.md
├── 0003-api-design-principles.md
└── ...
```

Each ADR should include frontmatter:

```markdown
---
title: Choose Database Technology
status: accepted
date: 2024-01-15
tags: [database, postgresql, performance]
---

# Choose Database Technology

## Status
Accepted

## Context
We need to select a database technology...
```

## Permissions

The action requires the following permissions:

```yaml
permissions:
  contents: read      # Read repository content
  issues: write       # Create issues (if create-issues: true)
  pull-requests: write # Comment on PRs
```

## Examples of Generated Issues

When `create-issues: true`, ADRScan creates structured GitHub issues:

**Title:** `[High] Undocumented Redis Usage Detected`

**Body:**
```markdown
## 🔍 Architectural Drift Detected

**Severity:** High  
**Category:** NewTechnology  
**Location:** `src/cache/redis.ts`

### Description
Redis caching implementation detected but no ADR documents this architectural decision.

### Recommended Actions
1. Review the architectural decision documented in your ADRs
2. If the change is intentional, update the relevant ADR
3. If the change violates architecture, consider reverting
4. Update team on architectural changes if approved

*This issue was automatically created by ADRScan*
```

## Integration with Development Workflow

### Pull Request Workflow
1. **Developer creates PR**
2. **ADRScan runs automatically**
3. **Comments added to PR with drift summary**
4. **Issues created for significant drift**
5. **PR fails if `fail-on-drift: true`**

### Continuous Monitoring
- **Scheduled runs** detect drift over time
- **Baseline snapshots** track architectural evolution
- **Trend analysis** shows drift patterns

## Configuration

Create `.adrscan.yaml` in your repository root:

```yaml
adr_dir: ./docs/adr
include_patterns:
  - "**/*.rs"
  - "**/*.ts"
  - "**/Cargo.toml"
  - "**/package.json"
exclude_patterns:
  - "**/node_modules/**"
  - "**/target/**"
template:
  format: madr
drift:
  enabled: true
  detection_patterns:
    - name: "Redis Usage"
      file_pattern: "**/*.{rs,ts,js}"
      content_pattern: "redis|Redis"
      category: "caching"
```

## Troubleshooting

### Common Issues

**No ADRs found:**
```
⚠️ ADR directory not found: ./docs/adr
Creating basic ADR structure...
```
*Solution: Let ADRScan create the initial structure or specify correct path*

**Permission errors:**
```
Error: Resource not accessible by integration
```
*Solution: Add required permissions to your workflow*

**Large repositories:**
```
Warning: Scan took longer than expected
```
*Solution: Use exclude patterns to skip non-relevant directories*

### Debug Mode

Enable debug output:

```yaml
- name: Debug ADRScan
  run: |
    export RUST_LOG=debug
    adrscan diff --adr-dir ./docs/adr --format json
```

## Contributing

See the main [PhotonDrift repository](https://github.com/tbowman01/PhotonDrift) for contribution guidelines.

## License

MIT License - see [LICENSE](LICENSE) for details.