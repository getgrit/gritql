'use client';

/* eslint-disable complexity */

import { FaPencilRuler, FaPlay } from 'react-icons/fa';

import { WithChildren } from '@/custom-types/shared';
import { useSidebarContext } from '@/hooks/sidebar';
import { MonacoEditor, useStandaloneEditor } from '@getgrit/editor';

import { CopyButton, TryButton } from './buttons';
import { extractCodeString } from './extract';
import { SnippetHeading } from './heading';

export type MarkdocCodeFenceProps = WithChildren<{
  language: string;
  readOnly: boolean;
  fileName?: string;
  diff?: boolean;
  match?: boolean;
  snippet?: boolean;
  short?: boolean;
  firstInPair?: boolean;
  secondInPair?: boolean;
  title?: string;
}>;

export type FenceProps = {
  props: {
    children: string | FenceProps[];
  };
};

export function MonacoBlock(props: MarkdocCodeFenceProps) {
  const {
    children,
    language,
    fileName,
    diff,
    match,
    snippet,
    firstInPair = false,
    secondInPair = false,
  } = props;
  const { setPattern, setInput } = useStandaloneEditor();
  const { setShowEditorSidebar } = useSidebarContext();
  let editorLang = language;

  if (diff || match) {
    return (
      <>
        Placeholder content, you expected a diff or match block here.
        <pre>{children}</pre>
      </>
    );
  }

  let code = '';
  let sample = '';

  if (snippet) {
    const [snippet, pattern] = children as FenceProps[];
    code = extractCodeString(pattern ? [pattern] : '');
    sample = extractCodeString(snippet ? [snippet] : '');
    editorLang = 'grit';
  } else if (typeof children === 'string') {
    code = (children as string).trim();
    sample = code;
  }

  if (props.short) {
    return (
      <div className='relative'>
        <pre className='my-0 codeblock'>{code}</pre>
      </div>
    );
  }

  const options = { readOnly: true };

  const title = props.title ?? (firstInPair ? 'Before' : secondInPair ? 'After' : language);

  return (
    <div className='bg-codeblock rounded-md overflow-hidden my-4'>
      <div className='flex justify-between px-3 py-2 bg-black'>
        <SnippetHeading fileName={fileName} title={title} />
        <div className='flex gap-2 w-72 justify-end'>
          <CopyButton data={code} />
          {snippet && (
            <TryButton
              className='float-right'
              onClick={() => {
                setPattern(code);
                setInput(sample);
                setShowEditorSidebar(true);
              }}
            >
              <FaPlay size={7} /> Run Pattern
            </TryButton>
          )}
          {firstInPair && (
            <TryButton
              className='float-right'
              onClick={() => {
                setInput(sample);
                setShowEditorSidebar(true);
              }}
            >
              <FaPencilRuler size={10} /> Edit
            </TryButton>
          )}
        </div>
      </div>
      <MonacoEditor options={options} value={code.trim()} language={editorLang} />
    </div>
  );
}
