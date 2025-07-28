#!/usr/bin/env node
/**
 * CI/CD Integration Example for ADRScan WASM
 * 
 * This example demonstrates how to use ADRScan WASM module in CI/CD pipelines
 * for automated architectural drift detection and ADR compliance.
 */

const fs = require('fs').promises;
const path = require('path');
const { ADRScan, utils } = require('../wasm');

class CiCdIntegration {
    constructor(options = {}) {
        this.scanner = new ADRScan({
            adrDir: options.adrDir || './docs/adr',
            templateFormat: options.templateFormat || 'madr',
            driftEnabled: options.driftEnabled !== false
        });
        
        this.baselineFile = options.baselineFile || '.adrscan-baseline.json';
        this.exitOnDrift = options.exitOnDrift !== false;
        this.verbose = options.verbose || false;
    }

    /**
     * Read all files in a directory recursively
     */
    async readFilesRecursively(dir, patterns = ['**/*.js', '**/*.ts', '**/*.json', '**/*.md']) {
        const files = {};
        
        async function readDir(currentDir) {
            const entries = await fs.readdir(currentDir, { withFileTypes: true });
            
            for (const entry of entries) {
                const fullPath = path.join(currentDir, entry.name);
                const relativePath = path.relative(process.cwd(), fullPath);
                
                // Skip node_modules and other common ignore patterns
                if (entry.name === 'node_modules' || entry.name === '.git' || entry.name.startsWith('.')) {
                    continue;
                }
                
                if (entry.isDirectory()) {
                    await readDir(fullPath);
                } else if (entry.isFile()) {
                    // Check if file matches patterns
                    const shouldInclude = patterns.some(pattern => {
                        const regex = new RegExp(pattern.replace(/\*\*/g, '.*').replace(/\*/g, '[^/]*'));
                        return regex.test(relativePath);
                    });
                    
                    if (shouldInclude) {
                        try {
                            files[relativePath] = await fs.readFile(fullPath, 'utf8');
                        } catch (err) {
                            console.warn(`Warning: Could not read ${relativePath}: ${err.message}`);
                        }
                    }
                }
            }
        }
        
        await readDir(dir);
        return files;
    }

    /**
     * Load baseline from file
     */
    async loadBaseline() {
        try {
            const baselineData = await fs.readFile(this.baselineFile, 'utf8');
            return JSON.parse(baselineData);
        } catch (err) {
            if (this.verbose) {
                console.log(`No baseline found at ${this.baselineFile}, will create new one`);
            }
            return null;
        }
    }

    /**
     * Save current state as baseline
     */
    async saveBaseline(files) {
        const baseline = {
            timestamp: new Date().toISOString(),
            files: files,
            metadata: {
                version: utils.version(),
                file_count: Object.keys(files).length,
                total_size: Object.values(files).reduce((sum, content) => sum + content.length, 0)
            }
        };
        
        await fs.writeFile(this.baselineFile, JSON.stringify(baseline, null, 2));
        console.log(`✅ Baseline saved to ${this.baselineFile}`);
    }

    /**
     * Run ADR inventory check
     */
    async runInventoryCheck() {
        console.log('\n📋 Running ADR Inventory Check...');
        
        try {
            const adrFiles = await this.readFilesRecursively('./docs/adr', ['**/*.md']);
            
            if (Object.keys(adrFiles).length === 0) {
                console.log('⚠️  No ADR files found');
                return { success: false, reason: 'no_adrs' };
            }
            
            const inventory = this.scanner.inventory(adrFiles);
            
            console.log(`📊 ADR Inventory Results:`);
            console.log(`   Total ADRs: ${inventory.total_count}`);
            console.log(`   Status breakdown:`);
            
            for (const [status, count] of Object.entries(inventory.status_breakdown)) {
                console.log(`     - ${status}: ${count}`);
            }
            
            if (inventory.tag_breakdown && Object.keys(inventory.tag_breakdown).length > 0) {
                console.log(`   Most common tags:`);
                const sortedTags = Object.entries(inventory.tag_breakdown)
                    .sort(([,a], [,b]) => b - a)
                    .slice(0, 5);
                    
                for (const [tag, count] of sortedTags) {
                    console.log(`     - ${tag}: ${count}`);
                }
            }
            
            console.log(`   Statistics:`);
            console.log(`     - Average file size: ${inventory.statistics.average_file_size.toFixed(0)} bytes`);
            console.log(`     - Average lines per ADR: ${inventory.statistics.average_lines_per_adr.toFixed(0)}`);
            
            return { success: true, inventory };
            
        } catch (err) {
            console.error(`❌ Inventory check failed: ${err.message}`);
            return { success: false, reason: 'error', error: err.message };
        }
    }

