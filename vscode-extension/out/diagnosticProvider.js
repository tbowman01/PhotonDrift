"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || (function () {
    var ownKeys = function(o) {
        ownKeys = Object.getOwnPropertyNames || function (o) {
            var ar = [];
            for (var k in o) if (Object.prototype.hasOwnProperty.call(o, k)) ar[ar.length] = k;
            return ar;
        };
        return ownKeys(o);
    };
    return function (mod) {
        if (mod && mod.__esModule) return mod;
        var result = {};
        if (mod != null) for (var k = ownKeys(mod), i = 0; i < k.length; i++) if (k[i] !== "default") __createBinding(result, mod, k[i]);
        __setModuleDefault(result, mod);
        return result;
    };
})();
Object.defineProperty(exports, "__esModule", { value: true });
exports.ADRDiagnosticProvider = void 0;
const vscode = __importStar(require("vscode"));
class ADRDiagnosticProvider {
    diagnosticCollection;
    currentDiagnostics = new Map();
    constructor() {
        this.diagnosticCollection = vscode.languages.createDiagnosticCollection('photondrift');
    }
    async updateDiagnostics(driftResults) {
        // Clear existing diagnostics
        this.diagnosticCollection.clear();
        this.currentDiagnostics.clear();
        const config = vscode.workspace.getConfiguration('photondrift');
        const showInlineWarnings = config.get('showInlineWarnings', true);
        if (!showInlineWarnings) {
            return;
        }
        // Group drift results by file
        const fileGroups = new Map();
        for (const result of driftResults) {
            if (result.location?.file) {
                const filePath = result.location.file;
                if (!fileGroups.has(filePath)) {
                    fileGroups.set(filePath, []);
                }
                fileGroups.get(filePath).push(result);
            }
        }
        // Create diagnostics for each file
        for (const [filePath, results] of fileGroups) {
            const diagnostics = [];
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
    createDiagnostic(result) {
        if (!result.location?.file) {
            return null;
        }
        // Determine range
        let range;
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
        }
        else {
            // Default to first line if no specific location
            range = new vscode.Range(0, 0, 0, 100);
        }
        // Determine severity
        const severity = this.getSeverity(result.severity);
        // Create diagnostic
        const diagnostic = new vscode.Diagnostic(range, `PhotonDrift: ${result.title} - ${result.description}`, severity);
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
        const relatedInfo = [];
        if (result.category) {
            relatedInfo.push(new vscode.DiagnosticRelatedInformation(new vscode.Location(vscode.Uri.file(result.location.file), range), `Category: ${result.category}`));
        }
        if (result.mlScore !== undefined) {
            relatedInfo.push(new vscode.DiagnosticRelatedInformation(new vscode.Location(vscode.Uri.file(result.location.file), range), `ML Anomaly Score: ${(result.mlScore * 100).toFixed(1)}%`));
        }
        if (relatedInfo.length > 0) {
            diagnostic.relatedInformation = relatedInfo;
        }
        // Add tags
        const tags = [];
        if (result.category?.toLowerCase().includes('deprecated')) {
            tags.push(vscode.DiagnosticTag.Deprecated);
        }
        if (tags.length > 0) {
            diagnostic.tags = tags;
        }
        return diagnostic;
    }
    getSeverity(severity) {
        const severityLower = severity.toLowerCase();
        if (severityLower.includes('critical') || severityLower.includes('high')) {
            return vscode.DiagnosticSeverity.Error;
        }
        else if (severityLower.includes('medium') || severityLower.includes('warning')) {
            return vscode.DiagnosticSeverity.Warning;
        }
        else if (severityLower.includes('low') || severityLower.includes('info')) {
            return vscode.DiagnosticSeverity.Information;
        }
        else {
            return vscode.DiagnosticSeverity.Hint;
        }
    }
    getDiagnosticsForFile(filePath) {
        return this.currentDiagnostics.get(filePath) || [];
    }
    clearDiagnosticsForFile(filePath) {
        const uri = vscode.Uri.file(filePath);
        this.diagnosticCollection.delete(uri);
        this.currentDiagnostics.delete(filePath);
    }
    clearAllDiagnostics() {
        this.diagnosticCollection.clear();
        this.currentDiagnostics.clear();
    }
    dispose() {
        this.diagnosticCollection.dispose();
        this.currentDiagnostics.clear();
    }
}
exports.ADRDiagnosticProvider = ADRDiagnosticProvider;
//# sourceMappingURL=diagnosticProvider.js.map