import { useRef, useMemo, useState, useEffect } from 'react';
import merge from 'lodash/merge';
import { DiffEditor, DiffEditorProps, useMonaco } from '@monaco-editor/react';

import { MatchIndex } from './highlights';
import { Match, Position } from '@/universal/matching/types';

const noop = () => {};

export const SSRStyle = {
  height: '100%',
  lineHeight: '18px',
  fontSize: '12px',
  borderRadius: 0,
  flex: 1,
  margin: 0,
};

export interface MonacoDiffProps extends DiffEditorProps {
  maxLines?: number;
  minLines?: number;
  noCliff?: boolean;
  highlightedVariable?: string | null;
  oldHighlights?: MatchIndex[];
  newHighlights?: MatchIndex[];
  oldVariables?: any[];
  newVariables?: any[];
  match?: Match;
  onCursorPositionChange?: (position?: Position) => void;
  onChange?: (original: string, modified: string) => void;
  placeholderColor?: string;
  focusIfEmpty?: boolean;
}

export const MonacoDiffEditor = ({
  original,
  modified,
  language = 'plaintext',
  options,
  noCliff,
  maxLines,
  minLines = 1,
  onCursorPositionChange = noop,
  onChange = noop,
  ...rest
}: MonacoDiffProps) => {
  const monaco = useMonaco();
  const readOnly = options?.readOnly ?? true;
  const editorRef = useRef<any>(null);
  const [didMount, setDidMount] = useState(false);
  const [isClient, setIsClient] = useState(false);

  const height = useMemo(() => {
    const lines = Math.max(
      (original ?? '').split('\n').length,
      (modified ?? '').split('\n').length,
    );
    return Math.max(minLines, Math.min(maxLines ?? lines, lines)) * 18;
  }, [original, modified, maxLines, minLines]);

  const handleEditorDidMount = async (editor: any) => {
    editorRef.current = editor;
    setDidMount(true);
    editor.getModifiedEditor().onDidChangeCursorPosition(onCursorPositionChange);
    editor.getOriginalEditor().onDidChangeCursorPosition(onCursorPositionChange);
  };

  useEffect(() => {
    if (!didMount || !editorRef.current) return;
    editorRef.current.getModifiedEditor().setValue(modified ?? '');
    editorRef.current.getOriginalEditor().setValue(original ?? '');
  }, [original, modified, didMount]);

  // NOTE: return plain text side by side if SSR, Monaco doesn't handle this internally.
  useEffect(() => setIsClient(true), []);

  return isClient ? (
    <DiffEditor
      theme='grit'
      loading={<Loading original={original ?? ''} modified={modified ?? ''} />}
      height={noCliff ? '100%' : `${height}px`}
      options={merge(editorOptions, readOnly && { ...readOnlyOptions }, options)}
      onMount={handleEditorDidMount}
      language={language}
      {...rest}
    />
  ) : (
    <Loading original={original ?? ''} modified={modified ?? ''} />
  );
};

const Loading = ({ original, modified }: { original: string; modified: string }) => (
  <div style={{ display: 'flex', gap: '1rem' }}>
    <pre style={SSRStyle}>{original}</pre>
    <pre style={SSRStyle}>{modified}</pre>
  </div>
);

const editorOptions = {
  minimap: { enabled: false },
  scrollBeyondLastLine: false,
  scrollbar: {
    vertical: 'hidden',
    horizontal: 'hidden',
  },
  lineNumbers: 'off',
  glyphMargin: false,
  folding: false,
  lineDecorationsWidth: 0,
  lineNumbersMinChars: 0,
  renderLineHighlight: 'none',
  overviewRulerBorder: false,
  hideCursorInOverviewRuler: true,
  overviewRulerLanes: 0,
  contextmenu: false,
  wordWrap: 'on',
  padding: { top: 8, bottom: 8 },
  renderSideBySide: true,
};

const readOnlyOptions = {
  readOnly: true,
  domReadOnly: true,
  contextmenu: false,
  quickSuggestions: false,
  suggestOnTriggerCharacters: false,
  acceptSuggestionOnEnter: 'off',
  tabCompletion: 'off',
  wordBasedSuggestions: 'off',
  parameterHints: { enabled: false },
  hover: { enabled: false },
  links: false,
  find: { addExtraSpaceOnTop: false },
  folding: false,
  lineNumbers: 'off',
  glyphMargin: false,
  lineDecorationsWidth: 0,
  lineNumbersMinChars: 0,
  renderLineHighlight: 'none',
  overviewRulerBorder: false,
  hideCursorInOverviewRuler: true,
  overviewRulerLanes: 0,
}; 