//! ADR template completion provider for LSP
//! 
//! This module provides intelligent completion suggestions for ADR documents,
//! including templates, section headers, and common patterns.

use tower_lsp::lsp_types::*;
use crate::config::Config;

/// Provide completion suggestions for ADR documents
pub async fn provide_completions(
    text: &str,
    position: Position,
    config: &Config,
) -> Option<CompletionResponse> {
    let lines: Vec<&str> = text.lines().collect();
    let current_line = lines.get(position.line as usize)?;
    let line_prefix = &current_line[..position.character as usize.min(current_line.len())];

    let mut completion_items = Vec::new();

    // Determine completion context
    let context = CompletionContext::analyze(line_prefix, &lines, position.line as usize);

    match context {
        CompletionContext::SectionHeader => {
            completion_items.extend(create_section_completions());
        }
        CompletionContext::Status => {
            completion_items.extend(create_status_completions());
        }
        CompletionContext::Template => {
            completion_items.extend(create_template_completions(config));
        }
        CompletionContext::Reference => {
            completion_items.extend(create_reference_completions(config));
        }
        CompletionContext::List => {
            completion_items.extend(create_list_completions());
        }
        CompletionContext::General => {
            completion_items.extend(create_general_completions());
        }
    }

    if completion_items.is_empty() {
        return None;
    }

    Some(CompletionResponse::Array(completion_items))
}

/// Completion context analysis
enum CompletionContext {
    SectionHeader,
    Status,
    Template,
    Reference,
    List,
    General,
}

impl CompletionContext {
    fn analyze(line_prefix: &str, lines: &[&str], current_line: usize) -> Self {
        // Check if we're typing a section header
        if line_prefix.trim().starts_with("#") {
            return Self::SectionHeader;
        }

        // Check if we're in a status section
        if let Some(section) = find_current_section(lines, current_line) {
            if section.to_lowercase().contains("status") {
                return Self::Status;
            }
        }

        // Check if we're starting a new document (for template completion)
        if current_line < 3 && lines.iter().take(current_line + 1).all(|line| line.trim().is_empty() || line.starts_with('#')) {
            return Self::Template;
        }

        // Check if we're typing a reference
        if line_prefix.contains("ADR") || line_prefix.contains("@") {
            return Self::Reference;
        }

        // Check if we're in a list context
        if line_prefix.trim().starts_with("-") || line_prefix.trim().starts_with("*") {
            return Self::List;
        }

        Self::General
    }
}

/// Find the current section based on cursor position
fn find_current_section(lines: &[&str], current_line: usize) -> Option<&str> {
    for i in (0..=current_line).rev() {
        if let Some(line) = lines.get(i) {
            if line.starts_with("# ") {
                return Some(line.trim_start_matches("# "));
            } else if line.starts_with("## ") {
                return Some(line.trim_start_matches("## "));
            }
        }
    }
    None
}

/// Create section header completions
fn create_section_completions() -> Vec<CompletionItem> {
    let sections = vec![
        ("Status", "Status of this ADR"),
        ("Context", "Context and problem statement"),
        ("Decision", "The architecture decision"),
        ("Consequences", "Consequences of this decision"),
        ("Alternatives", "Alternative solutions considered"),
        ("Related", "Related decisions and references"),
    ];

    sections
        .into_iter()
        .map(|(title, detail)| CompletionItem {
            label: format!("## {}", title),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some(detail.to_string()),
            documentation: Some(Documentation::String(format!(
                "Insert '{}' section header for ADR",
                title
            ))),
            insert_text: Some(format!("## {}\n\n${{1:Content for {} section}}\n", title, title.to_lowercase())),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        })
        .collect()
}

/// Create status completions
fn create_status_completions() -> Vec<CompletionItem> {
    let statuses = vec![
        ("Proposed", "This ADR is proposed and under review"),
        ("Accepted", "This ADR has been accepted and is active"),
        ("Rejected", "This ADR was considered but rejected"),
        ("Superseded", "This ADR has been replaced by another decision"),
        ("Deprecated", "This ADR is no longer recommended"),
    ];

    statuses
        .into_iter()
        .map(|(status, description)| CompletionItem {
            label: status.to_string(),
            kind: Some(CompletionItemKind::VALUE),
            detail: Some(description.to_string()),
            documentation: Some(Documentation::String(format!(
                "Set ADR status to '{}'",
                status
            ))),
            insert_text: Some(status.to_string()),
            ..Default::default()
        })
        .collect()
}

