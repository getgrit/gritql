import { expect, describe, it } from 'bun:test';

import { QueryBuilder } from '../__generated__/index.js';

describe('GritQL apply bindings', () => {
  const file = {
    path: 'test.js',
    content: `console.log("hello")
console.log("world")`,
  };

  it('can apply a rewrite in a synthetic file', async () => {
    const query = new QueryBuilder(`js"console.log($msg)" => js"console.error($msg)"`);
    const modified = await query.applyToFile(file);
    expect(modified).toBeDefined();
    expect(modified!.content).toBe(`console.error("hello")
console.error("world")`);
  });

  it('can apply a rewrite with a pattern', async () => {
    const query = new QueryBuilder(`import_statement() as $candidate where {
    $candidate <: within program($statements),
    $statements <: some bubble($last_import) import_statement() as $this_import where $last_import = $this_import,
    $candidate <: $last_import
}`);
    query.filter((arg: any) => {
      arg.insertAfter(`<<<INSERT HERE>>>\n`);
      return true;
    });
    const modified = await query.applyToFile({
      path: 'test.js',
      content: `import { foo } from 'bar';
import { baz } from 'qux';
import { quux } from 'corge';

function foo() {
  console.log('foo');
}`,
    });
    console.log(modified);
    expect(modified).toBeDefined();
    expect(modified!.content).toBe(`import { foo } from 'bar';
import { baz } from 'qux';
import { quux } from 'corge';
<<<INSERT HERE>>>


function foo() {
  console.log('foo');
}`);
  });

  it('can apply a rewrite via a callback', async () => {
    const query = new QueryBuilder(`js"console.log($msg)"`);
    query.filter((arg) => {
      arg.insertAfter('\n^^^inserted^^^');
      return true;
    });
    const modified = await query.applyToFile(file);
    expect(modified).toBeDefined();
    expect(modified!.content).toBe(
      `console.log("hello")
^^^inserted^^^
console.log("world")
^^^inserted^^^`,
    );
  });

  it('returns null if no matches', async () => {
    const query = new QueryBuilder(`js"console.loud($msg)" => js"console.error($msg)"`);
    const modified = await query.applyToFile(file);
    expect(modified).toBeNull();
  });
});
