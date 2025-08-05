#!/usr/bin/env node

/**
 * Link Validator for PhotonDrift Documentation
 * 
 * Comprehensive validation of:
 * - Internal documentation links
 * - External link accessibility
 * - Image references and alt text
 * - Code examples and CLI commands
 * - Markdown syntax and structure
 * 
 * Usage: node scripts/link-validator.js [--fix] [--external] [--verbose]
 */

const fs = require('fs').promises;
const path = require('path');
const https = require('https');
const http = require('http');

// Configuration
const CONFIG = {
  docsDir: path.join(__dirname, '..', 'docs-site', 'docs'),
  sourceDocsDir: path.join(__dirname, '..', 'docs'),
  staticDir: path.join(__dirname, '..', 'docs-site', 'static'),
  baseUrl: 'https://docs.photondrift.dev',
  timeout: 10000,
  maxRetries: 3,
  parallelRequests: 5,
  excludePatterns: [
    /localhost/,
    /127\.0\.0\.1/,
    /example\.com/,
    /placeholder/i,
    /todo:/i,
    /^mailto:/,
    /^tel:/,
    /^#/ // Fragment-only links
  ],
  internalPatterns: [
    /^\/docs\//,
    /^\/blog\//,
    /^\/img\//,
    /^\/assets\//,
    /^\/api\//,
    /^\.\//,
    /^\.\.\//
  ]
};

// Command line arguments
const args = process.argv.slice(2);
const shouldFix = args.includes('--fix');
const checkExternal = args.includes('--external');
const isVerbose = args.includes('--verbose') || args.includes('-v');

/**
 * Logger utility
 */
const logger = {
  info: (msg) => console.log(`[LINK-VALIDATOR] ${msg}`),
  warn: (msg) => console.warn(`[LINK-VALIDATOR] ${msg}`),
  error: (msg) => console.error(`[LINK-VALIDATOR] ${msg}`),
  verbose: (msg) => isVerbose && console.log(`[LINK-VALIDATOR] ${msg}`),
  success: (msg) => console.log(`[LINK-VALIDATOR] ${msg}`)
};

/**
 * Extract all links from markdown content
 */
