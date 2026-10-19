import type { Monaco } from '@monaco-editor/react';
import type { editor, languages } from 'monaco-editor';

export const GRIT_LANGUAGE_ID = 'grit';
export const GRIT_THEME = 'grit';

/** Monaco language ids for the target languages GritQL patterns can declare. */
const MONACO_LANGUAGES: Record<string, string> = {
  css: 'css',
  csharp: 'csharp',
  go: 'go',
  hcl: 'hcl',
  html: 'html',
  java: 'java',
  js: 'typescript',
  json: 'json',
  kotlin: 'kotlin',
  markdown: 'markdown',
  php: 'php',
  python: 'python',
  ruby: 'ruby',
  rust: 'rust',
  sol: 'sol',
  solidity: 'sol',
  sql: 'sql',
  vue: 'html',
  yaml: 'yaml',
};

/**
 * Returns the Monaco language id for the code a pattern runs against, based on the pattern's
 * `language` declaration. Patterns without a declaration target JavaScript/TypeScript.
 */
export function getInputLanguage(pattern: string) {
  const declared = /^\s*language\s+([a-z]+)/m.exec(pattern)?.[1] ?? 'js';
  return MONACO_LANGUAGES[declared] ?? 'plaintext';
}

const KEYWORDS = [
  'and', 'or', 'not', 'maybe', 'any', 'some', 'every', 'contains', 'within', 'until', 'after',
  'before', 'as', 'where', 'if', 'else', 'bubble', 'limit', 'sequential', 'multifile', 'orelse',
  'includes', 'like', 'return', 'private', 'true', 'false', 'undefined',
];

// Token names follow the scopes of `src/grammars/gritql.tmLanguage.json`, used for the docs.
const gritLanguage: languages.IMonarchLanguage = {
  keywords: KEYWORDS,
  tokenizer: {
    root: [
      [/\/\/.*$/, 'comment'],
      [/^(\s*)(language|engine)(\s+)([a-z_]+)/, ['', 'keyword.control.grit', '', 'meta.language.grit']],
      [
        /\b(pattern|predicate|function)(\s+)([A-Za-z_]\w*)/,
        ['keyword.control.grit', '', 'entity.name.function.grit'],
      ],
      [/\b(raw|js|ts|css|json|html|py|python|rb|ruby|rs|rust|go|java|sol|hcl|sql|yaml|md|markdown|toml|php|cs)(`)/, ['markup.quote.literal.language.grit', { token: 'snippet.grit', next: '@snippet' }]],
      [/`/, { token: 'snippet.grit', next: '@snippet' }],
      [/\br"/, { token: 'regexp', next: '@regex' }],
      [/"/, { token: 'string', next: '@string' }],
      [/\$(\.\.\.|[A-Za-z_]\w*)/, 'variable'],
      [/\d+(\.\d+)?/, 'number'],
      [/[A-Za-z_]\w*(?=\s*\()/, 'entity.name.function.grit'],
      [/[A-Za-z_]\w*/, { cases: { '@keywords': 'keyword.control.grit', '@default': 'identifier' } }],
      [/<:|=>|\+=|==|!=|<=|>=|[!=<>+\-*/%]/, 'keyword.operator.grit'],
    ],
    snippet: [
      [/\\[`$\\]/, 'snippet.grit'],
      [/\$(\.\.\.|[A-Za-z_]\w*)/, 'variable.interpolated.grit'],
      [/`/, { token: 'snippet.grit', next: '@pop' }],
      [/[^`\\$]+/, 'snippet.grit'],
      [/./, 'snippet.grit'],
    ],
    string: [
      [/\\./, 'string'],
      [/\$(\.\.\.|[A-Za-z_]\w*)/, 'variable.interpolated.grit'],
      [/"/, { token: 'string', next: '@pop' }],
      [/[^"\\$]+/, 'string'],
      [/./, 'string'],
    ],
    regex: [
      [/\\./, 'regexp'],
      [/"/, { token: 'regexp', next: '@pop' }],
      [/[^"\\]+/, 'regexp'],
    ],
  },
};

const gritTheme: editor.IStandaloneThemeData = {
  base: 'vs-dark',
  inherit: true,
  rules: [
    { token: 'meta.language.grit', foreground: '#F5D544' },
    { token: 'variable.interpolated.grit', foreground: '#ff7b72', fontStyle: 'italic' },
    { token: 'keyword.control.grit', foreground: '#58a6ff' },
    { token: 'variable', foreground: '#e2e2e2' },
    { token: 'snippet.grit', foreground: '#a5d6ff' },
    { token: 'comment', foreground: '#3fb950' },
    { token: 'keyword.operator.grit', foreground: '#F8B8BC', fontStyle: 'bold' },
    { token: 'markup.quote.literal.language.grit', foreground: '#F5D544' },
    { token: 'entity.name.function.grit', foreground: '#D098FD' },
  ],
  colors: {
    'editor.background': '#1a1a1a',
  },
};

let registered = false;

/** Registers the GritQL language and the editor theme. Pass as `beforeMount` to the editors. */
export function setupMonaco(monaco: Monaco) {
  if (!registered) {
    monaco.languages.register({ id: GRIT_LANGUAGE_ID });
    monaco.languages.setMonarchTokensProvider(GRIT_LANGUAGE_ID, gritLanguage);
    registered = true;
  }
  monaco.editor.defineTheme(GRIT_THEME, gritTheme);
}
