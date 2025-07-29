---
id: "documentation-style-guide"
title: "Documentation Style Guide"
sidebar_label: "Style Guide"
sidebar_position: "5"
description: "Comprehensive style guide for PhotonDrift documentation"
slug: "/development/documentation-style-guide"
tags: "["documentation", "style", "contributing", "guidelines"]"
---


# Documentation Style Guide

This guide ensures consistent, high-quality documentation across the PhotonDrift project. Following these guidelines helps create a better experience for users and contributors.

## 📋 Quick Reference

### File Structure
- Use kebab-case for filenames: `user-guide.md`
- Organize by logical categories in `docs/` subdirectories
- Include descriptive frontmatter in every file

### Writing Style
- **Clear and Concise**: Get to the point quickly
- **Active Voice**: "Run the command" vs "The command should be run"
- **Consistent Terminology**: Use the same terms throughout
- **User-Focused**: Write for the reader's needs

### Code Examples
- Always include complete, runnable examples
- Use syntax highlighting for all code blocks
- Include expected output when helpful
- Test all examples before publishing

## 📝 Content Guidelines

### Page Structure

Every documentation page should follow this structure:

```markdown
---
title: "Descriptive Page Title"
sidebar_label: "Short Label"
sidebar_position: 1
description: "Brief description for SEO and navigation"
tags: ["relevant", "tags", "here"]
---

# Page Title (H1 - automatic from frontmatter)

Brief introduction paragraph explaining what this page covers.

## Main Section (H2)

Content organized into logical sections.

### Subsection (H3)

Detailed content with examples.

#### Deep Details (H4 - use sparingly)

Only when necessary for complex topics.
```

### Frontmatter Standards

**Required Fields:**
```yaml
---
title: "Human-readable page title"
sidebar_label: "Short navigation label"
description: "Brief description for SEO and cards"
---
```

**Optional Fields:**
```yaml
---
sidebar_position: 1              # Order in sidebar
tags: ["tag1", "tag2"]          # For categorization
slug: "/custom-url"             # Custom URL (rarely needed)
keywords: ["seo", "keywords"]   # Additional SEO terms
---
```

**Tag Conventions:**
- `getting-started` - User onboarding content
- `development` - Developer/contributor content
- `architecture` - Technical design content
- `deployment` - Operations and deployment
- `ml-features` - AI/ML related content
- `cli` - Command-line interface documentation
- `api` - API reference documentation
- `troubleshooting` - Problem-solving guides

## ✍️ Writing Style

### Tone and Voice

**DO:**
- Write in a friendly, helpful tone
- Use "you" to address the reader directly
- Be encouraging and supportive
- Explain the "why" behind instructions

**DON'T:**
- Use overly formal or academic language
- Assume extensive prior knowledge
- Use jargon without explanation
- Write in passive voice

### Language Guidelines

**Consistency:**
- PhotonDrift (not photondrift or Photon Drift)
- ADR (not adr or Adr)
- CLI (not cli or Cli)
- WebAssembly (not WASM in prose, WASM in code)

**Formatting:**
- **Bold** for UI elements, important terms, and emphasis
- `Code formatting` for commands, file names, and code
- *Italics* for subtle emphasis (use sparingly)

**Lists:**
- Use parallel structure in bullet points
- Start with action verbs when appropriate
- Keep items concise and scannable

### Technical Writing

**Code References:**
- Use backticks for inline code: `adrscan diff`
- Include full commands: `adrscan diff --adr-dir ./docs/adr`
- Specify working directory when needed

**File Paths:**
- Use forward slashes: `docs/getting-started/quick-start.md`
- Include file extensions: `package.json`
- Use relative paths when possible

**Version References:**
- Specify versions when important: "Node.js 18 or later"
- Use "latest" for current stable versions
- Include version compatibility notes

## 💻 Code Examples

### Code Block Standards

**Basic Code Block:**
````markdown
```bash
# Comment explaining what this does
adrscan init --adr-dir ./docs/adr
```
````

**With Title:**
````markdown
```bash title="Initialize ADR structure"
# Create ADR directory structure
adrscan init --adr-dir ./docs/adr
```
````

**With Output:**
````markdown
```bash
$ adrscan inventory --adr-dir ./docs/adr
# Output:
Found 3 ADRs:
  - 0001-use-rust-for-cli.md
  - 0002-adopt-ml-for-drift-detection.md
  - 0003-containerization-strategy.md
```
````

### Language-Specific Guidelines

**Bash/Shell:**
- Include the `$` prompt for single commands
- Use `#` for comments
- Show working directory when relevant

**Rust:**
```rust
// Use clear, commented examples
fn main() {
    println!("Hello, PhotonDrift!");
}
```

**YAML Configuration:**
```yaml
# Include comments explaining options
adr_dir: "./docs/decisions"
ml:
  enabled: true
  confidence_threshold: 0.7
```

**JSON:**
```json
{
  "description": "Use consistent formatting",
  "version": "1.0.0"
}
```

## 🎨 Interactive Components

### CliCommand Component

Use for interactive CLI examples:

```markdown
import CliCommand from '@site/src/components/CliCommand';

<CliCommand
  command="adrscan diff --adr-dir ./docs/adr --directory ./src"
  description="Detect architectural drift with AI analysis"
  title="Drift Detection Example"
  showCopy={true}
/>
```

