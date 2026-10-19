// Components available to the MDX content.
export { default as Collapse } from './Collapse.astro';
export { default as Gate } from './Gate.astro';

// The code groups share a component: `src/markdown/plugins.ts` lays out their code blocks and
// passes the pattern (and its sample input) for the "Run Pattern" link.
export { default as DiffEditor } from './CodeGroup.astro';
export { default as MatchEditor } from './CodeGroup.astro';
export { default as Snippet } from './CodeGroup.astro';
