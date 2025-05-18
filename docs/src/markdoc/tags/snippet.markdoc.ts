import { ServerMonacoBlock } from '@/components/code-block/server-block';
import { Config, Node, Tag } from '@markdoc/markdoc';

export const snippet = {
  render: ServerMonacoBlock,
  children: ['fence'],
  attributes: { className: { type: String } },
  transform(node: Node, config: Config) {
    const attributes = node.transformAttributes(config);
    const children = node.transformChildren(config);
    // @ts-ignore
    return new Tag(this.render, { ...attributes, snippet: true }, children);
  },
};
