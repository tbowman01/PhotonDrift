# PhotonDrift Documentation Site

Modern documentation website for PhotonDrift built with Docusaurus v3, featuring automated content synchronization, AI-enhanced CLI documentation generation, and comprehensive validation.

## 🚀 Quick Start

### Development

```bash
# Install dependencies
npm install

# Sync documentation from source
npm run sync-docs

# Generate CLI documentation
npm run generate-cli-docs

# Start development server
npm start
```

### Production Build

```bash
# Full build with validation
npm run build-and-validate

# Or step by step
npm run prebuild  # Sync, generate, validate
npm run build     # Build site
```

## 📁 Architecture Overview

```
docs-site/
├── docusaurus.config.js      # Main configuration
├── sidebars.js               # Navigation structure
├── package.json              # Dependencies & scripts
├── src/
│   ├── components/           # Custom React components
│   │   ├── CliCommand/       # CLI command display
│   │   └── FeatureGrid/      # Feature showcase
│   ├── css/
│   │   └── custom.css        # PhotonDrift theming
│   └── pages/
│       └── index.tsx         # Landing page
├── static/
│   ├── img/                  # Images and assets
│   └── manifest.json         # PWA manifest
├── docs/                     # Processed documentation
└── build/                    # Generated static site
```

## 🔄 Content Pipeline

### Automated Synchronization

The documentation pipeline automatically processes content from the source `docs/` directory:

1. **Content Sync** (`scripts/docs-sync.js`)
   - Processes markdown files from source
   - Converts internal links to Docusaurus format
   - Adds/updates frontmatter metadata
   - Organizes content by category

2. **CLI Documentation** (`scripts/cli-docs-generator.js`)
   - Extracts help information from Rust CLI
   - Generates interactive command documentation
   - Validates examples against actual CLI
   - Creates searchable command reference

3. **Link Validation** (`scripts/link-validator.js`)
   - Validates internal documentation links
   - Checks external link accessibility
   - Verifies image references
   - Generates validation reports

### Content Categories

Documentation is organized into logical categories:

- **🚀 getting-started/** - User guides, CLI reference, configuration
- **👥 development/** - Development guides, contributing, GitHub management
- **🏗️ architecture/** - System architecture, requirements, ADRs
- **🚀 deployment/** - Docker guides, versioning, container management
- **🤖 ml-features/** - Machine learning capabilities, neural training
- **📋 phase-planning/** - Project phases, roadmaps, technical planning

## ⚙️ Configuration

### Environment Variables

```bash
# Analytics
GOOGLE_ANALYTICS_ID=G-XXXXXXXX

# Search (Algolia)
ALGOLIA_APP_ID=your_app_id
ALGOLIA_SEARCH_API_KEY=your_search_key
ALGOLIA_INDEX_NAME=photondrift-docs

# Deployment
NETLIFY_AUTH_TOKEN=your_token
NETLIFY_SITE_ID=your_site_id
```

### Key Configuration Files

#### `docusaurus.config.js`
- Site metadata and URL configuration
- Theme and plugin configuration
- Navigation and footer setup
- Analytics and search integration
- PWA and performance settings

#### `sidebars.js`
- Documentation navigation structure
- Category organization and labeling
- Generated index pages
- Auto-collapsing sections

#### `package.json`
- Build scripts and automation
- Development dependencies
- Pre-build hooks for content processing

## 🎨 Theming & Design

### PhotonDrift Brand Integration

The site uses PhotonDrift's brand colors and styling:

```css
:root {
  --photon-primary: #6366f1;    /* Indigo */
  --photon-secondary: #a855f7;  /* Purple */
  --photon-accent: #06b6d4;     /* Cyan */
  --photon-gradient: linear-gradient(135deg, #a855f7 0%, #6366f1 50%, #06b6d4 100%);
}
```

### Custom Components

- **CliCommand** - Interactive CLI command display with copy-to-clipboard
- **FeatureGrid** - Responsive feature showcase with status indicators
- **StatusBadge** - Visual status indicators for features and documentation

### Responsive Design

- Mobile-first responsive design
- Optimized for tablets and desktops
- Touch-friendly navigation
- Accessible keyboard navigation

## 🚀 Deployment

### GitHub Actions Pipeline

The site deploys automatically via GitHub Actions:

1. **Build & Validate** - Content sync, CLI docs generation, link validation
2. **GitHub Pages** - Production deployment (main/develop branches)
3. **Netlify Preview** - PR preview deployments
4. **Performance Testing** - Lighthouse CI audits
5. **Search Index** - Algolia index updates

### Deployment Targets

- **Production**: GitHub Pages at `docs.photondrift.dev`
- **Staging**: Staging environment for testing
- **Previews**: Netlify preview deployments for PRs

### Manual Deployment

```bash
# Deploy to GitHub Pages
npm run deploy

