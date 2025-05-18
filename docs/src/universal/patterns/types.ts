export type FilePath = string;

// Collect files to be transferred to the engine for parsing
export type GritFiles = Record<FilePath, string>;

export type FilePin = {
  name?: string;
  filePath: string;
  fileContents: undefined | string;
  rewrittenContents: undefined | string;
};

export const Language = {
  Css: 'CSS',
  CSharp: 'C_SHARP',
  Go: 'GO',
  Grit: 'GRIT',
  Hcl: 'HCL',
  Html: 'HTML',
  Java: 'JAVA',
  Js: 'JS',
  Json: 'JSON',
  Markdown: 'MARKDOWN',
  Php: 'PHP',
  Python: 'PYTHON',
  Ruby: 'RUBY',
  Rust: 'RUST',
  Sol: 'SOL',
  Sql: 'SQL',
  Toml: 'TOML',
  Universal: 'UNIVERSAL',
  Yaml: 'YAML'
} as const;

export type Repo = {
  owner: string;
  name: string;
  host: string;
  full_name: string;
};

export function makeRepo(fullName: string, host: string) {
  const nameParts = fullName.split('/');
  return {
    owner: nameParts[0]!,
    name: nameParts.slice(1).join('/'),
    full_name: fullName,
    host,
  };
}

export type RefInput = {
  sha?: string;
  branch?: string;
};

export type RunnableRef = {
  repo?: Repo;
  ref?: RefInput;
};

export type LivePattern = {
  globs: string;
  patternBody: string;
  isInitialized: boolean;
};

export type LiveData = {
  pins: FilePin[];
  pattern: LivePattern;
};

// Add missing exports
export type ImplicitFile = any;
export const extractPath = (path: any): string => path;
export const exhaustive = (x: never): never => x;
export const ImplicitFile = { PlaygroundPattern: 'playground-pattern' }; 