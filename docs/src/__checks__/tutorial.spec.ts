import { expect, Page, test } from '@playwright/test';

import config from './helpers/config';
import { inputPattern } from './helpers/editor';
import { registerHelpers } from './helpers/request';

const OUTPUT_LOCATOR = '.modified-in-monaco-diff-editor .lines-content';

const runLastPattern = async (page: Page) => {
  const runPattern = page.locator('text=Run Pattern').last();
  await runPattern.waitFor({ state: 'visible' });
  await runPattern.click();
  await page.locator(OUTPUT_LOCATOR).getByText('winston.info(42)').waitFor({ state: 'visible' });
};

test.describe('Tutorial', () => {
  registerHelpers(test);

  test.beforeEach(async ({ page }) => {
    await page.goto(`${config.DOCS_TEST_URL}/tutorials/gritql`);
  });

  test('Shows tutorial snippet input', async ({ page }) => {
    await page.locator('text=Run Pattern').first().click();
    await expect(page.locator('.original-in-monaco-diff-editor')).toContainText(
      'console.log("This message is different");',
    );
  });

  test('Runs a rewrite', async ({ page }) => {
    const runPattern = page.locator('text=Run Pattern').last();
    await runPattern.waitFor({ state: 'visible' });
    // We have a bug where clicking "Run Pattern" after two seconds results in an empty pattern editor
    await page.waitForTimeout(2000);
    await runPattern.click();
    const editorPattern = page
      .locator('.monaco-pattern-editor')
      .getByText('`console.log($my_message)`');
    await expect(editorPattern).toBeVisible();
    const rewritten = page.locator('text=winston.info(42)');
    await rewritten.waitFor({ state: 'visible' });
  });

  test('Displays a pattern error', async ({ page }) => {
    await runLastPattern(page);
    await inputPattern(`\`\` =`, page);
    const error = page.locator(':has-text("Pattern syntax error")').first();
    await expect(error).toBeVisible();
  });

  test('Does not error on user log', async ({ page }) => {
    await runLastPattern(page);
    await inputPattern(`\`console.log("Hello world!")\` where log(variable=$program)`, page);
    const matchHighlight = page.locator('.match-highlight').first();
    await matchHighlight.waitFor({ state: 'visible' });
    const error = page.locator(':has-text("Error:")').first();
    await expect(error).not.toBeVisible();
  });

  test('Can rewrite to empty output', async ({ page }) => {
    await runLastPattern(page);
    await inputPattern('program() => .', page);
    await expect(page.locator(OUTPUT_LOCATOR, { hasNotText: 'main' })).toBeVisible();
    expect((await page.locator(OUTPUT_LOCATOR).allInnerTexts()).join('')).toBe('');
  });

  test('Shows overlay when no results found', async ({ page }) => {
    await runLastPattern(page);
    await inputPattern('`console.info` => .', page);
    const overlay = await page
      .locator('.is-dirty.empty .modified-in-monaco-diff-editor')
      .evaluate((el) => window.getComputedStyle(el, ':after').content);
    expect(overlay).toBe('"No results found"');
  });

  test('Shows overlay on error', async ({ page }) => {
    await runLastPattern(page);
    await inputPattern(`\`\` =`, page);
    const overlay = await page
      .locator('.is-dirty.error .modified-in-monaco-diff-editor')
      .evaluate((el) => window.getComputedStyle(el, ':after').content);
    expect(overlay).toBe('"Error!"');
  });
});
