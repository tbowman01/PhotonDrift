//! Hover information provider for ADR documents
//! 
//! This module provides contextual information when hovering over
//! ADR elements, including section descriptions, references, and metadata.

use tower_lsp::lsp_types::*;
use regex::Regex;
use crate::config::Config;

/// Provide hover information for ADR documents
pub async fn provide_hover_info(
    text: &str,
    position: Position,
    config: &Config,
) -> Option<Hover> {
    let lines: Vec<&str> = text.lines().collect();
    let current_line = lines.get(position.line as usize)?;
    let character = position.character as usize;

    // Find the word/element under the cursor
    let hover_context = HoverContext::analyze(current_line, character, &lines, position.line as usize);

    match hover_context {
        HoverContext::SectionHeader { section } => {
            create_section_hover(&section)
        }
        HoverContext::Status { status } => {
            create_status_hover(&status)
        }
        HoverContext::AdrReference { adr_num } => {
            create_adr_reference_hover(adr_num, config)
        }
        HoverContext::Link { url, text: link_text } => {
            create_link_hover(&url, &link_text)
        }
        HoverContext::Date { date } => {
            create_date_hover(&date)
        }
        HoverContext::Keyword { keyword } => {
            create_keyword_hover(&keyword)
        }
        HoverContext::None => None,
    }
}

/// Context analysis for hover information
enum HoverContext {
    SectionHeader { section: String },
    Status { status: String },
    AdrReference { adr_num: String },
    Link { url: String, text: String },
    Date { date: String },
    Keyword { keyword: String },
    None,
}

impl HoverContext {
    fn analyze(line: &str, character: usize, lines: &[&str], line_num: usize) -> Self {
        // Get word boundaries around cursor position
        let (start, end) = find_word_boundaries(line, character);
        let word = &line[start..end];

        // Check for section headers
        if line.trim().starts_with("#") {
            if let Some(section) = extract_section_name(line) {
                return Self::SectionHeader { section };
            }
        }

        // Check for status keywords
        if is_in_status_section(lines, line_num) {
            let status_keywords = vec!["Proposed", "Accepted", "Rejected", "Superseded", "Deprecated"];
            for status in status_keywords {
                if word.contains(status) {
                    return Self::Status { status: status.to_string() };
                }
            }
        }

        // Check for ADR references
        let adr_ref_regex = Regex::new(r"ADR-?(\d+)").unwrap();
        if let Some(cap) = adr_ref_regex.captures(word) {
            let adr_num = cap.get(1).unwrap().as_str().to_string();
            return Self::AdrReference { adr_num };
        }

        // Check for markdown links
        let link_regex = Regex::new(r"\[([^\]]+)\]\(([^)]+)\)").unwrap();
        if let Some(cap) = link_regex.captures(line) {
            let link_start = cap.get(0).unwrap().start();
            let link_end = cap.get(0).unwrap().end();
            
            if character >= link_start && character <= link_end {
                let link_text = cap.get(1).unwrap().as_str().to_string();
                let url = cap.get(2).unwrap().as_str().to_string();
                return Self::Link { url, text: link_text };
            }
        }

        // Check for dates
        let date_regex = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
        if let Some(cap) = date_regex.find(word) {
            return Self::Date { date: cap.as_str().to_string() };
        }

        // Check for ADR-specific keywords
        let keywords = vec![
            "Context", "Decision", "Consequences", "Alternatives", 
            "Status", "Supersedes", "Superseded", "Related"
        ];
        
        for keyword in keywords {
            if word.contains(keyword) {
                return Self::Keyword { keyword: keyword.to_string() };
            }
        }

        Self::None
    }
}

/// Find word boundaries around a character position
fn find_word_boundaries(line: &str, character: usize) -> (usize, usize) {
    let chars: Vec<char> = line.chars().collect();
    
    if character >= chars.len() {
        return (line.len(), line.len());
    }

    // Find start of word
    let mut start = character;
    while start > 0 {
        if chars[start - 1].is_whitespace() || "()[]{}\"'".contains(chars[start - 1]) {
            break;
        }
        start -= 1;
    }

    // Find end of word
    let mut end = character;
    while end < chars.len() {
        if chars[end].is_whitespace() || "()[]{}\"'".contains(chars[end]) {
            break;
        }
        end += 1;
    }

    // Convert back to byte positions
    let start_byte = chars[..start].iter().map(|c| c.len_utf8()).sum();
    let end_byte = chars[..end].iter().map(|c| c.len_utf8()).sum();

    (start_byte, end_byte)
}

/// Extract section name from header line
fn extract_section_name(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if let Some(name) = trimmed.strip_prefix("### ") {
        return Some(name.to_string());
    }
    if let Some(name) = trimmed.strip_prefix("## ") {
        return Some(name.to_string());
    }
    if let Some(name) = trimmed.strip_prefix("# ") {
        return Some(name.to_string());
    }
    None
}

