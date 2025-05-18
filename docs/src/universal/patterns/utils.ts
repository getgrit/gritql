import { Language, exhaustive } from './types';
// import { Language } from '../matching/types';
import { isLanguage } from '../matching/types';

const matchLanguageDeclaration = (patternBody: string) => {
  const regex = /^\s*language\s+([a-z]+)/m;
  return regex.exec(patternBody);
};

export function extractLanguageFromPatternBody(patternBody: string, fallback: any): any;
export function extractLanguageFromPatternBody(patternBody: string): any | undefined;
export function extractLanguageFromPatternBody(
  patternBody: string,
  fallback?: any,
): any | undefined {
  const selected = matchLanguageDeclaration(patternBody);
  if (selected && selected[1]) {
    const lang = selected[1].toUpperCase();
    if (isLanguage(lang)) return lang;
  }
  return fallback;
} 

/**
 * Given a pattern definition, compute a title
 */
export const getPatternTitle = (pattern: {
  title?: string;
  localName: string;
}) => {
  return pattern.title ?? pattern.localName;
};

/**
 * Given a pattern definition, compute an optimized description
 */
export const getPatternDescription = (
  pattern: {
    description?: string;
    localName: string;
  },
) => {
  return pattern.description ?? `Automatically migrate ${pattern.localName}.`;
};

export function getEditorLangIdFromLanguage(lang: any): string {
  switch (lang) {
    case 'CSS':
      return 'css';
    case 'C_SHARP':
      return 'cs';
    case 'GO':
      return 'go';
    case 'GRIT':
      return 'grit';
    case 'HCL':
      return 'hcl';
    case 'HTML':
      return 'html';
    case 'JAVA':
      return 'java';
    case 'JS':
      return 'typescript';
    case 'JSON':
      return 'json';
    case 'PYTHON':
      return 'py';
    case 'RUBY':
      return 'rb';
    case 'RUST':
      return 'rust';
    case 'SOL':
      return 'solidity';
    case 'SQL':
      return 'sql';
    case 'YAML':
      return 'yaml';
    case 'MARKDOWN':
      return 'markdown';
    case 'TOML':
      return 'toml';
    case 'PHP':
      return 'php';
    case 'UNIVERSAL':
      return 'universal';
    default:
      return 'unknown';
  }
}

