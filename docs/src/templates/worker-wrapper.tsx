'use client';

import { usePathname } from 'next/navigation';
import { WorkerAnalysisProvider } from 'src/workers/provider';

import { doesPathHaveEditor } from '@/libs/dynamic';
import { StandaloneEditorProvider } from '@/components/editor/standalone-editor';

export const WorkerWrapper = ({ children }: { children: React.ReactNode }) => {
  const pathname = usePathname() ?? '';
  const withWorker = doesPathHaveEditor(pathname);

  if (!withWorker) {
    return <>{children}</>;
  }

  const getToken = async () => {
    try {
      const req = await fetch('/api/info/token');
      const data = await req.json();
      const token = data.accessToken;
      return token;
    } catch (e) {
      console.error('Token fetch failed', e);
      return '';
    }
  };

  return (
    <WorkerAnalysisProvider api_endpoint={'https://api2.grit.io'} getToken={getToken}>
      <StandaloneEditorProvider>{children}</StandaloneEditorProvider>
    </WorkerAnalysisProvider>
  );
};
