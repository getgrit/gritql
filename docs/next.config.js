const withMarkdoc = require("@markdoc/next.js");

const plugins = [
  withMarkdoc({
    mode: "static",
    schemaPath: "./src/markdoc",
    tokenizerOptions: { allowComments: true },
  }),
];

/** @type {import('next').NextConfig} */
const nextConfig = {
  output: "export",
  images: {
    unoptimized: true,
  },
  eslint: {
    ignoreDuringBuilds: true,
  },
  webpack: (config, {}) => {
    if (!config.resolve) config.resolve = {};
    if (!config.resolve.fallback) config.resolve.fallback = {};
    config.resolve.fallback.fs = false;

    return config;
  },
  experimental: {
    optimizeCss: true,
  },
  reactStrictMode: true,
  pageExtensions: ["md", "mdx", "js", "ts", "jsx", "tsx", "mdoc"],
  redirects() {
    return [
      {
        source: "/discord",
        destination: "https://discord.gg/ARExD4gvFB",
        permanent: true,
      },
      {
        source: "/install",
        destination:
          "https://github.com/getgrit/gritql/releases/latest/download/grit-installer.sh",
        permanent: false,
      },
      {
        source: "/s/call",
        destination: "https://app.reclaim.ai/m/morgante/demo",
        permanent: true,
      },
      {
        source: "/getting-started/introduction",
        destination: "/",
        permanent: true,
      },
      {
        source: "/data",
        destination: "/security",
        permanent: true,
      },
      {
        source: "/getting-started/install",
        destination: "/cli/quickstart#installation",
        permanent: true,
      },
      {
        source: "/getting-started/cli-quickstart",
        destination: "/cli/quickstart",
        permanent: true,
      },
      {
        source: "/guides/cli",
        destination: "/cli/quickstart",
        permanent: true,
      },
      {
        source: "/guides/migrations",
        destination: "/workflows/overview",
        permanent: true,
      },
      {
        source: "/language",
        destination: "/language/overview",
        permanent: true,
      },
      {
        source: "/language/reference",
        destination: "/language/syntax",
        permanent: true,
      },
      {
        source: "/language/introduction",
        destination: "/language/overview",
        permanent: true,
      },
      {
        source: "/language/standard-library",
        destination: "/language/functions",
        permanent: false,
      },
      {
        source: "/s/builtin-functions",
        destination: "/language/functions#builtin-functions",
        permanent: false,
      },
      {
        source: "/s/api-stdlib",
        destination: "/sdk/api/Namespace.stdlib",
        permanent: false,
      },
      {
        source: "/s/vsc-lsp-start",
        destination: "/guides/vscode#unable-to-connect-to-language-server",
        permanent: false,
      },
      {
        source: "/language/best-practices",
        destination: "/language/idioms",
        permanent: false,
      },
    ];
  },
  rewrites() {
    return {
      beforeFiles:
        process.env.NODE_ENV === "production"
          ? [{ source: "/authoring", destination: "/404" }]
          : [],
    };
  },
  async headers() {
    // Keep these in sync with apps/web/next.config.js
    return [
      {
        source: "/:path*",
        headers: [
          {
            key: "X-Frame-Options",
            value: "DENY",
          },
          {
            key: "X-Content-Type-Options",
            value: "nosniff",
          },
        ],
      },
    ];
  },
};

module.exports = () => plugins.reduce((acc, next) => next(acc), nextConfig);
