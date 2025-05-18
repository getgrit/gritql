import { useEffect } from 'react';

import Head from 'next/head';


export default function Custom404() {
  return (
    <>
      <Head>
        <title>Grit Docs - Page not found</title>
      </Head>
      <h1>404</h1>
      <p>Oops! you&apos;re looking for something that doesn&apos;t exist. </p>
    </>
  );
}
