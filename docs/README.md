# Docs App

The GritQL documentation and playground, built with [Astro](https://astro.build) and [Starlight](https://starlight.astro.build).

## Dev

```shell
pnpm install
pnpm dev
```

The site is served at <http://localhost:3200>.

## Build

```shell
pnpm build
pnpm preview
```

The build fails on broken internal links (checked by `starlight-links-validator`). Run `pnpm check` to type-check the project.

## Structure

- `src/content/docs/`: the pages, written in [MDX](https://mdxjs.com). See the [authoring guide](src/content/docs/authoring.mdx) for the custom components (`<DiffEditor>`, `<MatchEditor>`, `<Snippet>`, …), exported from `src/components/index.ts`.
- `src/markdown/plugins.ts`: Markdown plugins laying out the code blocks of `<DiffEditor>`, `<MatchEditor>` and `<Snippet>`, and supporting custom heading ids (`## Heading \{#custom-id\}`).
- `src/partials/`: content shared between pages, imported with the `~/partials/…` alias: MDX partials, and the sample code used by `<Snippet>`.
- `src/playground/`: the playground, a React island running patterns through the WASM bindings in `__generated__/grit-wasm-bindings` inside a web worker.
- `src/grammars/gritql.tmLanguage.json`: the GritQL grammar used to highlight code blocks.
- `public/`: static files, including the tree-sitter parsers loaded by the playground (`wasm_parsers/`).
