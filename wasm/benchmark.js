#!/usr/bin/env node
/**
 * WASM Performance Benchmark for ADRScan
 * Tests various operations and measures performance
 */

const fs = require('fs');
const path = require('path');
const { performance } = require('perf_hooks');

// Try to load the WASM module
let wasmModule;
try {
    wasmModule = require('./index.js');
} catch (error) {
    console.error('❌ Failed to load WASM module:', error.message);
    console.log('💡 Run "npm run build" first to generate the WASM module');
    process.exit(1);
}

const { AdrscanWasm, WasmConfig, WasmUtils } = wasmModule;

// Performance measurement utilities
class PerformanceTracker {
    constructor() {
        this.measurements = new Map();
    }
    
    start(name) {
        this.measurements.set(name, performance.now());
    }
    
    end(name) {
        const startTime = this.measurements.get(name);
        if (!startTime) throw new Error(`No start time found for ${name}`);
        
        const duration = performance.now() - startTime;
        this.measurements.delete(name);
        return duration;
    }
    
    measure(name, fn) {
        this.start(name);
        const result = fn();
        const duration = this.end(name);
        return { result, duration };
    }
    
    async measureAsync(name, fn) {
        this.start(name);
        const result = await fn();
        const duration = this.end(name);
        return { result, duration };
    }
}

// Test data generation
function generateTestFiles(count = 100) {
    const files = {};
    const technologies = ['mongodb', 'redis', 'postgresql', 'docker', 'kubernetes', 'react', 'nodejs'];
    
    for (let i = 1; i <= count; i++) {
        const tech = technologies[i % technologies.length];
        const filename = `docs/adr/${String(i).padStart(4, '0')}-use-${tech}.md`;
        
        files[filename] = `---
title: Use ${tech} for ${tech === 'mongodb' ? 'database' : tech === 'redis' ? 'caching' : 'infrastructure'}
status: ${i % 3 === 0 ? 'accepted' : i % 3 === 1 ? 'proposed' : 'deprecated'}
date: 2024-${String(Math.floor(i / 30) + 1).padStart(2, '0')}-${String((i % 30) + 1).padStart(2, '0')}
tags: [${tech}, architecture, infrastructure]
---

# Use ${tech} for ${tech === 'mongodb' ? 'Database' : tech === 'redis' ? 'Caching' : 'Infrastructure'}

## Status

${i % 3 === 0 ? 'Accepted' : i % 3 === 1 ? 'Proposed' : 'Deprecated'}

## Context

We need to choose a technology for our ${tech === 'mongodb' ? 'data persistence' : tech === 'redis' ? 'caching layer' : 'infrastructure'}.

## Decision

We will use ${tech} because:

- Performance benefits
- Scalability
- Community support
- Integration capabilities
- ${'x'.repeat(Math.floor(Math.random() * 500) + 100)} // Variable content size

## Consequences

### Positive

- Improved performance
- Better scalability
- Reduced complexity

### Negative

- Learning curve
- Additional dependencies
- Maintenance overhead

## Implementation

\`\`\`javascript
// Example ${tech} usage
const ${tech}Client = require('${tech}');
const client = new ${tech}Client(${JSON.stringify({ host: 'localhost', port: tech === 'mongodb' ? 27017 : tech === 'redis' ? 6379 : 8080 })});
\`\`\`

Additional implementation details...
${'Implementation details '.repeat(Math.floor(Math.random() * 20) + 5)}
`;
    }
    
    return files;
}

