import * as vscode from 'vscode';
import { DriftResult } from './photonDriftCLI';

export class ADRDiagnosticProvider implements vscode.Disposable {
    private diagnosticCollection: vscode.DiagnosticCollection;
    private currentDiagnostics: Map<string, vscode.Diagnostic[]> = new Map();

    constructor() {
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection('photondrift');
    }

    public async updateDiagnostics(driftResults: DriftResult[]): Promise<void> {
        // Clear existing diagnostics
        this.diagnosticCollection.clear();
        this.currentDiagnostics.clear();

        const config = vscode.workspace.getConfiguration('photondrift');
        const showInlineWarnings = config.get<boolean>('showInlineWarnings', true);
        
        if (!showInlineWarnings) {
            return;
        }

        // Group drift results by file
        const fileGroups = new Map<string, DriftResult[]>();
        
        for (const result of driftResults) {
            if (result.location?.file) {
                const filePath = result.location.file;
                if (!fileGroups.has(filePath)) {
                    fileGroups.set(filePath, []);
                }
                fileGroups.get(filePath)!.push(result);
            }
        }

        // Create diagnostics for each file
        for (const [filePath, results] of fileGroups) {
            const diagnostics: vscode.Diagnostic[] = [];
            
            for (const result of results) {
                const diagnostic = this.createDiagnostic(result);
                if (diagnostic) {
                    diagnostics.push(diagnostic);
                }
            }
            
            if (diagnostics.length > 0) {
                const uri = vscode.Uri.file(filePath);
                this.diagnosticCollection.set(uri, diagnostics);
                this.currentDiagnostics.set(filePath, diagnostics);
            }
        }
    }

    private createDiagnostic(result: DriftResult): vscode.Diagnostic | null {
        if (!result.location?.file) {
            return null;
        }

        // Determine range
        let range: vscode.Range;
        if (result.location.line !== undefined) {
            const line = Math.max(0, result.location.line - 1); // VS Code is 0-indexed
            const column = result.location.column !== undefined ? 
                Math.max(0, result.location.column - 1) : 0;
            
            // Try to get the actual document to determine line length
            const uri = vscode.Uri.file(result.location.file);
            const document = vscode.workspace.textDocuments.find(doc => doc.uri.toString() === uri.toString());
            
            let endColumn = column + 10; // Default length
            if (document && line < document.lineCount) {
                const lineText = document.lineAt(line).text;
                endColumn = Math.max(column + 1, lineText.length);
            }
            
            range = new vscode.Range(line, column, line, endColumn);
        } else {
            // Default to first line if no specific location
            range = new vscode.Range(0, 0, 0, 100);
        }

        // Determine severity
        const severity = this.getSeverity(result.severity);

        // Create diagnostic
        const diagnostic = new vscode.Diagnostic(
            range,
            `PhotonDrift: ${result.title} - ${result.description}`,
            severity
        );

        // Add additional information
        diagnostic.source = 'PhotonDrift';
        diagnostic.code = result.id;
        
        if (result.suggestion) {
            diagnostic.message += `\n\nSuggestion: ${result.suggestion}`;
        }

        if (result.confidence !== undefined) {
            diagnostic.message += `\n\nConfidence: ${(result.confidence * 100).toFixed(1)}%`;
        }

        // Add related information if available
        const relatedInfo: vscode.DiagnosticRelatedInformation[] = [];
        
        if (result.category) {
            relatedInfo.push(new vscode.DiagnosticRelatedInformation(
                new vscode.Location(vscode.Uri.file(result.location.file), range),
                `Category: ${result.category}`
            ));
        }

        if (result.mlScore !== undefined) {
            relatedInfo.push(new vscode.DiagnosticRelatedInformation(
                new vscode.Location(vscode.Uri.file(result.location.file), range),
                `ML Anomaly Score: ${(result.mlScore * 100).toFixed(1)}%`
            ));
        }

        if (relatedInfo.length > 0) {
            diagnostic.relatedInformation = relatedInfo;
        }

        // Add tags
        const tags: vscode.DiagnosticTag[] = [];
        if (result.category?.toLowerCase().includes('deprecated')) {
            tags.push(vscode.DiagnosticTag.Deprecated);
        }
        if (tags.length > 0) {
            diagnostic.tags = tags;
        }

        return diagnostic;
    }

    private getSeverity(severity: string): vscode.DiagnosticSeverity {
        const severityLower = severity.toLowerCase();
        
        if (severityLower.includes('critical') || severityLower.includes('high')) {
            return vscode.DiagnosticSeverity.Error;
        } else if (severityLower.includes('medium') || severityLower.includes('warning')) {
            return vscode.DiagnosticSeverity.Warning;
        } else if (severityLower.includes('low') || severityLower.includes('info')) {
            return vscode.DiagnosticSeverity.Information;
        } else {
            return vscode.DiagnosticSeverity.Hint;
        }
    }

    public getDiagnosticsForFile(filePath: string): vscode.Diagnostic[] {
        return this.currentDiagnostics.get(filePath) || [];
    }

    public clearDiagnosticsForFile(filePath: string): void {
        const uri = vscode.Uri.file(filePath);
        this.diagnosticCollection.delete(uri);
        this.currentDiagnostics.delete(filePath);
    }

    public clearAllDiagnostics(): void {
        this.diagnosticCollection.clear();
        this.currentDiagnostics.clear();
    }

    dispose(): void {
        this.diagnosticCollection.dispose();
        this.currentDiagnostics.clear();
    }
}