import * as vscode from 'vscode';
import { spawn, ChildProcess } from 'child_process';
import * as path from 'path';
import * as fs from 'fs';

export interface DriftResult {
    id: string;
    title: string;
    severity: string;
    category: string;
    description: string;
    suggestion?: string;
    location?: {
        file: string;
        line?: number;
        column?: number;
    };
    confidence?: number;
    mlScore?: number;
}

export interface ADRInventoryItem {
    title: string;
    status: string;
    date: string;
    file: string;
    summary?: string;
}

export interface ADRProposal {
    title: string;
    context: string;
    decision: string;
    consequences: string;
    confidence: string;
}

export class PhotonDriftCLI {
    private executablePath: string;

    constructor() {
        this.executablePath = this.getExecutablePath();
    }

    private getExecutablePath(): string {
        const config = vscode.workspace.getConfiguration('photondrift');
        const configPath = config.get<string>('executable', 'adrscan');
        
        // Check if it's a relative path and resolve it
        if (!path.isAbsolute(configPath)) {
            const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
            if (workspaceFolder) {
                const workspacePath = path.join(workspaceFolder.uri.fsPath, configPath);
                if (fs.existsSync(workspacePath)) {
                    return workspacePath;
                }
            }
        }
        
        return configPath;
    }

    private async runCommand(args: string[], workingDir: string, token?: vscode.CancellationToken): Promise<string> {
        return new Promise((resolve, reject) => {
            const process = spawn(this.executablePath, args, {
                cwd: workingDir,
                stdio: ['ignore', 'pipe', 'pipe']
            });

            let stdout = '';
            let stderr = '';

            process.stdout.on('data', (data) => {
                stdout += data.toString();
            });

            process.stderr.on('data', (data) => {
                stderr += data.toString();
            });

            process.on('close', (code) => {
                if (code === 0) {
                    resolve(stdout);
                } else {
                    reject(new Error(`Command failed with code ${code}: ${stderr || stdout}`));
                }
            });

            process.on('error', (error) => {
                reject(new Error(`Failed to run command: ${error.message}`));
            });

            // Handle cancellation
            if (token) {
                token.onCancellationRequested(() => {
                    process.kill();
                    reject(new Error('Command cancelled'));
                });
            }
        });
    }

    async init(workspaceDir: string): Promise<void> {
        await this.runCommand(['init'], workspaceDir);
    }

    async createAdr(workspaceDir: string, title: string): Promise<string> {
        const config = vscode.workspace.getConfiguration('photondrift');
        const template = config.get<string>('templateFormat', 'madr');
        
        const output = await this.runCommand(['new', title, '--template', template], workspaceDir);
        
        // Parse output to get the created file path
        const lines = output.split('\n');
        for (const line of lines) {
            if (line.includes('Created:') || line.includes('created:')) {
                const match = line.match(/([^\s]+\.adr\.md)/);
                if (match) {
                    return path.resolve(workspaceDir, match[1]);
                }
            }
        }
        
        // Fallback: construct expected path
        const adrDir = config.get<string>('adrDirectory', 'docs/adr');
        const slug = title.toLowerCase().replace(/[^a-z0-9]+/g, '-');
        const timestamp = new Date().toISOString().split('T')[0];
        return path.join(workspaceDir, adrDir, `${timestamp}-${slug}.adr.md`);
    }

    async scanForDrift(workspaceDir: string, token?: vscode.CancellationToken): Promise<DriftResult[]> {
        const config = vscode.workspace.getConfiguration('photondrift');
        const enableML = config.get<boolean>('enableMLFeatures', true);
        const confidenceThreshold = config.get<number>('confidenceThreshold', 0.7);

        const args = ['scan'];
        if (enableML) {
            args.push('--ml');
            args.push('--confidence', confidenceThreshold.toString());
        }
        args.push('--format', 'json');

        const output = await this.runCommand(args, workspaceDir, token);
        
        try {
            const results = JSON.parse(output);
            return this.parseDriftResults(results);
        } catch (error) {
            // Fallback: parse text output
            return this.parseTextDriftResults(output);
        }
    }

