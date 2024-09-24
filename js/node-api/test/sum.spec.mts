import { expect, describe, it } from 'bun:test';

import { sum } from '../__generated__/index.js';

describe('Node bindings', () => {
  it('work with a basic function', async () => {
    const result = sum(1, 2);
    expect(result).toBe(3);
  });
});
