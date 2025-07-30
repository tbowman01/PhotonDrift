#!/usr/bin/env node

/**
 * Documentation Synchronization Script for PhotonDrift
 * 
 * This script syncs markdown files from docs/ to docs-site/docs/
 * with intelligent processing:
 * - Converts internal links to Docusaurus format
 * - Adds/updates frontmatter metadata
 * - Validates content and structure
 * - Handles category-based organization
 * 
 * Usage: node scripts/docs-sync.js [--dry-run] [--verbose]
 */

const fs = require('fs').promises;
const path = require('path');
const { execSync } = require('child_process');

// Configuration
const CONFIG = {
  sourceDir: path.join(__dirname, '..', 'docs'),
  targetDir: path.join(__dirname, '..', 'docs-site', 'docs'),
  categories: {
    'getting-started': { 
      position: 1, 
      label: '🚀 Getting Started',
      description: 'Get up and running with PhotonDrift quickly'
    },
    'development': { 
      position: 2, 
      label: '👥 Development',
      description: 'Development guides and contributing guidelines'
    },
    'architecture': { 
      position: 3, 
      label: '🏗️ Architecture',
      description: 'System architecture and design decisions'
    },
    'deployment': { 
      position: 4, 
      label: '🚀 Deployment',
      description: 'Deploy and operate PhotonDrift in production'
    },
    'ml-features': { 
      position: 5, 
      label: '🤖 ML Features',
      description: 'Machine learning and AI capabilities'
    },
    'phase-planning': { 
      position: 6, 
      label: '📋 Project Planning',
      description: 'Development phases and strategic planning'
    },
    'adr': { 
      position: 7, 
      label: '📄 Architecture Decision Records',
      description: 'Architecture Decision Records (ADRs)'
    }
  },
  excludedFiles: [
    'README.md',
    'CONFIGURATION.md', // Legacy file, replaced by getting-started/config.md
  ],
  linkMappings: {
    // Map old paths to new Docusaurus paths
    'docs/USER_GUIDE.md': '/docs/getting-started/user-guide',
    'docs/QUICK_START.md': '/docs/getting-started/quick-start',
    'docs/CLI.md': '/docs/getting-started/cli',
    'docs/CONFIG.md': '/docs/getting-started/config',
    'docs/DEVELOPMENT.md': '/docs/development/development',
    'docs/DOCKER_BUILD_GUIDE.md': '/docs/deployment/docker-build-guide',
    'CHANGELOG.md': 'https://github.com/tbowman01/PhotonDrift/blob/main/CHANGELOG.md',
    'ROADMAP.md': 'https://github.com/tbowman01/PhotonDrift/blob/main/ROADMAP.md',
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
  info: (msg) => console.log(`[INFO] ${msg}`),
  warn: (msg) => console.warn(`[WARN] ${msg}`),
  error: (msg) => console.error(`[ERROR] ${msg}`),
  verbose: (msg) => isVerbose && console.log(`[VERBOSE] ${msg}`),
  success: (msg) => console.log(`[SUCCESS] ${msg}`)
};

/**
 * Parse frontmatter from markdown content
 */
function parseFrontmatter(content) {
  const frontmatterRegex = /^---\n([\s\S]*?)\n---\n([\s\S]*)$/;
  const match = content.match(frontmatterRegex);
  
  if (!match) {
    return { frontmatter: {}, content };
  }
  
  const frontmatterLines = match[1].split('\n');
  const frontmatter = {};
  
  for (const line of frontmatterLines) {
    const [key, ...valueParts] = line.split(':');
    if (key && valueParts.length > 0) {
      let value = valueParts.join(':').trim();
      // Handle arrays first
      if (value.startsWith('[') && value.endsWith(']')) {
        try {
          // Parse as JSON array
          frontmatter[key.trim()] = JSON.parse(value);
        } catch (e) {
          // If JSON parsing fails, keep as string without quotes
          if ((value.startsWith('"') && value.endsWith('"')) ||
              (value.startsWith("'") && value.endsWith("'"))) {
            value = value.slice(1, -1);
          }
          frontmatter[key.trim()] = value;
        }
      } else {
        // Remove quotes if present
        if ((value.startsWith('"') && value.endsWith('"')) || 
            (value.startsWith("'") && value.endsWith("'"))) {
          value = value.slice(1, -1);
        }
        frontmatter[key.trim()] = value;
      }
    }
  }
  
  return { frontmatter, content: match[2] };
}

/**
 * Generate frontmatter for a document
 */
function generateFrontmatter(filePath, category, existingFrontmatter = {}) {
  const fileName = path.basename(filePath, '.md');
  const categoryConfig = CONFIG.categories[category];
  
  // Generate slug from file path
  const slug = category === 'adr' 
    ? `/${category}/${fileName}`
    : `/${category}/${fileName.toLowerCase().replace(/_/g, '-')}`;
  
  // Determine position within category
  let position = existingFrontmatter.sidebar_position || 1;
  
  // Special positioning rules
  if (category === 'getting-started') {
    const positionMap = {
      'quick-start': 1,
      'user-guide': 2,
      'cli': 3,
      'config': 4
    };
    position = positionMap[fileName.toLowerCase().replace(/_/g, '-')] || position;
  }
  
  const frontmatter = {
    id: fileName.toLowerCase().replace(/_/g, '-'),
    title: existingFrontmatter.title || generateTitle(fileName),
    sidebar_label: existingFrontmatter.sidebar_label || generateSidebarLabel(fileName),
    sidebar_position: position,
    description: existingFrontmatter.description || categoryConfig.description,
    slug,
    tags: existingFrontmatter.tags || [category],
    ...existingFrontmatter
  };
  
  // Add last_update information
  try {
    const gitLog = execSync(`git log -1 --format="%ci|%an" -- "${filePath}"`, { encoding: 'utf-8' }).trim();
    if (gitLog) {
      const [date, author] = gitLog.split('|');
      frontmatter.last_update = {
        date: new Date(date).toISOString().split('T')[0],
        author: author.trim()
      };
    }
  } catch (error) {
    logger.verbose(`Could not get git info for ${filePath}: ${error.message}`);
  }
  
  return frontmatter;
}

/**
 * Generate title from filename
 */
function generateTitle(fileName) {
  return fileName
    .replace(/[-_]/g, ' ')
    .replace(/\b\w/g, char => char.toUpperCase())
    .replace(/adr/gi, 'ADR')
    .replace(/cli/gi, 'CLI')
    .replace(/api/gi, 'API')
    .replace(/ml/gi, 'ML')
    .replace(/ai/gi, 'AI');
}

/**
 * Generate sidebar label from filename
 */
function generateSidebarLabel(fileName) {
  const title = generateTitle(fileName);
  
  // Add emojis for common patterns
  const emojiMap = {
    'Quick Start': '⚡ Quick Start',
    'User Guide': '📖 User Guide',
    'CLI': '💻 CLI Reference',
    'Config': '⚙️ Configuration',
    'Development': '👥 Development',
    'Docker': '🐳 Docker Guide',
    'ML': '🤖 ML Features',
    'Security': '🔒 Security',
    'Performance': '📊 Performance',
    'Neural': '🧠 Neural Training'
  };
  
  return emojiMap[title] || title;
}

/**
 * Convert internal links to Docusaurus format
 */
function convertInternalLinks(content, sourceFilePath, category) {
  // Convert relative documentation links
  content = content.replace(/\[([^\]]+)\]\(([^)]+\.md)\)/g, (match, text, link) => {
    // Skip external links
    if (link.startsWith('http')) {
      return match;
    }
    
    // Handle relative paths
    let resolvedPath = link;
    if (link.startsWith('./') || link.startsWith('../')) {
      resolvedPath = path.resolve(path.dirname(sourceFilePath), link);
      resolvedPath = path.relative(CONFIG.sourceDir, resolvedPath);
    }
    
    // Apply link mappings
    const fullPath = `docs/${resolvedPath}`;
    if (CONFIG.linkMappings[fullPath]) {
      return `[${text}](${CONFIG.linkMappings[fullPath]})`;
    }
    
    // Convert to Docusaurus format
    const docPath = resolvedPath
      .replace(/\.md$/, '')
      .replace(/\\/g, '/')
      .toLowerCase()
      .replace(/_/g, '-');
    
    return `[${text}](/${docPath})`;
  });
  
  // Convert image links to use static folder
  content = content.replace(/!\[([^\]]*)\]\(([^)]+)\)/g, (match, alt, src) => {
    if (src.startsWith('http')) {
      return match;
    }
    
    // Convert relative image paths to static folder
    if (src.startsWith('../assets/') || src.startsWith('./assets/') || src.startsWith('assets/')) {
      const imgPath = src.replace(/^\.\.?\//, '').replace(/^assets\//, 'img/');
      return `![${alt}](/img/${imgPath})`;
    }
    
    return match;
  });
  
  return content;
}

