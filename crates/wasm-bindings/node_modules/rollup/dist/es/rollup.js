/*
  @license
	Rollup.js v4.14.1
	Sun, 07 Apr 2024 07:35:08 GMT - commit 0b665c31833525c923c0fc20f43ebfca748c6670

	https://github.com/rollup/rollup

	Released under the MIT License.
*/
export { version as VERSION, defineConfig, rollup, watch } from './shared/node-entry.js';
import './shared/parseAst.js';
import '../native.js';
import 'node:path';
import 'path';
import 'node:process';
import 'node:perf_hooks';
import 'node:fs/promises';
import 'tty';
