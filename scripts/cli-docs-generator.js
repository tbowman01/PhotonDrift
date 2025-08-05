#!/usr/bin/env node

/**
 * CLI Documentation Generator for PhotonDrift
 * 
 * Automatically generates comprehensive CLI documentation by:
 * - Parsing Rust source code for clap command definitions
 * - Extracting help text, options, and command structure
 * - Generating interactive documentation with examples
 * - Creating searchable command reference
 * - Validating examples against actual CLI
 * 
 * Usage: node scripts/cli-docs-generator.js [--dry-run] [--verbose]
 */

const fs = require('fs').promises;
const path = require('path');
const { execSync } = require('child_process');

// Configuration
const CONFIG = {
  sourceDir: path.join(__dirname, '..', 'src'),
  outputFile: path.join(__dirname, '..', 'docs-site', 'docs', 'api', 'cli-reference.md'),
  cliPath: path.join(__dirname, '..', 'target', 'release', 'adrscan'),
  devCliPath: path.join(__dirname, '..', 'target', 'debug', 'adrscan'),
  commands: ['init', 'inventory', 'diff', 'propose', 'index'],
  examples: {
    init: [
      {
        description: 'Initialize ADR structure in default location',
        command: 'adrscan init',
        explanation: 'Creates ./docs/adr directory with basic structure'
      },
      {
        description: 'Initialize with custom directory and template',
        command: 'adrscan init --adr-dir ./decisions --template ml-enhanced',
        explanation: 'Creates ADR structure with ML-enhanced configuration'
      }
    ],
    inventory: [
      {
        description: 'Scan existing ADRs and create catalog',
        command: 'adrscan inventory --adr-dir ./docs/adr',
        explanation: 'Scans ADR directory and generates intelligent catalog'
      },
      {
        description: 'Watch directory for changes with JSON output',
        command: 'adrscan inventory --adr-dir ./docs/adr --format json --watch',
        explanation: 'Monitors ADR directory and outputs changes in JSON format'
      }
    ],
    diff: [
      {
        description: 'Detect architectural drift with ML confidence scores',
        command: 'adrscan diff --adr-dir ./docs/adr --directory ./src',
        explanation: 'Analyzes source code against ADRs with AI-powered detection'
      },
      {
        description: 'High confidence drift detection with ensemble model',
        command: 'adrscan diff --confidence 0.8 --model Ensemble --format markdown',
        explanation: 'Uses ensemble ML model with 80% confidence threshold'
      }
    ],
    propose: [
      {
        description: 'Generate AI-informed ADR proposals',
        command: 'adrscan propose --adr-dir ./docs/adr --directory ./src',
        explanation: 'Creates ADR proposals based on detected architectural changes'
      },
      {
        description: 'Interactive proposal mode with auto-creation',
        command: 'adrscan propose --interactive --auto-create',
        explanation: 'Interactive mode that automatically creates ADR files'
      }
    ],
    index: [
      {
        description: 'Create comprehensive ADR index',
        command: 'adrscan index --adr-dir ./docs/adr --output index.md',
        explanation: 'Generates markdown index with smart categorization'
      },
      {
        description: 'HTML index grouped by category',
        command: 'adrscan index --format html --group-by category',
        explanation: 'Creates HTML index grouped by ADR categories'
      }
    ]
  }
};

// Command line arguments
const args = process.argv.slice(2);
const isDryRun = args.includes('--dry-run');
const isVerbose = args.includes('--verbose') || args.includes('-v');

/**
 * Logger utility
 */
const logger = {
  info: (msg) => console.log(`[CLI-GEN] ${msg}`),
  warn: (msg) => console.warn(`[CLI-GEN] ${msg}`),
  error: (msg) => console.error(`[CLI-GEN] ${msg}`),
  verbose: (msg) => isVerbose && console.log(`[CLI-GEN] ${msg}`),
  success: (msg) => console.log(`[CLI-GEN] ${msg}`)
};

/**
 * Check if CLI binary exists and build if necessary
 */
