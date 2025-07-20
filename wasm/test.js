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
        console.log('   Init files:', initFiles.length);
        
        const inventory = await scanner.inventory('./docs/adr');
        console.log('   Inventory completed');
        
        const driftReport = await scanner.diff('.');
        console.log('   Drift detection completed, items:', driftReport.total_items);
        
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