import type { Metadata } from 'next';

import { RootClientLayout } from '@/templates/root';

export const metadata: Metadata = {
  title: {
    template: '%s | Grit',
    absolute: 'Grit Documentation',
  },
};

export default async function DocLayout({ children }: { children: React.ReactNode }) {
  return <RootClientLayout layout='docs'>{children}</RootClientLayout>;
}
