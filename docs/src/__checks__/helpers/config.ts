const isPreview =
  process.env.ENVIRONMENT_NAME === 'preview' ||
  process.env.ENVIRONMENT_URL?.includes('.docs.grit.io');

const config = {
  DOCS_TEST_URL: process.env.NEXT_PUBLIC_DOCS_URL!,
  AUTH_SETUP_FILE: 'playwright/.auth/user.json',
  IN_CI: process.env.ENV_NAME === 'ci',
  IN_PREVIEW: isPreview,
};

if (!config.DOCS_TEST_URL || config.DOCS_TEST_URL === '') {
  throw new Error('DOCS_TEST_URL is required');
}

export default config;
