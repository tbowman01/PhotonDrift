//! ADR template completion provider for LSP

use lsp_types::{CompletionItem, CompletionItemKind, InsertTextFormat, Position};

/// Provides intelligent completion for ADR templates and content
pub struct CompletionProvider {
    templates: Vec<CompletionItem>,
}

impl CompletionProvider {
    pub fn new() -> Self {
        Self {
            templates: ADR_TEMPLATE_COMPLETIONS.to_vec(),
        }
    }

    /// Get completions based on current position and context
    pub async fn get_completions(&self, content: &str, position: Position) -> Vec<CompletionItem> {
        let mut completions = Vec::new();

        // Get the current line
        let lines: Vec<&str> = content.lines().collect();
        let current_line = lines.get(position.line as usize).unwrap_or(&"");

        // Determine context and provide appropriate completions
        if current_line.starts_with("# ") {
            // Title completion
            completions.extend(self.get_title_completions());
        } else if current_line.starts_with("## ") {
            // Section header completions
            completions.extend(self.get_section_completions());
        } else if current_line.to_lowercase().contains("status") {
            // Status value completions
            completions.extend(self.get_status_completions());
        } else if position.character == 0 || current_line.trim().is_empty() {
            // General template completions at line start
            completions.extend(self.templates.clone());
        }

        // Add ADR number suggestions if we're in a title
        if current_line.starts_with("# ADR-") {
            completions.extend(self.get_adr_number_suggestions());
        }

        completions
    }

    fn get_title_completions(&self) -> Vec<CompletionItem> {
        vec![CompletionItem {
            label: "ADR Title Template".to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some("Standard ADR title format".to_string()),
            documentation: Some(lsp_types::Documentation::String(
                "Creates a properly formatted ADR title with sequential numbering".to_string(),
            )),
            insert_text: Some("# ADR-${1:001}: ${2:Title of the decision}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        }]
    }

    fn get_section_completions(&self) -> Vec<CompletionItem> {
        let sections = [
            ("Status", "Current status of this ADR"),
            ("Context", "The issue motivating this decision"),
            (
                "Decision",
                "The change that we're proposing or have agreed to implement",
            ),
            (
                "Consequences",
                "What becomes easier or more difficult to do and any risks introduced",
            ),
            ("Alternatives", "Other options considered"),
            ("Related", "Related decisions and references"),
        ];

        sections
            .iter()
            .map(|(section, description)| CompletionItem {
                label: format!("## {}", section),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some(description.to_string()),
                documentation: Some(lsp_types::Documentation::String(format!(
                    "Standard ADR section: {}",
                    description
                ))),
                insert_text: Some(format!("## {}\n\n${{1:Content goes here}}", section)),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                ..Default::default()
            })
            .collect()
    }

    fn get_status_completions(&self) -> Vec<CompletionItem> {
        let statuses = [
            ("Proposed", "This ADR is proposed and under consideration"),
            ("Accepted", "This ADR has been accepted and is active"),
            (
                "Deprecated",
                "This ADR is no longer in effect but kept for historical reference",
            ),
            (
                "Superseded",
                "This ADR has been replaced by a newer decision",
            ),
            ("Rejected", "This ADR was considered but rejected"),
        ];

        statuses
            .iter()
            .map(|(status, description)| CompletionItem {
                label: status.to_string(),
                kind: Some(CompletionItemKind::VALUE),
                detail: Some(description.to_string()),
                documentation: Some(lsp_types::Documentation::String(description.to_string())),
                insert_text: Some(status.to_string()),
                ..Default::default()
            })
            .collect()
    }

    fn get_adr_number_suggestions(&self) -> Vec<CompletionItem> {
        // This would ideally scan existing ADRs to suggest the next number
        // For now, provide some common patterns
        vec![CompletionItem {
            label: "001".to_string(),
            kind: Some(CompletionItemKind::VALUE),
            detail: Some("ADR numbering suggestion".to_string()),
            insert_text: Some("001".to_string()),
            ..Default::default()
        }]
    }
}

/// Pre-defined ADR template completions
pub const ADR_TEMPLATE_COMPLETIONS: &[CompletionItem] = &[
    CompletionItem {
        label: "Full ADR Template".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Complete ADR template with all sections".to_string()),
        documentation: Some(lsp_types::Documentation::String(
            "Inserts a complete ADR template with all standard sections".to_string(),
        )),
        insert_text: Some(
            r#"# ADR-${1:001}: ${2:Title of the decision}

## Status

${3:Proposed}

## Context

${4:What is the issue that we're seeing that is motivating this decision or change?}

## Decision

${5:What is the change that we're proposing or have agreed to implement?}

## Consequences

${6:What becomes easier or more difficult to do and any risks introduced by this decision?}

## Related

${7:List related ADRs, documents, or decisions}
"#
            .to_string(),
        ),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..CompletionItem::default()
    },
    CompletionItem {
        label: "Simple ADR Template".to_string(),
        kind: Some(CompletionItemKind::SNIPPET),
        detail: Some("Minimal ADR template with core sections".to_string()),
        documentation: Some(lsp_types::Documentation::String(
            "Inserts a minimal ADR template with essential sections only".to_string(),
        )),
        insert_text: Some(
            r#"# ADR-${1:001}: ${2:Title}

## Status
${3:Proposed}

## Decision
${4:What we decided to do}

## Consequences
${5:What this means going forward}
"#
            .to_string(),
        ),
        insert_text_format: Some(InsertTextFormat::SNIPPET),
        ..CompletionItem::default()
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_section_completions() {
        let provider = CompletionProvider::new();
        let content = "# ADR-001: Test\n\n## ";
        let position = Position {
            line: 2,
            character: 3,
        };

        let completions = provider.get_completions(content, position).await;

        assert!(!completions.is_empty());
        assert!(completions.iter().any(|c| c.label.contains("Status")));
        assert!(completions.iter().any(|c| c.label.contains("Context")));
        assert!(completions.iter().any(|c| c.label.contains("Decision")));
    }

    #[tokio::test]
    async fn test_status_completions() {
        let provider = CompletionProvider::new();
        let content = "# ADR-001: Test\n\n## Status\n";
        let position = Position {
            line: 3,
            character: 0,
        };

        let completions = provider.get_completions(content, position).await;

        assert!(completions.iter().any(|c| c.label == "Proposed"));
        assert!(completions.iter().any(|c| c.label == "Accepted"));
        assert!(completions.iter().any(|c| c.label == "Deprecated"));
    }

    #[tokio::test]
    async fn test_template_completions() {
        let provider = CompletionProvider::new();
        let content = "";
        let position = Position {
            line: 0,
            character: 0,
        };

        let completions = provider.get_completions(content, position).await;

        assert!(completions
            .iter()
            .any(|c| c.label.contains("Full ADR Template")));
        assert!(completions
            .iter()
            .any(|c| c.label.contains("Simple ADR Template")));
    }

    #[test]
    fn test_predefined_templates() {
        assert_eq!(ADR_TEMPLATE_COMPLETIONS.len(), 2);

        let full_template = &ADR_TEMPLATE_COMPLETIONS[0];
        assert_eq!(full_template.label, "Full ADR Template");
        assert!(matches!(
            full_template.kind,
            Some(CompletionItemKind::SNIPPET)
        ));
        assert!(full_template
            .insert_text
            .as_ref()
            .unwrap()
            .contains("# ADR-"));
    }
}
