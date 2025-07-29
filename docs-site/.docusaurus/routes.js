import React from 'react';
import ComponentCreator from '@docusaurus/ComponentCreator';

export default [
  {
    path: '/__docusaurus/debug',
    component: ComponentCreator('/__docusaurus/debug', '5ff'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/config',
    component: ComponentCreator('/__docusaurus/debug/config', '5ba'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/content',
    component: ComponentCreator('/__docusaurus/debug/content', 'a2b'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/globalData',
    component: ComponentCreator('/__docusaurus/debug/globalData', 'c3c'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/metadata',
    component: ComponentCreator('/__docusaurus/debug/metadata', '156'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/registry',
    component: ComponentCreator('/__docusaurus/debug/registry', '88c'),
    exact: true
  },
  {
    path: '/__docusaurus/debug/routes',
    component: ComponentCreator('/__docusaurus/debug/routes', '000'),
    exact: true
  },
  {
    path: '/search',
    component: ComponentCreator('/search', '5de'),
    exact: true
  },
  {
    path: '/docs',
    component: ComponentCreator('/docs', '4c8'),
    routes: [
      {
        path: '/docs',
        component: ComponentCreator('/docs', '0ae'),
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
            path: '/docs/tags/cli',
            component: ComponentCreator('/docs/tags/cli', '537'),
            exact: true
          },
          {
            path: '/docs/tags/commands',
            component: ComponentCreator('/docs/tags/commands', 'ea6'),
            exact: true
          },
          {
            path: '/docs/tags/containerization',
            component: ComponentCreator('/docs/tags/containerization', '02c'),
            exact: true
          },
          {
            path: '/docs/tags/contributing',
            component: ComponentCreator('/docs/tags/contributing', '36c'),
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
            path: '/docs/tags/documentation',
            component: ComponentCreator('/docs/tags/documentation', 'a92'),
            exact: true
          },
          {
            path: '/docs/tags/getting-started',
            component: ComponentCreator('/docs/tags/getting-started', 'eda'),
            exact: true
          },
          {
            path: '/docs/tags/guidelines',
            component: ComponentCreator('/docs/tags/guidelines', '219'),
            exact: true
          },
          {
            path: '/docs/tags/ide',
            component: ComponentCreator('/docs/tags/ide', 'd31'),
            exact: true
          },
          {
            path: '/docs/tags/infrastructure',
            component: ComponentCreator('/docs/tags/infrastructure', '49b'),
            exact: true
          },
          {
            path: '/docs/tags/integration',
            component: ComponentCreator('/docs/tags/integration', '42d'),
            exact: true
          },
          {
            path: '/docs/tags/journey',
            component: ComponentCreator('/docs/tags/journey', 'b47'),
            exact: true
          },
          {
            path: '/docs/tags/language',
            component: ComponentCreator('/docs/tags/language', '21b'),
            exact: true
          },
          {
            path: '/docs/tags/lsp',
            component: ComponentCreator('/docs/tags/lsp', 'e43'),
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
            path: '/docs/tags/phase-planning',
            component: ComponentCreator('/docs/tags/phase-planning', 'e6d'),
            exact: true
          },
          {
            path: '/docs/tags/project',
            component: ComponentCreator('/docs/tags/project', 'f7b'),
            exact: true
          },
          {
            path: '/docs/tags/reference',
            component: ComponentCreator('/docs/tags/reference', '00b'),
            exact: true
          },
          {
            path: '/docs/tags/roadmap',
            component: ComponentCreator('/docs/tags/roadmap', 'f70'),
            exact: true
          },
          {
            path: '/docs/tags/security',
            component: ComponentCreator('/docs/tags/security', '056'),
            exact: true
          },
          {
            path: '/docs/tags/style',
            component: ComponentCreator('/docs/tags/style', '307'),
            exact: true
          },
          {
            path: '/docs/tags/tooling',
            component: ComponentCreator('/docs/tags/tooling', '309'),
            exact: true
          },
          {
            path: '/docs',
            component: ComponentCreator('/docs', '890'),
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
                path: '/docs/api/cli',
                component: ComponentCreator('/docs/api/cli', 'af9'),
                exact: true
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
                path: '/docs/deployment/container-versioning-diagram',
                component: ComponentCreator('/docs/deployment/container-versioning-diagram', 'b3f'),
                exact: true
              },
              {
                path: '/docs/deployment/container-versioning-implementation',
                component: ComponentCreator('/docs/deployment/container-versioning-implementation', '26f'),
                exact: true
              },
              {
                path: '/docs/deployment/container-versioning-quickref',
                component: ComponentCreator('/docs/deployment/container-versioning-quickref', 'd6a'),
                exact: true
              },
              {
                path: '/docs/deployment/container-versioning-strategy',
                component: ComponentCreator('/docs/deployment/container-versioning-strategy', '6e5'),
                exact: true
              },
              {
                path: '/docs/deployment/docker-build-guide',
                component: ComponentCreator('/docs/deployment/docker-build-guide', '569'),
                exact: true
              },
              {
                path: '/docs/deployment/versioning-strategy',
                component: ComponentCreator('/docs/deployment/versioning-strategy', 'ae0'),
                exact: true
              },
              {
                path: '/docs/development',
                component: ComponentCreator('/docs/development', 'dd8'),
                exact: true,
                sidebar: "guidesSidebar"
              },
              {
                path: '/docs/development/development',
                component: ComponentCreator('/docs/development/development', 'e3c'),
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
                path: '/docs/development/documentation-style-guide',
                component: ComponentCreator('/docs/development/documentation-style-guide', 'c32'),
                exact: true
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
                path: '/docs/misc/journey',
                component: ComponentCreator('/docs/misc/journey', '268'),
                exact: true
              },
              {
                path: '/docs/misc/lsp-integration',
                component: ComponentCreator('/docs/misc/lsp-integration', '8cd'),
                exact: true
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
              },
              {
                path: '/docs/phase-planning/automation-comprehensive-guide',
                component: ComponentCreator('/docs/phase-planning/automation-comprehensive-guide', 'd33'),
                exact: true
              },
              {
                path: '/docs/phase-planning/build-simplification-summary',
                component: ComponentCreator('/docs/phase-planning/build-simplification-summary', 'b3c'),
                exact: true
              },
              {
                path: '/docs/phase-planning/dependency-analysis-report',
                component: ComponentCreator('/docs/phase-planning/dependency-analysis-report', '2b8'),
                exact: true
              },
              {
                path: '/docs/phase-planning/phase-2.5-dependency-automation',
                component: ComponentCreator('/docs/phase-planning/phase-2.5-dependency-automation', '03d'),
                exact: true
              },
              {
                path: '/docs/phase-planning/phase-2.5-dependency-update-final',
                component: ComponentCreator('/docs/phase-planning/phase-2.5-dependency-update-final', '8c6'),
                exact: true
              },
              {
                path: '/docs/phase-planning/phase-2.5-implementation-summary',
                component: ComponentCreator('/docs/phase-planning/phase-2.5-implementation-summary', '42b'),
                exact: true
              },
              {
                path: '/docs/phase-planning/phase-3-roadmap',
                component: ComponentCreator('/docs/phase-planning/phase-3-roadmap', '87d'),
                exact: true
              },
              {
                path: '/docs/phase-planning/phase-3-strategic-plan',
                component: ComponentCreator('/docs/phase-planning/phase-3-strategic-plan', 'ef2'),
                exact: true
              },
              {
                path: '/docs/phase-planning/repository-analysis-and-roadmap',
                component: ComponentCreator('/docs/phase-planning/repository-analysis-and-roadmap', 'f1f'),
                exact: true
              },
              {
                path: '/docs/phase-planning/temporal-analysis',
                component: ComponentCreator('/docs/phase-planning/temporal-analysis', 'e63'),
                exact: true
              },
              {
                path: '/docs/phase-planning/wasm-file-io',
                component: ComponentCreator('/docs/phase-planning/wasm-file-io', 'bd1'),
                exact: true
              },
              {
                path: '/docs/phase-planning/wasmtime-v35-update-checklist',
                component: ComponentCreator('/docs/phase-planning/wasmtime-v35-update-checklist', '5e6'),
                exact: true
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
