---
id: "lsp-integration"
title: "LSP Integration Guide"
sidebar_label: "LSP Integration"
sidebar_position: "2"
description: "Language Server Protocol implementation for IDE integration with real-time drift detection and ADR management"
slug: "/misc/lsp-integration"
tags: ["lsp", "ide", "integration", "development"]
last_update:
  date: "2025-07-22"
  author: "tbowman01"
---


# PhotonDrift LSP Integration Guide

PhotonDrift provides a Language Server Protocol (LSP) implementation that enables IDE integration for Architecture Decision Record (ADR) management with real-time drift detection.

## Features

- **Real-time Diagnostics**: Detects ADR structural issues, outdated references, and drift patterns
- **Smart Completion**: ADR templates, section headers, status values, and reference suggestions  
- **Contextual Hover**: Information about ADR sections, status values, dates, and references
- **Document Management**: Full document lifecycle support with change tracking
- **Performance Optimized**: <100ms response times for all operations
- **Workspace Integration**: Multi-folder workspace support with configuration discovery

## Installation and Setup

### Building the LSP Server

```bash
# Build with LSP feature enabled
cargo build --release --features lsp --bin adrscan-lsp

# The binary will be available at:
# target/release/adrscan-lsp
```

### IDE Configuration

#### VS Code

Create a VS Code extension or use the generic LSP client. Add to your `settings.json`:

```json
{
  "languageServerExample.enable": true,
  "languageServerExample.command": "/path/to/adrscan-lsp"
}
```

#### Neovim (with nvim-lspconfig)

```lua
local lspconfig = require('lspconfig')

-- Custom LSP configuration for PhotonDrift
local configs = require('lspconfig.configs')

if not configs.photon_drift then
  configs.photon_drift = {
    default_config = {
      cmd = { '/path/to/adrscan-lsp' },
      filetypes = { 'markdown' },
      root_dir = function(fname)
        return lspconfig.util.find_git_root(fname) or 
               lspconfig.util.path.dirname(fname)
      end,
      settings = {},
    },
  }
end

lspconfig.photon_drift.setup {
  on_attach = function(client, bufnr)
    -- Enable completion triggered by <c-x><c-o>
    vim.bo[bufnr].omnifunc = 'v:lua.vim.lsp.omnifunc'
  end,
}
```

#### Emacs (with lsp-mode)

```elisp
(use-package lsp-mode
  :config
  (add-to-list 'lsp-language-id-configuration '(markdown-mode . "markdown"))
  (lsp-register-client
   (make-lsp-client
    :new-connection (lsp-stdio-connection "/path/to/adrscan-lsp")
    :activation-fn (lsp-activate-on "markdown")
    :server-id 'photon-drift-lsp)))
```

#### Generic LSP Client

Most editors with LSP support can be configured to use PhotonDrift:

- **Command**: `/path/to/adrscan-lsp`
- **Language ID**: `markdown`
- **File Extensions**: `*.md`
- **Root Directory**: Git repository root or folder containing `.adrscan.yml`

## Configuration

The LSP server uses the same configuration as the PhotonDrift CLI. Create a `.adrscan.yml` file in your project root:

```yaml
# Basic configuration
adr_dir: "docs/adr"
include_patterns:
  - "**/*.md"
exclude_patterns:
  - "**/target/**"
  - "**/node_modules/**"

# Template configuration for completion
template:
  format: "madr"  # or "custom"
  custom_path: "templates/adr-template.md"  # if format is "custom"

# Drift detection settings
drift:
  enabled: true
  detection_patterns:
    - name: "Database Dependencies"
      file_pattern: "**/Cargo.toml"
      content_pattern: "(postgres|mysql|sqlite|mongodb)"
      category: "database"
    - name: "Cloud Provider References"
      file_pattern: "**/*.tf"
      content_pattern: "(aws|azure|gcp|google)"
      category: "cloud"

# LSP-specific templates (optional)
templates:
  - name: "API Decision Template"
    description: "Template for API-related decisions"
    content: |
      # ADR-${1:number}: ${2:API Decision Title}
      
      ## Status
      
      ${3:Proposed}
      
      ## Context
      
      ${4:API design context and requirements}
      
      ## Decision
      
      ${5:API architecture decision}
      
      ## API Specification
      
      ${6:API endpoints and contracts}
      
      ## Consequences
      
      ${7:Impact on API consumers and maintainers}
```

## LSP Capabilities

### Text Document Synchronization
- **Full document sync**: Complete document content is sent on changes
- **Change notifications**: Real-time updates as you edit ADRs
- **Open/Close tracking**: Document lifecycle management

