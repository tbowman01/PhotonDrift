//! Hover information provider for ADR files

use lsp_types::{Hover, HoverContents, MarkupContent, MarkupKind, Position, Range};

/// Provides contextual hover information for ADR elements
pub struct HoverProvider;

impl HoverProvider {
    pub fn new() -> Self {
        Self
    }

    /// Get hover information for the given position in the document
    pub async fn get_hover_info(&self, content: &str, position: Position) -> Option<Hover> {
        let lines: Vec<&str> = content.lines().collect();
        let current_line = lines.get(position.line as usize)?;

        // Determine what the user is hovering over
        if let Some(word) = self.get_word_at_position(current_line, position.character) {
            // Check for ADR-specific elements
            if word.starts_with("ADR-") {
                return self.get_adr_reference_hover(&word);
            }

            // Check for status values
            if self.is_status_value(&word) {
                return self.get_status_hover(&word);
            }

            // Check for section headers
            if current_line.starts_with("## ") && current_line.contains(&word) {
                return self.get_section_hover(&word);
            }

            // Check for common ADR terminology
            if let Some(hover) = self.get_terminology_hover(&word) {
                return Some(hover);
            }
        }

        // Check for entire line context
        if current_line.starts_with("# ADR-") {
            return self.get_title_hover(current_line);
        }

        None
    }

    fn get_word_at_position(&self, line: &str, character: u32) -> Option<String> {
        let chars: Vec<char> = line.chars().collect();
        let pos = character as usize;

        if pos >= chars.len() {
            return None;
        }

        // Find word boundaries
        let mut start = pos;
        let mut end = pos;

        // Move start backward to find word start
        while start > 0
            && (chars[start - 1].is_alphanumeric()
                || chars[start - 1] == '-'
                || chars[start - 1] == '_')
        {
            start -= 1;
        }

        // Move end forward to find word end
        while end < chars.len()
            && (chars[end].is_alphanumeric() || chars[end] == '-' || chars[end] == '_')
        {
            end += 1;
        }

        if start < end {
            Some(chars[start..end].iter().collect())
        } else {
            None
        }
    }

    fn get_adr_reference_hover(&self, adr_ref: &str) -> Option<Hover> {
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!(
                    r#"**Architecture Decision Record Reference**

`{}` - Reference to another ADR in this collection.

ADR references help maintain traceability between related architectural decisions. Consider including a brief summary of the referenced decision for context.

**Best Practices:**
- Always check that referenced ADRs exist
- Include the decision title for clarity
- Update references when ADRs are superseded
"#,
                    adr_ref
                ),
            }),
            range: None,
        })
    }

    fn is_status_value(&self, word: &str) -> bool {
        matches!(
            word.to_lowercase().as_str(),
            "proposed" | "accepted" | "deprecated" | "superseded" | "rejected"
        )
    }

    fn get_status_hover(&self, status: &str) -> Option<Hover> {
        let (description, guidance) = match status.to_lowercase().as_str() {
            "proposed" => (
                "This ADR is under consideration and not yet implemented.",
                "Use this status while the decision is being discussed and refined."
            ),
            "accepted" => (
                "This ADR has been approved and is actively being implemented or is in effect.",
                "This is the target status for most ADRs. It indicates the decision is final and should be followed."
            ),
            "deprecated" => (
                "This ADR is no longer in effect but is kept for historical reference.",
                "Use this when the decision is no longer relevant but you want to preserve the reasoning."
            ),
            "superseded" => (
                "This ADR has been replaced by a newer decision.",
                "Always reference the superseding ADR. This creates a clear decision trail."
            ),
            "rejected" => (
                "This ADR was considered but ultimately not adopted.",
                "Useful for documenting why certain approaches were not taken."
            ),
            _ => return None,
        };

        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!(
                    r#"**ADR Status: {}**

{}

**Guidance:** {}
"#,
                    status.to_uppercase(),
                    description,
                    guidance
                ),
            }),
            range: None,
        })
    }

    fn get_section_hover(&self, section: &str) -> Option<Hover> {
        let info = match section.to_lowercase().as_str() {
            "status" => (
                "Status Section",
                "Indicates the current state of this architectural decision.",
                "Should be one of: Proposed, Accepted, Deprecated, Superseded, or Rejected."
            ),
            "context" => (
                "Context Section", 
                "Describes the issue or situation that motivates this architectural decision.",
                "Explain the problem, constraints, and requirements that led to this decision."
            ),
            "decision" => (
                "Decision Section",
                "States the architectural decision that has been made.",
                "Be clear and specific about what is being decided. This is the core of the ADR."
            ),
            "consequences" => (
                "Consequences Section",
                "Describes the results of applying this decision.",
                "Include both positive and negative outcomes, trade-offs, and any risks introduced."
            ),
            "alternatives" => (
                "Alternatives Section",
                "Lists other options that were considered.",
                "Briefly describe alternatives and why they were not chosen."
            ),
            "related" => (
                "Related Section",
                "References to related ADRs, documents, or external resources.",
                "Help readers understand the broader context and decision dependencies."
            ),
            _ => return None,
        };

        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!(
                    r#"**{}**

{}

**Best Practice:** {}
"#,
                    info.0, info.1, info.2
                ),
            }),
            range: None,
        })
    }

    fn get_terminology_hover(&self, term: &str) -> Option<Hover> {
        let definition = match term.to_lowercase().as_str() {
            "adr" => "Architecture Decision Record - A document that captures an important architectural decision made along with its context and consequences.",
            "architectural" => "Relating to the fundamental structures of a system and the principles guiding their design and evolution.",
            "decision" => "A choice made from available alternatives, typically involving trade-offs and having significant impact.",
            "consequences" => "The results or effects that follow from a decision, including both intended and unintended outcomes.",
            "trade-off" => "A balance achieved between two desirable but incompatible features; a compromise.",
            "constraint" => "A limitation or restriction that affects the design or implementation of a system.",
            "requirement" => "A condition or capability needed by a stakeholder to solve a problem or achieve an objective.",
            _ => return None,
        };

        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: format!("**{}**\n\n{}", term, definition),
            }),
            range: None,
        })
    }

    fn get_title_hover(&self, title_line: &str) -> Option<Hover> {
        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: r#"**ADR Title Format**

The title should follow the pattern: `# ADR-XXX: Brief description`

**Components:**
- **ADR-XXX**: Sequential number (001, 002, etc.)
- **Brief description**: Concise summary of the decision

**Tips:**
- Use consistent numbering across your ADR collection
- Keep titles brief but descriptive
- Use action-oriented language when possible
"#
                .to_string(),
            }),
            range: Some(Range {
                start: lsp_types::Position {
                    line: 0,
                    character: 0,
                },
                end: lsp_types::Position {
                    line: 0,
                    character: title_line.len() as u32,
                },
            }),
        })
    }
}

