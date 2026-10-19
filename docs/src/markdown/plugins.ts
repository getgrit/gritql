import { defineMdastPlugin, type MdxJsxAttributeNode } from 'satteri';

const HL_START = '{hl}';
const HL_END = '{/hl}';

/** Titles of the code blocks inside each code group component, in order. */
const GROUP_TITLES: Record<string, string[]> = {
  // A GritQL pattern, followed by the code before and after applying it.
  DiffEditor: ['Pattern', 'Before', 'After'],
  // A GritQL pattern, followed by code where the matches are wrapped in `{hl}…{/hl}`.
  MatchEditor: ['Pattern', 'Matches'],
  // A GritQL pattern; its sample input is passed with the `input` prop.
  Snippet: [],
};

/**
 * Removes the `{hl}…{/hl}` markers used to annotate matches and returns the (1-based) lines that
 * contained a highlight, so they can be marked by Expressive Code.
 */
function extractHighlights(code: string) {
  const lines = new Set<number>();
  let inHighlight = false;
  const cleaned = code.split('\n').map((line, index) => {
    if (inHighlight || line.includes(HL_START)) lines.add(index + 1);
    const starts = line.lastIndexOf(HL_START);
    const ends = line.lastIndexOf(HL_END);
    if (starts !== ends) inHighlight = starts > ends;
    return line.replaceAll(HL_START, '').replaceAll(HL_END, '');
  });
  return { code: cleaned.join('\n'), lines: [...lines] };
}

const attribute = (name: string, value: string): MdxJsxAttributeNode => ({
  type: 'mdxJsxAttribute',
  name,
  value,
});

/**
 * Lays out the code blocks of `<DiffEditor>`, `<MatchEditor>` and `<Snippet>`: titles them, strips
 * the highlight markers, marks the highlighted lines, and passes the pattern and its input to the
 * component so it can link to the playground.
 */
export const codeGroups = defineMdastPlugin({
  name: 'grit-code-groups',
  mdxJsxFlowElement(node, ctx) {
    const titles = node.name ? GROUP_TITLES[node.name] : undefined;
    if (!titles) return;

    const codes: string[] = [];
    const children = node.children.map((child) => {
      if (child.type !== 'code') return child;
      const index = codes.length;
      const { code, lines } = extractHighlights(child.value);
      codes.push(code);
      const title = titles[index];
      const marks = node.name === 'MatchEditor' && index === 1 && lines.length > 0;
      // The group's titles replace the fence's own, like on the previous site.
      const ownMeta = title ? child.meta?.replace(/\btitle="[^"]*"/, '').trim() : child.meta;
      const meta = [ownMeta, title && `title="${title}"`, marks && `{${lines.join(',')}}`];
      return { ...child, value: code, meta: meta.filter(Boolean).join(' ') };
    });

    const [pattern, input] = codes;
    if (pattern === undefined) return;
    // JSX attributes can't be set in place, so the element is rebuilt.
    ctx.replaceNode(node, {
      ...node,
      attributes: [
        ...node.attributes,
        attribute('pattern', pattern),
        ...(input !== undefined && node.name !== 'Snippet' ? [attribute('input', input)] : []),
      ],
      children,
    });
  },
});

/**
 * Supports custom heading ids with `## Heading \{#custom-id\}`. The braces are escaped because
 * MDX reads `{…}` as an expression.
 */
export const customHeadingIds = defineMdastPlugin({
  name: 'custom-heading-ids',
  heading(node, ctx) {
    const last = node.children.at(-1);
    if (last?.type !== 'text') return;
    const match = /\s*\{#([\w-]+)\}\s*$/.exec(last.value);
    if (!match) return;
    ctx.setProperty(last, 'value', last.value.slice(0, match.index));
    ctx.setProperty(node, 'data', { ...node.data, hProperties: { id: match[1] } });
  },
});
