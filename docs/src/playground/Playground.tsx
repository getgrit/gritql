import { DiffEditor, type DiffOnMount, Editor } from '@monaco-editor/react';
import type { editor } from 'monaco-editor';
import { useEffect, useMemo, useRef, useState } from 'react';

import { getInputLanguage, GRIT_LANGUAGE_ID, GRIT_THEME, setupMonaco } from './monaco-setup';
import { readPlaygroundHash } from './share';
import { isMatch, isRewrite } from './types';
import { useGritAnalysis } from './use-grit-analysis';

import './playground.css';

const DEFAULT_PATTERN =
  '// Enter your Grit pattern here, for example:\n`console.log($msg)` => `console.info($msg)`\n';
const DEFAULT_INPUT =
  '// Enter your code to transform here\nconsole.log("Hello, world!");\nconsole.log("This is a test");';

const SHARED_OPTIONS: editor.IEditorOptions = {
  automaticLayout: true,
  minimap: { enabled: false },
  lineNumbers: 'on',
  glyphMargin: true,
  scrollBeyondLastLine: false,
  scrollbar: {
    alwaysConsumeMouseWheel: false,
    handleMouseWheel: true,
    vertical: 'auto',
    horizontal: 'auto',
  },
};

const DIFF_OPTIONS: editor.IDiffEditorConstructionOptions = {
  ...SHARED_OPTIONS,
  // The output (right side) is read-only; the input (left side) is editable.
  readOnly: true,
  originalEditable: true,
  renderSideBySide: true,
  renderIndicators: true,
  renderOverviewRuler: false,
};

/** Returns `true` once `value` has been `true` for `delay` ms, to avoid flashing loaders. */
function useDelayedFlag(value: boolean, delay = 300) {
  const [delayed, setDelayed] = useState(false);
  useEffect(() => {
    if (!value) {
      setDelayed(false);
      return;
    }
    const timeout = setTimeout(() => setDelayed(true), delay);
    return () => clearTimeout(timeout);
  }, [value, delay]);
  return delayed;
}

export default function Playground() {
  const [pattern, setPattern] = useState(
    () => readPlaygroundHash(window.location.hash).pattern ?? DEFAULT_PATTERN,
  );
  const [input, setInput] = useState(
    () => readPlaygroundHash(window.location.hash).input ?? DEFAULT_INPUT,
  );
  const { analysis, pending } = useGritAnalysis(pattern, input);
  const showPending = useDelayedFlag(pending);

  const result = analysis.status === 'loaded' ? analysis.result : undefined;
  const match = result && isMatch(result) ? result : undefined;
  const output = result && isRewrite(result) ? result.rewritten.content : input;
  const inputLanguage = useMemo(() => getInputLanguage(pattern), [pattern]);

  const diffEditorRef = useRef<editor.IStandaloneDiffEditor | null>(null);
  const decorationsRef = useRef<editor.IEditorDecorationsCollection | null>(null);
  const [diffMounted, setDiffMounted] = useState(false);

  const onDiffMount: DiffOnMount = (diffEditor) => {
    diffEditorRef.current = diffEditor;
    const original = diffEditor.getOriginalEditor();
    original.onDidChangeModelContent(() => setInput(original.getValue()));
    decorationsRef.current = original.createDecorationsCollection();
    setDiffMounted(true);
  };

  // The models are synced by hand: passing `original`/`modified` as props makes
  // `@monaco-editor/react` reset the input (and the cursor) on every keystroke.
  useEffect(() => {
    const original = diffEditorRef.current?.getOriginalEditor();
    if (diffMounted && original && original.getValue() !== input) original.setValue(input);
  }, [input, diffMounted]);

  useEffect(() => {
    const modified = diffEditorRef.current?.getModifiedEditor();
    if (diffMounted && modified && modified.getValue() !== output) modified.setValue(output);
  }, [output, diffMounted]);

  useEffect(() => {
    decorationsRef.current?.set(
      (match?.ranges ?? []).map(({ start, end }) => ({
        range: {
          startLineNumber: start.line,
          startColumn: start.column,
          endLineNumber: end.line,
          endColumn: end.column,
        },
        options: { inlineClassName: 'match-highlight' },
      })),
    );
  }, [match, diffMounted]);

  const status = showPending
    ? 'Applying pattern…'
    : analysis.status === 'empty'
      ? 'No matches'
      : undefined;

  return (
    <div className="playground">
      <section className="playground-panel" aria-label="Pattern">
        <header>Pattern Editor</header>
        <div className="playground-editor">
          <Editor
            path="playground/pattern.grit"
            language={GRIT_LANGUAGE_ID}
            theme={GRIT_THEME}
            value={pattern}
            onChange={(value) => setPattern(value ?? '')}
            beforeMount={setupMonaco}
            options={SHARED_OPTIONS}
            loading="Loading editor…"
          />
        </div>
      </section>
      <section className="playground-panel" aria-label="Input and output">
        <header>
          <span>{match ? 'Input (matches highlighted)' : 'Input'}</span>
          {status && (
            <span className="playground-status" role="status">
              {status}
            </span>
          )}
          {!match && <span>Output</span>}
        </header>
        <div className={`playground-editor playground-diff${match ? ' is-match' : ''}`}>
          <DiffEditor
            originalModelPath="playground/input"
            modifiedModelPath="playground/output"
            language={inputLanguage}
            theme={GRIT_THEME}
            beforeMount={setupMonaco}
            onMount={onDiffMount}
            options={DIFF_OPTIONS}
            loading="Loading editor…"
          />
        </div>
      </section>
      {analysis.status === 'error' && (
        <div className="playground-error" role="alert" data-testid="grit-error">
          <b>Error:</b> {analysis.message}
        </div>
      )}
    </div>
  );
}
