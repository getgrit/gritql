'use client';

import { PropsWithChildren, useCallback, useMemo } from 'react';

import type { ResolvedGritPattern } from '@getgrit/api';
import { LibraryContext } from '@getgrit/editor';
import { Language } from '@getgrit/universal';

type StandardLibraryPattern = Pick<ResolvedGritPattern, 'name' | 'body' | 'language'>;

export const StandardLibraryProvider: React.FC<
  PropsWithChildren<{ patterns: StandardLibraryPattern[] }>
> = ({ children, patterns }) => {
  const getPatternsForLanguage = useCallback(
    async (language: Language) => {
      const filtered =
        patterns.filter((p) => p.language === language || p.language === Language.Universal) ?? [];
      console.log('Found', filtered.length, 'patterns for language', language, filtered);
      return {
        paths: filtered.map((p, i) => `${p.name}_${i}.grit`),
        contents: filtered.map((p) => p.body),
      };
    },
    [patterns],
  );

  const context = useMemo(() => {
    return {
      getPatternsForLanguage,
    };
  }, [getPatternsForLanguage]);
  return <LibraryContext.Provider value={context}>{children}</LibraryContext.Provider>;
};
