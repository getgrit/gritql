import React from 'react';

import Link from 'next/link';

import { PatternGitHubButton, PatternStudioButton } from '@/components/patterns/buttons';
import Markdoc from '@markdoc/markdoc';

import type { DocPattern } from '../../app/(doclike)/(default)/patterns/page';
import { PatternLanguageButton } from './languages';

export type PatternsListProps = {
  patterns: DocPattern[];
};

export const PatternsList = (props: PatternsListProps) => {
  const { patterns } = props;
  return (
    <div className='flex flex-col gap-6'>
      {patterns.map((pattern, i) => (
        <PatternItem key={pattern.name ?? `pattern_${i}`} pattern={pattern} />
      ))}
    </div>
  );
};

const PatternItem = ({ pattern }: { pattern: DocPattern }) => {
  const patternTitle = (
    <>
      <span className='group-hover:underline'>{pattern.title}</span>
      <span className='font-normal'>{pattern.name ? ` (${pattern.name})` : ''}</span>
      {pattern.preview ? '*' : ''}
    </>
  );
  return (
    <div className='bg-gray-50 rounded-md overflow-hidden border border-gray-100'>
      <div className='justify-between bg-gray-200 p-0 relative'>
        <h4 className='m-0 p-2 pr-32'>
          {pattern.gitHubUrl ? (
            <Link href={`/patterns/library/${pattern.name}`} className='no-underline group block'>
              {patternTitle}
            </Link>
          ) : (
            patternTitle
          )}
        </h4>
        <div className='flex gap-2 items-center absolute right-2 top-2'>
          <PatternLanguageButton size='sm' pattern={pattern} />
          <PatternStudioButton size='sm' pattern={pattern} />
          <PatternGitHubButton size='sm' pattern={pattern} />
        </div>
      </div>
      {pattern.description && <PatternDescription description={pattern.description} />}
    </div>
  );
};

const PatternDescription = ({ description }: { description: string }) => {
  const ast = Markdoc.parse(description);
  const content = Markdoc.transform(ast);

  return <div className='px-2'>{Markdoc.renderers.react(content, React)}</div>;
};