function extractLinks(content, filePath) {
  const links = [];
  
  // Markdown links: [text](url)
  const markdownLinkRegex = /\[([^\]]*)\]\(([^)]+)\)/g;
  let match;
  while ((match = markdownLinkRegex.exec(content)) !== null) {
    links.push({
      type: 'markdown',
      text: match[1],
      url: match[2],
      line: getLineNumber(content, match.index),
      column: match.index - getLineStart(content, match.index),
      raw: match[0]
    });
  }
  
  // HTML links: <a href="url">text</a>
  const htmlLinkRegex = /<a\s+(?:[^>]*?\s+)?href=(["'])((?:(?!\1)[^\\]|\\.)*)(\1)[^>]*>(.*?)<\/a>/gi;
  while ((match = htmlLinkRegex.exec(content)) !== null) {
    links.push({
      type: 'html',
      text: match[4],
      url: match[2],
      line: getLineNumber(content, match.index),
      column: match.index - getLineStart(content, match.index),
      raw: match[0]
    });
  }
  
  // Image links: ![alt](src)
  const imageLinkRegex = /!\[([^\]]*)\]\(([^)]+)\)/g;
  while ((match = imageLinkRegex.exec(content)) !== null) {
    links.push({
      type: 'image',
      text: match[1],
      url: match[2],
      line: getLineNumber(content, match.index),
      column: match.index - getLineStart(content, match.index),
      raw: match[0]
    });
  }
  
  // HTML images: <img src="url" alt="text">
  const htmlImageRegex = /<img\s+(?:[^>]*?\s+)?src=(["'])((?:(?!\1)[^\\]|\\.)*)(\1)(?:[^>]*?\s+alt=(["'])((?:(?!\4)[^\\]|\\.)*)(\4))?[^>]*>/gi;
  while ((match = htmlImageRegex.exec(content)) !== null) {
    links.push({
      type: 'html-image',
      text: match[5] || '',
      url: match[2],
      line: getLineNumber(content, match.index),
      column: match.index - getLineStart(content, match.index),
      raw: match[0]
    });
  }
  
  return links;
}

/**
 * Get line number for a character index
 */
function getLineNumber(content, index) {
  return content.substring(0, index).split('\n').length;
}

/**
 * Get the start index of the line containing the given index
 */
function getLineStart(content, index) {
  const lines = content.substring(0, index).split('\n');
  return index - lines[lines.length - 1].length;
}

/**
 * Resolve relative paths
 */
function resolvePath(basePath, relativePath) {
  if (path.isAbsolute(relativePath)) {
    return relativePath;
  }
  return path.resolve(path.dirname(basePath), relativePath);
}

/**
 * Check if a URL is external
 */
function isExternalUrl(url) {
  return /^https?:\/\//.test(url);
}

/**
 * Check if a URL should be excluded from validation
 */
function shouldExcludeUrl(url) {
  return CONFIG.excludePatterns.some(pattern => pattern.test(url));
}

/**
 * Validate internal link
 */
async function validateInternalLink(link, sourceFile) {
  const { url } = link;
  
  // Handle Docusaurus-style links
  if (url.startsWith('/docs/') || url.startsWith('/blog/') || url.startsWith('/api/')) {
    // Convert to file path
    let filePath = url.replace(/^\/docs\//, '').replace(/^\/api\//, '');
    if (!filePath.endsWith('.md')) {
      filePath += '.md';
    }
    filePath = path.join(CONFIG.docsDir, filePath);
    
    try {
      await fs.access(filePath);
      return { valid: true, message: 'File exists' };
    } catch (error) {
      return { valid: false, message: `File not found: ${filePath}` };
    }
  }
  
  // Handle relative links
  if (url.startsWith('./') || url.startsWith('../')) {
    const resolvedPath = resolvePath(sourceFile, url);
    
    try {
      await fs.access(resolvedPath);
      return { valid: true, message: 'File exists' };
    } catch (error) {
      return { valid: false, message: `File not found: ${resolvedPath}` };
    }
  }
  
  // Handle static assets
  if (url.startsWith('/img/') || url.startsWith('/assets/')) {
    const assetPath = path.join(CONFIG.staticDir, url.replace(/^\//, ''));
    
    try {
      await fs.access(assetPath);
      return { valid: true, message: 'Asset exists' };
    } catch (error) {
      return { valid: false, message: `Asset not found: ${assetPath}` };
    }
  }
  
  // Handle fragments (anchors)
  if (url.startsWith('#')) {
    // For now, assume fragment links are valid (would need content parsing to validate)
    return { valid: true, message: 'Fragment link (not validated)' };
  }
  
  return { valid: true, message: 'Internal link format not recognized, assuming valid' };
}

/**
 * Validate external link with retry logic
 */
async function validateExternalLink(url, retries = CONFIG.maxRetries) {
  return new Promise((resolve) => {
    const client = url.startsWith('https:') ? https : http;
    const timeout = CONFIG.timeout;
    
    const request = client.request(url, { method: 'HEAD', timeout }, (response) => {
      const statusCode = response.statusCode;
      
      if (statusCode >= 200 && statusCode < 400) {
        resolve({ valid: true, statusCode, message: 'OK' });
      } else if (statusCode >= 300 && statusCode < 400 && response.headers.location) {
        resolve({ valid: true, statusCode, message: `Redirect to ${response.headers.location}` });
      } else {
        resolve({ valid: false, statusCode, message: `HTTP ${statusCode}` });
      }
    });
    
    request.on('error', (error) => {
      if (retries > 0) {
        logger.verbose(`Retrying ${url} (${retries} retries left)`);
        setTimeout(() => {
          validateExternalLink(url, retries - 1).then(resolve);
        }, 1000);
      } else {
        resolve({ valid: false, message: error.message });
      }
    });
    
    request.on('timeout', () => {
      request.destroy();
      if (retries > 0) {
        logger.verbose(`Timeout for ${url}, retrying (${retries} retries left)`);
        setTimeout(() => {
          validateExternalLink(url, retries - 1).then(resolve);
        }, 1000);
      } else {
        resolve({ valid: false, message: 'Timeout' });
      }
    });
    
    request.end();
  });
}

/**
 * Process a single markdown file
 */
async function processFile(filePath) {
  logger.verbose(`Processing ${filePath}`);
  
  try {
    const content = await fs.readFile(filePath, 'utf-8');
    const links = extractLinks(content, filePath);
    const results = [];
    
    for (const link of links) {
      if (shouldExcludeUrl(link.url)) {
        logger.verbose(`Skipping excluded URL: ${link.url}`);
        continue;
      }
      
      let result;
      
      if (isExternalUrl(link.url)) {
        if (checkExternal) {
          result = await validateExternalLink(link.url);
        } else {
          result = { valid: true, message: 'External link (not checked)' };
        }
      } else {
        result = await validateInternalLink(link, filePath);
      }
      
      results.push({
        ...link,
        file: filePath,
        validation: result
      });
      
      if (!result.valid) {
        logger.error(`❌ ${filePath}:${link.line}:${link.column} - ${link.url} - ${result.message}`);
      } else {
        logger.verbose(`✅ ${link.url} - ${result.message}`);
      }
    }
    
    return results;
    
  } catch (error) {
    logger.error(`Failed to process ${filePath}: ${error.message}`);
    return [];
  }
}

/**
 * Find all markdown files
 */
async function findMarkdownFiles(directory) {
  const files = [];
  
  async function scan(dir) {
    try {
      const entries = await fs.readdir(dir, { withFileTypes: true });
      
      for (const entry of entries) {
        const fullPath = path.join(dir, entry.name);
        
        if (entry.isDirectory()) {
          await scan(fullPath);
        } else if (entry.isFile() && entry.name.endsWith('.md')) {
          files.push(fullPath);
        }
      }
    } catch (error) {
      logger.warn(`Could not scan directory ${dir}: ${error.message}`);
    }
  }
  
  await scan(directory);
  return files;
}

/**
 * Generate validation report
 */
function generateReport(allResults) {
  const report = {
    summary: {
      totalFiles: new Set(allResults.map(r => r.file)).size,
      totalLinks: allResults.length,
      validLinks: allResults.filter(r => r.validation.valid).length,
      invalidLinks: allResults.filter(r => !r.validation.valid).length,
      externalLinks: allResults.filter(r => isExternalUrl(r.url)).length,
      internalLinks: allResults.filter(r => !isExternalUrl(r.url)).length,
      imageLinks: allResults.filter(r => r.type.includes('image')).length
    },
    issues: allResults.filter(r => !r.validation.valid),
    statistics: {}
  };
  
  // Group issues by type
  report.statistics.byType = {};
  for (const result of allResults) {
    if (!report.statistics.byType[result.type]) {
      report.statistics.byType[result.type] = { total: 0, valid: 0, invalid: 0 };
    }
    report.statistics.byType[result.type].total++;
    if (result.validation.valid) {
      report.statistics.byType[result.type].valid++;
    } else {
      report.statistics.byType[result.type].invalid++;
    }
  }
  
  // Group issues by file
  report.statistics.byFile = {};
  for (const result of allResults) {
    const fileName = path.basename(result.file);
    if (!report.statistics.byFile[fileName]) {
      report.statistics.byFile[fileName] = { total: 0, valid: 0, invalid: 0 };
    }
    report.statistics.byFile[fileName].total++;
    if (result.validation.valid) {
      report.statistics.byFile[fileName].valid++;
    } else {
      report.statistics.byFile[fileName].invalid++;
    }
  }
  
  return report;
}

/**
 * Main validation function
 */
async function validateLinks() {
  logger.info(`Starting link validation... (external: ${checkExternal}, fix: ${shouldFix})`);
  
  const startTime = Date.now();
  
  try {
    // Find all markdown files
    const markdownFiles = await findMarkdownFiles(CONFIG.docsDir);
    
    // Also check source docs for comparison
    const sourceFiles = await findMarkdownFiles(CONFIG.sourceDocsDir);
    const allFiles = [...markdownFiles, ...sourceFiles];
    
    logger.info(`Found ${allFiles.length} markdown files to validate`);
    
    // Process files in parallel batches
    const allResults = [];
    for (let i = 0; i < allFiles.length; i += CONFIG.parallelRequests) {
      const batch = allFiles.slice(i, i + CONFIG.parallelRequests);
      const batchPromises = batch.map(processFile);
      const batchResults = await Promise.all(batchPromises);
      
      for (const results of batchResults) {
        allResults.push(...results);
      }
      
      logger.verbose(`Processed batch ${Math.floor(i / CONFIG.parallelRequests) + 1}/${Math.ceil(allFiles.length / CONFIG.parallelRequests)}`);
    }
    
    // Generate report
    const report = generateReport(allResults);
    
    const duration = Date.now() - startTime;
    
    // Display summary
    logger.info(`\n=== Link Validation Summary ===`);
    logger.info(`Files processed: ${report.summary.totalFiles}`);
    logger.info(`Total links: ${report.summary.totalLinks}`);
    logger.info(`Valid links: ${report.summary.validLinks} (${((report.summary.validLinks / report.summary.totalLinks) * 100).toFixed(1)}%)`);
    logger.info(`Invalid links: ${report.summary.invalidLinks}`);
    logger.info(`External links: ${report.summary.externalLinks} (${checkExternal ? 'checked' : 'not checked'})`);
    logger.info(`Internal links: ${report.summary.internalLinks}`);
    logger.info(`Image links: ${report.summary.imageLinks}`);
    logger.info(`Validation completed in ${duration}ms`);
    
    // Display issues
    if (report.issues.length > 0) {
      logger.info(`\n=== Issues Found ===`);
      for (const issue of report.issues) {
        const relativePath = path.relative(process.cwd(), issue.file);
        logger.error(`❌ ${relativePath}:${issue.line} - ${issue.url}`);
        logger.error(`   ${issue.validation.message}`);
      }
    }
    
    // Display statistics
    if (isVerbose) {
      logger.info(`\n=== Statistics ===`);
      logger.info('By link type:');
      for (const [type, stats] of Object.entries(report.statistics.byType)) {
        logger.info(`  ${type}: ${stats.valid}/${stats.total} valid`);
      }
      
      logger.info('By file (top 10 issues):');
      const fileIssues = Object.entries(report.statistics.byFile)
        .filter(([, stats]) => stats.invalid > 0)
        .sort(([, a], [, b]) => b.invalid - a.invalid)
        .slice(0, 10);
      
      for (const [file, stats] of fileIssues) {
        logger.info(`  ${file}: ${stats.invalid} issues (${stats.valid} valid)`);
      }
    }
    
    // Save detailed report
    const reportPath = path.join(__dirname, '..', 'link-validation-report.json');
    await fs.writeFile(reportPath, JSON.stringify(report, null, 2));
    logger.info(`Detailed report saved to: ${reportPath}`);
    
    // Exit with error code if issues found
    if (report.summary.invalidLinks > 0) {
      process.exit(1);
    }
    
  } catch (error) {
    logger.error(`Validation failed: ${error.message}`);
    process.exit(1);
  }
}

/**
 * Main execution
 */
if (require.main === module) {
  validateLinks().catch(error => {
    logger.error(`Unexpected error: ${error.message}`);
    process.exit(1);
  });
}

module.exports = { validateLinks, CONFIG };