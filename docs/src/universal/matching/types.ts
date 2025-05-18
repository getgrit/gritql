const isObject = (obj: unknown): obj is Record<string, unknown> => {
  return typeof obj === 'object' && obj !== null;
};

export type Language = string;

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
  range: Range;
  sourceFile: string;
  message?: string;
};

export type Rewrite = {
  __typename: 'Rewrite';
  original: {
    sourceFile: string;
    range: Range;
  };
  rewritten: {
    sourceFile: string;
    range: Range;
  };
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
};

export type InputFile = {
  __typename: 'InputFile';
  sourceFile: string;
};

export type PatternInfo = {
  __typename: 'PatternInfo';
  name: string;
  description?: string;
};

export type DoneFile = {
  __typename: 'DoneFile';
  sourceFile: string;
};

export type AllDone = {
  __typename: 'AllDone';
};

export type MatchResult = Match | Rewrite | CreateFile | RemoveFile | AnalysisLog | InputFile | PatternInfo | DoneFile | AllDone;

export type Result = Match | Rewrite | CreateFile | RemoveFile;

export type AmbiguousResult = MatchResult | Result;

export const isMatch = (obj: AmbiguousResult): obj is Match => {
  return obj.__typename === 'Match';
};

export const isRewrite = (obj: AmbiguousResult): obj is Rewrite => {
  return obj.__typename === 'Rewrite';
};

export const isInputFile = (obj: AmbiguousResult): obj is InputFile => {
  return obj.__typename === 'InputFile';
};

export const isCreateFile = (obj: AmbiguousResult): obj is CreateFile => {
  return obj.__typename === 'CreateFile';
};

export const isRemoveFile = (obj: AmbiguousResult): obj is RemoveFile => {
  return obj.__typename === 'RemoveFile';
};

export const isLog = (obj: AmbiguousResult): obj is AnalysisLog => {
  return obj.__typename === 'AnalysisLog';
};

export const isAnalysisLog = (obj: AmbiguousResult): obj is AnalysisLog => {
  return isLog(obj);
};

export const isUserLog = (obj: AnalysisLog): obj is AnalysisLog & { level: 441 } => {
  return obj.level === 441;
};

export const isErrorLog = (log: AmbiguousResult): log is AnalysisLog => {
  return isLog(log) && (log?.level || 0) < 300;
};

export function isResult(obj: MatchResult): obj is Result {
  return isMatch(obj) || isRewrite(obj) || isCreateFile(obj) || isRemoveFile(obj);
}

export function isMatchResult(obj: unknown): obj is MatchResult {
  if (!isObject(obj) || obj === null || obj === undefined) {
    return false;
  }
  const typename = (obj as { __typename?: string }).__typename;
  if (!typename) {
    return false;
  }
  const validTypes = [
    'Match',
    'Rewrite',
    'CreateFile',
    'RemoveFile',
    'AnalysisLog',
    'InputFile',
    'PatternInfo',
    'DoneFile',
    'AllDone',
  ];
  return validTypes.includes(typename);
}

export const isPatternInfo = (obj: AmbiguousResult): obj is PatternInfo => {
  return obj.__typename === 'PatternInfo';
};

export const isDoneFile = (obj: AmbiguousResult): obj is DoneFile => {
  return obj.__typename === 'DoneFile';
};

export const isAllDone = (obj: MatchResult): obj is AllDone => {
  return obj.__typename === 'AllDone';
};

export const isLanguage = (lang: any): lang is Language => {
  return typeof lang === 'string';
};

export type FileResultMessage = {
  type: 'file';
  file: RichFile;
};

export type PatternResultMessage = {
  type: 'pattern';
  pattern: string;
};

export type RichFile = {
  path: string;
  content: string;
  language?: string;
};

export type ImplicitFile = {
  path: string;
  content: string;
};

export const makeAnalysisLog = (message: string, level: number = 300): AnalysisLog => ({
  __typename: 'AnalysisLog',
  level,
  message,
});
