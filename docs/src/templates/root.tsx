'use client';

import Head from 'next/head';
import { usePathname } from 'next/navigation';

import config from '@/statics/config';
import { Template, TemplateProps } from '@/templates/plain';
import { useMonacoEditorInit } from '@getgrit/editor';
import { AnalyticsProvider, GritFeatureFlagProvider } from '@/components/analytics';

import '@/styles/main.css';

export type LayoutProps = Omit<TemplateProps, 'path'>;

export function RootClientLayout({ children, ...props }: LayoutProps) {
  useMonacoEditorInit({ theme: 'dark' });

  const pathname = usePathname() ?? '';
  return (
    <html lang='en'>
      <body>
        <AnalyticsProvider>
          <GritFeatureFlagProvider>
            <Template path={pathname} {...props}>
              <Head>
                <link rel='icon' href='/favicon.svg' />
              </Head>
              {children}
            </Template>
          </GritFeatureFlagProvider>
        </AnalyticsProvider>
      </body>
    </html>
  );
}
