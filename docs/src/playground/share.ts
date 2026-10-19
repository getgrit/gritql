export interface PlaygroundState {
  pattern: string;
  input?: string;
}

/** Builds a link that opens the playground with the given pattern and input. */
export function getPlaygroundHref({ pattern, input }: PlaygroundState) {
  const params = new URLSearchParams({ pattern });
  if (input !== undefined) params.set('input', input);
  return `/playground/#${params}`;
}

/** Reads the state encoded by `getPlaygroundHref()` from a URL hash. */
export function readPlaygroundHash(hash: string): Partial<PlaygroundState> {
  const params = new URLSearchParams(hash.replace(/^#/, ''));
  return {
    pattern: params.get('pattern') ?? undefined,
    input: params.get('input') ?? undefined,
  };
}
