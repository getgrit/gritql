import Head from 'next/head';
import { BiLayerPlus, BiLibrary } from 'react-icons/bi';
import { MdOutlineTransform } from 'react-icons/md';

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
        <title>GritQL Documentation</title>
      </Head>

      <h1>GritQL Language Documentation</h1>
      <p>
        GritQL language is Grit&apos;s embedded query language for searching and transforming source
        code. It is designed to match developer intuition wherever possible, reducing the overhead
        of adding it to your stack, while offering the power and versatility to execute complex
        transformations in just a few lines of code.
      </p>
      <p>
        The best place to start for writing custom GritQL patterns is the{' '}
        <a href='/tutorials/gritql'>tutorial</a>.
      </p>

      <h2>Topics</h2>
      <CardGrid
        cards={[
          {
            title: 'Intro tutorial',
            Icon: BiLibrary,
            text: 'Learn the basics of GritQL',
            color: 'blue',
            href: '/tutorials/gritql',
          },
          {
            title: 'Syntax reference',
            Icon: BiLayerPlus,
            text: 'Cheatsheet for GritQL syntax',
            color: 'yellow',
            href: '/language/syntax',
          },
          {
            title: 'Configuration',
            Icon: MdOutlineTransform,
            text: 'Configure GritQL in your repo',
            color: 'green',
            href: '/guides/config',
          },
        ]}
      />
    </>
  );
}
