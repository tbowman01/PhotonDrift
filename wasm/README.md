# @adrscan/wasm

> WebAssembly module for ADRScan - AI-powered Architecture Decision Record management with ML-enhanced drift detection

[![npm version](https://badge.fury.io/js/%40adrscan%2Fwasm.svg)](https://badge.fury.io/js/%40adrscan%2Fwasm)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Installation

```bash
npm install @adrscan/wasm
```

## Quick Start

### Node.js

```javascript
const { ADRScan, utils } = require('@adrscan/wasm');

// Create scanner instance
const scanner = new ADRScan({
    adrDir: './docs/adr',
    templateFormat: 'madr',
    driftEnabled: true
});

// Initialize ADR directory structure
const initFiles = await scanner.init('./docs/adr');
console.log('Files to create:', Object.keys(initFiles));

// Parse ADR content
const adrContent = `---
title: Use MongoDB for data storage
status: accepted
date: 2024-01-01
---

# Use MongoDB for data storage

We will use MongoDB as our primary database.`;

const parsed = scanner.parseAdr(adrContent, 'example.md');
console.log('Parsed ADR:', parsed);

// Detect drift in codebase
const files = {
    'src/database.js': 'const mongodb = require("mongodb");',
    'src/cache.js': 'const redis = require("redis");'
};

const driftReport = scanner.detectDrift(files);
console.log('Drift detected:', driftReport.total_items);

// Generate proposals
const proposals = await scanner.propose(driftReport);
console.log('Generated proposals:', proposals.length);
```

### Browser/ES Modules

```javascript
import init, { ADRScan, utils } from '@adrscan/wasm';

await init();

const scanner = new ADRScan({
    adrDir: './docs/adr'
});

// Use the same API as Node.js
```

## API Reference

### ADRScan Class

#### Constructor
- `new ADRScan(config)` - Create new instance with configuration

#### Methods
- `init(directory)` - Initialize ADR directory structure
- `inventory(directory)` - Scan and inventory ADRs
- `diff(directory, baseline?)` - Detect architectural drift
- `propose(driftReport)` - Generate ADR proposals
- `getConfig()` - Get current configuration
- `updateConfig(config)` - Update configuration

### Utils

- `utils.version()` - Get ADRScan version
- `utils.features()` - Get supported features
- `utils.parseFrontmatter(content)` - Parse ADR frontmatter
- `utils.validateTemplate(template)` - Validate ADR template
- `utils.getDefaultTemplate()` - Get default MADR template

## Configuration

```javascript
const config = {
    adrDir: './docs/adr',           // ADR directory path
    templateFormat: 'madr',         // Template format
    driftEnabled: true,             // Enable drift detection
    includePatterns: ['**/*.md'],   // File patterns to include
    excludePatterns: ['**/node_modules/**'] // File patterns to exclude
};
```

## Features

- ✅ ADR frontmatter parsing
- ✅ Template validation and generation
- ✅ Configuration management
- ✅ Drift detection (basic structure)
- ✅ ADR proposal generation
- ✅ Cross-platform compatibility (Node.js, Browser)

## Limitations

The WASM module has some limitations compared to the native CLI:

1. **File System Access**: Requires host environment to provide file contents
2. **Async Operations**: Some operations are simplified for WASM compatibility
3. **Performance**: May be slower than native for large repositories

## Building from Source

```bash
# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Build for Node.js
wasm-pack build --target nodejs --out-dir wasm

# Build for Web
wasm-pack build --target web --out-dir wasm-web

# Build for Bundlers
wasm-pack build --target bundler --out-dir wasm-bundler
```

## Testing

```bash
cd wasm
npm test
```

## License

MIT - See [LICENSE](../LICENSE) for details.