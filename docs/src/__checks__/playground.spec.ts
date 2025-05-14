import { expect, test } from '@playwright/test';

import config from './helpers/config';
import { inputPattern } from './helpers/editor';
import { registerHelpers } from './helpers/request';

test.describe('Playground', () => {
  registerHelpers(test);

  test.beforeEach(async ({ page }) => {
    await page.goto(`${config.DOCS_TEST_URL}/playground`);
  });

  test('Shows and dismisses pattern placeholder', async ({ page }) => {
    const placeholder = page.getByText('Write a custom pattern here');
    await expect(placeholder).toBeVisible({ timeout: 30000 });
    await page.locator('.monaco-pattern-editor').click();
    await page.locator('.monaco-pattern-editor').press('a');
    await expect(placeholder).toBeHidden();
  });

  test('Shows and dismisses input placeholder', async ({ page }) => {
    const placeholder = page
      .locator('.original-in-monaco-diff-editor div')
      .getByText('Paste your input code here');
    await expect(placeholder).toBeVisible({ timeout: 10000 });
    await page.locator('.editor.original').click();
    await page.locator('.editor.original').press('a');
    await expect(placeholder).toBeHidden();
  });

  test('Syntax errors are shown', async ({ page }) => {
    await inputPattern('`a` => nonsense', page);

    await page.locator('.editor.original').click();
    await page.locator('.editor.original').press('a');
    const error = page.getByTestId('grit-error');
    await expect(error).toContainText('Pattern syntax error');
  });

  test.skip('llm_chat is available', async ({ page }) => {
    await inputPattern('`a` => llm_chat(messages=[], pattern=or {`yes`, `no`})', page);

    await page.locator('.editor.original').click();
    await page.locator('.editor.original').press('a');
    const error = page.getByTestId('grit-error');
    await expect(error).toContainText('AI request');
  });

  test('Focuses empty input editor', async ({ page }) => {
    await page
      .locator('.original-in-monaco-diff-editor div')
      .getByText('Paste your input code here')
      .waitFor({ state: 'visible' });
    await page.keyboard.type('console.log("Hello world!")');
    await expect(page.locator('.editor.original')).toContainText('console.log("Hello world!")');
  });
});