/**
 * Process a single markdown file
 */
async function processMarkdownFile(sourceFile, targetFile, category) {
  logger.verbose(`Processing ${sourceFile} -> ${targetFile}`);
  
  try {
    const content = await fs.readFile(sourceFile, 'utf-8');
    const { frontmatter: existingFrontmatter, content: markdownContent } = parseFrontmatter(content);
    
    // Generate new frontmatter
    const frontmatter = generateFrontmatter(sourceFile, category, existingFrontmatter);
    
    // Convert internal links
    const processedContent = convertInternalLinks(markdownContent, sourceFile, category);
    
    // Generate final content
    const frontmatterYaml = Object.entries(frontmatter)
      .map(([key, value]) => {
        if (typeof value === 'object' && value !== null) {
          if (Array.isArray(value)) {
            return `${key}: [${value.map(v => `"${v}"`).join(', ')}]`;
          } else {
            return `${key}:\n${Object.entries(value).map(([k, v]) => `  ${k}: "${v}"`).join('\n')}`;
          }
        }
        return `${key}: "${value}"`;
      })
      .join('\n');
    
    const finalContent = `---\n${frontmatterYaml}\n---\n\n${processedContent}`;
    
    if (!isDryRun) {
      await fs.mkdir(path.dirname(targetFile), { recursive: true });
      await fs.writeFile(targetFile, finalContent, 'utf-8');
    }
    
    logger.verbose(`✅ Processed ${path.basename(sourceFile)}`);
    return true;
  } catch (error) {
    logger.error(`Failed to process ${sourceFile}: ${error.message}`);
    return false;
  }
}