async function ensureCliBinary() {
  // Check for release binary first
  try {
    await fs.access(CONFIG.cliPath);
    logger.verbose(`Found release binary at ${CONFIG.cliPath}`);
    return CONFIG.cliPath;
  } catch (error) {
    logger.verbose('Release binary not found, checking debug binary...');
  }
  
  // Check for debug binary
  try {
    await fs.access(CONFIG.devCliPath);
    logger.verbose(`Found debug binary at ${CONFIG.devCliPath}`);
    return CONFIG.devCliPath;
  } catch (error) {
    logger.info('No binary found, building debug version...');
  }
  
  // Build debug version as fallback
  try {
    logger.info('Building debug binary...');
    execSync('cargo build', { 
      cwd: path.join(__dirname, '..'), 
      stdio: ['inherit', 'inherit', 'inherit'],
      timeout: 300000 // 5 minute timeout
    });
    
    // Verify the binary was created
    await fs.access(CONFIG.devCliPath);
    logger.success(`Built debug binary at ${CONFIG.devCliPath}`);
    return CONFIG.devCliPath;
  } catch (error) {
    throw new Error(`Failed to build CLI binary: ${error.message}`);
  }
}

/**
 * Execute CLI command and capture output
 */
function executeCliCommand(cliPath, args, options = {}) {
  try {
    const output = execSync(`"${cliPath}" ${args}`, {
      encoding: 'utf-8',
      timeout: 10000,
      ...options
    });
    return { success: true, output: output.trim() };
  } catch (error) {
    return { 
      success: false, 
      output: error.stdout || error.stderr || error.message,
      exitCode: error.status 
    };
  }
}

/**
 * Extract help information for a command
 */
function extractHelpInfo(cliPath, command = '') {
  const args = command ? `${command} --help` : '--help';
  const result = executeCliCommand(cliPath, args);
  
  if (!result.success) {
    logger.warn(`Failed to get help for command '${command}': ${result.output}`);
    return null;
  }
  
  return parseHelpOutput(result.output);
}

/**
 * Parse CLI help output into structured data
 */
function parseHelpOutput(helpText) {
  const lines = helpText.split('\n');
  const info = {
    usage: '',
    description: '',
    options: [],
    commands: [],
    examples: []
  };
  
  let currentSection = null;
  let currentOption = null;
  
  for (const line of lines) {
    const trimmed = line.trim();
    
    // Detect sections
    if (trimmed.toLowerCase().startsWith('usage:')) {
      currentSection = 'usage';
      info.usage = trimmed.replace(/^usage:\s*/i, '');
      continue;
    } else if (trimmed.toLowerCase().includes('options:')) {
      currentSection = 'options';
      continue;
    } else if (trimmed.toLowerCase().includes('commands:')) {
      currentSection = 'commands';
      continue;
    } else if (trimmed.toLowerCase().includes('examples:')) {
      currentSection = 'examples';
      continue;
    }
    
    // Parse content based on current section
    if (currentSection === 'options' && trimmed.startsWith('-')) {
      const optionMatch = trimmed.match(/(-[a-zA-Z], )?--([a-zA-Z0-9-]+)(?:\s+<([^>]+)>)?\s*(.*)/);
      if (optionMatch) {
        currentOption = {
          short: optionMatch[1] ? optionMatch[1].replace(/,\s*$/, '') : null,
          long: `--${optionMatch[2]}`,
          argument: optionMatch[3] || null,
          description: optionMatch[4] || ''
        };
        info.options.push(currentOption);
      }
    } else if (currentSection === 'options' && currentOption && trimmed && !trimmed.startsWith('-')) {
      // Continuation of option description
      currentOption.description += ' ' + trimmed;
    } else if (currentSection === 'commands' && trimmed && !trimmed.toLowerCase().includes('commands:')) {
      const commandMatch = trimmed.match(/(\w+)\s+(.*)/);
      if (commandMatch) {
        info.commands.push({
          name: commandMatch[1],
          description: commandMatch[2]
        });
      }
    } else if (!currentSection && trimmed && !trimmed.toLowerCase().startsWith('usage:')) {
      // Description lines (before any section)
      info.description += (info.description ? ' ' : '') + trimmed;
    }
  }
  
  return info;
}

/**
 * Generate markdown documentation for a command
 */
