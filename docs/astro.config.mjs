// @ts-check
import { satteri } from '@astrojs/markdown-satteri';
import react from '@astrojs/react';
import starlight from '@astrojs/starlight';
import { defineConfig } from 'astro/config';
import starlightLinksValidator from 'starlight-links-validator';

import gritGrammar from './src/grammars/gritql.tmLanguage.json' with { type: 'json' };
import { codeGroups, customHeadingIds } from './src/markdown/plugins.ts';

export default defineConfig({
  site: 'https://docs.grit.io',
  markdown: {
    processor: satteri({ mdastPlugins: [codeGroups, customHeadingIds] }),
  },
  integrations: [
    starlight({
      title: 'Grit',
      description: 'GritQL is a declarative query language for searching and modifying source code.',
      logo: {
        light: './src/assets/grit-logo.svg',
        dark: './src/assets/grit-logo-dark.svg',
        replacesTitle: true,
      },
      favicon: '/favicon.svg',
      social: [
        { icon: 'github', label: 'GitHub', href: 'https://github.com/getgrit/gritql' },
        { icon: 'discord', label: 'Discord', href: '/discord/' },
      ],
      editLink: {
        baseUrl: 'https://github.com/getgrit/gritql/edit/main/docs/',
      },
      customCss: ['./src/styles/custom.css'],
      expressiveCode: {
        shiki: {
          langs: [gritGrammar],
        },
      },
      components: {
        SocialIcons: './src/components/overrides/SocialIcons.astro',
      },
      sidebar: [
        {
          label: 'Getting Started',
          items: [
            { label: 'Overview', slug: 'index' },
            { label: 'CLI Quickstart', slug: 'cli/quickstart' },
            { label: 'CLI Reference', slug: 'cli/reference' },
            { label: 'Pattern Library', slug: 'patterns' },
            { label: 'Config', slug: 'guides/config' },
          ],
        },
        {
          label: 'Language',
          items: [
            { label: 'Overview', slug: 'language/overview' },
            { label: 'Tutorial', slug: 'tutorials/gritql' },
            { label: 'Patterns', slug: 'language/patterns' },
            { label: 'Conditions', slug: 'language/conditions' },
            { label: 'Pattern Modifiers', slug: 'language/modifiers' },
            { label: 'Target Languages', slug: 'language/target-languages' },
            { label: 'Bubble and Scoping', slug: 'language/bubble' },
            { label: 'Defining Patterns', slug: 'guides/patterns' },
            { label: 'Functions', slug: 'language/functions' },
            { label: 'Common Idioms', slug: 'language/idioms' },
            { label: 'Syntax Reference', slug: 'language/syntax' },
            { label: 'Testing GritQL', slug: 'guides/testing' },
          ],
        },
        {
          label: 'Guides',
          items: [
            { label: 'Continuous Integration', slug: 'guides/ci' },
            { label: 'Authoring GritQL', slug: 'guides/authoring' },
            { label: 'Imports', slug: 'guides/imports' },
            { label: 'Sharing Patterns', slug: 'guides/sharing' },
          ],
        },
      ],
      plugins: [
        starlightLinksValidator({
          exclude: [
            // Pages rendered from `src/pages/` rather than the docs collection.
            '/playground/',
            '/language/request/',
            // Pre-existing links to pages that were not carried over when the docs were open-sourced.
            '/guides/vscode/',
            '/workflows/**',
          ],
        }),
      ],
    }),
    react(),
  ],
  vite: {
    optimizeDeps: {
      // These packages locate their `.wasm` files relative to `import.meta.url`, which breaks
      // when Vite pre-bundles them into `.vite/deps`.
      exclude: ['grit-wasm-bindings', 'quickjs-emscripten'],
      // CommonJS dependencies of the excluded packages still need converting. Pre-bundling them
      // together also makes the worker and the bindings share one `web-tree-sitter` instance.
      include: ['web-tree-sitter', 'grit-wasm-bindings > web-tree-sitter-wasm-bindgen'],
    },
    worker: {
      format: 'es',
    },
  },
});
