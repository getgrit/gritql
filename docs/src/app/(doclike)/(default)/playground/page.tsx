import type { Metadata } from 'next';

import { PlaygroundEditor } from '@/components/playground';

export const metadata: Metadata = {
  title: 'Playground',
};

export default async function Playground() {
  return (
    <>
      <h1 className='mb-4'>Grit Playground</h1>
      TEST!
      <div className='h-[80vh]'>
        <PlaygroundEditor />
      </div>
    </>
  );
}
