# WebAssembly File I/O Guide

The ADRScan WASM module is designed to work across different environments (Node.js, browsers, CI/CD) where direct file system access may not be available or desired. This guide explains how to handle file I/O operations effectively.

## Architecture Overview

The WASM module follows a **host-provided file content** pattern:

- **Host Environment** (Node.js, browser, CI/CD): Responsible for reading files from disk, network, or other sources
- **WASM Module**: Processes the file contents provided by the host
- **No Direct File Access**: The WASM module never directly accesses the file system

This design ensures:
- ✅ **Universal Compatibility**: Works in browsers, Node.js, and serverless environments
- ✅ **Security**: Host controls what files the WASM module can access
- ✅ **Flexibility**: Files can come from any source (disk, network, database)
- ✅ **Performance**: Host can implement caching and optimization strategies

## File Content Format

All WASM functions that process files expect a JSON object mapping file paths to contents:

```javascript
const files = {
    "relative/path/to/file.js": "file content as string",
    "another/file.md": "markdown content...",
    "docs/adr/0001-decision.md": "---\ntitle: My ADR\n---\n# Decision..."
};
```

### Key Requirements

1. **Relative Paths**: Use relative paths from your project root
2. **String Content**: All content must be UTF-8 strings
3. **Consistent Separators**: Use forward slashes (`/`) for path separators
4. **Complete Content**: Include entire file contents, not excerpts

## Implementation Examples

### Node.js - Recursive File Reading

```javascript
const fs = require('fs').promises;
const path = require('path');
const { ADRScan } = require('@adrscan/wasm');

async function readProjectFiles(rootDir, patterns = ['**/*.js', '**/*.ts', '**/*.md']) {
    const files = {};
    
    async function readDir(currentDir) {
        const entries = await fs.readdir(currentDir, { withFileTypes: true });
        
        for (const entry of entries) {
            const fullPath = path.join(currentDir, entry.name);
            const relativePath = path.relative(rootDir, fullPath);
            
            // Skip common ignore patterns
            if (shouldIgnore(entry.name)) continue;
            
            if (entry.isDirectory()) {
                await readDir(fullPath);
            } else if (shouldInclude(relativePath, patterns)) {
                files[relativePath] = await fs.readFile(fullPath, 'utf8');
            }
        }
    }
    
    await readDir(rootDir);
    return files;
}

function shouldIgnore(name) {
    const ignorePatterns = [
        'node_modules', '.git', '.vscode', 'target', 'dist', 'build',
        '.env', '.env.local', '.DS_Store'
    ];
    return ignorePatterns.some(pattern => name.includes(pattern));
}

function shouldInclude(filePath, patterns) {
    return patterns.some(pattern => {
        const regex = new RegExp(
            pattern.replace(/\*\*/g, '.*').replace(/\*/g, '[^/]*')
        );
        return regex.test(filePath);
    });
}

// Usage
async function main() {
    const scanner = new ADRScan();
    
    // Read all relevant project files
    const projectFiles = await readProjectFiles('.', [
        '**/*.js', '**/*.ts', '**/*.json', 
        '**/Dockerfile', '**/*.yml'
    ]);
    
    // Read ADR files separately
    const adrFiles = await readProjectFiles('./docs/adr', ['**/*.md']);
    
    // Run analysis
    const inventory = scanner.inventory(adrFiles);
    const driftReport = scanner.detectDrift(projectFiles);
    
    console.log('Inventory:', inventory.total_count, 'ADRs');
    console.log('Drift:', driftReport.total_items, 'items');
}
```

### Browser - File Upload Interface

