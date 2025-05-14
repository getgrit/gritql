import type { Metadata } from 'next';

import { StandardLibraryProvider } from '@/components/providers/pattern-library';
import { getStdlib } from '@/libs/patterns';

export const metadata: Metadata = {
  title: {
    template: '%s | Grit',
    absolute: 'Grit Documentation',
  },
};

export default async function RootLayout({ children }: { children: React.ReactNode }) {
  const remotePatterns = await getStdlib();

  return <StandardLibraryProvider patterns={remotePatterns}>{children}</StandardLibraryProvider>;
}
