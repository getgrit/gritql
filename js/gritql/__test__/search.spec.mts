import { expect, describe, it, mock } from 'bun:test';

import path from 'path';

import { QueryBuilder } from '../__generated__/index.js';

// These appear to be broken in CI
describe('GritQL bindings', () => {
  it.skip('can do a basic search on the rewriter repo', async () => {
    const fixtureDir = path.resolve(__dirname, '../../../');
    const query = new QueryBuilder(`js"console.log($_)" as $match where {
    $msg = $match,
    $something = length($msg),
    $something <: true
}`);

    const mockOne = mock();
    const mockTwo = mock();

    query.filter((arg) => {
      try {
        mockOne(arg);
        const length = arg.text().length;
        const isOdd = length % 2 === 1;
        return isOdd;
      } catch (e) {
        console.trace(e);
        return false;
      }
    });

    query.filter((arg) => {
      try {
        mockTwo(arg);
        return arg.text().length > 19;
      } catch (e) {
        console.trace(e);
        return false;
      }
    });

    const files = await query.run({ targetPaths: [fixtureDir], stepId: 'test' });
    const total = files.length;
    expect(total).toBeGreaterThan(50);

    // Filter one must be called 10 times
    const calls = mockOne.mock.calls.length;
    expect(calls).toBeGreaterThan(100);

    // Filter two must be called 91 times
    const callsTwo = mockTwo.mock.calls.length;
    expect(callsTwo).toBeGreaterThan(100);
    expect(callsTwo).toBeLessThan(calls);
  });

  it(
    'can save text from the repo',
    async () => {
      const fixtureDir = path.resolve(__dirname, '../../../');
      const query = new QueryBuilder(`js"console.log($_)"`);

      const allTextMessages: string[] = [];

      query.filter((arg) => {
        const text = arg.text();
        // console.log(`pushed ${text}`);
        allTextMessages.push(text);
        return true;
      });

      await query.run({ targetPaths: [fixtureDir], stepId: 'test' });

      expect(allTextMessages.length).toBeGreaterThan(40);
    },
    {
      timeout: 20000,
    },
  );
});
