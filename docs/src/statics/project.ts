import docsConfig from './config';

const config = {
  guides: {
    sidebar: [
      {
        title: 'Getting Started',
        pages: [
          {
            path: '/',
            title: 'Overview',
          },
          { path: '/cli/quickstart', title: 'CLI Quickstart' },
          { path: '/cli/reference', title: 'CLI Reference' },
          { path: '/patterns', title: 'Pattern Library' },
          { path: '/guides/config', title: 'Config' },
          { path: '/security', title: 'Data Security' },
          {
            path: '/workflows/healing',
            title: 'Auto Healing',
          },
          {
            path: '/guides/agent',
            title: 'Grit Agent',
          },
        ],
      },
      {
        pages: [
          {
            path: '/language/overview',
            title: 'Overview',
          },
          {
            path: '/tutorials/gritql',
            title: 'Tutorial',
          },
          {
            path: '/language/patterns',
            title: 'Patterns',
          },
          {
            path: '/language/conditions',
            title: 'Conditions',
          },
          {
            path: '/language/modifiers',
            title: 'Pattern Modifiers',
          },
          {
            path: '/language/target-languages',
            title: 'Target Languages',
          },
          { path: '/language/bubble', title: 'Bubble and Scoping' },
          { path: '/guides/patterns', title: 'Defining Patterns' },
          { path: '/language/functions', title: 'Functions' },
          { path: '/language/idioms', title: 'Common Idioms' },
          { path: '/language/syntax', title: 'Syntax Reference' },
          { path: '/guides/testing', title: 'Testing GritQL' },
        ],
        title: 'Language',
      },
      {
        pages: [
          {
            path: '/guides/autoreview',
            title: 'Autoreview',
          },
          {
            path: '/workflows/sequence',
            title: 'Autopilot Sequences',
          },
          {
            path: '/workflows/drift-detection',
            title: 'Drift Detection',
          },
        ],
        title: 'Workflows',
      },
      {
        pages: [
          { path: '/guides/feedback', title: 'Pull Requests' },
          { path: '/guides/ci', title: 'Continuous Integration' },
          { path: '/guides/authoring', title: 'Authoring GritQL' },
          { path: '/guides/imports', title: 'Imports' },
          { path: '/guides/gitlab', title: 'GitLab' },
          { path: '/guides/vscode', title: 'VS Code' },
          { path: '/guides/sharing', title: 'Sharing Patterns' },
          { path: '/guides/secrets', title: 'Configuring Secrets' },
          { path: '/guides/settings', title: 'Migration Settings' },
          { path: '/architecture', title: 'System Architecture' },
        ],
        title: 'Guides',
      },
    ],
  },
  navbar: {
    tabs: [],
    topLinks: [
      { title: 'GitHub', href: 'https://github.com/getgrit/gritql' },
      { title: 'Discord', href: 'https://docs.grit.io/discord' },
      { title: 'Blog', href: '/blog' },
      { title: 'Tutorial', href: '/tutorials/gritql' },
      { title: 'Studio', href: `${docsConfig.WEB_URL}/studio` },
      { title: 'Security', href: '/security' },
    ],
  },
};

export default config;
