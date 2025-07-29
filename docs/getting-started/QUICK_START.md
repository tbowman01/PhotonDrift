# ADRScan Quick Start Guide

> **Get up and running with ADRScan in 5 minutes**

## Installation

### Option 1: Install from Crates.io (Coming Soon)
```bash
cargo install adrscan
```

### Option 2: Build from Source
```bash
git clone https://github.com/tbowman01/PhotonDrift.git
cd PhotonDrift
cargo build --release
# Binary available at: target/release/adrscan
```

### Option 3: GitHub Action (Immediate)
No installation needed - use directly in workflows:
```yaml
- uses: tbowman01/PhotonDrift@main
```

## 5-Minute Setup

### Step 1: Initialize Your ADR Directory (30 seconds)

```bash
# Initialize in standard location
adrscan init ./docs/adr

# Or custom location
adrscan init ./architecture/decisions
```

This creates:
- 📁 ADR directory structure
- 📄 First ADR: "Record Architecture Decisions"  
- ⚙️ Configuration file (`.adrscan.yaml`)
- 📋 README with instructions

### Step 2: Create Your First Real ADR (2 minutes)

```bash
cd docs/adr
cp 0001-record-architecture-decisions.md 0002-choose-database.md
```

Edit `0002-choose-database.md`:

```markdown
---
title: Choose Database Technology
status: accepted
date: 2024-01-20
tags: [database, postgresql]
---

# Choose Database Technology

## Status
Accepted

## Context
We need a primary database for our web application that can handle:
- User data storage
- High read/write throughput
- ACID compliance
- JSON document storage

## Decision
We will use PostgreSQL as our primary database.

## Consequences
### Positive
- Mature, battle-tested
- Excellent JSON support
- Strong ACID guarantees
- Rich ecosystem

### Negative  
- More complex than SQLite
- Requires PostgreSQL expertise
```

### Step 3: Scan Your Project (1 minute)

```bash
# Inventory your ADRs
adrscan inventory

# Check for architectural drift
adrscan diff

# Generate proposals for any detected drift
adrscan propose --dry-run
```

### Step 4: Add to CI/CD (1 minute)

Create `.github/workflows/architecture.yml`:

```yaml
name: Architecture Check

on:
  pull_request:
    branches: [main]

jobs:
  adrscan:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
    
    - name: Check Architecture Compliance
      uses: tbowman01/PhotonDrift@main
      with:
        adr-directory: './docs/adr'
        fail-on-drift: true
```

### Step 5: Test the Setup (30 seconds)

```bash
# Verify everything works
adrscan inventory --format json | jq .
adrscan diff --format console
```

## What You've Accomplished

✅ **ADR Management**: Structured decision documentation  
✅ **Drift Detection**: Automatic compliance checking  
✅ **CI/CD Integration**: Automated architecture governance  
✅ **Team Workflow**: Foundation for architectural discipline  

## Common First Use Cases

### For New Projects
```bash
# Document key early decisions
adrscan init
# Then create ADRs for:
# - Technology stack choices
# - Architecture patterns
# - Development processes
```

### For Existing Projects
```bash
# Document current state
adrscan init
adrscan diff  # See what's not documented
adrscan propose  # Generate ADRs for current tech
```

### For CI/CD Integration
```yaml
# Basic PR check
- uses: tbowman01/PhotonDrift@main
  with:
    fail-on-drift: true

# Full governance
- uses: tbowman01/PhotonDrift@main
  with:
    create-issues: true
    severity-threshold: 'medium'
```

## Next Steps

📖 **Read the [Full User Guide](USER_GUIDE.md)** for advanced features  
🔧 **Customize** detection patterns in `.adrscan.yaml`  
👥 **Set up team workflow** for ADR reviews  
🤖 **Configure notifications** for critical drift  

## Quick Reference

| Command | Purpose |
|---------|---------|
| `adrscan init` | Initialize ADR structure |
| `adrscan inventory` | List all ADRs |
| `adrscan diff` | Detect architectural drift |
| `adrscan propose` | Generate ADR proposals |
| `adrscan index` | Update ADR index |

| Files | Purpose |
|-------|---------|
| `.adrscan.yaml` | Configuration |
| `docs/adr/` | ADR directory |
| `0001-*.md` | First ADR |
| `.adrscan_snapshot.json` | Baseline state |

## Getting Help

- 🐛 **Issues**: [GitHub Issues](https://github.com/tbowman01/PhotonDrift/issues)
- 📖 **Docs**: [User Guide](USER_GUIDE.md)
- 💬 **Questions**: [Discussions](https://github.com/tbowman01/PhotonDrift/discussions)

---

**Ready to start documenting architecture decisions? Your future self will thank you! 🏗️**