### Diagnostics
- **Structural validation**: Missing required sections (Status, Context, Decision)
- **Content quality checks**: Thin content warnings, placeholder links
- **Drift detection**: Outdated technology references, broken links
- **Metadata validation**: Future dates, invalid status values
- **Reference checking**: High ADR numbers, potentially missing references

### Completion
- **Section headers**: Status, Context, Decision, Consequences, etc.
- **Status values**: Proposed, Accepted, Rejected, Superseded, Deprecated
- **ADR templates**: Basic and extended templates with placeholders
- **References**: ADR cross-references and superseding relationships
- **List items**: Structured pros/cons and alternative options

### Hover Information
- **Section descriptions**: Detailed explanations of ADR sections
- **Status explanations**: What each status means and when to use it
- **Date information**: Relative time and day of week for dates
- **Link previews**: Information about external and internal links
- **Reference details**: Context about ADR references

### Workspace Support
- **Multi-folder workspaces**: Support for complex project structures
- **Configuration discovery**: Automatic config file detection
- **File operations**: Creation, deletion, and rename notifications

## Performance

The LSP server is optimized for performance:

- **Response time**: <100ms for all operations
- **Concurrent operations**: Supports multiple simultaneous requests
- **Caching**: URI-to-path conversion caching
- **Efficient parsing**: Fast ADR document analysis
- **Memory management**: Bounded memory usage for large projects

## Troubleshooting

### Enable Debug Logging

Set the `RUST_LOG` environment variable before starting the LSP server:

```bash
RUST_LOG=debug adrscan-lsp
```

### Common Issues

#### LSP Server Not Starting
- Verify the binary path is correct
- Check file permissions (binary should be executable)
- Ensure all dependencies are available

#### No Completions or Diagnostics
- Verify the file is recognized as an ADR (contains ADR markers)
- Check that the workspace contains a valid configuration
- Ensure the document is properly opened in the LSP client

#### Performance Issues
- Check if large files are causing slowdowns
- Verify the workspace configuration excludes build directories
- Monitor memory usage with large numbers of ADRs

#### Configuration Not Loading
- Verify `.adrscan.yml` is in the workspace root
- Check YAML syntax is valid
- Ensure file permissions allow reading

### Debug Configuration

Create a debug configuration for development:

```yaml
# .adrscan-debug.yml
adr_dir: "test/adr"
include_patterns: ["**/*.md"]
exclude_patterns: []
snapshot_file: "debug_snapshot.json"
template:
  format: "madr"
drift:
  enabled: true
  detection_patterns: []
```

## Example Usage

### Creating a New ADR

1. Create a new `.md` file in your ADR directory
2. Type `# ADR-` and use completion to select a template
3. Fill in the template placeholders using tab navigation
4. Save the file to trigger drift detection

### Reviewing Existing ADRs

1. Open an existing ADR file
2. Hover over status values to see explanations
3. Check diagnostics panel for potential issues
4. Use completion for consistent formatting

### Managing ADR References

1. Type `ADR-` to get reference completion
2. Use hover to verify referenced ADRs exist
3. Update superseding relationships with completion help
4. Check diagnostics for broken or questionable references

## Integration Examples

### Automated ADR Validation

Use the LSP diagnostics in CI/CD pipelines:

```bash
# Start LSP server in background
adrscan-lsp &
LSP_PID=$!

# Use LSP client to validate all ADRs
# (Implementation depends on your LSP client)

# Clean up
kill $LSP_PID
```

### Template Management

Maintain organization-specific ADR templates:

```yaml
# .adrscan.yml
templates:
  - name: "Security Decision"
    description: "Template for security-related decisions"
    content: |
      # ADR-${1:number}: ${2:Security Decision}
      
      ## Security Impact Assessment
      ${3:Impact analysis}
      
      ## Threat Model
      ${4:Identified threats}
  
  - name: "Performance Decision"  
    description: "Template for performance-related decisions"
    content: |
      # ADR-${1:number}: ${2:Performance Decision}
      
      ## Performance Requirements
      ${3:Performance criteria}
      
      ## Benchmarks
      ${4:Performance measurements}
```

## Contributing

To extend the LSP server functionality:

1. **Add new diagnostic rules** in `src/lsp/diagnostics.rs`
2. **Extend completion providers** in `src/lsp/completion.rs`
3. **Add hover information** in `src/lsp/hover.rs`
4. **Write tests** in `tests/lsp/`
5. **Update documentation** in this file

The LSP implementation follows the Language Server Protocol specification and uses the `tower-lsp` framework for robust protocol handling.