// Benchmark tests
async function runBenchmarks() {
    console.log('🚀 Starting ADRScan WASM Performance Benchmarks\n');
    
    const tracker = new PerformanceTracker();
    const results = [];
    
    // Test 1: Module initialization
    console.log('📋 Test 1: Module Initialization');
    const { result: config, duration: initDuration } = tracker.measure('initialization', () => {
        return new WasmConfig();
    });
    results.push({ test: 'Module Initialization', duration: initDuration, unit: 'ms' });
    console.log(`   ✅ Completed in ${initDuration.toFixed(2)}ms\n`);
    
    // Test 2: ADRScan instance creation
    console.log('📋 Test 2: ADRScan Instance Creation');
    const { result: scanner, duration: createDuration } = tracker.measure('instance_creation', () => {
        return new AdrscanWasm(config);
    });
    results.push({ test: 'Instance Creation', duration: createDuration, unit: 'ms' });
    console.log(`   ✅ Completed in ${createDuration.toFixed(2)}ms\n`);
    
    // Test 3: Small file parsing (single ADR)
    console.log('📋 Test 3: Single ADR Parsing');
    const sampleAdr = `---
title: Sample ADR
status: accepted
date: 2024-01-01
tags: [test, benchmark]
---

# Sample ADR

This is a test ADR for benchmarking purposes.

## Decision

We will use this for testing.
`;
    
    const { result: parsedAdr, duration: parseDuration } = tracker.measure('single_parse', () => {
        return scanner.parse_adr(sampleAdr, 'test.md');
    });
    results.push({ test: 'Single ADR Parse', duration: parseDuration, unit: 'ms' });
    console.log(`   ✅ Completed in ${parseDuration.toFixed(2)}ms\n`);
    
    // Test 4: Drift detection on small dataset
    console.log('📋 Test 4: Small Dataset Drift Detection (10 files)');
    const smallFiles = generateTestFiles(10);
    const { result: smallDrift, duration: smallDriftDuration } = tracker.measure('small_drift', () => {
        return scanner.detect_drift(JSON.stringify(smallFiles));
    });
    results.push({ test: 'Small Drift Detection (10 files)', duration: smallDriftDuration, unit: 'ms' });
    console.log(`   ✅ Detected ${smallDrift.total_items} drift items in ${smallDriftDuration.toFixed(2)}ms\n`);
    
    // Test 5: Inventory on medium dataset
    console.log('📋 Test 5: Medium Dataset Inventory (50 files)');
    const mediumFiles = generateTestFiles(50);
    const { result: mediumInventory, duration: mediumInventoryDuration } = tracker.measure('medium_inventory', () => {
        return scanner.inventory(JSON.stringify(mediumFiles));
    });
    const inventoryResult = JSON.parse(mediumInventory);
    results.push({ test: 'Medium Inventory (50 files)', duration: mediumInventoryDuration, unit: 'ms' });
    console.log(`   ✅ Processed ${inventoryResult.total_count} ADRs in ${mediumInventoryDuration.toFixed(2)}ms\n`);
    
    // Test 6: Large dataset drift detection
    console.log('📋 Test 6: Large Dataset Drift Detection (200 files)');
    const largeFiles = generateTestFiles(200);
    const { result: largeDrift, duration: largeDriftDuration } = tracker.measure('large_drift', () => {
        return scanner.detect_drift(JSON.stringify(largeFiles));
    });
    results.push({ test: 'Large Drift Detection (200 files)', duration: largeDriftDuration, unit: 'ms' });
    console.log(`   ✅ Detected ${largeDrift.total_items} drift items in ${largeDriftDuration.toFixed(2)}ms\n`);
    
    // Test 7: Proposal generation
    console.log('📋 Test 7: Proposal Generation');
    const { result: proposals, duration: proposalDuration } = tracker.measure('proposals', () => {
        return scanner.propose(largeDrift);
    });
    const proposalResult = JSON.parse(proposals);
    results.push({ test: 'Proposal Generation', duration: proposalDuration, unit: 'ms' });
    console.log(`   ✅ Generated ${proposalResult.length} proposals in ${proposalDuration.toFixed(2)}ms\n`);
    
    // Test 8: Memory usage simulation
    console.log('📋 Test 8: Memory Usage Simulation');
    const memoryBefore = process.memoryUsage();
    
    // Create multiple instances and process data
    const instances = [];
    for (let i = 0; i < 10; i++) {
        instances.push(new AdrscanWasm(config));
    }
    
    // Process data with all instances
    const { duration: memoryTestDuration } = tracker.measure('memory_test', () => {
        instances.forEach((instance, idx) => {
            const testFiles = generateTestFiles(20);
            instance.detect_drift(JSON.stringify(testFiles));
        });
    });
    
    const memoryAfter = process.memoryUsage();
    const memoryIncrease = memoryAfter.heapUsed - memoryBefore.heapUsed;
    
    results.push({ test: 'Memory Test (10 instances)', duration: memoryTestDuration, unit: 'ms', memory: memoryIncrease });
    console.log(`   ✅ Completed in ${memoryTestDuration.toFixed(2)}ms`);
    console.log(`   📊 Memory increase: ${(memoryIncrease / 1024 / 1024).toFixed(2)}MB\n`);
    
    // Test 9: Utility functions
    console.log('📋 Test 9: Utility Functions');
    const { duration: utilDuration } = tracker.measure('utilities', () => {
        WasmUtils.parse_frontmatter(sampleAdr);
        WasmUtils.validate_template(sampleAdr);
        WasmUtils.get_default_template();
    });
    results.push({ test: 'Utility Functions', duration: utilDuration, unit: 'ms' });
    console.log(`   ✅ Completed in ${utilDuration.toFixed(2)}ms\n`);
    
    // Performance summary
    console.log('📊 Performance Summary');
    console.log('=' .repeat(80));
    console.log('Test Name                              | Duration (ms) | Throughput');
    console.log('-' .repeat(80));
    
    results.forEach(result => {
        const name = result.test.padEnd(38);
        const duration = result.duration.toFixed(2).padStart(11);
        
        let throughput = '';
        if (result.test.includes('10 files')) {
            throughput = `${(10 / result.duration * 1000).toFixed(0)} files/sec`;
        } else if (result.test.includes('50 files')) {
            throughput = `${(50 / result.duration * 1000).toFixed(0)} files/sec`;
        } else if (result.test.includes('200 files')) {
            throughput = `${(200 / result.duration * 1000).toFixed(0)} files/sec`;
        }
        
        console.log(`${name} | ${duration} | ${throughput}`);
    });
    
    console.log('-' .repeat(80));
    
    // Calculate overall performance metrics
    const totalDuration = results.reduce((sum, r) => sum + r.duration, 0);
    const avgDuration = totalDuration / results.length;
    
    console.log(`Total execution time: ${totalDuration.toFixed(2)}ms`);
    console.log(`Average test duration: ${avgDuration.toFixed(2)}ms`);
    
    // Performance rating
    const rating = avgDuration < 10 ? 'Excellent' : 
                  avgDuration < 50 ? 'Good' : 
                  avgDuration < 100 ? 'Fair' : 'Needs Optimization';
    console.log(`Performance rating: ${rating}`);
    
    // Recommendations
    console.log('\n💡 Optimization Recommendations:');
    if (avgDuration > 50) {
        console.log('  - Consider reducing WASM module size');
        console.log('  - Optimize frequently called functions');
        console.log('  - Use wasm-opt for further optimizations');
    }
    if (results.find(r => r.memory && r.memory > 50 * 1024 * 1024)) {
        console.log('  - Memory usage is high, consider optimizing allocations');
        console.log('  - Use wee_alloc for smaller memory footprint');
    }
    if (results.some(r => r.duration > 100)) {
        console.log('  - Some operations are slow, profile with browser dev tools');
        console.log('  - Consider breaking large operations into smaller chunks');
    }
    
    console.log('\n🎉 Benchmark completed successfully!');
    
    // Save results to file
    const resultsFile = path.join(__dirname, 'benchmark-results.json');
    fs.writeFileSync(resultsFile, JSON.stringify({
        timestamp: new Date().toISOString(),
        results,
        summary: {
            totalDuration,
            avgDuration,
            rating
        }
    }, null, 2));
    
    console.log(`📄 Results saved to: ${resultsFile}`);
}

// Run benchmarks if this file is executed directly
if (require.main === module) {
    runBenchmarks().catch(error => {
        console.error('❌ Benchmark failed:', error);
        process.exit(1);
    });
}

module.exports = { runBenchmarks, generateTestFiles, PerformanceTracker };