# Build for custom hosting
npm run build
# Then serve the build/ directory
```

## 📊 Performance & SEO

### Optimization Features

- **Code Splitting** - Route-based and component-based chunks
- **Image Optimization** - Responsive images with modern formats
- **Bundle Analysis** - Webpack bundle analyzer integration
- **Progressive Web App** - Service worker and offline support
- **Search Optimization** - Full-text search with Algolia

### Performance Targets

- **Lighthouse Score**: 90+ for Performance, Accessibility, SEO
- **Bundle Size**: <500KB JavaScript bundle
- **Page Load**: <3 seconds initial load
- **Core Web Vitals**: All metrics in green

## 🔍 Search Integration

### Algolia DocSearch

The site integrates with Algolia DocSearch for powerful search capabilities:

- **Full-text search** across all documentation
- **Faceted search** by category and version
- **Auto-complete** with query suggestions
- **Analytics** for search performance tracking

### Search Configuration

```javascript
algolia: {
  appId: 'PHOTONDRIFT_APP_ID',
  apiKey: 'public-search-key',
  indexName: 'photondrift-docs',
  contextualSearch: true,
  searchParameters: {
    facetFilters: ['language:en'],
  }
}
```

## 📱 Progressive Web App

### PWA Features

- **Offline Support** - Service worker caching
- **App Installation** - Installable via browser
- **Push Notifications** - Future feature for updates
- **Native Feel** - App-like experience on mobile

### Manifest Configuration

The site includes a comprehensive PWA manifest with:
- App metadata and branding
- Icon configurations for multiple sizes
- Shortcuts to key documentation sections
- Screenshots for app store listings

## 🧪 Testing & Validation

### Automated Testing

```bash
# Run all validations
npm run validate-links

# Test build process
npm run build-and-validate

# Performance testing
npm run lighthouse
```

### Quality Assurance

- **Link Validation** - Internal and external link checking
- **Content Validation** - Markdown syntax and structure
- **Accessibility Testing** - WCAG 2.1 AA compliance
- **Performance Monitoring** - Core Web Vitals tracking
- **Cross-browser Testing** - Modern browser compatibility

## 🛠️ Development

### Adding New Content

1. **Add markdown files** to appropriate category in `docs/`
2. **Run sync script** to process content: `npm run sync-docs`
3. **Update navigation** if needed in `sidebars.js`
4. **Test locally** with `npm start`

### Custom Components

To add new React components:

1. Create component in `src/components/ComponentName/`
2. Include `index.tsx` and `styles.module.css`
3. Export from component directory
4. Use in markdown with MDX syntax

### Styling Guidelines

- Use CSS custom properties for theming
- Follow Docusaurus conventions
- Ensure mobile responsiveness
- Include dark mode support
- Add accessibility features

## 📚 Available Scripts

### Content Management

- `npm run sync-docs` - Sync documentation from source
- `npm run generate-cli-docs` - Generate CLI documentation
- `npm run validate-links` - Validate all links

### Development

- `npm start` - Start development server
- `npm run dev` - Sync docs and start dev server
- `npm run build` - Build for production
- `npm run serve` - Serve built site locally

### Quality Assurance

- `npm run typecheck` - TypeScript type checking
- `npm run build-and-validate` - Full build with validation
- `npm run clear` - Clear Docusaurus cache

## 🤝 Contributing

### Development Setup

1. **Clone repository**
2. **Install dependencies**: `npm install`
3. **Sync documentation**: `npm run sync-docs`
4. **Start development**: `npm start`

### Content Guidelines

- Follow existing markdown formatting
- Include appropriate frontmatter metadata
- Use descriptive headings and structure
- Add code examples where helpful
- Include links to related documentation

### Component Development

- Follow React and TypeScript best practices
- Include comprehensive styling
- Ensure accessibility compliance
- Add proper documentation and examples
- Test across different screen sizes

## 📞 Support

- **Documentation Issues**: Create issue with `documentation` label
- **Feature Requests**: Use GitHub Discussions
- **Bug Reports**: Include reproduction steps and environment details

---

*This documentation site is built with ❤️ using Docusaurus v3 and features automated content synchronization, AI-enhanced CLI documentation, and comprehensive validation for the best developer experience.*