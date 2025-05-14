import Head from 'next/head';
import { BiCodeAlt, BiLibrary, BiTerminal } from 'react-icons/bi';

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
        <title>Grit</title>
      </Head>

      <h1>Grit</h1>
      <p>Grit is a developer tool to put software maintenance on autopilot.</p>

      <p>We automatically handle your:</p>
      <ul>
        <li>Dependency upgrades</li>
        <li>Large migrations</li>
        <li>Code quality improvements</li>
      </ul>

      <hr />

      <CardGrid
        cards={[
          {
            title: 'Grit studio',
            Icon: BiLibrary,
            text: 'Open Grit Studio',
            color: 'blue',
            href: 'https://app.grit.io',
          },
          {
            title: 'Language reference',
            Icon: BiCodeAlt,
            text: 'Learn about GritQL',
            color: 'blue',
            href: '/language/overview',
          },
          {
            title: 'CLI Docs',
            text: 'Try the Grit CLI',
            Icon: BiTerminal,
            color: 'blue',
            href: '/cli/quickstart',
          },
        ]}
      />

      <hr />

      <Subscribe cta='Subscribe to learn more' />
    </>
  );
}
