import { Monaco, MonacoDiffEditor } from '@monaco-editor/react';

interface Range {
  start: {
    line: number;
    column: number;
  };
  end: {
    line: number;
    column: number;
  };
}

export interface MatchIndex {
  startIndex: number;
  endIndex: number;
}