    async scanSingleFile(filePath: string): Promise<DriftResult[]> {
        const workspaceDir = path.dirname(filePath);
        const relativePath = path.relative(workspaceDir, filePath);
        
        try {
            const output = await this.runCommand(['scan', relativePath, '--format', 'json'], workspaceDir);
            const results = JSON.parse(output);
            return this.parseDriftResults(results);
        } catch (error) {
            console.error('Single file scan failed:', error);
            return [];
        }
    }

    async generateIndex(workspaceDir: string): Promise<string> {
        await this.runCommand(['index'], workspaceDir);
        
        const config = vscode.workspace.getConfiguration('photondrift');
        const adrDir = config.get<string>('adrDirectory', 'docs/adr');
        return path.join(workspaceDir, adrDir, 'README.md');
    }

    async getInventory(workspaceDir: string): Promise<ADRInventoryItem[]> {
        const output = await this.runCommand(['list', '--format', 'json'], workspaceDir);
        
        try {
            const results = JSON.parse(output);
            return this.parseInventoryResults(results);
        } catch (error) {
            // Fallback: parse text output
            return this.parseTextInventoryResults(output);
        }
    }

    async proposeAdr(workspaceDir: string, context: string): Promise<ADRProposal> {
        const args = ['propose'];
        
        // Write context to temporary file
        const tempFile = path.join(workspaceDir, '.photondrift-context.tmp');
        fs.writeFileSync(tempFile, context);
        
        try {
            const output = await this.runCommand([...args, '--context-file', tempFile, '--format', 'json'], workspaceDir);
            
            try {
                const result = JSON.parse(output);
                return this.parseProposalResult(result);
            } catch (error) {
                return this.parseTextProposalResult(output);
            }
        } finally {
            // Clean up temp file
            if (fs.existsSync(tempFile)) {
                fs.unlinkSync(tempFile);
            }
        }
    }

    async generateReport(workspaceDir: string, driftResults: DriftResult[]): Promise<string> {
        // Write drift results to temporary file
        const tempFile = path.join(workspaceDir, '.photondrift-results.tmp');
        fs.writeFileSync(tempFile, JSON.stringify(driftResults, null, 2));
        
        try {
            await this.runCommand(['report', '--input', tempFile], workspaceDir);
            return path.join(workspaceDir, 'drift-report.md');
        } finally {
            // Clean up temp file
            if (fs.existsSync(tempFile)) {
                fs.unlinkSync(tempFile);
            }
        }
    }

    private parseDriftResults(results: any): DriftResult[] {
        if (!Array.isArray(results)) {
            return [];
        }

        return results.map((result: any, index: number) => ({
            id: result.id || `drift_${index}`,
            title: result.title || result.name || 'Unnamed Drift',
            severity: result.severity || 'Medium',
            category: result.category || 'Unknown',
            description: result.description || result.message || 'No description',
            suggestion: result.suggestion || result.fix,
            location: result.location ? {
                file: result.location.file || result.location.path,
                line: result.location.line,
                column: result.location.column
            } : undefined,
            confidence: result.confidence || result.ml_confidence,
            mlScore: result.ml_score || result.anomaly_score
        }));
    }

    private parseTextDriftResults(output: string): DriftResult[] {
        const results: DriftResult[] = [];
        const lines = output.split('\n');
        let currentResult: Partial<DriftResult> = {};
        let index = 0;

        for (const line of lines) {
            const trimmed = line.trim();
            if (!trimmed) continue;

            if (trimmed.startsWith('Found drift:') || trimmed.startsWith('Drift detected:')) {
                if (currentResult.title) {
                    results.push(this.finalizeTextResult(currentResult, index++));
                    currentResult = {};
                }
                currentResult.title = trimmed.replace(/^(Found drift:|Drift detected:)\s*/, '');
            } else if (trimmed.startsWith('Severity:')) {
                currentResult.severity = trimmed.replace('Severity:', '').trim();
            } else if (trimmed.startsWith('Category:')) {
                currentResult.category = trimmed.replace('Category:', '').trim();
            } else if (trimmed.startsWith('File:')) {
                currentResult.location = { file: trimmed.replace('File:', '').trim() };
            } else if (trimmed.startsWith('Description:')) {
                currentResult.description = trimmed.replace('Description:', '').trim();
            } else if (trimmed.startsWith('Suggestion:')) {
                currentResult.suggestion = trimmed.replace('Suggestion:', '').trim();
            }
        }

        if (currentResult.title) {
            results.push(this.finalizeTextResult(currentResult, index));
        }

        return results;
    }

