const { readdirSync } = require('fs');
const directories = readdirSync('src', { withFileTypes: true })
  .filter((dir) => dir.isDirectory())
  .map((dir) => dir.name)
  .join('|');

/** @type {import("eslint").Linter.BaseConfig} */
module.exports = {
  root: true,
  extends: ['next'],
  plugins: ['@typescript-eslint', 'simple-import-sort'],
  settings: {
    next: {
      rootDir: 'src',
    },
  },
  overrides: [
    {
      files: ['*.mdoc', '*.mdx', '*.md'],
      plugins: ['markdownlint'],
      parser: 'eslint-plugin-markdownlint/parser',
      rules: {},
    },
  ],
  rules: {
    'simple-import-sort/imports': [
      'error',
      {
        groups: [
          // Side effect imports.
          ['^\\u0000'],
          // React always comes first, if present.
          ['^react$'],
          // NPM packages
          [`^@?(?!getgrit|${directories})\\w`],
          // Absolute imports.
          ['^'],
          // Relative imports.
          ['^\\.'],
          // CSS imports.
          ['\\.css$'],
        ],
      },
    ],
    'simple-import-sort/exports': 'error',
    complexity: ['error', 10],
    '@typescript-eslint/no-unused-vars': [
      'error',
      {
        ignoreRestSiblings: true,
        argsIgnorePattern: '^(_|unused)',
        varsIgnorePattern: '^(_|unused|React)',
      },
    ],
  },
};
