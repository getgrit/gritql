'use client';

import { BiTerminal } from 'react-icons/bi';

import { MonacoEditor } from '@getgrit/editor';

import { CopyButton } from './buttons';

export function TerminalCommandBlock({
  command,
  title,
}: { command: string; title: React.ReactNode }) {
  return (
    <div className='bg-codeblock rounded-md overflow-hidden my-4'>
      <div className='flex justify-between px-3 py-2 bg-black'>
        <span className='flex text-gray-400 w-full text-xs items-center'>
          <BiTerminal className={'text-gray-400 w-4 h-4 self-center mr-1 inline-block'} /> {title}
        </span>
        <div className='flex gap-2 w-72 justify-end'>
          <CopyButton data={command} />
        </div>
      </div>
      <MonacoEditor options={{ readOnly: true }} value={command} language={'bash'} />
    </div>
  );
}
