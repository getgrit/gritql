import { Page } from '@playwright/test';

export const inputPattern = async (pattern: string, page: Page) => {
  const editor = page.locator('.monaco-pattern-editor .view-lines');
  await editor.click();
  await editor.press('Control+A');
  await editor.press('Backspace');
  await page.keyboard.type(pattern);
  await page.keyboard.press('Enter');
};
