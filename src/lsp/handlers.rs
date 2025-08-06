//! Additional LSP handlers and utilities

#![cfg(feature = "lsp")]

use lsp_types::{
    CodeActionParams, CodeActionResponse, DocumentFormattingParams, DocumentSymbolParams,
    DocumentSymbolResponse, TextEdit, WorkspaceEdit,
};
use std::collections::HashMap;
use tower_lsp::jsonrpc::Result;

use crate::lsp::protocol::{extract_adr_number, line_range};

/// Additional handlers for extended LSP functionality
pub struct LspHandlers;

impl LspHandlers {
    /// Provide code actions for common ADR issues
    pub async fn code_action(
        &self,
        params: CodeActionParams,
    ) -> Result<Option<CodeActionResponse>> {
        let uri = params.text_document.uri;
        let range = params.range;

        // For now, return some example code actions
        // In a full implementation, this would analyze the specific context
        let actions = vec![lsp_types::CodeActionOrCommand::CodeAction(
            lsp_types::CodeAction {
                title: "Add missing Status section".to_string(),
                kind: Some(lsp_types::CodeActionKind::QUICKFIX),
                diagnostics: Some(params.context.diagnostics),
                edit: Some(WorkspaceEdit {
                    changes: Some({
                        let mut changes = HashMap::new();
                        changes.insert(
                            uri.clone(),
                            vec![TextEdit {
                                range: lsp_types::Range {
                                    start: lsp_types::Position {
                                        line: 2,
                                        character: 0,
                                    },
                                    end: lsp_types::Position {
                                        line: 2,
                                        character: 0,
                                    },
                                },
                                new_text: "\n## Status\nProposed\n".to_string(),
                            }],
                        );
                        changes
                    }),
                    document_changes: None,
                    change_annotations: None,
                }),
                command: None,
                data: None,
                is_preferred: Some(true),
                disabled: None,
            },
        )];

        Ok(Some(actions))
    }

    /// Provide document symbols for navigation
    pub async fn document_symbol(
        &self,
        params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        // This would analyze the document structure and return symbols
        // For ADRs, this could include sections, references, etc.

        let symbols = vec![lsp_types::DocumentSymbol {
            name: "Title".to_string(),
            detail: Some("ADR Title".to_string()),
            kind: lsp_types::SymbolKind::STRING,
            range: lsp_types::Range {
                start: lsp_types::Position {
                    line: 0,
                    character: 0,
                },
                end: lsp_types::Position {
                    line: 0,
                    character: 50,
                },
            },
            selection_range: lsp_types::Range {
                start: lsp_types::Position {
                    line: 0,
                    character: 0,
                },
                end: lsp_types::Position {
                    line: 0,
                    character: 50,
                },
            },
            children: None,
            tags: None,
            deprecated: None,
        }];

        Ok(Some(DocumentSymbolResponse::Nested(symbols)))
    }

    /// Format ADR document
    pub async fn formatting(
        &self,
        params: DocumentFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        // This would implement ADR-specific formatting rules
        // For now, return empty - formatting could include:
        // - Consistent section ordering
        // - Proper spacing between sections
        // - Status value normalization

        Ok(Some(Vec::new()))
    }

    /// Validate ADR structure and provide suggestions
    pub fn validate_adr_structure(&self, content: &str) -> Vec<String> {
        let mut issues = Vec::new();

        // Check for required sections
        let required_sections = ["status", "context", "decision"];
        for section in required_sections {
            if !content.to_lowercase().contains(&format!("## {}", section)) {
                issues.push(format!("Missing required section: {}", section));
            }
        }

        // Check for title format
        if !content.lines().any(|line| line.starts_with("# ADR-")) {
            issues.push("Title should follow format: # ADR-XXX: Description".to_string());
        }

        // Check for empty sections
        let lines: Vec<&str> = content.lines().collect();
        for (i, line) in lines.iter().enumerate() {
            if line.starts_with("## ") {
                // Check if the section has content
                let mut has_content = false;
                for next_line in lines.iter().skip(i + 1) {
                    if next_line.starts_with("## ") {
                        break;
                    }
                    if !next_line.trim().is_empty() {
                        has_content = true;
                        break;
                    }
                }

                if !has_content {
                    issues.push(format!("Empty section: {}", line.trim()));
                }
            }
        }

        issues
    }

    /// Suggest related ADRs based on content analysis
    pub fn suggest_related_adrs(&self, content: &str, _available_adrs: &[String]) -> Vec<String> {
        let mut suggestions = Vec::new();

        // This is a simplified implementation
        // In practice, this could use ML or keyword analysis

        let keywords = self.extract_keywords(content);

        // For now, just return some placeholder suggestions based on keywords
        if keywords.iter().any(|k| k.contains("database")) {
            suggestions.push("ADR-002: Database Selection".to_string());
        }

        if keywords.iter().any(|k| k.contains("security")) {
            suggestions.push("ADR-003: Security Framework".to_string());
        }

        suggestions
    }