    private finalizeTextResult(result: Partial<DriftResult>, index: number): DriftResult {
        return {
            id: `drift_${index}`,
            title: result.title || 'Unnamed Drift',
            severity: result.severity || 'Medium',
            category: result.category || 'Unknown',
            description: result.description || 'No description',
            suggestion: result.suggestion,
            location: result.location,
            confidence: result.confidence,
            mlScore: result.mlScore
        };
    }

    private parseInventoryResults(results: any): ADRInventoryItem[] {
        if (!Array.isArray(results)) {
            return [];
        }

        return results.map((result: any) => ({
            title: result.title || 'Untitled',
            status: result.status || 'Unknown',
            date: result.date || result.created || 'Unknown',
            file: result.file || result.path || 'Unknown',
            summary: result.summary || result.description
        }));
    }

    private parseTextInventoryResults(output: string): ADRInventoryItem[] {
        const results: ADRInventoryItem[] = [];
        const lines = output.split('\n');

        for (const line of lines) {
            const trimmed = line.trim();
            if (!trimmed || trimmed.startsWith('#') || trimmed.startsWith('-')) continue;

            // Try to parse line format: "001-title.adr.md - Status - Date"
            const match = trimmed.match(/^(.+\.adr\.md)\s*-\s*(.+?)\s*-\s*(.+)$/);
            if (match) {
                results.push({
                    title: path.basename(match[1], '.adr.md'),
                    status: match[2].trim(),
                    date: match[3].trim(),
                    file: match[1].trim()
                });
            }
        }

        return results;
    }

    private parseProposalResult(result: any): ADRProposal {
        return {
            title: result.title || 'Generated ADR Proposal',
            context: result.context || 'Context analysis based on provided input',
            decision: result.decision || 'Proposed decision based on analysis',
            consequences: result.consequences || 'Potential consequences of this decision',
            confidence: result.confidence || 'Medium'
        };
    }

    private parseTextProposalResult(output: string): ADRProposal {
        const lines = output.split('\n');
        const proposal: Partial<ADRProposal> = {};

        let currentSection = '';
        let content = '';

        for (const line of lines) {
            const trimmed = line.trim();
            
            if (trimmed.startsWith('Title:')) {
                proposal.title = trimmed.replace('Title:', '').trim();
            } else if (trimmed.startsWith('Context:')) {
                if (content && currentSection) {
                    (proposal as any)[currentSection] = content.trim();
                }
                currentSection = 'context';
                content = trimmed.replace('Context:', '').trim();
            } else if (trimmed.startsWith('Decision:')) {
                if (content && currentSection) {
                    (proposal as any)[currentSection] = content.trim();
                }
                currentSection = 'decision';
                content = trimmed.replace('Decision:', '').trim();
            } else if (trimmed.startsWith('Consequences:')) {
                if (content && currentSection) {
                    (proposal as any)[currentSection] = content.trim();
                }
                currentSection = 'consequences';
                content = trimmed.replace('Consequences:', '').trim();
            } else if (trimmed.startsWith('Confidence:')) {
                proposal.confidence = trimmed.replace('Confidence:', '').trim();
            } else if (currentSection && trimmed) {
                content += ' ' + trimmed;
            }
        }

        if (content && currentSection) {
            (proposal as any)[currentSection] = content.trim();
        }

        return {
            title: proposal.title || 'Generated ADR Proposal',
            context: proposal.context || 'Context analysis based on provided input',
            decision: proposal.decision || 'Proposed decision based on analysis',
            consequences: proposal.consequences || 'Potential consequences of this decision',
            confidence: proposal.confidence || 'Medium'
        };
    }
}