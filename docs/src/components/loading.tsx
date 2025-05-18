import cx from 'classnames';
import Head from 'next/head';
import { BiLoaderAlt } from 'react-icons/bi';

export const Loading = ({ className }: { className?: string }) => (
  <div className='p-6 sm:px-8 pb-32 mx-auto max-w-screen-md antialiased prose'>
    <Head>
      <title>Grit Documentation - Loading</title>
    </Head>
    <BiLoaderAlt className={cx(className, 'w-10 h-10 mx-auto text-slate-500 animate-spin')} />
  </div>
);
