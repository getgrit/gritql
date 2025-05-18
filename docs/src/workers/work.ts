/* eslint-disable no-restricted-globals */
/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable no-console */

import init, { matchPattern, parseInputFiles } from 'grit-wasm-bindings';
import { getQuickJS, getQuickJSSync, shouldInterruptAfterDeadline } from 'quickjs-emscripten';
import TreeSitter from 'web-tree-sitter';

import { AnalyzerData } from '@/components/editor/wasm-provider';
import {
  exhaustive,
  ImplicitFile,
  isAnalysisLog,
  makeAnalysisLog,
  MatchResult,
} from '../universal';

// We need to prep quickJS before we can use it
// But the core of the worker is
async function prepQuickJS() {
  await getQuickJS();
}

(globalThis as any).gritExternalFunctionCall = (
  code: string,
  params: string[],
  param_inputs: string[],
): string => {
  let runtime: any;
  let vm: any;
  try {
    const QuickJS = getQuickJSSync();

    runtime = QuickJS.newRuntime();
    runtime.setInterruptHandler(shouldInterruptAfterDeadline(Date.now() + 1000));

    vm = runtime.newContext();

    // This part based on https://github.com/getgrit/gritql/blob/29dcd72d9eba0b37256d8afdd814a4baf57d3d17/crates/externals/src/static/sandbox.js#L4
    for (let i = 0; i < params.length; i++) {
      const handle = vm.newObject();
      vm.setProp(handle, 'text', vm.newString(param_inputs[i]!));
      vm.setProp(vm.global, params[i]!, handle);
      handle.dispose();
    }

    // Wrap the user code in an IIFE
    const wrappedCode = `(function() { ${code} })()`;

    const result = vm.evalCode(wrappedCode);
    if (result.error) {
      const error = vm.dump(result.error);
      result.error.dispose();
      throw error;
    } else {
      const value = vm.dump(result.value);
      result.value.dispose();
      try {
        return value.toString();
      } catch (e) {
        throw new Error('Failed to serialize function return value: ' + value);
      }
    }
  } catch (e: any) {
    // grit-ignore
    console.error('External function call failed', e.message);
    throw e;
  } finally {
    if (vm) vm.dispose();
    if (runtime) runtime.dispose();
  }
};

(globalThis as any).gritApiRequest = (url: string, headers: string, body: string): string => {
  try {
    var xhr = new XMLHttpRequest();
    // You *must* open it first
    xhr.open('POST', url, false); // false for synchronous request

    const headersObj = JSON.parse(headers);
    for (const key in headersObj) {
      xhr.setRequestHeader(key, headersObj[key]);
    }

    xhr.send(body);
    if (xhr.status != 200) {
      throw new Error(`Invalid status code: ${xhr.status}`);
    }
    try {
      const response = xhr.responseText;
      return response;
    } catch (e) {
      throw new Error('Failed to parse JSON response: ' + xhr.responseText);
    }
  } catch (e: any) {
    console.error('LLM API request failed', e);
    throw e;
  }
};

type AnalyzerEvent = {
  data: {
    id: string;
    request: AnalyzerData;
    api_key?: string;
    api_endpoint?: string;
  };
};

async function processAnalysis(
  { command, pattern, file_paths, file_contents, lib_paths, lib_contents }: AnalyzerData,
  api_endpoint?: string,
  api_key?: string,
) {
  try {
    await init();
    // @ts-expect-error The generated bindings insert these into the global scope
    globalThis.Parser = TreeSitter;
    // @ts-expect-error The generated bindings insert these into the global scope
    globalThis.Language = TreeSitter.Language;
    await TreeSitter.init({
      locateFile(scriptName: string, _scriptDirectory: string) {
        return `/${scriptName}`;
      },
    });
    await prepQuickJS();
    const results: MatchResult[] =
      command === 'match'
        ? await matchPattern(
            pattern,
            file_paths,
            file_contents,
            lib_paths,
            lib_contents,
            api_endpoint ?? '',
            api_key ?? '',
          )
        : command === 'parse'
          ? await parseInputFiles(pattern, file_paths, file_contents, lib_paths, lib_contents)
          : exhaustive(command);
    return results.map((m) => {
      if (isAnalysisLog(m) && m.file === '') {
        m.file = ImplicitFile.PlaygroundPattern;
      }
      return m;
    });
  } catch (e: any) {
    // grit-ignore no_console_error: This is the best way to log errors in workers
    console.error('Error processing analysis', e);
    return [
      // @ts-expect-error makeAnalysisLog is not typed
      makeAnalysisLog({
        message: 'Unknown error',
        file: ImplicitFile.PlaygroundPattern,
      }),
    ];
  }
}

addEventListener('message', (event: AnalyzerEvent) => {
  (async () => {
    const { id, request, api_key, api_endpoint } = event.data;
    const results = await processAnalysis(request, api_endpoint, api_key);

    self.postMessage({ id, data: results });
  })();
});

export {};
