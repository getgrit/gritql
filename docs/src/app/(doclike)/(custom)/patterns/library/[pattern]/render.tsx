/* eslint-disable complexity */
import React from 'react';

import { isObject } from 'lodash';
import { DocPattern } from 'src/app/(doclike)/(default)/patterns/page';

// Other components used during rendering
import { CollapsedCodeBlock, PatternBlockInfo } from '@/components/code-block/collapsed';
import { PatternTitle } from '@/components/patterns/title';
import * as nodes from '@/markdoc/nodes/index';
import { collapsedpattern } from '@/markdoc/tags/collapsedpattern.markdoc';
import { diffeditor } from '@/markdoc/tags/diffeditor.markdoc';
import { note } from '@/markdoc/tags/note.markdoc';
import { patterntitle } from '@/markdoc/tags/patterntitle.markdoc';
// Not all tags are SSR-capable so this is our hack
import { snippet } from '@/markdoc/tags/snippet.markdoc';
import Markdoc, { RenderableTreeNode, Tag } from '@markdoc/markdoc';

// Remove the first heading node for our custom title component
function swapHeading(
  node: RenderableTreeNode,
  trackState: { didSwapHeading: boolean },
  patternInfo: Pick<DocPattern, 'gitHubUrl' | 'language'>,
) {
  if (
    node &&
    isObject(node) &&
    'children' in node &&
    Array.isArray(node.children) &&
    node.children.length > 0
  ) {
    node.children = node.children.map((child) => {
      if (Tag.isTag(child)) {
        if (child.name === 'h1' || (Tag.isTag(child) && child.attributes.level === 1)) {
          trackState.didSwapHeading = true;
          return new Tag(
            // @ts-expect-error
            PatternTitle,
            {
              pattern: patternInfo,
            },
            child.children,
          );
        }

        return swapHeading(child, trackState, patternInfo);
      }
      return child;
    });
  }
  return node;
}

/**
 * Find the first code fence and make it into a collapse pattern
 */
function swapPattern(node: RenderableTreeNode, pattern?: PatternBlockInfo) {
  if (
    node &&
    isObject(node) &&
    'children' in node &&
    Array.isArray(node.children) &&
    node.children.length > 0
  ) {
    node.children = node.children.map((child) => {
      if (Tag.isTag(child)) {
        if (
          child.$$mdtype === 'Tag' &&
          child.attributes?.isFence === true &&
          child.attributes?.language === 'grit'
        ) {
          // @ts-expect-error
          return new Tag(CollapsedCodeBlock, { pattern }, [child]);
        }

        return swapPattern(child);
      }
      return child;
    });
  }
  return node;
}

/**
 * Find before/after examples and mark them as pairs
 */
function markPairs(node: RenderableTreeNode) {
  if (Tag.isTag(node)) {
    let firstSnippet = null;
    let insideBlock = false;

    for (const child of node.children) {
      if (Tag.isTag(child)) {
        if (
          child.attributes?.level === 2 ||
          child.attributes?.level === 1 ||
          child.name === 'h2' ||
          child.name === 'h1'
        ) {
          firstSnippet = null;
          insideBlock = true;
        }

        if (
          insideBlock &&
          child.$$mdtype === 'Tag' &&
          child.attributes?.isFence === true &&
          child.attributes?.language !== 'grit'
        ) {
          if (firstSnippet) {
            child.attributes.secondInPair = true;
            firstSnippet.attributes.firstInPair = true;
            firstSnippet = null;
          } else {
            firstSnippet = child;
          }
        }
      }
    }
  }
  return node;
}

export const MarkdownPatternPage: React.FC<{
  markdown: string;
  patternInfo?: Pick<DocPattern, 'gitHubUrl' | 'language' | 'title' | 'name'> & PatternBlockInfo;
}> = ({ markdown, patternInfo }) => {
  try {
    const ast = Markdoc.parse(markdown);

    const frontmatter = ast.attributes.frontmatter ?? '';

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
    const trackState = { didSwapHeading: false };
    content = swapHeading(content, trackState, patternInfo ?? {});
    content = frontmatter?.includes('full-examples') ? content : swapPattern(content, patternInfo);
    content = markPairs(content);

    const details = Markdoc.renderers.react(content, React, {});
    return (
      <>
        {!trackState.didSwapHeading && patternInfo ? (
          <PatternTitle pattern={patternInfo}>{patternInfo.title}</PatternTitle>
        ) : null}
        {details}
      </>
    );
  } catch (e: any) {
    // grit-ignore custom_no_console_log
    // eslint-disable-next-line no-console
    <>
      <p>Error rendering pattern markdown</p>
    </>;
  }
};
