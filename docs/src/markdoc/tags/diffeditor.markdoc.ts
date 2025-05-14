import { ServerDiffEditor } from '@/components/code-block/diff-server';
import { Config, Node, Tag } from '@markdoc/markdoc';

export const diffeditor = {
  render: ServerDiffEditor,
  children: ['fence'],
  attributes: { className: { type: String } },
  transform(node: Node, config: Config) {
    const diff = true;
    const attributes = node.transformAttributes(config);
    const children = node.transformChildren(config);
    // NOTE[chai]: false type error in the withMdoc parser
    // @ts-ignore
    return new Tag(this.render, { ...attributes, diff }, children);
  },
};
