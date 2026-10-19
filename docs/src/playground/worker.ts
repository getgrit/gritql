/// <reference lib="webworker" />

import init, { matchPattern, parseInputFiles } from 'grit-wasm-bindings';
import { getQuickJS, getQuickJSSync, shouldInterruptAfterDeadline } from 'quickjs-emscripten';
import TreeSitter from 'web-tree-sitter';

import {
  type AnalysisLog,
  type AnalyzerRequest,
  isAnalysisLog,
  type MatchResult,
  PLAYGROUND_PATTERN_FILE,
  type WorkerRequest,
  type WorkerResponse,
} from './types';

declare const self: DedicatedWorkerGlobalScope;

// The generated bindings call these globals to run JavaScript functions and AI requests.
Object.assign(globalThis, {
  gritExternalFunctionCall(code: string, params: string[], paramInputs: string[]): string {
    const runtime = getQuickJSSync().newRuntime();
    runtime.setInterruptHandler(shouldInterruptAfterDeadline(Date.now() + 1000));
    const vm = runtime.newContext();
    try {
      // Based on https://github.com/getgrit/gritql/blob/29dcd72d9eba0b37256d8afdd814a4baf57d3d17/crates/externals/src/static/sandbox.js#L4
      params.forEach((param, i) => {
        const handle = vm.newObject();
        vm.setProp(handle, 'text', vm.newString(paramInputs[i] ?? ''));
        vm.setProp(vm.global, param, handle);
        handle.dispose();
      });

      const result = vm.evalCode(`(function() { ${code} })()`);
      if (result.error) {
        const error = vm.dump(result.error);
        result.error.dispose();
        throw error;
      }
      const value = vm.dump(result.value);
      result.value.dispose();
      try {
        return value.toString();
      } catch {
        throw new Error(`Failed to serialize function return value: ${value}`);
      }
    } catch (e) {
      console.error('External function call failed', e instanceof Error ? e.message : e);
      throw e;
    } finally {
      vm.dispose();
      runtime.dispose();
    }
  },

  gritApiRequest(url: string, headers: string, body: string): string {
    // The bindings expect a synchronous call.
    const xhr = new XMLHttpRequest();
    xhr.open('POST', url, false);
    for (const [key, value] of Object.entries<string>(JSON.parse(headers))) {
      xhr.setRequestHeader(key, value);
    }
    xhr.send(body);
    if (xhr.status !== 200) {
      throw new Error(`Invalid status code: ${xhr.status}`);
    }
    return xhr.responseText;
  },
});

let ready: Promise<void> | undefined;

function initialize() {
  ready ??= (async () => {
    await init();
    // The generated bindings expect these in the global scope.
    Object.assign(globalThis, { Parser: TreeSitter, Language: TreeSitter.Language });
    await TreeSitter.init({
      // Served from `public/`, like the language parsers in `public/wasm_parsers/`.
      locateFile: (scriptName: string) => `/${scriptName}`,
    });
    await getQuickJS();
  })().catch((e) => {
    // Allow the next request to retry.
    ready = undefined;
    throw e;
  });
  return ready;
}

async function analyze(
  { command, pattern, file_paths, file_contents, lib_paths, lib_contents }: AnalyzerRequest,
  apiEndpoint = '',
  apiKey = '',
): Promise<MatchResult[]> {
  try {
    await initialize();
    const results: MatchResult[] =
      command === 'match'
        ? await matchPattern(
            pattern,
            file_paths,
            file_contents,
            lib_paths,
            lib_contents,
            apiEndpoint,
            apiKey,
          )
        : await parseInputFiles(pattern, file_paths, file_contents, lib_paths, lib_contents);
    return results.map((result) =>
      isAnalysisLog(result) && result.file === '' ? { ...result, file: PLAYGROUND_PATTERN_FILE } : result,
    );
  } catch (e) {
    console.error('Error processing analysis', e);
    const log: AnalysisLog = {
      __typename: 'AnalysisLog',
      level: 300,
      message: 'Unknown error',
      file: PLAYGROUND_PATTERN_FILE,
    };
    return [log];
  }
}

self.addEventListener('message', async (event: MessageEvent<WorkerRequest>) => {
  const { id, request, api_key, api_endpoint } = event.data;
  const data = await analyze(request, api_endpoint, api_key);
  self.postMessage({ id, data } satisfies WorkerResponse);
});
