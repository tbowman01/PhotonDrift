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
exports.DriftStatusBar = void 0;
const vscode = __importStar(require("vscode"));
class DriftStatusBar {
    statusBarItem;
    isScanning = false;
    driftCount = 0;
    lastScanTime;
    constructor() {
        this.statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 10);
        this.statusBarItem.command = 'photondrift.runDriftDetection';
        this.updateDisplay();
        this.show();
    }
    setScanning(scanning) {
        this.isScanning = scanning;
        this.updateDisplay();
    }
    updateDriftCount(count) {
        this.driftCount = count;
        this.lastScanTime = new Date();
        this.updateDisplay();
    }
    show() {
        const config = vscode.workspace.getConfiguration('photondrift');
        const enabled = config.get('statusBarEnabled', true);
        if (enabled) {
            this.statusBarItem.show();
        }
    }
    hide() {
        this.statusBarItem.hide();
    }
    updateDisplay() {
        if (this.isScanning) {
            this.statusBarItem.text = '$(sync~spin) PhotonDrift: Scanning...';
            this.statusBarItem.tooltip = 'PhotonDrift is scanning for drift...';
            this.statusBarItem.backgroundColor = new vscode.ThemeColor('statusBarItem.warningBackground');
        }
        else {
            const icon = this.getDriftIcon();
            const text = `$(${icon}) PhotonDrift: ${this.driftCount} drift${this.driftCount !== 1 ? 's' : ''}`;
            this.statusBarItem.text = text;
            this.statusBarItem.tooltip = this.getTooltip();
            this.statusBarItem.backgroundColor = this.getBackgroundColor();
        }
    }
    getDriftIcon() {
        if (this.driftCount === 0) {
            return 'check';
        }
        else if (this.driftCount <= 3) {
            return 'warning';
        }
        else {
            return 'error';
        }
    }
    getBackgroundColor() {
        if (this.driftCount === 0) {
            return undefined; // Default background
        }
        else if (this.driftCount <= 3) {
            return new vscode.ThemeColor('statusBarItem.warningBackground');
        }
        else {
            return new vscode.ThemeColor('statusBarItem.errorBackground');
        }
    }
    getTooltip() {
        const tooltip = new vscode.MarkdownString();
        tooltip.appendMarkdown('**PhotonDrift Drift Detection**\n\n');
        if (this.driftCount === 0) {
            tooltip.appendMarkdown('✅ No drift detected');
        }
        else {
            tooltip.appendMarkdown(`⚠️ ${this.driftCount} drift item${this.driftCount !== 1 ? 's' : ''} detected`);
        }
        if (this.lastScanTime) {
            const timeStr = this.lastScanTime.toLocaleTimeString();
            tooltip.appendMarkdown(`\n\n*Last scan: ${timeStr}*`);
        }
        tooltip.appendMarkdown('\n\nClick to run drift detection');
        return tooltip;
    }
    dispose() {
        this.statusBarItem.dispose();
    }
}
exports.DriftStatusBar = DriftStatusBar;
//# sourceMappingURL=driftStatusBar.js.map