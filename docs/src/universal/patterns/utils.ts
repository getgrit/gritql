import { Language } from '../matching/types';
import { isLanguage } from '../matching/types';

const matchLanguageDeclaration = (patternBody: string) => {
  const regex = /^\s*language\s+([a-z]+)/m;
  return regex.exec(patternBody);
};

export function extractLanguageFromPatternBody(patternBody: string, fallback: Language): Language;
export function extractLanguageFromPatternBody(patternBody: string): Language | undefined;
export function extractLanguageFromPatternBody(
  patternBody: string,
  fallback?: Language,
): Language | undefined {
  const selected = matchLanguageDeclaration(patternBody);
  if (selected && selected[1]) {
    const lang = selected[1].toUpperCase();
    if (isLanguage(lang)) return lang;
  }
  return fallback;
} 