    /**
     * Run architectural drift detection
     */
    async runDriftDetection() {
        console.log('\n🔍 Running Architectural Drift Detection...');
        
        try {
            const currentFiles = await this.readFilesRecursively('.', ['**/*.js', '**/*.ts', '**/*.json', '**/Dockerfile', '**/*.yml', '**/*.yaml']);
            const baseline = await this.loadBaseline();
            
            let driftReport;
            
            if (baseline) {
                console.log(`📊 Comparing against baseline from ${baseline.timestamp}`);
                driftReport = this.scanner.diff(currentFiles, baseline.files);
            } else {
                console.log('📊 Running drift detection without baseline');
                driftReport = this.scanner.detectDrift(currentFiles);
            }
            
            console.log(`🎯 Drift Detection Results:`);
            console.log(`   Items detected: ${driftReport.total_items}`);
            
            if (driftReport.total_items > 0) {
                console.log(`   Summary: ${driftReport.summary}`);
                
                // Generate proposals
                try {
                    const proposals = await this.scanner.propose(driftReport);
                    
                    if (proposals.length > 0) {
                        console.log(`\n📝 Suggested ADRs (${proposals.length}):`);
                        proposals.forEach((proposal, index) => {
                            console.log(`   ${index + 1}. ${proposal.title}`);
                            console.log(`      Status: ${proposal.status}`);
                            console.log(`      Context: ${proposal.context}`);
                        });
                    }
                } catch (err) {
                    console.warn(`⚠️  Could not generate proposals: ${err.message}`);
                }
                
                return { success: false, driftReport, currentFiles };
            } else {
                console.log('✅ No architectural drift detected');
                return { success: true, driftReport, currentFiles };
            }
            
        } catch (err) {
            console.error(`❌ Drift detection failed: ${err.message}`);
            return { success: false, reason: 'error', error: err.message };
        }
    }

    /**
     * Main CI/CD check runner
     */
    async run() {
        console.log(`🚀 ADRScan CI/CD Integration`);
        console.log(`   Version: ${utils.version()}`);
        console.log(`   Features: ${utils.features().join(', ')}`);
        
        // Run inventory check
        const inventoryResult = await this.runInventoryCheck();
        
        // Run drift detection
        const driftResult = await this.runDriftDetection();
        
        // Overall results
        console.log('\n📋 CI/CD Check Summary:');
        console.log(`   ADR Inventory: ${inventoryResult.success ? '✅ PASS' : '❌ FAIL'}`);
        console.log(`   Drift Detection: ${driftResult.success ? '✅ PASS' : '❌ FAIL'}`);
        
        // Update baseline if requested
        if (process.argv.includes('--save-baseline') && driftResult.currentFiles) {
            await this.saveBaseline(driftResult.currentFiles);
        }
        
        // Exit with error code if drift detected and exit-on-drift is enabled
        if (this.exitOnDrift && !driftResult.success && driftResult.driftReport) {
            console.error(`\n❌ CI/CD FAILED: Architectural drift detected (${driftResult.driftReport.total_items} items)`);
            process.exit(1);
        }
        
        if (!inventoryResult.success && inventoryResult.reason === 'no_adrs') {
            console.error('\n❌ CI/CD FAILED: No ADR files found');
            process.exit(1);
        }
        
        console.log('\n✅ CI/CD checks completed successfully');
        return {
            inventory: inventoryResult,
            drift: driftResult
        };
    }
}

// CLI Interface
async function main() {
    const args = process.argv.slice(2);
    
    const options = {
        adrDir: args.includes('--adr-dir') ? args[args.indexOf('--adr-dir') + 1] : './docs/adr',
        baselineFile: args.includes('--baseline') ? args[args.indexOf('--baseline') + 1] : '.adrscan-baseline.json',
        exitOnDrift: !args.includes('--no-exit-on-drift'),
        verbose: args.includes('--verbose')
    };
    
    if (args.includes('--help')) {
        console.log(`
ADRScan CI/CD Integration

Usage: node ci-cd-integration.js [options]

Options:
  --adr-dir <path>        ADR directory (default: ./docs/adr)
  --baseline <file>       Baseline file (default: .adrscan-baseline.json)
  --save-baseline         Save current state as new baseline
  --no-exit-on-drift      Don't exit with error code on drift detection
  --verbose               Verbose output
  --help                  Show this help

Examples:
  # Run full check
  node ci-cd-integration.js
  
  # Create new baseline
  node ci-cd-integration.js --save-baseline
  
  # Check with custom ADR directory
  node ci-cd-integration.js --adr-dir ./architecture/decisions
        `);
        return;
    }
    
    const integration = new CiCdIntegration(options);
    await integration.run();
}

// GitHub Actions Integration
function createGitHubActionsWorkflow() {
    return `
name: ADR Compliance Check

on:
  pull_request:
    branches: [main, develop]
  push:
    branches: [main, develop]

jobs:
  adr-compliance:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0  # Full history for baseline comparison
    
    - name: Setup Node.js
      uses: actions/setup-node@v4
      with:
        node-version: '18'
    
    - name: Install ADRScan WASM
      run: npm install @adrscan/wasm
    
    - name: Download CI/CD integration script
      run: |
        curl -o ci-cd-integration.js \\
          https://raw.githubusercontent.com/tbowman01/PhotonDrift/main/examples/ci-cd-integration.js
        chmod +x ci-cd-integration.js
    
    - name: Restore baseline from cache
      uses: actions/cache@v4
      with:
        path: .adrscan-baseline.json
        key: adr-baseline-\${{ runner.os }}-\${{ github.ref }}
        restore-keys: |
          adr-baseline-\${{ runner.os }}-
    
    - name: Run ADR compliance check
      run: node ci-cd-integration.js --verbose
    
    - name: Save baseline (on main branch)
      if: github.ref == 'refs/heads/main'
      run: node ci-cd-integration.js --save-baseline
    
    - name: Upload artifacts on failure
      if: failure()
      uses: actions/upload-artifact@v4
      with:
        name: adr-compliance-report
        path: |
          .adrscan-baseline.json
          adr-compliance-*.json
`;
}

// Run if called directly
if (require.main === module) {
    main().catch(err => {
        console.error('💥 Unexpected error:', err);
        process.exit(1);
    });
}

module.exports = { CiCdIntegration, createGitHubActionsWorkflow };