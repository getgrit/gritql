import { MatchEditor } from '@/components/code-block';
import { Config, Node, Tag } from '@markdoc/markdoc';

export const matcheditor = {
  render: MatchEditor,
  children: ['fence'],
  attributes: { className: { type: String } },
  transform(node: Node, config: Config) {
    const match = true;
    const attributes = node.transformAttributes(config);
    const children = node.transformChildren(config);
    // NOTE[chai]: false type error in the withMdoc parser
    // @ts-ignore
    return new Tag(this.render, { ...attributes, match }, children);
  },
};