/// Create hover information for any ADR element
pub fn create_hover_info(content: &str, element_type: &str) -> Option<Hover> {
    let info = match element_type {
        "adr_best_practices" => {
            r#"**ADR Best Practices**

1. **Keep it brief** - ADRs should be concise and focused
2. **Be specific** - Clearly state what is being decided
3. **Include context** - Explain why this decision is needed
4. **Document alternatives** - Show what options were considered
5. **Update status** - Keep the status current as decisions evolve
6. **Link related ADRs** - Build a coherent decision history

**Common Anti-patterns:**
- Too much technical detail
- Missing context or rationale
- Outdated status information
- No consideration of alternatives
"#
        }
        "adr_lifecycle" => {
            r#"**ADR Lifecycle**

1. **Proposed** - Initial draft, under discussion
2. **Accepted** - Approved and implemented
3. **Superseded** - Replaced by newer decision
4. **Deprecated** - No longer relevant
5. **Rejected** - Considered but not adopted

**Transitions:**
- Proposed → Accepted (normal flow)
- Accepted → Superseded (evolved decision)
- Accepted → Deprecated (no longer needed)
- Proposed → Rejected (decided against)
"#
        }
        _ => return None,
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: info.to_string(),
        }),
        range: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_status_hover() {
        let provider = HoverProvider::new();
        let content = "## Status\nAccepted";
        let position = Position {
            line: 1,
            character: 2,
        }; // Position in "Accepted"

        let hover = provider.get_hover_info(content, position).await;
        assert!(hover.is_some());

        if let Some(h) = hover {
            if let HoverContents::Markup(markup) = h.contents {
                assert!(markup.value.contains("ACCEPTED"));
                assert!(markup.value.contains("approved"));
            }
        }
    }

    #[tokio::test]
    async fn test_adr_reference_hover() {
        let provider = HoverProvider::new();
        let content = "Related to ADR-001";
        let position = Position {
            line: 0,
            character: 12,
        }; // Position in "ADR-001"

        let hover = provider.get_hover_info(content, position).await;
        assert!(hover.is_some());

        if let Some(h) = hover {
            if let HoverContents::Markup(markup) = h.contents {
                assert!(markup
                    .value
                    .contains("Architecture Decision Record Reference"));
            }
        }
    }

    #[tokio::test]
    async fn test_section_hover() {
        let provider = HoverProvider::new();
        let content = "## Context";
        let position = Position {
            line: 0,
            character: 5,
        }; // Position in "Context"

        let hover = provider.get_hover_info(content, position).await;
        assert!(hover.is_some());

        if let Some(h) = hover {
            if let HoverContents::Markup(markup) = h.contents {
                assert!(markup.value.contains("Context Section"));
                assert!(markup.value.contains("motivates"));
            }
        }
    }

    #[test]
    fn test_word_extraction() {
        let provider = HoverProvider::new();

        // Test normal word
        let word = provider.get_word_at_position("Hello world", 2);
        assert_eq!(word, Some("Hello".to_string()));

        // Test ADR reference
        let word = provider.get_word_at_position("See ADR-001 for details", 7);
        assert_eq!(word, Some("ADR-001".to_string()));

        // Test at word boundary
        let word = provider.get_word_at_position("test-word", 4);
        assert_eq!(word, Some("test-word".to_string()));
    }

    #[test]
    fn test_status_recognition() {
        let provider = HoverProvider::new();

        assert!(provider.is_status_value("Proposed"));
        assert!(provider.is_status_value("accepted"));
        assert!(provider.is_status_value("DEPRECATED"));
        assert!(!provider.is_status_value("Unknown"));
    }
}
