import { useEffect } from 'react';

import type { AppProps } from 'next/app';
import Head from 'next/head';
import NProgress from 'nprogress';

import config from '@/statics/config';
import { Template } from '@/templates/plain';
import { WrapperContainer } from '@/templates/wrapper';
import { useMonacoEditorInit } from '@/components/editor/monaco-editor-init';
import { AnalyticsProvider } from '@/components/analytics/analytics';

import '@/styles/main.css';
import 'nprogress/nprogress.css';

if (typeof window !== 'undefined') {
  initTracing(config.SENTRY_DSN);
}

function MyApp({ Component, pageProps, router }: AppProps) {
  NProgress.configure({ showSpinner: false });
  useMonacoEditorInit({ theme: 'dark' });

  useEffect(() => {
    const handleStart = () => {
      NProgress.start();
    };

    const handleStop = () => {
      NProgress.done();
    };

    router.events.on('routeChangeStart', handleStart);
    router.events.on('routeChangeComplete', handleStop);
    router.events.on('routeChangeError', handleStop);

    return () => {
      router.events.off('routeChangeStart', handleStart);
      router.events.off('routeChangeComplete', handleStop);
      router.events.off('routeChangeError', handleStop);
    };
  }, [router]);

  // NOTE[chai]: Use title from the Markdoc meta when available.
  const title = pageProps.markdoc?.frontmatter?.title;

  return (
    <AnalyticsProvider>
        <Template path={router.pathname} layout='docs'>
          <Head>
            {title && <title>{title}</title>}
            <link rel='icon' href='/favicon.svg' />
          </Head>
          <WrapperContainer frontmatter={pageProps.markdoc?.frontmatter ?? {}}>
            <Component {...pageProps} />
          </WrapperContainer>
        </Template>
    </AnalyticsProvider>
  );
}

export default MyApp;
