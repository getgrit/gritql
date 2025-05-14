import { PropsWithChildren } from 'react';

import Link from 'next/link';

import { TerminalCommandBlock } from './cli';
import { CollapsedEditorPlaceholder } from './collapsed-client';
import { extractCodeString } from './extract';
import type { FenceProps } from './monaco';

export type PatternBlockInfo = {
  name?: string;
  repo?: {
    host: string;
    fullName: string;
  };
};

export const CollapsedCodeBlock: React.FC<PropsWithChildren<{ pattern?: PatternBlockInfo }>> = (
  props,
) => {
  const pattern = extractCodeString(props.children as FenceProps);
  const applyString =
    props.pattern && props.pattern.name
      ? props.pattern.repo
        ? `grit apply ${props.pattern.repo.host}/${props.pattern.repo.fullName}#${props.pattern.name}`
        : `grit apply ${props.pattern.name}`
      : null;
  return (
    <>
      <div className='flex flex-col self-stretch px-4 pt-3 pb-5 font-medium bg-white rounded-2xl border border-solid shadow-md border-zinc-900 border-opacity-10 not-prose'>
        <CollapsedEditorPlaceholder pattern={pattern} />
        {applyString ? (
          <>
            <hr className='mt-4 mb-0' />
            <TerminalCommandBlock
              command={applyString}
              title={
                <span>
                  Apply with the{' '}
                  <Link href='/cli/quickstart' className='text-white hover:underline'>
                    Grit CLI
                  </Link>
                </span>
              }
            />
          </>
        ) : null}
      </div>
    </>
  );
};
