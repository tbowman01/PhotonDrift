import * as vscode from 'vscode';

export class DriftStatusBar implements vscode.Disposable {
    private statusBarItem: vscode.StatusBarItem;
    private isScanning: boolean = false;
    private driftCount: number = 0;
    private lastScanTime?: Date;

    constructor() {
        this.statusBarItem = vscode.window.createStatusBarItem(
            vscode.StatusBarAlignment.Left,
            10
        );
        
        this.statusBarItem.command = 'photondrift.runDriftDetection';
        this.updateDisplay();
        this.show();
    }

    public setScanning(scanning: boolean): void {
        this.isScanning = scanning;
        this.updateDisplay();
    }

    public updateDriftCount(count: number): void {
        this.driftCount = count;
        this.lastScanTime = new Date();
        this.updateDisplay();
    }

    public show(): void {
        const config = vscode.workspace.getConfiguration('photondrift');
        const enabled = config.get<boolean>('statusBarEnabled', true);
        
        if (enabled) {
            this.statusBarItem.show();
        }
    }

    public hide(): void {
        this.statusBarItem.hide();
    }

    private updateDisplay(): void {
        if (this.isScanning) {
            this.statusBarItem.text = '$(sync~spin) PhotonDrift: Scanning...';
            this.statusBarItem.tooltip = 'PhotonDrift is scanning for drift...';
            this.statusBarItem.backgroundColor = new vscode.ThemeColor('statusBarItem.warningBackground');
        } else {
            const icon = this.getDriftIcon();
            const text = `$(${icon}) PhotonDrift: ${this.driftCount} drift${this.driftCount !== 1 ? 's' : ''}`;
            
            this.statusBarItem.text = text;
            this.statusBarItem.tooltip = this.getTooltip();
            this.statusBarItem.backgroundColor = this.getBackgroundColor();
        }
    }

    private getDriftIcon(): string {
        if (this.driftCount === 0) {
            return 'check';
        } else if (this.driftCount <= 3) {
            return 'warning';
        } else {
            return 'error';
        }
    }

    private getBackgroundColor(): vscode.ThemeColor | undefined {
        if (this.driftCount === 0) {
            return undefined; // Default background
        } else if (this.driftCount <= 3) {
            return new vscode.ThemeColor('statusBarItem.warningBackground');
        } else {
            return new vscode.ThemeColor('statusBarItem.errorBackground');
        }
    }

    private getTooltip(): vscode.MarkdownString {
        const tooltip = new vscode.MarkdownString();
        
        tooltip.appendMarkdown('**PhotonDrift Drift Detection**\n\n');
        
        if (this.driftCount === 0) {
            tooltip.appendMarkdown('✅ No drift detected');
        } else {
            tooltip.appendMarkdown(`⚠️ ${this.driftCount} drift item${this.driftCount !== 1 ? 's' : ''} detected`);
        }
        
        if (this.lastScanTime) {
            const timeStr = this.lastScanTime.toLocaleTimeString();
            tooltip.appendMarkdown(`\n\n*Last scan: ${timeStr}*`);
        }
        
        tooltip.appendMarkdown('\n\nClick to run drift detection');
        
        return tooltip;
    }

    dispose(): void {
        this.statusBarItem.dispose();
    }
}