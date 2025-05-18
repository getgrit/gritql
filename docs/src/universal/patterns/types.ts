export type FilePath = string;

// Collect files to be transferred to the engine for parsing
export type GritFiles = Record<FilePath, string>;

export type FilePin = {
  name?: string;
  filePath: string;
  fileContents: undefined | string;
  rewrittenContents: undefined | string;
};

export type Repo = {
  owner: string;
  name: string;
};

export function makeRepo(fullName: string, host: string) {
  const nameParts = fullName.split('/');
  return {
    owner: nameParts[0]!,
    name: nameParts.slice(1).join('/'),
  };
}

export type RefInput = {
  sha?: string;
  branch?: string;
};

export type RunnableRef = {
  repo?: Repo;
  ref?: RefInput;
};

export type LivePattern = {
  globs: string;
  patternBody: string;
  isInitialized: boolean;
};

export type LiveData = {
  pins: FilePin[];
  pattern: LivePattern;
}; 