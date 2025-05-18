import { MonacoEditor } from '@/components/editor/monaco-editor';
import { MatchIndex } from '@/components/editor/highlights';

import { CopyButton } from './buttons';
import { SnippetHeading } from './heading';

const HL_START = '{hl}';
const HL_END = '{/hl}';

const getMatchIndex = (matches: RegExpMatchArray[]): MatchIndex[] => {
  const offset = HL_START.length + HL_END.length;
  return matches
    .filter((match) => match.index !== undefined)
    .map((match, i) => ({
      startIndex: match.index! - offset * i,
      endIndex: match.index! + match[0]!.length - offset * (i + 1),
    }));
};

export const getHighlights = (code: string) => {
  const highlightRegex = new RegExp(HL_START + '(.*?)' + HL_END, 'gs');
  const matches = Array.from(code.matchAll(highlightRegex));
  return getMatchIndex(matches);
};

export const cleanHlTags = (code: string) => {
  return code.replaceAll(HL_START, '').replaceAll(HL_END, '');
};

type EditorProps = {
  title?: string;
  code: string;
  language: string;
};

export const SnippetEditor = ({ title, code, language }: EditorProps) => {
  const highlights = getHighlights(code);
  const formattedCode = cleanHlTags(code);
  return (
    <div className='bg-codeblock h-full overflow-hidden'>
      <div className='flex m-0 justify-between px-3 py-2 bg-black'>
        <SnippetHeading title={title || language} />
        <CopyButton data={formattedCode} />
      </div>
      <MonacoEditor
        {...{ highlights, value: formattedCode.trim(), language }}
        options={{ readOnly: true, lineNumbers: 'off' }}
      />
    </div>
  );
};
