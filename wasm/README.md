# ADRScan WebAssembly Module

WebAssembly bindings for ADRScan - Architecture Decision Record management and drift detection.

## Installation

```bash
npm install @adrscan/wasm
```

## Usage

### Node.js

```javascript
const { ADRScan, utils } = require('@adrscan/wasm');

// Create scanner instance
const scanner = new ADRScan({
    adrDir: './docs/adr',
    templateFormat: 'madr',
    driftEnabled: true
});

// Initialize ADR directory
const initFiles = await scanner.init('./docs/adr');
console.log('Files to create:', initFiles);

// Scan existing ADRs
const inventory = await scanner.inventory('./docs/adr');
console.log('Inventory:', inventory);

// Detect drift
const driftReport = await scanner.diff('.');
console.log('Drift items:', driftReport.total_items);

// Generate proposals
const proposals = await scanner.propose(driftReport);
console.log('Generated proposals:', proposals);
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