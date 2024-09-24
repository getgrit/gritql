import { expect, describe, it } from 'bun:test';

const { UncompiledPatternBuilder } = require('../__generated__/grit-node-api.darwin-arm64.node');

describe('Grit builder bindings', () => {
  it.skip('can save text from the repo', async () => {
    console.log('class', UncompiledPatternBuilder, UncompiledPatternBuilder.new_snippet);
    // const thing = new UncompiledPatternBuilder();
    // const query = UncompiledPatternBuilder.new_snippet('console.log');
    // const result = await query.runOnFiles([
    //   { path: 'test/file1.js', content: 'console.log("hello")' },
    //   { path: 'test/file2.js', content: 'throw new Error("oh no")' },
    // ]);
    // expect(result).toBe(3);
    // const result2 = await query.runOnFiles([
    //   { path: 'test/file1.js', content: 'console.error("hello")' },
    //   { path: 'test/file2.js', content: 'throw new Error("oh no")' },
    // ]);
    // expect(result2).toBe(2);
  });

  //   it('can use nested contains', async () => {
  //     const query = UncompiledPatternBuilder.new_snippet('console.log').contains(
  //       UncompiledPatternBuilder.new_snippet('log'),
  //     );
  //     const result = await query.runOnFiles([
  //       { path: 'test', content: 'console.log("hello")' },
  //       { path: 'not_a_file', content: 'throw new Error("oh no")' },
  //     ]);
  //     expect(result).toBe(3);
  //     // This one shoud not match, because there is not a fun inside;
  //     const result2 = await UncompiledPatternBuilder.new_snippet('console.log($_)')
  //       .contains(UncompiledPatternBuilder.new_snippet('fun'))
  //       .runOnFiles([
  //         { path: 'test', content: 'console.log(food); fun' },
  //         { path: 'not_a_file', content: 'throw new Error("oh no")' },
  //       ]);
  //     expect(result2).toBe(2);
  //   });

  //   it('can use a callback', async () => {
  //     let callbackCalledCounter = {
  //       value: 0,
  //     };
  //     const query = UncompiledPatternBuilder.new_snippet('console.log').filter((data) => {
  //       console.log('The callback was called');
  //       callbackCalledCounter.value++;
  //       return true;
  //     });
  //     const result = await query.runOnFiles([
  //       {
  //         path: 'test',
  //         content: 'function foo() { console.log("hello"); console.log("world"); } foo();',
  //       },
  //       { path: 'not_a_file', content: 'throw new Error("oh no")' },
  //     ]);
  //     expect(result).toBe(3);
  //     expect(callbackCalledCounter.value).toBe(2);
  //   });

  //   it('can apply a rewrite with a pattern', async () => {
  //     const query = UncompiledPatternBuilder.new_snippet('import $_').filter((arg: any) => {
  //       arg.append(`<<<INSERT HERE>>>\n`);
  //       return true;
  //     });

  //     const modified = await query.runOnFile({
  //       path: 'test.js',
  //       content: `import { foo } from 'bar';
  // import { baz } from 'qux';
  // import { quux } from 'corge';

  // function foo() {
  //   console.log('foo');
  // }`,
  //     });
  //     expect(modified).toBeDefined();
  //     expect(modified!.content).toBe(`import { foo } from 'bar';
  // <<<INSERT HERE>>>

  // import { baz } from 'qux';
  // <<<INSERT HERE>>>

  // import { quux } from 'corge';
  // <<<INSERT HERE>>>

  // function foo() {
  //   console.log('foo');
  // }`);
  //   });

  //   it('can filter content correctly in callbacks', async () => {
  //     const query = UncompiledPatternBuilder.new_snippet('import $_')
  //       .filter((arg: any) => {
  //         const text = arg.text();
  //         if (text.includes('baz')) {
  //           return true;
  //         }
  //         return false;
  //       })
  //       .filter((arg: any) => {
  //         arg.append(`<<<INSERT HERE>>>\n`);
  //         return true;
  //       });

  //     const modified = await query.runOnFile({
  //       path: 'test.js',
  //       content: `import { foo } from 'bar';
  // import { baz } from 'qux';
  // import { quux } from 'corge';

  // function foo() {
  //   console.log('foo');
  // }`,
  //     });
  //     expect(modified).toBeDefined();
  //     expect(modified!.content).toBe(`import { foo } from 'bar';
  // import { baz } from 'qux';
  // <<<INSERT HERE>>>

  // import { quux } from 'corge';

  // function foo() {
  //   console.log('foo');
  // }`);
  //   });

  //   it('can find variables inside the context ', async () => {
  //     const query = UncompiledPatternBuilder.new_snippet('import { $items } from "$source"')
  //       .filter((arg: any) => {
  //         const source = arg.findVarText('$source');
  //         if (source === 'qux') {
  //           return true;
  //         }
  //         return false;
  //       })
  //       .filter((arg: any) => {
  //         arg.append(`<<<INSERT HERE>>>\n`);
  //         return true;
  //       });

  //     const modified = await query.runOnFile({
  //       path: 'test.js',
  //       content: `import { foo } from 'bar';
  // import { baz } from 'qux';
  // import { quux } from 'corge';

  // function foo() {
  //   console.log('foo');
  // }`,
  //     });
  //     expect(modified).toBeDefined();
  //     expect(modified!.content).toBe(`import { foo } from 'bar';
  // import { baz } from 'qux';
  // <<<INSERT HERE>>>

  // import { quux } from 'corge';

  // function foo() {
  //   console.log('foo');
  // }`);
  //   });
});
