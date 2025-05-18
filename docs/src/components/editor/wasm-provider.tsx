import { PropsWithChildren, useCallback, useMemo, useState } from 'react';

import {
  FileResultMessage,
  ImplicitFile,
  MatchResult,
  PatternResultMessage,
  RichFile,
  extractLanguageFromPatternBody,
  extractPath,
  isAllDone,
  isPatternInfo,
  makeAnalysisLog,
} from '../../universal';

export interface AnalyzerData {
  command: 'parse' | 'match';
  pattern: string;
  file_paths: string[];
  file_contents: string[];
  lib_paths: string[];
  lib_contents: string[];
}

interface AnalyzerInput {
  analyze: (data: AnalyzerData) => Promise<MatchResult[]>;
}

export const WasmProvider: React.FC<PropsWithChildren<AnalyzerInput>> = ({ children, analyze }) => {
  const [parseResults, setParseResults] = useState<FileResultMessage[]>([]);
  const [analyzeResults, setAnalyzeResults] = useState<FileResultMessage[]>([]);
  const [patternInfo, setPatternInfo] = useState<PatternResultMessage>();
  const [dispatched, setDispatched] = useState<{ pattern: string; file: RichFile }[]>([]);

  const reset = useCallback((excludeFilePaths?: string[]) => {
    if (excludeFilePaths) {
      setParseResults((prev) => prev.filter((r) => excludeFilePaths.includes(r.filePath)));
      setAnalyzeResults((prev) => prev.filter((r) => excludeFilePaths.includes(r.filePath)));
    } else {
      setParseResults([]);
      setAnalyzeResults([]);
    }
    setPatternInfo(undefined);
  }, []);

  const rawAnalyzeFiles = useCallback(
    async (files: RichFile[], pattern: string, justParse: boolean) => {
      const originalContentsByPath = new Map(files.map((f) => [f.path, f.content]));
      const wrapResult = (result: MatchResult): FileResultMessage => {
        const filePath = extractPath(result);
        return {
          filePath: filePath ?? ImplicitFile.PlaygroundPattern,
          originalContent: filePath ? originalContentsByPath.get(filePath) : undefined,
          result,
          pattern,
        };
      };
      const language = extractLanguageFromPatternBody(pattern, 'JS');
      const inputs = {
        pattern,
        file_paths: files.map((f) => f.path),
        file_contents: files.map((f) => f.content),
        lib_paths: [],
        lib_contents: [],
      };
      try {
        const promises = [
          analyze({
            command: 'parse',
            ...inputs,
          }).then((r) => updateFileResults(r, pattern, wrapResult, 'parse')),
        ];
        if (!justParse) {
          promises.push(
            analyze({
              command: 'match',
              ...inputs,
            }).then((r) => updateFileResults(r, pattern, wrapResult, 'match')),
          );
          setDispatched(files.map((f) => ({ pattern, file: f })));
        }
        await Promise.all(promises);
      } catch (e: any) {
        console.error(e);
        setAnalyzeResults([
          wrapResult(
            makeAnalysisLog({
              message: e.message ?? 'Unknown error',
              file: ImplicitFile.PlaygroundPattern,
              level: 'error',
            }),
          ),
        ]);
        return;
      }
    },
    [],
  );

  const updateFileResults = useCallback(
    (
      results: MatchResult[],
      pattern: string,
      wrapResult: (result: MatchResult) => FileResultMessage,
      command: AnalyzerData['command'],
    ) => {
      const ourResults = [];
      for (const result of results) {
        const filePath = extractPath(result);
        if (isAllDone(result) || !filePath) {
          continue;
        }
        if (isPatternInfo(result)) {
          setPatternInfo({
            pattern,
            result,
          });
          continue;
        } else {
          const wrapped = wrapResult(result);
          ourResults.push(wrapped);
        }
      }
      if (command === 'match') {
        setAnalyzeResults(ourResults);
      } else {
        setParseResults(ourResults);
      }
    },
    [],
  );

  const fileResults = useMemo(() => {
    return [...parseResults, ...analyzeResults];
  }, [parseResults, analyzeResults]);

  const analyzer = {
    analyzeFiles: rawAnalyzeFiles,
    fileResults,
    patternInfo,
    kind: 'wasm' as const,
    reset,
    dispatched,
  };

  return <AnalyzerContext.Provider value={analyzer}>{children}</AnalyzerContext.Provider>;
}; 