    fn extract_keywords(&self, content: &str) -> Vec<String> {
        // Simple keyword extraction
        let words: Vec<String> = content
            .split_whitespace()
            .map(|w| w.to_lowercase())
            .filter(|w| w.len() > 3) // Filter short words
            .collect();

        // Remove duplicates and return
        let mut keywords: Vec<String> = words.into_iter().collect();
        keywords.sort();
        keywords.dedup();
        keywords
    }

    /// Generate ADR template based on context
    pub fn generate_template(&self, adr_type: &str) -> String {
        match adr_type {
            "technical" => {
                r#"# ADR-XXX: [Technical Decision Title]

## Status
Proposed

## Context
What technical challenge or requirement led to this decision?

## Decision
What technical solution have we chosen?

## Consequences
### Positive
- What benefits does this bring?

### Negative
- What are the downsides or limitations?

### Risks
- What could go wrong?

## Alternatives Considered
- Alternative 1: Brief description and why it was rejected
- Alternative 2: Brief description and why it was rejected

## Implementation Notes
- Key implementation details
- Dependencies required
- Timeline considerations

## Related Decisions
- ADR-XXX: Related decision
"#
            }
            "process" => {
                r#"# ADR-XXX: [Process Decision Title]

## Status
Proposed

## Context
What process issue or need drove this decision?

## Decision
What process change are we implementing?

## Consequences
How will this affect our workflow and team?

## Implementation
- Steps to implement this process
- Training required
- Tools needed

## Success Metrics
How will we measure if this process improvement is working?

## Review Schedule
When will we review this process decision?
"#
            }
            _ => {
                r#"# ADR-XXX: [Decision Title]

## Status
Proposed

## Context
What is the issue that we're seeing that is motivating this decision or change?

## Decision
What is the change that we're proposing or have agreed to implement?

## Consequences
What becomes easier or more difficult to do and any risks introduced by this decision?
"#
            }
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_adr_structure() {
        let handlers = LspHandlers;

        // Test missing sections
        let content = "# ADR-001: Test\n\nSome content without proper sections";
        let issues = handlers.validate_adr_structure(content);

        assert!(!issues.is_empty());
        assert!(issues.iter().any(|i| i.contains("status")));
        assert!(issues.iter().any(|i| i.contains("context")));
        assert!(issues.iter().any(|i| i.contains("decision")));
    }

    #[test]
    fn test_validate_proper_adr() {
        let handlers = LspHandlers;

        let content = r#"# ADR-001: Use Rust for Backend

## Status
Accepted

## Context
We need a performant backend language.

## Decision
We will use Rust for our backend services.

## Consequences
Better performance but steeper learning curve.
"#;

        let issues = handlers.validate_adr_structure(content);
        // Should have minimal issues for a proper ADR
        assert!(issues.len() <= 1); // May have minor suggestions
    }

    #[test]
    fn test_empty_section_detection() {
        let handlers = LspHandlers;

        let content = r#"# ADR-001: Test

## Status

## Context
Some context here
"#;

        let issues = handlers.validate_adr_structure(content);
        assert!(issues
            .iter()
            .any(|i| i.contains("Empty section: ## Status")));
    }

    #[test]
    fn test_keyword_extraction() {
        let handlers = LspHandlers;

        let content = "We need to choose a database system for our application";
        let keywords = handlers.extract_keywords(content);

        assert!(keywords.contains(&"database".to_string()));
        assert!(keywords.contains(&"system".to_string()));
        assert!(keywords.contains(&"application".to_string()));
        assert!(!keywords.contains(&"we".to_string())); // Should filter short words
    }

    #[test]
    fn test_template_generation() {
        let handlers = LspHandlers;

        let technical_template = handlers.generate_template("technical");
        assert!(technical_template.contains("## Implementation Notes"));
        assert!(technical_template.contains("### Positive"));

        let process_template = handlers.generate_template("process");
        assert!(process_template.contains("## Success Metrics"));
        assert!(process_template.contains("## Review Schedule"));

        let default_template = handlers.generate_template("unknown");
        assert!(default_template.contains("## Status"));
        assert!(default_template.contains("## Context"));
    }

    #[test]
    fn test_related_adr_suggestions() {
        let handlers = LspHandlers;

        let content = "We need to implement database security measures for our application";
        let available_adrs = vec!["ADR-001: Use PostgreSQL".to_string()];

        let suggestions = handlers.suggest_related_adrs(content, &available_adrs);

        // Should suggest both database and security related ADRs
        assert!(suggestions.iter().any(|s| s.contains("Database")));
        assert!(suggestions.iter().any(|s| s.contains("Security")));
    }
}