```html
<!DOCTYPE html>
<html>
<head>
    <title>ADRScan WASM - Browser Demo</title>
</head>
<body>
    <h1>ADRScan Browser Demo</h1>
    
    <div>
        <label for="file-upload">Select project files:</label>
        <input type="file" id="file-upload" multiple webkitdirectory>
        <button onclick="analyzeFiles()">Analyze</button>
    </div>
    
    <div id="results"></div>
    
    <script type="module">
        import init, { ADRScan, utils } from './wasm-web/adrscan.js';
        
        let scanner;
        
        async function initializeWasm() {
            await init();
            scanner = new ADRScan({
                adrDir: 'docs/adr',
                templateFormat: 'madr'
            });
            
            console.log('WASM initialized:', utils.version());
        }
        
        async function analyzeFiles() {
            const fileInput = document.getElementById('file-upload');
            const files = {};
            
            // Process uploaded files
            for (const file of fileInput.files) {
                // Convert to relative path
                const relativePath = file.webkitRelativePath || file.name;
                
                // Filter relevant files
                if (isRelevantFile(relativePath)) {
                    try {
                        files[relativePath] = await file.text();
                    } catch (err) {
                        console.warn('Failed to read', relativePath, ':', err);
                    }
                }
            }
            
            if (Object.keys(files).length === 0) {
                alert('No relevant files found');
                return;
            }
            
            // Separate ADRs from other files
            const adrFiles = {};
            const sourceFiles = {};
            
            for (const [path, content] of Object.entries(files)) {
                if (path.includes('/adr/') && path.endsWith('.md')) {
                    adrFiles[path] = content;
                } else {
                    sourceFiles[path] = content;
                }
            }
            
            // Run analysis
            const results = {
                fileCount: Object.keys(files).length,
                adrCount: Object.keys(adrFiles).length
            };
            
            if (Object.keys(adrFiles).length > 0) {
                results.inventory = scanner.inventory(adrFiles);
            }
            
            if (Object.keys(sourceFiles).length > 0) {
                results.drift = scanner.detectDrift(sourceFiles);
            }
            
            displayResults(results);
        }
        
        function isRelevantFile(path) {
            const extensions = ['.js', '.ts', '.json', '.md', '.yml', '.yaml'];
            const isRelevantExt = extensions.some(ext => path.endsWith(ext));
            const isDockerfile = path.endsWith('Dockerfile');
            return isRelevantExt || isDockerfile;
        }
        
        function displayResults(results) {
            const resultsDiv = document.getElementById('results');
            resultsDiv.innerHTML = `
                <h2>Analysis Results</h2>
                <p>Files processed: ${results.fileCount}</p>
                <p>ADRs found: ${results.adrCount}</p>
                ${results.inventory ? `
                    <h3>ADR Inventory</h3>
                    <p>Total ADRs: ${results.inventory.total_count}</p>
                    <p>Status breakdown: ${JSON.stringify(results.inventory.status_breakdown, null, 2)}</p>
                ` : ''}
                ${results.drift ? `
                    <h3>Drift Detection</h3>
                    <p>Items detected: ${results.drift.total_items}</p>
                    <p>Summary: ${results.drift.summary}</p>
                ` : ''}
            `;
        }
        
        // Global function for button
        window.analyzeFiles = analyzeFiles;
        
        // Initialize when page loads
        initializeWasm();
    </script>
</body>
</html>
```

### GitHub Actions - Repository Analysis

