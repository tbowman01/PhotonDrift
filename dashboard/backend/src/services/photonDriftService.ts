import { spawn, ChildProcess } from 'child_process';
import { EventEmitter } from 'events';
import { logger } from '../utils/logger.js';
import { DriftEvent, ScanOptions, ScanResult, ArchitectureHealth } from '../models/types.js';

export class PhotonDriftService extends EventEmitter {
    private cliPath: string;
    private activeScans: Map<string, ChildProcess> = new Map();

    constructor() {
        super();
        this.cliPath = process.env.PHOTONDRIFT_CLI_PATH || 'adrscan';
        logger.info(`PhotonDrift CLI initialized with path: ${this.cliPath}`);
    }

    /**
     * Run a drift detection scan
     */
    async scanForDrift(repositoryPath: string, options: ScanOptions = {}): Promise<ScanResult> {
        const scanId = `scan_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
        
        try {
            logger.info(`Starting drift scan`, { scanId, repositoryPath, options });
            
            const args = this.buildScanArgs(options);
            const result = await this.runCLICommand(['scan', ...args], repositoryPath, scanId);
            
            const driftEvents = this.parseDriftResults(result.stdout);
            const scanResult: ScanResult = {
                id: scanId,
                repositoryPath,
                timestamp: new Date(),
                driftEvents,
                summary: {
                    totalEvents: driftEvents.length,
                    criticalEvents: driftEvents.filter(e => e.severity === 'critical').length,
                    highEvents: driftEvents.filter(e => e.severity === 'high').length,
                    mediumEvents: driftEvents.filter(e => e.severity === 'medium').length,
                    lowEvents: driftEvents.filter(e => e.severity === 'low').length,
                    scanDuration: result.duration
                },
                options,
                success: true
            };

            // Emit scan completed event
            this.emit('scan:completed', scanResult);
            
            logger.info(`Drift scan completed`, { 
                scanId, 
                totalEvents: driftEvents.length,
                duration: result.duration 
            });

            return scanResult;

        } catch (error) {
            logger.error(`Drift scan failed`, { scanId, repositoryPath, error });
            
            const errorResult: ScanResult = {
                id: scanId,
                repositoryPath,
                timestamp: new Date(),
                driftEvents: [],
                summary: {
                    totalEvents: 0,
                    criticalEvents: 0,
                    highEvents: 0,
                    mediumEvents: 0,
                    lowEvents: 0,
                    scanDuration: 0
                },
                options,
                success: false,
                error: error instanceof Error ? error.message : 'Unknown error'
            };

            this.emit('scan:failed', errorResult);
            return errorResult;
        }
    }

    /**
     * Get repository health metrics
     */
    async getArchitectureHealth(repositoryPath: string): Promise<ArchitectureHealth> {
        try {
            logger.info(`Getting architecture health`, { repositoryPath });

            // Run health check command
            const result = await this.runCLICommand(['health', '--format', 'json'], repositoryPath);
            const healthData = JSON.parse(result.stdout);

            const health: ArchitectureHealth = {
                id: `health_${Date.now()}`,
                repository: repositoryPath,
                timestamp: new Date(),
                overallScore: healthData.overall_score || 0,
                metrics: {
                    driftCount: healthData.drift_count || 0,
                    coverage: healthData.adr_coverage || 0,
                    compliance: healthData.compliance_score || 0,
                    maintainability: healthData.maintainability || 0,
                    technicalDebt: healthData.technical_debt || 0
                },
                trends: {
                    direction: healthData.trend_direction || 'stable',
                    velocity: healthData.trend_velocity || 0
                }
            };

            logger.info(`Architecture health retrieved`, { repositoryPath, overallScore: health.overallScore });
            return health;

        } catch (error) {
            logger.error(`Failed to get architecture health`, { repositoryPath, error });
            
            // Return default health metrics on error
            return {
                id: `health_${Date.now()}`,
                repository: repositoryPath,
                timestamp: new Date(),
                overallScore: 0,
                metrics: {
                    driftCount: 0,
                    coverage: 0,
                    compliance: 0,
                    maintainability: 0,
                    technicalDebt: 0
                },
                trends: {
                    direction: 'stable',
                    velocity: 0
                }
            };
        }
    }

    /**
     * List available repositories/projects
     */
    async listRepositories(): Promise<string[]> {
        try {
            const result = await this.runCLICommand(['list', '--format', 'json']);
            const data = JSON.parse(result.stdout);
            return Array.isArray(data) ? data.map(item => item.path || item) : [];
        } catch (error) {
            logger.error('Failed to list repositories', { error });
            return [];
        }
    }

    /**
     * Initialize ADR structure in a repository
     */
    async initializeRepository(repositoryPath: string): Promise<boolean> {
        try {
            logger.info(`Initializing ADR structure`, { repositoryPath });
            
            await this.runCLICommand(['init'], repositoryPath);
            
            logger.info(`ADR structure initialized`, { repositoryPath });
            return true;
        } catch (error) {
            logger.error(`Failed to initialize repository`, { repositoryPath, error });
            return false;
        }
    }

    /**
     * Cancel an active scan
     */
    cancelScan(scanId: string): boolean {
        const process = this.activeScans.get(scanId);
        if (process) {
            process.kill('SIGTERM');
            this.activeScans.delete(scanId);
            logger.info(`Scan cancelled`, { scanId });
            return true;
        }
        return false;
    }

    /**
     * Get the status of active scans
     */
    getActiveScanStatus(): { scanId: string, pid?: number }[] {
        return Array.from(this.activeScans.entries()).map(([scanId, process]) => ({
            scanId,
            pid: process.pid
        }));
    }

    private buildScanArgs(options: ScanOptions): string[] {
        const args: string[] = [];

        if (options.format) {
            args.push('--format', options.format);
        } else {
            args.push('--format', 'json');
        }

        if (options.enableML) {
            args.push('--ml');
        }

        if (options.confidenceThreshold !== undefined) {
            args.push('--confidence', options.confidenceThreshold.toString());
        }

        if (options.includeFiles && options.includeFiles.length > 0) {
            args.push('--include', options.includeFiles.join(','));
        }

        if (options.excludeFiles && options.excludeFiles.length > 0) {
            args.push('--exclude', options.excludeFiles.join(','));
        }

        if (options.verbose) {
            args.push('--verbose');
        }

        return args;
    }

    private async runCLICommand(
        args: string[], 
        workingDir?: string, 
        scanId?: string
    ): Promise<{ stdout: string; stderr: string; duration: number }> {
        return new Promise((resolve, reject) => {
            const startTime = Date.now();
            
            const childProcess = spawn(this.cliPath, args, {
                cwd: workingDir || process.cwd(),
                stdio: ['ignore', 'pipe', 'pipe']
            });

            if (scanId) {
                this.activeScans.set(scanId, childProcess);
            }

            let stdout = '';
            let stderr = '';

            childProcess.stdout?.on('data', (data: any) => {
                stdout += data.toString();
            });

            childProcess.stderr?.on('data', (data: any) => {
                stderr += data.toString();
            });

            childProcess.on('close', (code: any) => {
                const duration = Date.now() - startTime;
                
                if (scanId) {
                    this.activeScans.delete(scanId);
                }

                if (code === 0) {
                    resolve({ stdout, stderr, duration });
                } else {
                    reject(new Error(`CLI command failed with code ${code}: ${stderr || stdout}`));
                }
            });

            childProcess.on('error', (error: any) => {
                if (scanId) {
                    this.activeScans.delete(scanId);
                }
                reject(new Error(`Failed to run CLI command: ${error.message}`));
            });
        });
    }

    private parseDriftResults(output: string): DriftEvent[] {
        try {
            const data = JSON.parse(output);
            
            if (Array.isArray(data)) {
                return data.map((item, index) => this.mapToDriftEvent(item, index));
            } else if (data.drift_events && Array.isArray(data.drift_events)) {
                return data.drift_events.map((item: any, index: number) => this.mapToDriftEvent(item, index));
            } else {
                return [];
            }
        } catch (error) {
            logger.warn('Failed to parse drift results as JSON, attempting text parsing', { error });
            return this.parseTextDriftResults(output);
        }
    }

    private mapToDriftEvent(item: any, index: number): DriftEvent {
        return {
            id: item.id || `drift_${Date.now()}_${index}`,
            timestamp: item.timestamp ? new Date(item.timestamp) : new Date(),
            severity: item.severity || 'medium',
            category: item.category || 'unknown',
            title: item.title || item.name || 'Unnamed drift',
            description: item.description || item.message || '',
            location: {
                file: item.location?.file || item.file || '',
                line: item.location?.line || item.line,
                column: item.location?.column || item.column
            },
            mlScore: item.ml_score || item.anomaly_score,
            confidence: item.confidence || item.ml_confidence,
            resolved: item.resolved || false,
            assignee: item.assignee,
            tags: Array.isArray(item.tags) ? item.tags : [],
            suggestion: item.suggestion || item.fix || item.recommendation
        };
    }

    private parseTextDriftResults(output: string): DriftEvent[] {
        const results: DriftEvent[] = [];
        const lines = output.split('\n');
        let currentItem: Partial<DriftEvent> = {};
        let index = 0;

        for (const line of lines) {
            const trimmed = line.trim();
            if (!trimmed) continue;

            if (trimmed.startsWith('Found drift:') || trimmed.startsWith('Drift detected:')) {
                if (currentItem.title) {
                    results.push(this.finalizeDriftEvent(currentItem, index++));
                    currentItem = {};
                }
                currentItem.title = trimmed.replace(/^(Found drift:|Drift detected:)\s*/, '');
            } else if (trimmed.startsWith('Severity:')) {
                currentItem.severity = trimmed.replace('Severity:', '').trim() as any;
            } else if (trimmed.startsWith('Category:')) {
                currentItem.category = trimmed.replace('Category:', '').trim();
            } else if (trimmed.startsWith('File:')) {
                currentItem.location = { file: trimmed.replace('File:', '').trim() };
            } else if (trimmed.startsWith('Description:')) {
                currentItem.description = trimmed.replace('Description:', '').trim();
            }
        }

        if (currentItem.title) {
            results.push(this.finalizeDriftEvent(currentItem, index));
        }

        return results;
    }

    private finalizeDriftEvent(item: Partial<DriftEvent>, index: number): DriftEvent {
        return {
            id: `drift_${Date.now()}_${index}`,
            timestamp: new Date(),
            severity: (item.severity as any) || 'medium',
            category: item.category || 'unknown',
            title: item.title || 'Unnamed drift',
            description: item.description || '',
            location: item.location || { file: '' },
            resolved: false,
            tags: []
        };
    }
}