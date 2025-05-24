'use client';

// import { useMemo } from 'react';
// import cx from 'classnames';
// import { editor } from 'monaco-editor';

// import { CloseButton } from '@/components/code-block/buttons';
// import { SnippetHeading } from '@/components/code-block/heading';
// import { MonacoEditor } from './monaco-editor';
// import { MonacoDiffEditor } from './monaco-diff-editor';
// import { useDiffEditor } from '@/hooks/use-diff-editor';
// import { useEditorCursor } from '@/hooks/use-editor-cursor';
// import { useDelayedLoader } from '@/hooks/use-delayed-loader';
// import { extractMetavariables } from '../../utils/extract-metavariables';
// import { extractLanguageFromPatternBody, getEditorLangIdFromLanguage } from '@/universal/patterns/utils';
// import { isMatch } from '@/universal/matching/types';
// import { useStandaloneEditor } from './context';

// const EDITOR_OPTIONS: editor.IStandaloneEditorConstructionOptions = {
//   scrollbar: {
//     alwaysConsumeMouseWheel: false,
//     handleMouseWheel: true,
//     vertical: 'auto',
//     horizontal: 'auto',
//   },
//   scrollBeyondLastLine: false,
// };

export const StandaloneEditor: React.FC<{
  patternTitle?: string;
  resultTitle?: string;
}> = ({ }) => {
  return "bob";

  // const { pattern, setPattern, input, setInput } = useStandaloneEditor();

  // const language = useMemo(() => extractLanguageFromPatternBody(pattern), [pattern]);
  // const { output, onPatternChange, onDiffChange, state, editorState, usesAi, analyze } =
  //   useDiffEditor({
  //     pattern,
  //     setPattern,
  //     input,
  //     setInput,
  //     path: language ? `test.${getEditorLangIdFromLanguage(language)}` : undefined,
  //   });

  // const { metaVariables, oldVariables, newVariables } = useMemo(
  //   () => extractMetavariables(state),
  //   [state],
  // );

  // const match = useMemo(() => {
  //   return state.state === 'loaded' && isMatch(state.result) ? state.result : undefined;
  // }, [state.state, state.result]);

  // const { onCursorPositionChange } = useEditorCursor({
  //   variables: metaVariables,
  // });

  // const errorMessage = useMemo(() => ('log' in state ? (state.log as any)?.message : undefined), [state]);

  // const showDirty = useDelayedLoader(!!editorState);

  // return (
  //   <div className='flex relative flex-col gap-4 h-full w-full p-2 overflow-hidden rounded-lg bg-neutral-800 transition ease-in-out'>
  //     <div className='h-1/2 rounded-md overflow-hidden monaco-pattern-editor relative'>
  //       <div className='flex m-0 justify-between px-3 py-2 bg-black'>
  //         <SnippetHeading title='Pattern Editor' />
  //         <CloseButton />
  //       </div>
  //       <MonacoEditor
  //         noCliff
  //         path='docs/grit.grit'
  //         value={pattern}
  //         language={'grit'}
  //         onChange={onPatternChange}
  //         options={{
  //           readOnly: false,
  //           scrollbar: {
  //             alwaysConsumeMouseWheel: true,
  //             handleMouseWheel: true,
  //             vertical: 'auto',
  //           },
  //           lineNumbers: 'on',
  //           glyphMargin: true,
  //           scrollBeyondLastLine: true,
  //           ...EDITOR_OPTIONS,
  //         }}
  //         onCursorPositionChange={onCursorPositionChange}
  //         placeholderColor='#9ca3af'
  //       />
  //       {usesAi && (
  //         <div className='absolute bottom-0 left-0 right-0 flex justify-between items-center px-3 py-2 bg-neutral-700'>
  //           <button
  //             className='bg-blue-500 text-white px-4 py-2 rounded disabled:opacity-50'
  //             disabled={!usesAi}
  //             onClick={analyze}
  //           >
  //             Run Pattern
  //           </button>
  //         </div>
  //       )}
  //     </div>
  //     <div className='h-1/2 rounded-md overflow-hidden'>
  //       <div className='flex m-0 justify-between px-3 py-2 bg-black'>
  //         <SnippetHeading title='diff' />
  //       </div>
  //       <div
  //         className={cx(
  //           'monaco-diff-editor h-full',
  //           { 'is-dirty': showDirty, 'is-match': !!match },
  //           editorState,
  //         )}
  //       >
  //         <MonacoDiffEditor
  //           noCliff
  //           originalModelPath='docs/org-example.js'
  //           modifiedModelPath='docs/mod-example.js'
  //           original={input}
  //           language={language ? getEditorLangIdFromLanguage(language) : 'js'}
  //           modified={output}
  //           options={{
  //             renderIndicators: true,
  //             renderSideBySide: true,
  //             lineNumbers: 'on',
  //             originalEditable: true,
  //             ...EDITOR_OPTIONS,
  //           }}
  //           placeholderColor='#9ca3af'
  //         />
  //       </div>
  //     </div>
  //     {errorMessage && (
  //       <div
  //         className='animate-slideUp absolute bottom-0 w-full -mx-2 rounded-b-md z-10 bg-tart-600'
  //         role='alert'
  //         data-testid='grit-error'
  //       >
  //         <p className='overflow-ellipsis overflow-hidden line-clamp-4 my-0 py-1 px-4 text-sm text-white font-mono'>
  //           <b>Error:</b> {errorMessage}
  //         </p>
  //       </div>
  //     )}
  //   </div>
  // );
};