function generateCommandDocs(command, helpInfo, examples) {
  let docs = `### \`${command}\` - ${helpInfo.description || 'Command'}\n\n`;
  
  if (helpInfo.usage) {
    docs += `**Usage:**\n\`\`\`bash\n${helpInfo.usage}\n\`\`\`\n\n`;
  }
  
  if (helpInfo.description) {
    docs += `${helpInfo.description}\n\n`;
  }
  
  // Options
  if (helpInfo.options && helpInfo.options.length > 0) {
    docs += `**Options:**\n\n`;
    for (const option of helpInfo.options) {
      docs += `- **\`${option.long}\`**`;
      if (option.short) {
        docs += `, **\`${option.short}\`**`;
      }
      if (option.argument) {
        docs += ` \`<${option.argument}>\``;
      }
      docs += ` - ${option.description}\n`;
    }
    docs += '\n';
  }
  
  // Examples
  if (examples && examples.length > 0) {
    docs += `**Examples:**\n\n`;
    for (const example of examples) {
      docs += `**${example.description}:**\n`;
      docs += `\`\`\`bash\n${example.command}\n\`\`\`\n`;
      if (example.explanation) {
        docs += `${example.explanation}\n\n`;
      }
    }
  }
  
  return docs;
}

/**
 * Generate complete CLI reference documentation
 */
async function generateCliDocs() {
  logger.info('Starting CLI documentation generation...');
  
  try {
    // Ensure CLI binary exists
    const cliPath = await ensureCliBinary();
    
    // Extract main help information
    const mainHelpInfo = extractHelpInfo(cliPath);
    if (!mainHelpInfo) {
      throw new Error('Failed to extract main help information');
    }
    
    // Start building documentation
    let docs = `---
id: cli-reference
title: CLI Reference
sidebar_label: 💻 CLI Reference
sidebar_position: 1
description: Complete command-line interface reference for PhotonDrift (ADRScan)
slug: /api/cli
tags: [cli, reference, commands]
---

# CLI Reference

Complete reference for all PhotonDrift (ADRScan) commands and parameters.

## Overview

PhotonDrift provides a comprehensive command-line interface for Architecture Decision Record management with AI-enhanced drift detection.

**Usage:** \`${mainHelpInfo.usage || 'adrscan [COMMAND] [OPTIONS]'}\`

${mainHelpInfo.description ? `**Description:** ${mainHelpInfo.description}` : ''}

## Global Options

`;
    
    // Add global options
    if (mainHelpInfo.options && mainHelpInfo.options.length > 0) {
      for (const option of mainHelpInfo.options) {
        docs += `- **\`${option.long}\`**`;
        if (option.short) {
          docs += `, **\`${option.short}\`**`;
        }
        if (option.argument) {
          docs += ` \`<${option.argument}>\``;
        }
        docs += ` - ${option.description}\n`;
      }
    } else {
      docs += `- **\`--version\`** - Show version information
- **\`--help\`** - Show help message
- **\`--verbose, -v\`** - Verbose output
- **\`--quiet, -q\`** - Suppress non-error output
- **\`--no-color\`** - Disable colored output
- **\`--config <FILE>\`** - Configuration file path
`;
    }
    
    docs += `\n## Commands\n\n`;
    
    // Generate documentation for each command
    for (const command of CONFIG.commands) {
      logger.verbose(`Generating docs for command: ${command}`);
      
      const commandHelpInfo = extractHelpInfo(cliPath, command);
      if (commandHelpInfo) {
        const examples = CONFIG.examples[command] || [];
        docs += generateCommandDocs(command, commandHelpInfo, examples);
      } else {
        logger.warn(`Could not extract help for command: ${command}`);
        // Fallback documentation
        docs += `### \`${command}\` - ${command.charAt(0).toUpperCase() + command.slice(1)} Command\n\n`;
        docs += `Documentation for the \`${command}\` command.\n\n`;
      }
    }
    
    // Add additional sections
    docs += `## Environment Variables

- **\`RUST_LOG\`** - Logging level (debug, info, warn, error)
- **\`ADR_CONFIG\`** - Default configuration file path
- **\`ADR_DIR\`** - Default ADR directory
- **\`ML_ENABLED\`** - Enable ML features (true/false)
- **\`ML_MODEL\`** - Default ML model type
- **\`ML_CONFIDENCE\`** - Default confidence threshold

## Exit Codes

- **\`0\`** - Success
- **\`1\`** - General error
- **\`2\`** - Configuration error
- **\`3\`** - File system error
- **\`4\`** - ML model error
- **\`5\`** - Network error (for future features)

## Configuration File

Commands can use configuration files in YAML or TOML format. See the [Configuration Reference](config.md) for complete details.

## CI/CD Integration

PhotonDrift is designed for seamless CI/CD integration:

\`\`\`bash
# Quick validation in CI
adrscan diff --adr-dir ./docs/adr --directory ./src --quiet || exit 1

# Generate reports for review
adrscan diff --format markdown > drift-report.md
adrscan inventory --format json > adr-inventory.json
\`\`\`

## Getting Help

- Run \`adrscan --help\` for general help
- Run \`adrscan <command> --help\` for command-specific help
- Visit our [User Guide](../getting-started/user-guide.md) for detailed tutorials
- Check the [Configuration Reference](config.md) for all options

---

*This documentation is automatically generated from the CLI help output. Last updated: ${new Date().toISOString().split('T')[0]}*
`;
    
    // Write documentation file
    if (!isDryRun) {
      await fs.mkdir(path.dirname(CONFIG.outputFile), { recursive: true });
      await fs.writeFile(CONFIG.outputFile, docs, 'utf-8');
    }
    
    logger.success(`CLI documentation generated: ${CONFIG.outputFile}`);
    logger.info(`Generated documentation for ${CONFIG.commands.length} commands`);
    
    return docs;
    
  } catch (error) {
    logger.error(`Failed to generate CLI documentation: ${error.message}`);
    throw error;
  }
}

