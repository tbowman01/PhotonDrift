import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/search',
    component: ComponentCreator('/search', '5de'),
    exact: true
  },
  {
    path: '/docs',
    component: ComponentCreator('/docs', '9e0'),
    routes: [
      {
        path: '/docs',
        component: ComponentCreator('/docs', '14e'),
        routes: [
          {
            path: '/docs/tags',
            component: ComponentCreator('/docs/tags', 'fce'),
            exact: true
          },
          {
            path: '/docs/tags/architecture',
            component: ComponentCreator('/docs/tags/architecture', '926'),
            exact: true
          },
          {
            path: '/docs/tags/containerization',
            component: ComponentCreator('/docs/tags/containerization', '02c'),
            exact: true
          },
          {
            path: '/docs/tags/deployment',
            component: ComponentCreator('/docs/tags/deployment', '6c0'),
            exact: true
          },
          {
            path: '/docs/tags/development',
            component: ComponentCreator('/docs/tags/development', 'df8'),
            exact: true
          },
          {
            path: '/docs/tags/docker',
            component: ComponentCreator('/docs/tags/docker', '0d0'),
            exact: true
          },
          {
            path: '/docs/tags/getting-started',
            component: ComponentCreator('/docs/tags/getting-started', 'eda'),
            exact: true
          },
          {
            path: '/docs/tags/infrastructure',
            component: ComponentCreator('/docs/tags/infrastructure', '49b'),
            exact: true
          },
          {
            path: '/docs/tags/language',
            component: ComponentCreator('/docs/tags/language', '21b'),
            exact: true
          },
          {
            path: '/docs/tags/ml-features',
            component: ComponentCreator('/docs/tags/ml-features', 'fc8'),
            exact: true
          },
          {
            path: '/docs/tags/multi-platform',
            component: ComponentCreator('/docs/tags/multi-platform', 'db5'),
            exact: true
          },
          {
            path: '/docs/tags/performance',
            component: ComponentCreator('/docs/tags/performance', '427'),
            exact: true
          },
          {
            path: '/docs/tags/security',
            component: ComponentCreator('/docs/tags/security', '056'),
            exact: true
          },
          {
            path: '/docs/tags/tooling',
            component: ComponentCreator('/docs/tags/tooling', '309'),
            exact: true
          },
          {
            path: '/docs',
            component: ComponentCreator('/docs', '48f'),
            routes: [
              {
                path: '/docs/adr/0001-use-rust-for-cli',
                component: ComponentCreator('/docs/adr/0001-use-rust-for-cli', '94b'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/adr/0003-containerization-deployment-strategy',
                component: ComponentCreator('/docs/adr/0003-containerization-deployment-strategy', '4cb'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/architecture',
                component: ComponentCreator('/docs/architecture', '1b1'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/architecture/architecture-enhancements',
                component: ComponentCreator('/docs/architecture/architecture-enhancements', '48f'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/architecture/requirements-summary',
                component: ComponentCreator('/docs/architecture/requirements-summary', 'd09'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/development',
                component: ComponentCreator('/docs/development', 'dd8'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/development/development',
                component: ComponentCreator('/docs/development/development', '3b0'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/development/development-hooks',
                component: ComponentCreator('/docs/development/development-hooks', '7cc'),
                exact: true,
                sidebar: "apiSidebar"
              },
              {
                path: '/docs/development/github-labels',
                component: ComponentCreator('/docs/development/github-labels', 'f2e'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/development/github-management-summary',
                component: ComponentCreator('/docs/development/github-management-summary', '169'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/development/issue-management',
                component: ComponentCreator('/docs/development/issue-management', 'e1d'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/development/issue-triage-guide',
                component: ComponentCreator('/docs/development/issue-triage-guide', 'f50'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/getting-started',
                component: ComponentCreator('/docs/getting-started', 'd54'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/getting-started/cli',
                component: ComponentCreator('/docs/getting-started/cli', '919'),
                exact: true,
                sidebar: "apiSidebar"
              },
              {
                path: '/docs/getting-started/config',
                component: ComponentCreator('/docs/getting-started/config', '3ee'),
                exact: true,
                sidebar: "apiSidebar"
              },
              {
                path: '/docs/getting-started/quick-start',
                component: ComponentCreator('/docs/getting-started/quick-start', '47b'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/getting-started/user-guide',
                component: ComponentCreator('/docs/getting-started/user-guide', '479'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/ml-features',
                component: ComponentCreator('/docs/ml-features', 'e27'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/ml-features/ml-security-guide',
                component: ComponentCreator('/docs/ml-features/ml-security-guide', '427'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/ml-features/neural-training',
                component: ComponentCreator('/docs/ml-features/neural-training', '881'),
                exact: true,
                sidebar: "apiSidebar"
              },
              {
                path: '/docs/ml-features/performance-analysis',
                component: ComponentCreator('/docs/ml-features/performance-analysis', '57a'),
                exact: true,
                sidebar: "guidesSidebar"
              }
            ]
          }
        ]
      }
    ]
  },
  {
    path: '/',
    component: ComponentCreator('/', 'e5f'),
    exact: true
  },
  {
    path: '*',
    component: ComponentCreator('*'),
  },
];
