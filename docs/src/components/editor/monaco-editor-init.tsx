import { useEffect } from 'react';
import { useMonaco } from '@monaco-editor/react';

type MonacoEditorInitOptions = {
  theme?: 'light' | 'dark';
};

export const useMonacoEditorInit = ({ theme = 'dark' }: MonacoEditorInitOptions = {}) => {
  const monaco = useMonaco();

  useEffect(() => {
    if (!monaco) return;

    monaco.editor.defineTheme('grit', {
      base: theme === 'dark' ? 'vs-dark' : 'vs',
      inherit: true,
      rules: [],
      colors: {
        'editor.background': theme === 'dark' ? '#1a1a1a' : '#ffffff',
      },
    });
  }, [monaco, theme]);
}; 