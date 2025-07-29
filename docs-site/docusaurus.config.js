// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const {themes} = require('prism-react-renderer');
const lightCodeTheme = themes.github;
const darkCodeTheme = themes.dracula;

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'PhotonDrift',
  tagline: 'AI-Powered Architecture Decision Record Management with ML-Enhanced Drift Detection',
  favicon: 'img/favicon.ico',

  // Set the production url of your site here
  url: 'https://docs.photondrift.dev',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: '/',

  // GitHub pages deployment config.
  organizationName: 'tbowman01',
  projectName: 'PhotonDrift',

  onBrokenLinks: 'warn',
  onBrokenMarkdownLinks: 'warn',

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          // Please change this to your repo.
          editUrl:
            'https://github.com/tbowman01/PhotonDrift/tree/main/docs/',
          showLastUpdateAuthor: true,
          showLastUpdateTime: true,
          remarkPlugins: [],
          rehypePlugins: [],
        },
        blog: {
          showReadingTime: true,
          // Please change this to your repo.
          editUrl:
            'https://github.com/tbowman01/PhotonDrift/tree/main/docs-site/blog/',
          blogSidebarCount: 'ALL',
          blogSidebarTitle: 'All our posts',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
        gtag: {
          trackingID: process.env.GOOGLE_ANALYTICS_ID || 'G-PLACEHOLDER',
          anonymizeIP: true,
        },
      }),
    ],
  ],

  plugins: [
    [
      '@docusaurus/plugin-ideal-image',
      {
        quality: 70,
        max: 1030,
        min: 640,
        steps: 2,
        disableInDev: false,
      },
    ],
    [
      '@docusaurus/plugin-pwa',
      {
        debug: false,
        offlineModeActivationStrategies: [
          'appInstalled',
          'standalone',
          'queryString',
        ],
        pwaHead: [
          {
            tagName: 'link',
            rel: 'icon',
            href: '/img/photondrift_logo.png',
          },
          {
            tagName: 'link',
            rel: 'manifest',
            href: '/manifest.json',
          },
          {
            tagName: 'meta',
            name: 'theme-color',
            content: 'rgb(37, 194, 160)',
          },
          {
            tagName: 'meta',
            name: 'apple-mobile-web-app-capable',
            content: 'yes',
          },
          {
            tagName: 'meta',
            name: 'apple-mobile-web-app-status-bar-style',
            content: '#000',
          },
          {
            tagName: 'link',
            rel: 'apple-touch-icon',
            href: '/img/photondrift_logo.png',
          },
          {
            tagName: 'link',
            rel: 'mask-icon',
            href: '/img/photondrift_logo.png',
            color: 'rgb(37, 194, 160)',
          },
          {
            tagName: 'meta',
            name: 'msapplication-TileImage',
            content: '/img/photondrift_logo.png',
          },
          {
            tagName: 'meta',
            name: 'msapplication-TileColor',
            content: '#000',
          },
        ],
      },
    ],
    'plugin-image-zoom',
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      // Replace with your project's social card
      image: 'img/photondrift-social-card.jpg',
      navbar: {
        title: 'PhotonDrift',
        logo: {
          alt: 'PhotonDrift Logo',
          src: 'img/photondrift_logo.png',
          srcDark: 'img/photondrift_logo_dark.png',
        },
        items: [
          {
            type: 'docSidebar',
            sidebarId: 'gettingStartedSidebar',
            position: 'left',
            label: '🚀 Get Started',
          },
          {
            type: 'docSidebar',
            sidebarId: 'apiSidebar',
            position: 'left',
            label: '📖 CLI Reference',
          },
          {
            type: 'docSidebar',
            sidebarId: 'guidesSidebar',
            position: 'left',
            label: '📚 Guides',
          },
          {
            href: 'https://github.com/tbowman01/PhotonDrift',
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'Documentation',
            items: [
              {
                label: 'Quick Start',
                to: '/docs/getting-started/quick-start',
              },
              {
                label: 'CLI Reference',
                to: '/docs/getting-started/cli',
              },
              {
                label: 'Configuration',
                to: '/docs/getting-started/config',
              },
            ],
          },
          {
            title: 'Development',
            items: [
              {
                label: 'Contributing',
                to: '/docs/development/development',
              },
              {
                label: 'Architecture',
                to: '/docs/architecture/architecture-enhancements',
              },
              {
                label: 'ADRs',
                to: '/docs/adr/0001-use-rust-for-cli',
              },
            ],
          },
          {
            title: 'Community',
            items: [
              {
                label: 'GitHub',
                href: 'https://github.com/tbowman01/PhotonDrift',
              },
              {
                label: 'Issues',
                href: 'https://github.com/tbowman01/PhotonDrift/issues',
              },
              {
                label: 'Discussions',
                href: 'https://github.com/tbowman01/PhotonDrift/discussions',
              },
            ],
          },
          {
            title: 'More',
            items: [
              {
                label: 'Blog',
                to: '/blog',
              },
              {
                label: 'Releases',
                href: 'https://github.com/tbowman01/PhotonDrift/releases',
              },
              {
                label: 'Changelog',
                href: 'https://github.com/tbowman01/PhotonDrift/blob/main/CHANGELOG.md',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} PhotonDrift. Built with Docusaurus. <br/>🤖 Powered by AI-Enhanced Architecture Decision Record Management`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
        additionalLanguages: ['rust', 'toml', 'yaml', 'docker', 'bash', 'json'],
      },
      algolia: {
        // The application ID provided by Algolia
        appId: process.env.ALGOLIA_APP_ID || 'ALGOLIA_APP_ID_PLACEHOLDER',
        // Public API key: it is safe to commit it
        apiKey: process.env.ALGOLIA_SEARCH_API_KEY || 'ALGOLIA_SEARCH_API_KEY_PLACEHOLDER',
        indexName: process.env.ALGOLIA_INDEX_NAME || 'photondrift-docs',
        // Optional: see doc section below
        contextualSearch: true,
        // Optional: Algolia search parameters
        searchParameters: {
          facetFilters: ['language:en'],
        },
        // Optional: path for search page that enabled by default (`false` to disable it)
        searchPagePath: 'search',
      },
      colorMode: {
        defaultMode: 'light',
        disableSwitch: false,
        respectPrefersColorScheme: true,
      },
      docs: {
        sidebar: {
          hideable: true,
          autoCollapseCategories: true,
        },
      },
      metadata: [
        {
          name: 'keywords',
          content: 'architecture, decisions, records, ADR, documentation, CLI, rust, AI, ML, drift-detection',
        },
        {
          name: 'description',
          content: 'PhotonDrift - AI-Powered Architecture Decision Record Management with ML-Enhanced Drift Detection. Revolutionary CLI tool for intelligent development governance.',
        },
        {
          property: 'og:type',
          content: 'website',
        },
        {
          property: 'og:description',
          content: 'Revolutionary Architecture Decision Record (ADR) management with AI-powered drift detection and intelligent insights.',
        },
      ],
      announcementBar: {
        id: 'support_us',
        content:
          '⭐️ If you like PhotonDrift, give it a star on <a target="_blank" rel="noopener noreferrer" href="https://github.com/tbowman01/PhotonDrift">GitHub</a>! ⭐️',
        backgroundColor: '#fafbfc',
        textColor: '#091E42',
        isCloseable: true,
      },
    }),

  markdown: {
    mermaid: true,
  },
  themes: ['@docusaurus/theme-mermaid'],
};

module.exports = config;