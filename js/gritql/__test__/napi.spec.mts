import { expect, describe, it } from 'bun:test';
import { AsyncLocalStorage } from 'node:async_hooks';

import { QueryBuilder } from '../__generated__/index.js';
import type { RichFile } from '@getgrit/universal';

describe('Node API interfaces ', () => {
  const file: RichFile = {
    path: 'test.js',
    content: `console.log("hello")
console.log("world")`,
  };

  // Broken, see https://github.com/getgrit/rewriter/issues/10170
  it.skip('can run inside a storage context', async () => {
    const testStorage = new AsyncLocalStorage();

    const result = await testStorage.exit(async () => {
      const query = new QueryBuilder(`js"console.log($msg)"`);
      query.filter((log) => {
        log.insertAfter('\n<<inserted>>');
        return true;
      });
      const modified = await query.applyToFile(file);
      expect(modified).toBeDefined();
      expect(modified!.content).toBe(`console.log("hello")
<<inserted>>
console.log("world")
<<inserted>>`);

      return true;
    });
    expect(result).toBeTrue();
  });

  it('can do simple rewrites without callbacks', async () => {
    const testStorage = new AsyncLocalStorage();

    const result = await testStorage.exit(async () => {
      const query = new QueryBuilder(`js"console.log($msg)"`);
      query.setReplacement('console.log(REPLACED)');
      const modified = await query.applyToFile(file);
      expect(modified).toBeDefined();
      expect(modified!.content).toBe(`console.log(REPLACED)
console.log(REPLACED)`);

      return true;
    });
    expect(result).toBeTrue();
  });

  it('can do simple inserts without callbacks', async () => {
    const testStorage = new AsyncLocalStorage();

    const result = await testStorage.exit(async () => {
      const query = new QueryBuilder(`js"console.log($msg)"`);
      query.addInsertion('\n<<inserted>>');
      const modified = await query.applyToFile(file);
      expect(modified).toBeDefined();
      expect(modified!.content).toBe(`console.log("hello")
<<inserted>>
console.log("world")
<<inserted>>`);

      return true;
    });
    expect(result).toBeTrue();
  });
});