**Props:**
- `command` (required): The CLI command
- `description`: Brief explanation
- `title`: Display title
- `output`: Expected output
- `showCopy`: Show copy button (default: true)

### FeatureGrid Component

Use for showcasing features:

```markdown
import FeatureGrid from '@site/src/components/FeatureGrid';

<FeatureGrid 
  features={[
    {
      title: "Feature Name",
      description: "Brief feature description",
      icon: "🚀",
      status: "completed", // completed, in-progress, planned
      link: "/docs/feature-link"
    }
  ]}
  columns={3}
/>
```

### Admonitions

Use Docusaurus admonitions for important information:

```markdown
:::tip Pro Tip
This is a helpful tip for advanced users.
:::

:::warning Important
This is a warning about potential issues.
:::

:::danger Critical
This is critical information that could cause problems.
:::

:::info Note
This is general information or context.
:::
```

## 🔗 Links and References

### Internal Links

**To Other Docs:**
```markdown
[Quick Start Guide](/development/quick-start)
[Configuration](/getting-started/config)
```

**With Custom Text:**
```markdown
See the [CLI reference](/development/cli) for complete command options.
```

### External Links

**Basic External Link:**
```markdown
[Docusaurus](https://docusaurus.io)
```

**With Context:**
```markdown
PhotonDrift uses [Rust](https://rust-lang.org) for performance and safety.
```

### Code References

**Link to Code:**
```markdown
The parsing logic is implemented in [`src/parser.rs:45`](../../../src/parser.rs#L45).
```

## 📊 Tables and Data

### Simple Tables

```markdown
| Command | Description | Status |
|---------|-------------|--------|
| `init` | Initialize ADR structure | ✅ Complete |
| `diff` | Detect drift | 🔄 In Progress |
| `propose` | Generate proposals | 📋 Planned |
```

### Complex Data

For complex data, consider using:
- Nested lists instead of complex tables
- Multiple simple tables instead of one complex table
- Interactive components for dynamic data

## 🖼️ Images and Media

### Image Guidelines

**File Organization:**
```
docs-site/static/img/
├── screenshots/         # UI screenshots
├── diagrams/           # Architecture diagrams
├── logos/             # Brand assets
└── examples/          # Example outputs
```

**Image Syntax:**
```markdown
![Alt text for accessibility](./img/screenshot.png)
```

**With Caption:**
```markdown
![PhotonDrift CLI in action](./img/cli-example.png)
*Figure 1: PhotonDrift detecting architectural drift*
```

### Accessibility

- Always include descriptive alt text
- Keep images under 1MB when possible
- Use WebP format for better performance
- Provide text alternatives for complex diagrams

## 🔍 SEO and Discoverability

### Page Optimization

**Title Guidelines:**
- Keep under 60 characters
- Include relevant keywords
- Be descriptive and unique

**Description Guidelines:**
- 120-160 characters
- Include primary keywords
- Compelling summary of page content

**Header Structure:**
- Use only one H1 per page (automatic from title)
- Create logical hierarchy with H2, H3, H4
- Don't skip heading levels

### Search Optimization

**Keywords:**
- Use relevant technical terms naturally
- Include common abbreviations and synonyms
- Don't keyword stuff or sacrifice readability

**Internal Linking:**
- Link to related content within the site
- Use descriptive link text
- Create topic clusters with hub pages

## ✅ Quality Checklist

Before submitting documentation:

### Content Review
- [ ] Clear, concise writing
- [ ] Accurate technical information
- [ ] Complete code examples
- [ ] Proper frontmatter
- [ ] Consistent terminology

### Technical Review
- [ ] All links work correctly
- [ ] Code examples are tested
- [ ] Images have alt text
- [ ] Proper heading hierarchy
- [ ] Valid Markdown syntax

### Style Review
- [ ] Follows style guide
- [ ] Consistent formatting
- [ ] Appropriate tone
- [ ] Proper grammar and spelling
- [ ] Accessible to target audience

### Testing
- [ ] Local build succeeds
- [ ] Content syncs correctly
- [ ] Links validate
- [ ] Mobile-friendly display
- [ ] Search functionality works

## 🛠️ Tools and Automation

### Validation Tools

The documentation system includes automated checks:

- **Link Validation**: Checks internal and external links
- **Markdown Linting**: Ensures consistent formatting
- **Spell Checking**: Catches typos and errors
- **Build Validation**: Ensures site builds correctly

### Development Workflow

```bash
# Start development with live reload
./scripts/dev-docs.sh dev

# Sync content after changes
./scripts/dev-docs.sh sync

# Full build with validation
./scripts/build-docs.sh --clean
```

### CI/CD Integration

Documentation automatically:
- Validates on pull requests
- Deploys to GitHub Pages on merge
- Creates preview deployments for PRs
- Updates search index

## 📞 Getting Help

**Questions about documentation?**
- Open a [discussion](https://github.com/tbowman01/PhotonDrift/discussions)
- Check existing documentation in `docs/development/`
- Ask in pull request comments

**Found an error?**
- Open an [issue](https://github.com/tbowman01/PhotonDrift/issues)
- Submit a pull request with fixes
- Use the "documentation" label

---

*This style guide is a living document. Suggest improvements through pull requests or discussions.*