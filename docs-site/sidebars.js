/**
 * Creating a sidebar enables you to:
 - create an ordered group of docs
 - render a sidebar for each doc of that group
 - provide next/previous navigation

 The sidebars can be generated from the filesystem, or explicitly defined here.

 Create as many sidebars as you want.
 */

// @ts-check

/** @type {import('@docusaurus/plugin-content-docs').SidebarsConfig} */
const sidebars = {
  // Getting Started sidebar
  gettingStartedSidebar: [
    {
      type: 'category',
      label: '🚀 Getting Started',
      collapsed: false,
      items: [
        'getting-started/quick-start',
        'getting-started/user-guide',
        'getting-started/cli',
        'getting-started/config',
      ],
    },
  ],

  // Main documentation sidebar
  guidesSidebar: [
    {
      type: 'category',
      label: '🚀 Getting Started',
      collapsed: false,
      link: {
        type: 'generated-index',
        title: 'Getting Started with PhotonDrift',
        description: 'Learn how to get started with PhotonDrift - the AI-powered Architecture Decision Record management tool.',
        slug: '/getting-started',
      },
      items: [
        'getting-started/quick-start',
        'getting-started/user-guide',
      ],
    },
    {
      type: 'category',
      label: '👥 Development',
      collapsed: true,
      link: {
        type: 'generated-index',
        title: 'Development & Contributing',
        description: 'Guides for developers contributing to PhotonDrift.',
        slug: '/development',
      },
      items: [
        'development/development',
        'development/development-hooks',
        'development/issue-management',
        'development/issue-triage-guide',
        'development/github-labels',
        'development/github-management-summary',
      ],
    },
    {
      type: 'category',
      label: '🏗️ Architecture',
      collapsed: true,
      link: {
        type: 'generated-index',
        title: 'Architecture & Design',
        description: 'Technical architecture documentation and design decisions.',
        slug: '/architecture',
      },
      items: [
        'architecture/architecture-enhancements',
        'architecture/requirements-summary',
        {
          type: 'category',
          label: 'Architecture Decision Records',
          items: [
            'adr/0001-use-rust-for-cli',
            'adr/0003-containerization-deployment-strategy',
          ],
        },
      ],
    },
    {
      type: 'category',
      label: '🤖 ML Features',
      collapsed: true,
      link: {
        type: 'generated-index',
        title: 'Machine Learning & AI',
        description: 'Advanced AI and machine learning capabilities in PhotonDrift.',
        slug: '/ml-features',
      },
      items: [
        'ml-features/ml-security-guide',
        'ml-features/neural-training',
        'ml-features/performance-analysis',
      ],
    },
  ],

  // API Reference sidebar
  apiSidebar: [
    {
      type: 'category',
      label: '📖 CLI Reference',
      collapsed: false,
      items: [
        'getting-started/cli',
        'getting-started/config',
      ],
    },
    {
      type: 'category',
      label: '⚙️ Configuration',
      collapsed: false,
      items: [
        'getting-started/config',
      ],
    },
    {
      type: 'category',
      label: '🔧 Advanced Usage',
      collapsed: true,
      items: [
        'development/development-hooks',
        'ml-features/neural-training',
      ],
    },
  ],
};

module.exports = sidebars;