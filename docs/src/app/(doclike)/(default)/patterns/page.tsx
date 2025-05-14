import { groupBy } from 'lodash';
import type { Metadata } from 'next';

import { Heading } from '@/components/heading';
import { PatternsList } from '@/components/patterns/list';
import { getPatternsList } from '@/libs/patterns';
import { PatternNodeWithLanguage } from '@getgrit/universal';

export type DocPattern = Partial<Omit<PatternNodeWithLanguage, '__typename'>> & {
  preview?: boolean;
  link?: string;
};

type OrganizedPatterns = Record<'migration' | 'lint' | 'misc', DocPattern[]>;

const getPatterns = async () => {
  const remotePatterns = await getPatternsList();

  const organizedPatterns = groupBy(remotePatterns, (pattern) => {
    if (pattern.tags?.includes('migration')) {
      return 'migration';
    }

    if (pattern.tags?.includes('lint')) {
      return 'lint';
    }

    return 'misc';
  });

  const staticPatterns = getStaticPatterns();
  const finalPatterns = {
    lint: [...(organizedPatterns.lint || []), ...staticPatterns.lint],
    migration: [...(organizedPatterns.migration || []), ...staticPatterns.migration],
    misc: [...(organizedPatterns.misc || []), ...staticPatterns.misc],
  };

  return finalPatterns;
};

const getStaticPatterns = (): OrganizedPatterns => {
  return {
    migration: [
      {
        title: 'JavaScript to TypeScript',
        name: 'js_to_ts',
        preview: false,
      },
      {
        title: 'AngularJS to Angular',
        preview: true,
      },
      {
        title: 'Angular to React',
        preview: true,
      },
    ],
    lint: [
      {
        title: 'Harden DOM usage',
        preview: true,
      },
    ],
    misc: [],
  };
};

export const metadata: Metadata = {
  title: 'Available Patterns',
};

export default async function Page() {
  const patterns = await getPatterns();
  const patternCount = Object.values(patterns).flat().length;

  return (
    <>
      <h1 id={'main-heading'}>Pattern Library</h1>
      <p>
        Grit comes with {patternCount} out of the box patterns that can be leveraged immediately.
      </p>
      {patterns.migration.length > 0 && (
        <>
          <Heading id={'migrations'} level={2}>
            Migration Patterns
          </Heading>
          <p>
            Migration patterns can be used to automatically migrate you to a new framework or
            library.
          </p>
          <PatternsList patterns={patterns.migration} />
        </>
      )}
      {patterns.lint.length > 0 && (
        <>
          <Heading id={'linters'} level={2}>
            Linters
          </Heading>
          <p>
            These patterns can autofix many common JavaScript mistakes, including issues that eslint
            doesn&apos;t fix automatically.
          </p>
          <PatternsList patterns={patterns.lint} />
        </>
      )}
      {patterns.misc.length > 0 && (
        <>
          <Heading id={'Miscellaneous'} level={2}>
            Miscellaneous
          </Heading>
          <PatternsList patterns={patterns.misc} />
        </>
      )}
      <p>* Patterns with an asterisk are in private alpha with select customers.</p>
    </>
  );
}
