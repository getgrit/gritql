import { useEffect, useRef, useState } from 'react';

import {
  extractPath,
  isAnalysisLog,
  isResult,
  type MatchResult,
  type Result,
  type WorkerRequest,
  type WorkerResponse,
} from './types';

/** The playground runs patterns against a single file. */
const INPUT_FILE = 'test.js';

export type Analysis =
  | { status: 'loading' }
  | { status: 'loaded'; result: Result }
  | { status: 'empty' }
  | { status: 'error'; message: string };

function toAnalysis(results: MatchResult[]): Analysis {
  const result = results.find((r): r is Result => isResult(r) && extractPath(r) === INPUT_FILE);
  if (result) return { status: 'loaded', result };
  // Logs that are not tied to a file are informational.
  const log = results.find((r) => isAnalysisLog(r) && !!extractPath(r));
  if (log && isAnalysisLog(log)) return { status: 'error', message: log.message };
  return { status: 'empty' };
}

/**
 * Runs `pattern` against `input` in a web worker whenever either changes. While a run is pending,
 * the previous analysis is kept so the editors don't flash.
 */
export function useGritAnalysis(pattern: string, input: string) {
  const workerRef = useRef<Worker | null>(null);
  const callbacks = useRef(new Map<number, (results: MatchResult[]) => void>());
  const nextId = useRef(0);
  const [analysis, setAnalysis] = useState<Analysis>({ status: 'loading' });
  const [pending, setPending] = useState(true);

  // Declared before the effect below so the worker exists when the first run is requested.
  useEffect(() => {
    const worker = new Worker(new URL('./worker.ts', import.meta.url), { type: 'module' });
    const pendingCallbacks = callbacks.current;
    worker.addEventListener('message', (event: MessageEvent<WorkerResponse>) => {
      const { id, data } = event.data;
      pendingCallbacks.get(id)?.(data);
      pendingCallbacks.delete(id);
    });
    // Fired when the worker script itself fails to load, which would otherwise go unnoticed.
    worker.addEventListener('error', (event) => {
      console.error('The GritQL worker failed to load', event);
      setAnalysis({ status: 'error', message: 'Failed to load the GritQL engine.' });
      setPending(false);
    });
    workerRef.current = worker;
    return () => {
      worker.terminate();
      workerRef.current = null;
      pendingCallbacks.clear();
    };
  }, []);

  useEffect(() => {
    const worker = workerRef.current;
    if (!worker) return;

    let cancelled = false;
    const id = nextId.current++;
    callbacks.current.set(id, (results) => {
      if (cancelled) return;
      setAnalysis(toAnalysis(results));
      setPending(false);
    });
    worker.postMessage({
      id,
      request: {
        command: 'match',
        pattern,
        file_paths: [INPUT_FILE],
        file_contents: [input],
        lib_paths: [],
        lib_contents: [],
      },
    } satisfies WorkerRequest);
    setPending(true);

    return () => {
      cancelled = true;
    };
  }, [pattern, input]);

  return { analysis, pending };
}
