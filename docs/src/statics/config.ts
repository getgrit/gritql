const config = {
  // Analytics
  SEGMENT_KEY: process.env.NEXT_PUBLIC_SEGMENT_KEY,

  SKIP_AUTH: process.env.NEXT_PUBLIC_SKIP_AUTH === 'true',

  WEB_URL: process.env.NEXT_PUBLIC_POWDER_APP_URL,
  NEXT_PUBLIC_API_URL: process.env.NEXT_PUBLIC_API_URL || 'http://127.0.0.1:4000',
  DOCS_API_TOKEN: process.env.DOCS_API_TOKEN,
  DOCS_APP_URL: process.env.DOCS_APP_URL || 'https://docs.grit.io',
  SENTRY_DSN:
    process.env.NEXT_PUBLIC_SENTRY_DSN ||
    'https://2b8063474b8e4f4ba416e59321e6f5aa@o4504573272457216.ingest.sentry.io/4504573281042432',
  NEXT_PUBLIC_GRAPHQL_URL: process.env.NEXT_PUBLIC_GRAPHQL_URL ?? '',
};

export default config;