/// Create template completions for new ADRs
fn create_template_completions(config: &Config) -> Vec<CompletionItem> {
    let mut completions = vec![
        CompletionItem {
            label: "ADR Template (Basic)".to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some("Basic ADR template".to_string()),
            documentation: Some(Documentation::String(
                "Insert a basic ADR template with standard sections".to_string(),
            )),
            insert_text: Some(create_basic_template()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            sort_text: Some("00001".to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "ADR Template (Extended)".to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some("Extended ADR template with additional sections".to_string()),
            documentation: Some(Documentation::String(
                "Insert an extended ADR template with comprehensive sections".to_string(),
            )),
            insert_text: Some(create_extended_template()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            sort_text: Some("00002".to_string()),
            ..Default::default()
        },
    ];

    // Add custom templates from config if available
    if let Some(custom_templates) = &config.templates {
        for (i, template) in custom_templates.iter().enumerate() {
            completions.push(CompletionItem {
                label: format!("Custom Template: {}", template.name),
                kind: Some(CompletionItemKind::SNIPPET),
                detail: Some(template.description.clone()),
                insert_text: Some(template.content.clone()),
                insert_text_format: Some(InsertTextFormat::SNIPPET),
                sort_text: Some(format!("00{:03}", i + 10)),
                ..Default::default()
            });
        }
    }

    completions
}

/// Create reference completions
fn create_reference_completions(config: &Config) -> Vec<CompletionItem> {
    let mut completions = vec![
        CompletionItem {
            label: "ADR Reference".to_string(),
            kind: Some(CompletionItemKind::REFERENCE),
            detail: Some("Reference to another ADR".to_string()),
            insert_text: Some("[ADR-${1:number}](${2:path/to/adr})".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "Supersedes Reference".to_string(),
            kind: Some(CompletionItemKind::REFERENCE),
            detail: Some("Mark this ADR as superseding another".to_string()),
            insert_text: Some("Supersedes [ADR-${1:number}](${2:path/to/adr})".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "Superseded By Reference".to_string(),
            kind: Some(CompletionItemKind::REFERENCE),
            detail: Some("Mark this ADR as superseded by another".to_string()),
            insert_text: Some("Superseded by [ADR-${1:number}](${2:path/to/adr})".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
    ];

    completions
}

/// Create list item completions
fn create_list_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "Pro/Con Item".to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some("Pros and cons list item".to_string()),
            insert_text: Some("- **${1:Pro/Con}**: ${2:Description}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "Alternative Option".to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some("Alternative solution item".to_string()),
            insert_text: Some("- **Option ${1:number}**: ${2:Description}\n  - Pros: ${3:advantages}\n  - Cons: ${4:disadvantages}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
    ]
}

/// Create general completions
fn create_general_completions() -> Vec<CompletionItem> {
    vec![
        CompletionItem {
            label: "Date Placeholder".to_string(),
            kind: Some(CompletionItemKind::VALUE),
            detail: Some("Insert current date".to_string()),
            insert_text: Some(chrono::Utc::now().format("%Y-%m-%d").to_string()),
            ..Default::default()
        },
        CompletionItem {
            label: "Decision Rationale".to_string(),
            kind: Some(CompletionItemKind::SNIPPET),
            detail: Some("Template for decision rationale".to_string()),
            insert_text: Some("We decided to ${1:decision} because:\n\n- ${2:reason 1}\n- ${3:reason 2}\n- ${4:reason 3}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
    ]
}

/// Create basic ADR template
fn create_basic_template() -> String {
    format!(
        "# ADR-${{1:number}}: ${{2:Title}}\n\n\
         ## Status\n\n\
         ${{3:Proposed}}\n\n\
         ## Context\n\n\
         ${{4:Context and problem statement}}\n\n\
         ## Decision\n\n\
         ${{5:Architecture decision}}\n\n\
         ## Consequences\n\n\
         ${{6:Consequences of this decision}}\n\n\
         ---\n\
         Date: {}\n\
         Author: ${{7:Author name}}\n",
        chrono::Utc::now().format("%Y-%m-%d")
    )
}

/// Create extended ADR template
fn create_extended_template() -> String {
    format!(
        "# ADR-${{1:number}}: ${{2:Title}}\n\n\
         ## Status\n\n\
         ${{3:Proposed}}\n\n\
         ## Context\n\n\
         ${{4:Context and problem statement}}\n\n\
         ## Decision\n\n\
         ${{5:Architecture decision}}\n\n\
         ## Alternatives\n\n\
         ${{6:Alternative solutions considered}}\n\n\
         ## Consequences\n\n\
         ### Positive\n\n\
         ${{7:Positive consequences}}\n\n\
         ### Negative\n\n\
         ${{8:Negative consequences}}\n\n\
         ### Neutral\n\n\
         ${{9:Neutral consequences}}\n\n\
         ## Related\n\n\
         ${{10:Related decisions and references}}\n\n\
         ---\n\
         Date: {}\n\
         Author: ${{11:Author name}}\n\
         Reviewers: ${{12:Reviewer names}}\n",
        chrono::Utc::now().format("%Y-%m-%d")
    )
}

/// Template definition from configuration
#[derive(Debug, Clone)]
pub struct Template {
    pub name: String,
    pub description: String,
    pub content: String,
}