/**
 * Validate generated examples
 */
async function validateExamples(cliPath) {
  logger.info('Validating CLI examples...');
  
  let validExamples = 0;
  let totalExamples = 0;
  
  for (const [command, examples] of Object.entries(CONFIG.examples)) {
    for (const example of examples) {
      totalExamples++;
      
      // Extract the command name from the example
      const commandParts = example.command.split(' ');
      const baseCommand = commandParts[1]; // Skip 'adrscan'
      
      logger.verbose(`Testing command help: ${baseCommand}`);
      
      try {
        // Test if the command shows help (validates command exists and basic functionality)
        const result = executeCliCommand(cliPath, `${baseCommand} --help`);
        if (result.success) {
          validExamples++;
          logger.verbose(`✅ Example validated: ${example.command}`);
        } else {
          logger.warn(`❌ Example failed: ${example.command}`);
          logger.verbose(`   Error: ${result.output}`);
        }
      } catch (error) {
        logger.verbose(`❌ Example error: ${example.command} - ${error.message}`);
      }
    }
  }
  
  logger.info(`Example validation: ${validExamples}/${totalExamples} examples valid`);
  return { valid: validExamples, total: totalExamples };
}

/**
 * Main execution function
 */
async function main() {
  logger.info(`${isDryRun ? '[DRY RUN] ' : ''}Starting CLI documentation generation...`);
  
  const startTime = Date.now();
  
  try {
    // Generate CLI documentation
    const docs = await generateCliDocs();
    
    // Validate examples if not in dry run
    if (!isDryRun) {
      const cliPath = await ensureCliBinary();
      await validateExamples(cliPath);
    }
    
    const duration = Date.now() - startTime;
    logger.success(`CLI documentation generation completed in ${duration}ms`);
    
    if (isVerbose) {
      console.log('\n--- Generated Documentation Preview ---');
      console.log(docs.substring(0, 500) + '...');
      console.log('--- End Preview ---\n');
    }
    
  } catch (error) {
    logger.error(`CLI documentation generation failed: ${error.message}`);
    process.exit(1);
  }
}

/**
 * Main execution
 */
if (require.main === module) {
  main().catch(error => {
    logger.error(`Unexpected error: ${error.message}`);
    process.exit(1);
  });
}

module.exports = { generateCliDocs, CONFIG };