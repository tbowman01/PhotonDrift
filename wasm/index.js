/**
 * ADRScan WebAssembly Module
 * 
 * High-level JavaScript interface for the ADRScan WASM module.
 * Provides convenient functions for ADR management and drift detection.
 */

const wasm = require('./adrscan');

/**
 * ADRScan class - Main interface for WebAssembly functionality
 */
class ADRScan {
    constructor(config = {}) {
        const wasmConfig = new wasm.WasmConfig();
        
        if (config.adrDir) wasmConfig.adr_dir = config.adrDir;
        if (config.templateFormat) wasmConfig.template_format = config.templateFormat;
        if (config.driftEnabled !== undefined) wasmConfig.drift_enabled = config.driftEnabled;
        
        this.instance = new wasm.AdrscanWasm(wasmConfig);
    }

    /**
     * Initialize ADR directory structure
     * @param {string} directory - Target directory path
     * @returns {Promise<string[]>} - List of files to create
     */
    async init(directory = './docs/adr') {
        const result = await this.instance.init(directory);
        return JSON.parse(result);
    }

    /**
     * Perform inventory scan of ADRs
     * @param {string} directory - Directory to scan
     * @returns {Promise<Object>} - Inventory results
     */
    async inventory(directory = './docs/adr') {
        const result = await this.instance.inventory(directory);
        return JSON.parse(result);
    }

    /**
     * Detect architectural drift
     * @param {string} directory - Directory to analyze
     * @param {string} [baseline] - Optional baseline snapshot
     * @returns {Promise<DriftReport>} - Drift detection results
     */
    async diff(directory = '.', baseline = null) {
        return await this.instance.diff(directory, baseline);
    }

    /**
     * Generate ADR proposals from drift
     * @param {DriftReport} driftReport - Drift detection results
     * @returns {Promise<string[]>} - Generated ADR proposals
     */
    async propose(driftReport) {
        const result = await this.instance.propose(driftReport);
        return JSON.parse(result);
    }

    /**
     * Get current configuration
     * @returns {Object} - Current configuration
     */
    getConfig() {
        return this.instance.get_config();
    }

    /**
     * Update configuration
     * @param {Object} config - New configuration
     */
    updateConfig(config) {
        const wasmConfig = new wasm.WasmConfig();
        Object.assign(wasmConfig, config);
        this.instance.update_config(wasmConfig);
    }
}

/**
 * Utility functions
 */
const utils = {
    /**
     * Parse ADR frontmatter from markdown content
     * @param {string} content - Markdown content
     * @returns {Object} - Parsed ADR metadata
     */
    parseFrontmatter(content) {
        return wasm.WasmUtils.parse_frontmatter(content);
    },

    /**
     * Validate ADR template format
     * @param {string} template - Template content
     * @returns {boolean} - Whether template is valid
     */
    validateTemplate(template) {
        return wasm.WasmUtils.validate_template(template);
    },

    /**
     * Get default MADR template
     * @returns {string} - Default template content
     */
    getDefaultTemplate() {
        return wasm.WasmUtils.get_default_template();
    },

    /**
     * Get ADRScan version
     * @returns {string} - Version string
     */
    version() {
        return wasm.version();
    },

    /**
     * Get supported features
     * @returns {string[]} - List of supported features
     */
    features() {
        return wasm.features();
    }
};

module.exports = {
    ADRScan,
    utils,
    // Re-export low-level WASM bindings for advanced use
    wasm
};