```javascript
// .github/actions/adr-scan/action.js
const core = require('@actions/core');
const github = require('@actions/github');
const fs = require('fs').promises;
const path = require('path');
const { ADRScan } = require('@adrscan/wasm');

async function run() {
    try {
        const adrDir = core.getInput('adr-dir') || 'docs/adr';
        const includePatterns = core.getInput('include-patterns')?.split(',') || 
            ['**/*.js', '**/*.ts', '**/*.json', '**/*.md'];
        
        const scanner = new ADRScan({ adrDir });
        
        // Read files from GitHub workspace
        const workspacePath = process.env.GITHUB_WORKSPACE;
        const files = await readWorkspaceFiles(workspacePath, includePatterns);
        
        // Separate ADRs from source files
        const adrFiles = {};
        const sourceFiles = {};
        
        for (const [filePath, content] of Object.entries(files)) {
            if (filePath.includes(adrDir) && filePath.endsWith('.md')) {
                adrFiles[filePath] = content;
            } else {
                sourceFiles[filePath] = content;
            }
        }
        
        // Run analysis
        const results = {};
        
        if (Object.keys(adrFiles).length > 0) {
            results.inventory = scanner.inventory(adrFiles);
        }
        
        if (Object.keys(sourceFiles).length > 0) {
            results.drift = scanner.detectDrift(sourceFiles);
            
            if (results.drift.total_items > 0) {
                results.proposals = await scanner.propose(results.drift);
            }
        }
        
        // Set outputs
        core.setOutput('adr-count', results.inventory?.total_count || 0);
        core.setOutput('drift-items', results.drift?.total_items || 0);
        core.setOutput('has-drift', (results.drift?.total_items || 0) > 0);
        
        // Create summary
        const summary = createSummary(results);
        core.setOutput('summary', summary);
        
        // Fail if drift detected and fail-on-drift is true
        const failOnDrift = core.getInput('fail-on-drift') === 'true';
        if (failOnDrift && results.drift?.total_items > 0) {
            core.setFailed(`Architectural drift detected: ${results.drift.total_items} items`);
        }
        
    } catch (error) {
        core.setFailed(`Action failed: ${error.message}`);
    }
}

async function readWorkspaceFiles(workspace, patterns) {
    // Implementation similar to Node.js example above
    // but with workspace-specific paths
}

function createSummary(results) {
    const parts = [];
    
    if (results.inventory) {
        parts.push(`📋 ADRs: ${results.inventory.total_count}`);
    }
    
    if (results.drift) {
        parts.push(`🔍 Drift: ${results.drift.total_items} items`);
    }
    
    if (results.proposals) {
        parts.push(`📝 Proposals: ${results.proposals.length}`);
    }
    
    return parts.join(' | ');
}

run();
```

## Performance Optimization

### Efficient File Reading

```javascript
// ✅ Good: Filter files before reading
async function readRelevantFiles(dir) {
    const files = {};
    const entries = await fs.readdir(dir, { withFileTypes: true });
    
    for (const entry of entries) {
        if (isRelevantFile(entry.name)) {  // Filter first
            const content = await fs.readFile(path.join(dir, entry.name), 'utf8');
            files[entry.name] = content;
        }
    }
    return files;
}

// ❌ Avoid: Reading all files then filtering
async function readAllFiles(dir) {
    const files = {};
    const entries = await fs.readdir(dir);
    
    for (const entry of entries) {
        const content = await fs.readFile(path.join(dir, entry), 'utf8');  // Wasteful
        if (isRelevantFile(entry)) {
            files[entry] = content;
        }
    }
    return files;
}
```

### Memory Management

```javascript
// Process files in batches for large repositories
async function processBatches(files, batchSize = 100) {
    const fileEntries = Object.entries(files);
    const results = [];
    
    for (let i = 0; i < fileEntries.length; i += batchSize) {
        const batch = fileEntries.slice(i, i + batchSize);
        const batchFiles = Object.fromEntries(batch);
        
        const result = scanner.detectDrift(batchFiles);
        results.push(result);
        
        // Allow garbage collection
        if (global.gc) global.gc();
    }
    
    return mergeDriftReports(results);
}
```

### Caching Strategies

```javascript
// Cache file contents with checksums
const crypto = require('crypto');

class FileCache {
    constructor() {
        this.cache = new Map();
    }
    
    async getFiles(paths) {
        const files = {};
        const toRead = [];
        
        for (const filePath of paths) {
            const stats = await fs.stat(filePath);
            const key = `${filePath}:${stats.mtime.getTime()}:${stats.size}`;
            
            if (this.cache.has(key)) {
                files[filePath] = this.cache.get(key);
            } else {
                toRead.push({ path: filePath, key });
            }
        }
        
        // Read uncached files
        for (const { path: filePath, key } of toRead) {
            const content = await fs.readFile(filePath, 'utf8');
            this.cache.set(key, content);
            files[filePath] = content;
        }
        
        return files;
    }
}
```