/// Check if current position is in a status section
fn is_in_status_section(lines: &[&str], current_line: usize) -> bool {
    for i in (0..=current_line).rev() {
        if let Some(line) = lines.get(i) {
            if line.starts_with("##") || line.starts_with("#") {
                return line.to_lowercase().contains("status");
            }
        }
    }
    false
}

/// Create hover information for section headers
fn create_section_hover(section: &str) -> Option<Hover> {
    let documentation = match section.to_lowercase().as_str() {
        "status" => "**Status Section**\n\nDefines the current state of this ADR. Common values:\n- **Proposed**: Under consideration\n- **Accepted**: Approved and active\n- **Rejected**: Considered but not adopted\n- **Superseded**: Replaced by another ADR\n- **Deprecated**: No longer recommended",
        
        "context" => "**Context Section**\n\nDescribes the situation that motivates this decision. Should include:\n- Current architecture state\n- Problems or challenges\n- Constraints and requirements\n- Stakeholder concerns",
        
        "decision" => "**Decision Section**\n\nStates the architecture decision that addresses the context. Should be:\n- Clear and unambiguous\n- Actionable\n- Focused on architecture rather than implementation",
        
        "consequences" => "**Consequences Section**\n\nDescribes the results of applying this decision:\n- **Positive**: Benefits and improvements\n- **Negative**: Costs and risks\n- **Neutral**: Other impacts",
        
        "alternatives" => "**Alternatives Section**\n\nDocuments other options that were considered:\n- Alternative solutions\n- Why they were not chosen\n- Trade-offs comparison",
        
        "related" => "**Related Section**\n\nReferences to related decisions:\n- Links to other ADRs\n- External documentation\n- Standards or guidelines",
        
        _ => return None,
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: documentation.to_string(),
        }),
        range: None,
    })
}

/// Create hover information for status values
fn create_status_hover(status: &str) -> Option<Hover> {
    let documentation = match status.to_lowercase().as_str() {
        "proposed" => "**Proposed Status**\n\nThis ADR is under review and has not yet been decided. It may be accepted, rejected, or require modifications.",
        
        "accepted" => "**Accepted Status**\n\nThis ADR has been approved and should be implemented. The decision is currently active and in effect.",
        
        "rejected" => "**Rejected Status**\n\nThis ADR was considered but ultimately not adopted. The reasoning should be documented for future reference.",
        
        "superseded" => "**Superseded Status**\n\nThis ADR has been replaced by a newer decision. Check for references to the superseding ADR.",
        
        "deprecated" => "**Deprecated Status**\n\nThis ADR is no longer recommended but may still be in use. Plan migration to newer alternatives.",
        
        _ => return None,
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: documentation.to_string(),
        }),
        range: None,
    })
}

/// Create hover information for ADR references
fn create_adr_reference_hover(adr_num: String, config: &Config) -> Option<Hover> {
    // In a real implementation, you would look up the actual ADR file
    let documentation = format!(
        "**ADR Reference**\n\nReference to ADR-{}\n\n*Click to navigate to the referenced ADR document.*",
        adr_num
    );

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: documentation,
        }),
        range: None,
    })
}

/// Create hover information for links
fn create_link_hover(url: &str, link_text: &str) -> Option<Hover> {
    let documentation = if url.starts_with("http") {
        format!("**External Link**\n\n[{}]({})\n\n*Click to open in browser*", link_text, url)
    } else {
        format!("**Internal Link**\n\n{}\n\n*Path: {}*", link_text, url)
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: documentation,
        }),
        range: None,
    })
}

/// Create hover information for dates
fn create_date_hover(date: &str) -> Option<Hover> {
    // Parse the date and provide additional information
    if let Ok(parsed_date) = chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        let now = chrono::Utc::now().naive_utc().date();
        let days_diff = (now - parsed_date).num_days();
        
        let relative_time = if days_diff == 0 {
            "Today".to_string()
        } else if days_diff == 1 {
            "Yesterday".to_string()
        } else if days_diff > 0 {
            format!("{} days ago", days_diff)
        } else {
            format!("In {} days", -days_diff)
        };

        let documentation = format!(
            "**Date Information**\n\n{}\n\n*{}*\n\n*Day of week: {}*",
            date,
            relative_time,
            parsed_date.format("%A")
        );

        Some(Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: documentation,
            }),
            range: None,
        })
    } else {
        None
    }
}

/// Create hover information for ADR keywords
fn create_keyword_hover(keyword: &str) -> Option<Hover> {
    let documentation = match keyword.to_lowercase().as_str() {
        "supersedes" => "**Supersedes**\n\nIndicates that this ADR replaces or overrides a previous decision. The superseded ADR should be marked with 'Superseded' status.",
        
        "superseded" => "**Superseded**\n\nIndicates that this ADR has been replaced by a newer decision. This ADR should not be used for new implementations.",
        
        _ => return None,
    };

    Some(Hover {
        contents: HoverContents::Markup(MarkupContent {
            kind: MarkupKind::Markdown,
            value: documentation.to_string(),
        }),
        range: None,
    })
}