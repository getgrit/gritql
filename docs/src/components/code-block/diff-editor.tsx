'use client';

import { WithChildren } from '@/custom-types/shared';
import { MonacoDiffEditor } from '@getgrit/editor';

import { cleanHlTags, getHighlights, SnippetEditor } from './editor';
import { SnippetHeading } from './heading';

export type DiffEditorProps = WithChildren<{
  className?: string;
}>;

export const DiffEditor = ({ children }: DiffEditorProps) => {
  if (!Array.isArray(children)) return null;
  const [main, input, output] = children;

  return (
    <div className='grid rounded-md overflow-hidden'>
      <div className='grid grid-cols-1'>
        <SnippetEditor
          title='pattern'
          code={(main.props.children as string).trim()}
          language={main.props.language}
        />
      </div>
      <div className='bg-codeblock h-full overflow-hidden'>
        <div className='flex m-0 justify-between px-3 py-2 bg-black'>
          <SnippetHeading title='diff' />
        </div>
        <MonacoDiffEditor
          options={{ readOnly: true, lineNumbers: 'off' }}
          language={input.props.language}
          original={cleanHlTags(input.props.children).trim()}
          modified={cleanHlTags(output.props.children).trim()}
          oldHighlights={getHighlights(input.props.children)}
          newHighlights={getHighlights(output.props.children)}
          placeholderColor='#9ca3af'
        />
      </div>
    </div>
  );
};