## Error Handling

### Robust File Reading

```javascript
async function safeReadFiles(paths) {
    const files = {};
    const errors = [];
    
    const results = await Promise.allSettled(
        paths.map(async (filePath) => {
            try {
                const content = await fs.readFile(filePath, 'utf8');
                return { filePath, content };
            } catch (err) {
                throw { filePath, error: err.message };
            }
        })
    );
    
    for (const result of results) {
        if (result.status === 'fulfilled') {
            const { filePath, content } = result.value;
            files[filePath] = content;
        } else {
            errors.push(result.reason);
        }
    }
    
    return { files, errors };
}
```

### Graceful Degradation

```javascript
function analyzeWithFallback(files) {
    try {
        return scanner.detectDrift(files);
    } catch (err) {
        console.warn('Drift detection failed, trying simplified analysis:', err.message);
        
        // Fallback to basic analysis
        return {
            timestamp: new Date().toISOString(),
            scanned_directory: '.',
            total_items: 0,
            summary: 'Analysis failed, manual review required'
        };
    }
}
```

## Best Practices

### ✅ Do

- **Filter files before reading** to minimize memory usage
- **Use relative paths** consistently from your project root
- **Implement caching** for frequently accessed files
- **Handle errors gracefully** with meaningful fallbacks
- **Process files in batches** for large repositories
- **Use specific file patterns** to avoid processing irrelevant files

### ❌ Don't

- **Don't read binary files** - WASM expects UTF-8 text content
- **Don't use absolute paths** - use relative paths for portability
- **Don't ignore errors** - always handle file reading failures
- **Don't process everything** - filter files by relevance
- **Don't assume file access** - always check for file existence
- **Don't forget encoding** - explicitly specify UTF-8 encoding

## Integration Patterns

### Pattern 1: File System Watcher

```javascript
// Watch for file changes and re-analyze
const chokidar = require('chokidar');

const watcher = chokidar.watch(['src/**/*.js', 'docs/adr/**/*.md']);
watcher.on('change', async (path) => {
    console.log(`File changed: ${path}`);
    
    const files = await readRelevantFiles('.');
    const drift = scanner.detectDrift(files);
    
    if (drift.total_items > 0) {
        console.log('⚠️ Drift detected:', drift.summary);
    }
});
```

### Pattern 2: Git Hook Integration

```javascript
// Pre-commit hook
const { execSync } = require('child_process');

async function preCommitHook() {
    // Get staged files
    const stagedFiles = execSync('git diff --cached --name-only', { encoding: 'utf8' })
        .split('\n')
        .filter(Boolean);
    
    const files = {};
    for (const filePath of stagedFiles) {
        if (isRelevantFile(filePath)) {
            files[filePath] = await fs.readFile(filePath, 'utf8');
        }
    }
    
    const drift = scanner.detectDrift(files);
    if (drift.total_items > 0) {
        console.error('❌ Commit blocked: architectural drift detected');
        process.exit(1);
    }
}
```

### Pattern 3: API Server Integration

```javascript
// Express.js API endpoint
app.post('/api/analyze', upload.array('files'), (req, res) => {
    try {
        const files = {};
        
        for (const file of req.files) {
            files[file.originalname] = file.buffer.toString('utf8');
        }
        
        const drift = scanner.detectDrift(files);
        const proposals = scanner.propose(drift);
        
        res.json({ drift, proposals });
    } catch (err) {
        res.status(500).json({ error: err.message });
    }
});
```

This comprehensive file I/O guide ensures that developers can effectively use the ADRScan WASM module across all supported environments while maintaining performance, reliability, and security.