import Head from 'next/head';
import { BiCodeAlt, BiLibrary, BiTerminal } from 'react-icons/bi';

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
        <title>Grit Documentation</title>
      </Head>

      <h1>Documentation</h1>
      <p>
        Grit is a developer tool to simplify software maintenance. It includes a web interface
        through which you can generate pull requests from automated end-to-end migrations, as well
        as an optional CLI offering local control.
      </p>

      <p>
        The two main tools used by Grit under the hood are GritQL, an intuitive and powerful query
        language for manipulating code using static analysis, and AI-powered transformations that
        allow our migrations to adapt seamlessly to the conventions of your codebase. Together,
        GritQL and AI transforms perform much of the repetitive work in modernizing outdated code,
        freeing you to spend time on building the software you want.
      </p>

      <h2>Resources</h2>
      <CardGrid
        cards={[
          {
            title: 'Pattern library',
            Icon: BiLibrary,
            text: 'Patterns available out of the box',
            color: 'blue',
            href: '/patterns',
          },
          {
            title: 'Language reference',
            Icon: BiCodeAlt,
            text: 'Reference documentation for Grit',
            color: 'blue',
            href: '/language/overview',
          },
          {
            title: 'CLI Docs',
            text: 'Documentation for the CLI',
            Icon: BiTerminal,
            color: 'blue',
            href: '/cli/quickstart',
          },
        ]}
      />
    </>
  );
}
