/* eslint-disable complexity */
import React from 'react';

import * as nodes from '@/markdoc/nodes/index';
import { collapsedpattern } from '@/markdoc/tags/collapsedpattern.markdoc';
import { diffeditor } from '@/markdoc/tags/diffeditor.markdoc';
import { note } from '@/markdoc/tags/note.markdoc';
import { patterntitle } from '@/markdoc/tags/patterntitle.markdoc';
import { snippet } from '@/markdoc/tags/snippet.markdoc';
import type { RenderableTreeNodes } from '@markdoc/markdoc';
import Markdoc, { Tag } from '@markdoc/markdoc';

export function parsePlainText(markdown: string) {
  const ast = Markdoc.parse(markdown);
  let content = Markdoc.transform(ast);
  return content;
}

/**
 * Simple rendering of a tree of nodes to plain text.
 * This should not be used for long or complex documents.
 */
export function renderPlainText(node: RenderableTreeNodes): string {
  if (typeof node === 'string' || typeof node === 'number') return String(node);

  if (Array.isArray(node)) return node.map(renderPlainText).join('');

  if (node === null || typeof node !== 'object' || !Tag.isTag(node)) return '';

  const { name, children = [] } = node;

  if (!name) return renderPlainText(children);

  if (children && Array.isArray(children)) {
    return renderPlainText(children);
  }

  return '';
}

export function renderMarkdown({ markdown }: { markdown: string }) {
  const ast = Markdoc.parse(markdown);

  // @ts-expect-error
  let content = Markdoc.transform(ast, {
    nodes,
    tags: {
      snippet,
      collapsedpattern,
      diffeditor,
      note,
      patterntitle,
    },
  });

  const rendered = Markdoc.renderers.react(content, React, {});
  return rendered;
}
