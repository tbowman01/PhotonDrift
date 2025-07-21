/**
 * TypeScript definitions for ADRScan WASM module
 */

export interface WasmConfig {
    adrDir?: string;
    templateFormat?: string;
    driftEnabled?: boolean;
}

export interface DriftReport {
    timestamp: string;
    scanned_directory: string;
    total_items: number;
    summary: string;
    to_json(): string;
}

export interface AdrProposal {
    number: string;
    title: string;
    status: string;
    context: string;
    decision: string;
}

export interface ParsedAdr {
    frontmatter: Record<string, any>;
    content: string;
    title?: string;
    status?: string;
    date?: string;
}

export interface InventoryResult {
    total_count: number;
    status_breakdown: Record<string, number>;
    tag_breakdown: Record<string, number>;
    adrs: AdrSummary[];
    statistics: InventoryStats;
}

export interface AdrSummary {
    path: string;
    title: string;
    status: string;
    date?: string;
    tags: string[];
    file_size: number;
    line_count: number;
}

export interface InventoryStats {
    total_files: number;
    total_size_bytes: number;
    total_lines: number;
    average_file_size: number;
    average_lines_per_adr: number;
}

/**
 * Main ADRScan WebAssembly interface
 */
export declare class ADRScan {
    constructor(config?: WasmConfig);

    /**
     * Initialize ADR directory structure
     */
    init(directory?: string): Promise<Record<string, string>>;

    /**
     * Parse ADR content
     */
    parseAdr(content: string, filename: string): ParsedAdr;

    /**
     * Perform inventory scan of ADRs
     */
    inventory(files: Record<string, string>): InventoryResult;

    /**
     * Detect architectural drift in files
     */
    detectDrift(files: Record<string, string>): DriftReport;

    /**
     * Compare current state against baseline (full diff functionality)
     */
    diff(currentFiles: Record<string, string>, baseline?: Record<string, string>): DriftReport;

    /**
     * Generate ADR proposals from drift report
     */
    propose(driftReport: DriftReport): Promise<AdrProposal[]>;

    /**
     * Get current configuration
     */
    getConfig(): Record<string, any>;

    /**
     * Update configuration
     */
    updateConfig(config: WasmConfig): void;
}

/**
 * Utility functions
 */
export declare const utils: {
    /**
     * Parse frontmatter from markdown content
     */
    parseFrontmatter(content: string): ParsedAdr;

    /**
     * Validate ADR template format
     */
    validateTemplate(template: string): boolean;

    /**
     * Get default MADR template
     */
    getDefaultTemplate(): string;

    /**
     * Get ADRScan version
     */
    version(): string;

    /**
     * Get supported features
     */
    features(): string[];
};

/**
 * Low-level WASM bindings (advanced use)
 */
export declare const wasm: any;

// Default export
declare const _default: {
    ADRScan: typeof ADRScan;
    utils: typeof utils;
    wasm: typeof wasm;
};

export default _default;