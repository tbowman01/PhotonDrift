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
exports.ADRCompletionProvider = void 0;
const vscode = __importStar(require("vscode"));
class ADRCompletionProvider {
    templates = new Map();
    constructor() {
        this.initializeTemplates();
    }
    provideCompletionItems(document, position, token, context) {
        const lineText = document.lineAt(position).text;
        const linePrefix = lineText.substring(0, position.character);
        // Provide ADR template completions when user types '#' at the beginning of a line
        if (linePrefix.match(/^\s*#\s*$/)) {
            return this.getTemplateCompletions();
        }
        // Provide section completions when user types '##'
        if (linePrefix.match(/^\s*##\s*$/)) {
            return this.getSectionCompletions();
        }
        // Provide status completions when in status section
        if (this.isInStatusSection(document, position)) {
            return this.getStatusCompletions();
        }
        // Provide decision completions when user types decision-related keywords
        if (linePrefix.match(/\b(decision|decide|choose|option)\b/i)) {
            return this.getDecisionCompletions();
        }
        return undefined;
    }
    initializeTemplates() {
        // MADR Template
        this.templates.set('madr', {
            name: 'MADR (Markdown Architecture Decision Record)',
            description: 'Markdown Architectural Decision Records template',
            sections: [
                '# ADR: ${1:Title}',
                '',
                '## Status',
                '${2|Proposed,Accepted,Superseded,Deprecated,Rejected|}',
                '',
                '## Context',
                '${3:What is the issue that we\'re seeing that is motivating this decision or change?}',
                '',
                '## Decision',
                '${4:What is the change that we\'re proposing or have agreed to implement?}',
                '',
                '## Consequences',
                '${5:What becomes easier or more difficult to do and any risks introduced by the change that will need to be mitigated?}',
                '',
                '## Considered Alternatives',
                '${6:What other alternatives were considered?}',
                '',
                '## Links',
                '${7:Any relevant links, ADRs, or documentation}'
            ]
        });
        // Nygard Template
        this.templates.set('nygard', {
            name: 'Nygard Template',
            description: 'Michael Nygard\'s original ADR template',
            sections: [
                '# ${1:Title}',
                '',
                '## Status',
                '${2|Proposed,Accepted,Deprecated,Superseded|}',
                '',
                '## Context',
                '${3:The issue motivating this decision, and any context that influences or constrains the decision.}',
                '',
                '## Decision',
                '${4:The change that we\'re proposing or have agreed to implement.}',
                '',
                '## Consequences',
                '${5:What becomes easier or more difficult to do and any risks introduced by the change that will need to be mitigated.}'
            ]
        });
        // Alexandrian Template
        this.templates.set('alexandrian', {
            name: 'Alexandrian Pattern',
            description: 'Christopher Alexander\'s pattern language template',
            sections: [
                '# ${1:Pattern Name}',
                '',
                '## Context',
                '${2:The recurring design problem that arises in that context.}',
                '',
                '## Problem',
                '${3:The problem, in one or two sentences.}',
                '',
                '## Forces',
                '${4:The constraints and considerations that influence the solution.}',
                '',
                '## Solution',
                '${5:The solution, in one or two sentences.}',
                '',
                '## Resulting Context',
                '${6:The context that results from applying the pattern.}',
                '',
                '## Examples',
                '${7:Examples of the pattern in use.}',
                '',
                '## Related Patterns',
                '${8:Other patterns that are related to this one.}'
            ]
        });
    }
    getTemplateCompletions() {
        const completions = [];
        for (const [key, template] of this.templates) {
            const completion = new vscode.CompletionItem(`ADR Template: ${template.name}`, vscode.CompletionItemKind.Snippet);
            completion.detail = template.description;
            completion.documentation = new vscode.MarkdownString(`Insert ${template.name} template`);
            // Create snippet with template content
            completion.insertText = new vscode.SnippetString(template.sections.join('\n'));
            // Replace the entire line
            completion.range = new vscode.Range(new vscode.Position(0, 0), new vscode.Position(0, 0));
            completion.sortText = `0${key}`;
            completions.push(completion);
        }
        return completions;
    }
    getSectionCompletions() {
        const sections = [
            { name: 'Status', description: 'Current status of the decision' },
            { name: 'Context', description: 'Background and motivation for the decision' },
            { name: 'Decision', description: 'The decision that was made' },
            { name: 'Consequences', description: 'Expected outcomes and trade-offs' },
            { name: 'Alternatives', description: 'Other options that were considered' },
            { name: 'Considered Alternatives', description: 'Other options that were considered' },
            { name: 'Links', description: 'Related resources and references' },
            { name: 'Date', description: 'When the decision was made' },
            { name: 'Deciders', description: 'Who made the decision' },
            { name: 'Technical Story', description: 'Technical background and requirements' },
            { name: 'Problem Statement', description: 'Clear statement of the problem' },
            { name: 'Decision Drivers', description: 'Factors that influenced the decision' },
            { name: 'Solution Description', description: 'Detailed description of the solution' },
            { name: 'Validation', description: 'How the decision will be validated' },
            { name: 'Implementation', description: 'Implementation details and timeline' }
        ];
        return sections.map(section => {
            const completion = new vscode.CompletionItem(section.name, vscode.CompletionItemKind.Text);
            completion.detail = section.description;
            completion.insertText = `${section.name}\n\n$1`;
            completion.insertText = new vscode.SnippetString(`${section.name}\n\n$1`);
            return completion;
        });
    }
    getStatusCompletions() {
        const statuses = [
            { name: 'Proposed', description: 'The decision is proposed but not yet accepted' },
            { name: 'Accepted', description: 'The decision has been accepted and is active' },
            { name: 'Deprecated', description: 'The decision is no longer recommended' },
            { name: 'Superseded', description: 'The decision has been replaced by another' },
            { name: 'Rejected', description: 'The decision was considered but rejected' },
            { name: 'Draft', description: 'The decision is still being formulated' },
            { name: 'Under Review', description: 'The decision is being reviewed' },
            { name: 'Approved', description: 'The decision has been formally approved' }
        ];
        return statuses.map(status => {
            const completion = new vscode.CompletionItem(status.name, vscode.CompletionItemKind.Value);
            completion.detail = status.description;
            completion.insertText = status.name;
            return completion;
        });
    }
    getDecisionCompletions() {
        const decisionTemplates = [
            {
                name: 'We will use',
                description: 'Template for technology/tool decisions',
                text: 'We will use ${1:technology/tool} for ${2:purpose} because ${3:reasoning}.'
            },
            {
                name: 'We will adopt',
                description: 'Template for adopting practices or patterns',
                text: 'We will adopt ${1:practice/pattern} to ${2:achieve goal} and ${3:expected benefit}.'
            },
            {
                name: 'We decided to',
                description: 'General decision template',
                text: 'We decided to ${1:action} in order to ${2:achieve outcome}.'
            },
            {
                name: 'The team chose',
                description: 'Team decision template',
                text: 'The team chose ${1:option} over ${2:alternative} because ${3:reasoning}.'
            }
        ];
        return decisionTemplates.map(template => {
            const completion = new vscode.CompletionItem(template.name, vscode.CompletionItemKind.Snippet);
            completion.detail = template.description;
            completion.insertText = new vscode.SnippetString(template.text);
            return completion;
        });
    }
    isInStatusSection(document, position) {
        // Look backwards from current position to find if we're in a Status section
        for (let i = position.line - 1; i >= 0; i--) {
            const line = document.lineAt(i).text.trim();
            if (line.match(/^##\s+Status\s*$/i)) {
                return true;
            }
            // Stop if we hit another section
            if (line.match(/^##\s+/)) {
                return false;
            }
        }
        return false;
    }
}
exports.ADRCompletionProvider = ADRCompletionProvider;
//# sourceMappingURL=completionProvider.js.map