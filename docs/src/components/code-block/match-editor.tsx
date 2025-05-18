import { WithChildren } from '@/custom-types/shared';

import { SnippetEditor } from './editor';

type MatchEditorProps = WithChildren<{
  className?: string;
}>;

export const MatchEditor = ({ children }: MatchEditorProps) => {
  if (!Array.isArray(children)) return null;
  const [pattern, output] = children;
  const patternCode = (pattern.props.children as string).trim();
  const outputCode = (output.props.children as string).trim();

  return (
    <div className='grid rounded-md overflow-hidden'>
      <div className='grid grid-cols-1'>
        <SnippetEditor code={patternCode} language={pattern.props.language} />
        <SnippetEditor code={outputCode} language={output.props.language} />
      </div>
    </div>
  );
};
