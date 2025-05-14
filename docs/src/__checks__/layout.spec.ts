import { expect, test } from '@playwright/test';

import config from './helpers/config';
import { registerHelpers } from './helpers/request';

test.describe('Layout', () => {
  registerHelpers(test);

  test('Shows title', async ({ page }) => {
    await page.goto(`${config.DOCS_TEST_URL}/cli/quickstart`);
    await expect(page.locator('h1').getByText('CLI Quickstart')).toBeVisible();
  });
});
