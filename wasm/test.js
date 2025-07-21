/**
 * WASM Module Test Suite
 * 
 * Tests the WebAssembly module functionality
 */

const { ADRScan, utils } = require('./index.js');

async function runTests() {
    console.log('🧪 Starting WASM module tests...\n');
    
    // Test 1: Module loading and version
    try {
        console.log('✅ Test 1: Module Loading');
        console.log('   Version:', utils.version());
        console.log('   Features:', utils.features());
        console.log('');
    } catch (error) {
        console.error('❌ Test 1 Failed:', error.message);
        return false;
    }

    // Test 2: Configuration
    try {
        console.log('✅ Test 2: Configuration');
        const scanner = new ADRScan({
            adrDir: './docs/adr',
            templateFormat: 'madr',
            driftEnabled: true
        });
        
        const config = scanner.getConfig();
        console.log('   Config loaded successfully');
        console.log('');
    } catch (error) {
        console.error('❌ Test 2 Failed:', error.message);
        return false;
    }

    // Test 3: Template utilities
    try {
        console.log('✅ Test 3: Template Utilities');
        const template = utils.getDefaultTemplate();
        const isValid = utils.validateTemplate(template);
        console.log('   Template validation:', isValid ? 'PASS' : 'FAIL');
        console.log('   Template length:', template.length, 'characters');
        console.log('');
    } catch (error) {
        console.error('❌ Test 3 Failed:', error.message);
        return false;
    }

    // Test 4: Frontmatter parsing
    try {
        console.log('✅ Test 4: Frontmatter Parsing');
        const testContent = `---
title: Test ADR
status: accepted
date: 2024-01-01
---

# Test ADR

This is a test ADR.`;
        
        const parsed = utils.parseFrontmatter(testContent);
        console.log('   Frontmatter parsed successfully');
        console.log('');
    } catch (error) {
        console.error('❌ Test 4 Failed:', error.message);
        return false;
    }

    // Test 5: ADRScan instance methods
    try {
        console.log('✅ Test 5: ADRScan Methods');
        const scanner = new ADRScan();
        
        const initFiles = await scanner.init('./test-adr');
        console.log('   Init files:', Object.keys(initFiles).length);
        
        // Test parsing ADR content
        const adrContent = `---
title: Use MongoDB for data storage
status: accepted
date: 2024-01-01
---

# Use MongoDB for data storage

We will use MongoDB as our primary database.`;
        
        const parsedAdr = scanner.parseAdr(adrContent, 'test.md');
        console.log('   ADR parsing completed');
        
        // Test inventory with mock ADR files
        const mockAdrs = {
            'docs/adr/0001-use-mongodb.md': `---
title: Use MongoDB for data storage
status: accepted
date: 2024-01-01
tags: [database, storage]
---

# Use MongoDB for data storage

We will use MongoDB as our primary database.`,
            'docs/adr/0002-use-redis.md': `---
title: Use Redis for caching
status: proposed
date: 2024-01-02
tags: [cache, performance]
---

# Use Redis for caching

We will implement Redis for caching.`
        };
        
        const inventory = scanner.inventory(mockAdrs);
        console.log('   Inventory completed - ADRs:', inventory.total_count);
        console.log('   Status breakdown:', Object.keys(inventory.status_breakdown).length);
        
        // Test drift detection with mock files
        const mockFiles = {
            'src/database.js': 'const mongodb = require("mongodb");',
            'src/cache.js': 'const redis = require("redis");',
            'Dockerfile': 'FROM node:18'
        };
        
        const driftReport = scanner.detectDrift(mockFiles);
        console.log('   Drift detection completed, items:', driftReport.total_items);
        
        // Test diff with baseline
        const baselineFiles = {
            'src/database.js': 'const postgresql = require("pg");'
        };
        
        const diffReport = scanner.diff(mockFiles, baselineFiles);
        console.log('   Diff with baseline completed, items:', diffReport.total_items);
        
        // Test proposal generation
        const proposals = await scanner.propose(driftReport);
        console.log('   Generated proposals:', proposals.length);
        
        console.log('');
    } catch (error) {
        console.error('❌ Test 5 Failed:', error.message);
        return false;
    }

    console.log('🎉 All tests passed!\n');
    return true;
}

// Run tests if this file is executed directly
if (require.main === module) {
    runTests().then(success => {
        process.exit(success ? 0 : 1);
    }).catch(error => {
        console.error('💥 Test runner failed:', error);
        process.exit(1);
    });
}

module.exports = { runTests };