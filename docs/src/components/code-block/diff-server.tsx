import type { DiffEditorProps } from './diff-editor';
import { DiffEditor } from './diff-editor';

// We need this wrapper to provide a server-side placeholder
export const ServerDiffEditor = (props: DiffEditorProps) => {
  return <DiffEditor {...props} />;
};
