import type { Metadata } from 'next';

import { StandaloneEditor } from '@/components/editor/standalone-editor';

export const metadata: Metadata = {
  title: 'Playground',
};

export default async function Playground() {
  return (
    <>
      <h1 className='mb-4'>Grit Playground</h1>
      <div className='h-[80vh]'>
        <StandaloneEditor />
      </div>
    </>
  );
}
