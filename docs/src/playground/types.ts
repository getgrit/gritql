// Subset of the result types serialized by `grit-wasm-bindings`.

export type Position = {
  line: number;
  column: number;
};

export type Range = {
  start: Position;
  end: Position;
};

export type Match = {
  __typename: 'Match';
  ranges: Range[];
  sourceFile: string;
  message?: string;
};

export type Rewrite = {
  __typename: 'Rewrite';
  original: { sourceFile: string; content: string };
  rewritten: { sourceFile: string; content: string };
  message?: string;
};

export type CreateFile = {
  __typename: 'CreateFile';
  sourceFile: string;
  message?: string;
};

export type RemoveFile = {
  __typename: 'RemoveFile';
  sourceFile: string;
  message?: string;
};

export type AnalysisLog = {
  __typename: 'AnalysisLog';
  level: number;
  message: string;
  file?: string;
};

type OtherResult =
  | { __typename: 'InputFile'; sourceFile: string }
  | { __typename: 'PatternInfo'; name: string }
  | { __typename: 'DoneFile'; sourceFile: string }
  | { __typename: 'AllDone' };

export type Result = Match | Rewrite | CreateFile | RemoveFile;

export type MatchResult = Result | AnalysisLog | OtherResult;

export const isMatch = (result: MatchResult): result is Match => result.__typename === 'Match';

export const isRewrite = (result: MatchResult): result is Rewrite =>
  result.__typename === 'Rewrite';

export const isAnalysisLog = (result: MatchResult): result is AnalysisLog =>
  result.__typename === 'AnalysisLog';

export const isResult = (result: MatchResult): result is Result =>
  isMatch(result) ||
  isRewrite(result) ||
  result.__typename === 'CreateFile' ||
  result.__typename === 'RemoveFile';

/** The file a result applies to, if any. */
export const extractPath = (result: MatchResult) => {
  if (isRewrite(result)) return result.original.sourceFile;
  if (isAnalysisLog(result)) return result.file;
  if ('sourceFile' in result) return result.sourceFile;
  return undefined;
};

/** Logs about the pattern itself are reported against this file. */
export const PLAYGROUND_PATTERN_FILE = 'playground-pattern';

export interface AnalyzerRequest {
  command: 'parse' | 'match';
  pattern: string;
  file_paths: string[];
  file_contents: string[];
  lib_paths: string[];
  lib_contents: string[];
}

export interface WorkerRequest {
  id: number;
  request: AnalyzerRequest;
  api_key?: string;
  api_endpoint?: string;
}

export interface WorkerResponse {
  id: number;
  data: MatchResult[];
}
