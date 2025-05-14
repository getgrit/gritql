import { Page, test } from '@playwright/test';

const BLOCKED_REQUESTS = [
  'google',
  'analytics',
  'fonts',
  'posthog',
  'segment',
  'heapanalytics',
  'sentry',
];

const initTest = async (page: Page) => {
  return page.route('**/*', (route) => {
    const requestUrl = route.request().url();
    if (BLOCKED_REQUESTS.some((blocked) => requestUrl.includes(blocked))) {
      route.abort();
    } else {
      route.continue();
    }
  });
};

export const registerHelpers = (testRegister: typeof test) => {
  testRegister.beforeEach(async ({ page }) => {
    await initTest(page);
  });
};
