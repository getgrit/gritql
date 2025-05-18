import { useRef, useMemo, useState, useEffect } from 'react';
import merge from 'lodash/merge';
import Editor, { OnMount, EditorProps, useMonaco } from '@monaco-editor/react';
import { editor } from 'monaco-editor';

const noop = () => {};

export const SSRStyle = {
  height: '100%',
  lineHeight: '18px',
  fontSize: '12px',
  borderRadius: 0,
  flex: 1,
  margin: 0,
};

export interface MonacoProps extends EditorProps {
  minLines?: number;
  maxLines?: number;
  noCliff?: boolean;
  onCursorPositionChange?: (data: editor.ICursorPositionChangedEvent) => void;
  placeholderColor?: string;
}

export const MonacoEditor = ({
  value,
  language = 'plaintext',
  options,
  noCliff,
  maxLines,
  minLines = 1,
  onCursorPositionChange = noop,
  onChange = noop,
  placeholderColor,
  ...rest
}: MonacoProps) => {
  const monaco = useMonaco();
  const readOnly = options?.readOnly ?? true;
  const editorRef = useRef<editor.IStandaloneCodeEditor | null>(null);
  const [didMount, setDidMount] = useState(false);
  const [isClient, setIsClient] = useState(false);

  const height = useMemo(() => {
    return getHeight(value ?? '', maxLines, minLines);
  }, [value, maxLines, minLines]);

  const handleEditorDidMount: OnMount = async (editor, monaco) => {
    editorRef.current = editor;
    setDidMount(true);
    editor.onDidChangeCursorPosition(onCursorPositionChange);
    editor.onDidBlurEditorWidget((data: any) => {
      onCursorPositionChange(data);
    });

    editor.setValue(value ?? '');
  };

  useEffect(() => {
    if (!didMount || !editorRef.current) return;
    editorRef.current.setValue(value ?? '');
  }, [value, didMount]);

  // NOTE: return plain text side by side if SSR, Monaco doesn't handle this internally.
  useEffect(() => setIsClient(true), []);

  return isClient ? (
    <Editor
      theme='grit'
      loading={<Loading value={value ?? 'Loading...'} />}
      height={noCliff ? '100%' : `${height}px`}
      options={merge(editorOptions, readOnly && { ...readOnlyOptions }, options)}
      onChange={(value, editor) => {
        const hasFocus = editorRef.current?.hasTextFocus();
        if (hasFocus) onChange(value, editor);
      }}
      onMount={handleEditorDidMount}
      language={language}
      {...rest}
    />
  ) : (
    <Loading value={value ?? 'Loading...'} />
  );
};

const Loading = ({ value }: { value: string }) => <pre style={SSRStyle}>{value}</pre>;

const getHeight = (value: string, maxLines?: number, minLines = 1) => {
  const lines = value.split('\n').length;
  const height = Math.max(minLines, Math.min(maxLines ?? lines, lines)) * 18;
  return height;
};

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