import Head from 'next/head';
import Script from 'next/script';

export const getStaticProps = () => {
  return {
    props: {},
  };
};

export default function RequestFeature() {
  return (
    <>
      <Head>
        <title>Feature Request</title>
      </Head>

      <h1>Feature Request</h1>
      <p>Use this form to request support for a new migration, language, or other feature.</p>
      <iframe
        data-tally-src='https://tally.so/embed/nP1lqV?alignLeft=1&hideTitle=1&transparentBackground=1&dynamicHeight=1'
        loading='lazy'
        width='100%'
        height='715'
        title='Grit - Feature Request'
      ></iframe>
      <Script id='tally'>
        {`var d=document,w="https://tally.so/widgets/embed.js",v=function(){"undefined"!=typeof Tally?Tally.loadEmbeds():d.querySelectorAll("iframe[data-tally-src]:not([src])").forEach((function(e){e.src=e.dataset.tallySrc}))};if("undefined"!=typeof Tally)v();else if(d.querySelector('script[src="'+w+'"]')==null){var s=d.createElement("script");s.src=w,s.onload=v,s.onerror=v,d.body.appendChild(s);}`}
      </Script>
    </>
  );
}
