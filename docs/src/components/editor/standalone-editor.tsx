'use client';

import { useMemo, createContext, ReactNode, useContext, useEffect, useState } from 'react';
import { editor } from 'monaco-editor';
import cx from 'classnames';

import { CloseButton } from '@/components/code-block/buttons';
import { SnippetHeading } from '@/components/code-block/heading';
import { MonacoEditor } from './monaco-editor';
import { MonacoDiffEditor } from './monaco-diff-editor';
import { useDiffEditor } from '@/hooks/use-diff-editor';
import { useEditorCursor } from '@/hooks/use-editor-cursor';
import { useDelayedLoader } from '@/hooks/use-delayed-loader';
import { extractMetavariables } from '../../utils/extract-metavariables';

import {
  extractLanguageFromPatternBody,
  getEditorLangIdFromLanguage
} from '@/universal/patterns/utils';
import { isMatch } from '@/universal/matching/types';

const EDITOR_OPTIONS: editor.IStandaloneEditorConstructionOptions = {
  scrollbar: {
    alwaysConsumeMouseWheel: false,
    handleMouseWheel: true,
    vertical: 'auto',
    horizontal: 'auto',
  },
  scrollBeyondLastLine: false,
};

interface EditorState {
  pattern: string;
  setPattern: (newPattern: string) => void;
  input: string;
  setInput: (newInput: string) => void;
  path: string | undefined;
  setPath: (newPath: string) => void;
}

export const StandaloneEditorContext = createContext<EditorState>({
  pattern: '',
  setPattern: () => {},
  input: '',
  setInput: () => {},
  path: '',
  setPath: () => {},
});

export const StandaloneEditorProvider: React.FC<React.PropsWithChildren<{}>> = ({ children }) => {
  const [pattern, setPattern] = useState('');
  const [input, setInput] = useState('');
  const [path, setPath] = useState<string | undefined>(undefined);

  const value = {
    pattern,
    setPattern,
    input,
    setInput,
    path,
    setPath,
  };

  return (
    <StandaloneEditorContext.Provider value={value}>{children}</StandaloneEditorContext.Provider>
  );
};

export const useStandaloneEditor = () => {
  const context = useContext(StandaloneEditorContext);
  if (context === undefined) {
    throw new Error('useStandaloneEditor must be used within a StandaloneEditorProvider');
  }
  return context;
};

export const StandaloneEditor: React.FC<{
  patternTitle?: string;
  resultTitle?: string;
}> = ({}) => {
  const { pattern, setPattern, input, setInput } = useStandaloneEditor();

  const language = useMemo(() => extractLanguageFromPatternBody(pattern), [pattern]);
  const { output, onPatternChange, onDiffChange, state, editorState, usesAi, analyze } =
    useDiffEditor({
      pattern,
      setPattern,
      input,
      setInput,
      path: language ? `test.${getEditorLangIdFromLanguage(language)}` : undefined,
    });

  const { metaVariables, oldVariables, newVariables } = useMemo(
    () => extractMetavariables(state),
    [state],
  );

  const match = useMemo(() => {
    return state.state === 'loaded' && isMatch(state.result) ? state.result : undefined;
  }, [state.state, state.result]);

  const { onCursorPositionChange, highlightedVariable } = useEditorCursor({
    variables: metaVariables,
  });

  const errorMessage = useMemo(() => ('log' in state ? state.log.message : undefined), [state]);

  const showDirty = useDelayedLoader(!!editorState);

  return (
    <div className='flex relative flex-col gap-4 h-full w-full p-2 overflow-hidden rounded-lg bg-neutral-800 transition ease-in-out'>
      <div className='h-1/2 rounded-md overflow-hidden monaco-pattern-editor relative'>
        <div className='flex m-0 justify-between px-3 py-2 bg-black'>
          <SnippetHeading title='Pattern Editor' />
          <CloseButton />
        </div>
        <MonacoEditor
          noCliff
          path='docs/grit.grit'
          value={pattern}
          language={'grit'}
          onChange={onPatternChange}
          options={{
            readOnly: false,
            scrollbar: {
              alwaysConsumeMouseWheel: true,
              handleMouseWheel: true,
              vertical: 'auto',
            },
            lineNumbers: 'on',
            glyphMargin: true,
            scrollBeyondLastLine: true,
            ...EDITOR_OPTIONS,
          }}
          onCursorPositionChange={onCursorPositionChange}
          placeholderColor='#9ca3af'
        />
        {usesAi && (
          <div className='absolute bottom-0 left-0 right-0 flex justify-between items-center px-3 py-2 bg-neutral-700'>
            <button
              className='bg-blue-500 text-white px-4 py-2 rounded disabled:opacity-50'
              disabled={!usesAi}
              onClick={analyze}
            >
              Run Pattern
            </button>
          </div>
        )}
      </div>
      <div className='h-1/2 rounded-md overflow-hidden'>
        <div className='flex m-0 justify-between px-3 py-2 bg-black'>
          <SnippetHeading title='diff' />
        </div>
        <div
          className={cx(
            'monaco-diff-editor h-full',
            { 'is-dirty': showDirty, 'is-match': !!match },
            editorState,
          )}
        >
          <MonacoDiffEditor
            noCliff
            originalModelPath='docs/org-example.js'
            modifiedModelPath='docs/mod-example.js'
            original={input}
            language={language ? getEditorLangIdFromLanguage(language) : 'js'}
            modified={output}
            options={{
              renderIndicators: true,
              renderSideBySide: true,
              lineNumbers: 'on',
              originalEditable: true,
              ...EDITOR_OPTIONS,
            }}
            placeholderColor='#9ca3af'
          />
        </div>
      </div>
      {errorMessage && (
        <div
          className='animate-slideUp absolute bottom-0 w-full -mx-2 rounded-b-md z-10 bg-tart-600'
          role='alert'
          data-testid='grit-error'
        >
          <p className='overflow-ellipsis overflow-hidden line-clamp-4 my-0 py-1 px-4 text-sm text-white font-mono'>
            <b>Error:</b> {errorMessage}
          </p>
        </div>
      )}
    </div>
  );
};
