/* tslint:disable */
/* eslint-disable */
/**
* @returns {Promise<void>}
*/
export function initializeTreeSitter(): Promise<void>;
/**
* @param {string} pattern
* @param {(string)[]} paths
* @param {(string)[]} contents
* @param {(string)[]} lib_paths
* @param {(string)[]} lib_contents
* @returns {Promise<any>}
*/
export function parseInputFiles(pattern: string, paths: (string)[], contents: (string)[], lib_paths: (string)[], lib_contents: (string)[]): Promise<any>;
/**
* @param {string} pattern
* @param {(string)[]} paths
* @param {(string)[]} contents
* @param {(string)[]} lib_paths
* @param {(string)[]} lib_contents
* @param {string} llm_api_base
* @param {string} llm_api_bearer_token
* @returns {Promise<any>}
*/
export function matchPattern(pattern: string, paths: (string)[], contents: (string)[], lib_paths: (string)[], lib_contents: (string)[], llm_api_base: string, llm_api_bearer_token: string): Promise<any>;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly initializeTreeSitter: () => number;
  readonly parseInputFiles: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number) => number;
  readonly matchPattern: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number, j: number, k: number, l: number, m: number, n: number) => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6088d10f3dade2dc: (a: number, b: number, c: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h3abfb7d3f6993b4a: (a: number, b: number, c: number, d: number) => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
