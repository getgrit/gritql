'use client';

import { PropsWithChildren, useCallback, useMemo } from 'react';

import { Language } from '../../universal/patterns/types';
import { ResolvedGritPattern } from '@/libs/patterns';

type StandardLibraryPattern = Pick<ResolvedGritPattern, 'name' | 'body' | 'language'>;

export const StandardLibraryProvider: React.FC<
  PropsWithChildren<{ patterns: StandardLibraryPattern[] }>
> = ({ children, patterns }) => {
  const getPatternsForLanguage = useCallback(
    async (language: keyof typeof Language) => {
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
  const LibraryContext: any = {};
  return <LibraryContext.Provider value={context}>{children}</LibraryContext.Provider>;
};