/**
 * Sync files from a category directory
 */
async function syncCategoryDirectory(categoryPath, category) {
  const files = await fs.readdir(categoryPath);
  const results = [];
  
  for (const file of files) {
    if (!file.endsWith('.md') || CONFIG.excludedFiles.includes(file)) {
      continue;
    }
    
    const sourceFile = path.join(categoryPath, file);
    const targetFile = path.join(CONFIG.targetDir, category, file.toLowerCase().replace(/_/g, '-'));
    
    const result = await processMarkdownFile(sourceFile, targetFile, category);
    results.push({ file, success: result });
  }
  
  return results;
}

/**
 * Sync root level files
 */
async function syncRootFiles() {
  const files = await fs.readdir(CONFIG.sourceDir);
  const results = [];
  
  for (const file of files) {
    if (!file.endsWith('.md') || CONFIG.excludedFiles.includes(file)) {
      continue;
    }
    
    const sourceFile = path.join(CONFIG.sourceDir, file);
    const stats = await fs.stat(sourceFile);
    
    if (stats.isFile()) {
      // Determine target based on file name
      let targetCategory = 'misc';
      let targetFile = path.join(CONFIG.targetDir, file.toLowerCase().replace(/_/g, '-'));
      
      // Special handling for specific files
      if (file === 'CONFIGURATION.md') {
        continue; // Skip legacy configuration file
      }
      
      const result = await processMarkdownFile(sourceFile, targetFile, targetCategory);
      results.push({ file, success: result });
    }
  }
  
  return results;
}

/**
 * Clean target directory
 */
async function cleanTargetDirectory() {
  logger.info('Cleaning target directory...');
  
  try {
    await fs.rm(CONFIG.targetDir, { recursive: true, force: true });
    logger.verbose('Target directory cleaned');
  } catch (error) {
    logger.warn(`Could not clean target directory: ${error.message}`);
  }
}

/**
 * Copy static assets
 */
async function copyStaticAssets() {
  const assetsDir = path.join(CONFIG.sourceDir, '..', 'assets');
  const targetAssetsDir = path.join(CONFIG.targetDir, '..', 'static', 'img');
  
  try {
    const files = await fs.readdir(assetsDir);
    await fs.mkdir(targetAssetsDir, { recursive: true });
    
    for (const file of files) {
      const sourceFile = path.join(assetsDir, file);
      const targetFile = path.join(targetAssetsDir, file);
      
      if (!isDryRun) {
        await fs.copyFile(sourceFile, targetFile);
      }
      logger.verbose(`Copied asset: ${file}`);
    }
    
    logger.info(`Copied ${files.length} assets`);
  } catch (error) {
    logger.warn(`Could not copy assets: ${error.message}`);
  }
}

/**
 * Main sync function
 */
async function syncDocs() {
  logger.info(`${isDryRun ? '[DRY RUN] ' : ''}Starting documentation sync...`);
  
  const startTime = Date.now();
  let totalFiles = 0;
  let successfulFiles = 0;
  
  try {
    // Clean target directory
    if (!isDryRun) {
      await cleanTargetDirectory();
    }
    
    // Copy static assets
    await copyStaticAssets();
    
    // Sync each category
    for (const [category, config] of Object.entries(CONFIG.categories)) {
      const categoryPath = path.join(CONFIG.sourceDir, category);
      
      try {
        await fs.access(categoryPath);
        logger.info(`Syncing category: ${category}`);
        
        const results = await syncCategoryDirectory(categoryPath, category);
        const categorySuccess = results.filter(r => r.success).length;
        
        totalFiles += results.length;
        successfulFiles += categorySuccess;
        
        logger.info(`Category ${category}: ${categorySuccess}/${results.length} files synced`);
      } catch (error) {
        logger.warn(`Category directory not found: ${categoryPath}`);
      }
    }
    
    // Sync root level files
    logger.info('Syncing root level files...');
    const rootResults = await syncRootFiles();
    const rootSuccess = rootResults.filter(r => r.success).length;
    
    totalFiles += rootResults.length;
    successfulFiles += rootSuccess;
    
    const duration = Date.now() - startTime;
    
    logger.success(`Documentation sync completed in ${duration}ms`);
    logger.success(`Successfully processed: ${successfulFiles}/${totalFiles} files`);
    
    if (successfulFiles < totalFiles) {
      process.exit(1);
    }
    
  } catch (error) {
    logger.error(`Sync failed: ${error.message}`);
    process.exit(1);
  }
}

/**
 * Main execution
 */
if (require.main === module) {
  syncDocs().catch(error => {
    logger.error(`Unexpected error: ${error.message}`);
    process.exit(1);
  });
}

module.exports = { syncDocs, CONFIG };