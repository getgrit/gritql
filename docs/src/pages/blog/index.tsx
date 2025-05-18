import Head from 'next/head';

import { Subscribe } from '@/components/blog/subscribe';
import { CardGrid } from '@/components/card-grid';

export const getStaticProps = () => {
  return {
    props: {},
  };
};

export default function Index() {
  return (
    <>
      <Head>
        <title>Grit Blog</title>
      </Head>

      <h1 className='mb-4'>Grit Blog</h1>
      <CardGrid
        cards={[
          {
            title: 'July update',
            text: 'July 8, 2024',
            color: 'blue',
            href: '/blog/2024-july',
          },
          {
            title: 'May update',
            text: 'May 20, 2024',
            color: 'blue',
            href: '/blog/2024-may',
          },
          {
            title: 'We’ve raised $7M to erase technical debt',
            text: 'August 15, 2023',
            color: 'yellow',
            href: '/blog/seed',
          },
        ]}
      />
      <Subscribe cta='Subscribe to updates' />
    </>
  );
}
