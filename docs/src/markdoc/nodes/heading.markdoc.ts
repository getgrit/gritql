import { Heading } from '@/components/heading';
import { generateHeadingID } from '@/libs/global';
import { Config, Node, Tag } from '@markdoc/markdoc';

export const heading = {
  attributes: {
    className: { type: String },
    id: { type: String },
    level: { default: 1, required: true, type: Number },
  },
  children: ['inline'],
  render: Heading,
  transform(node: Node, config: Config) {
    const attributes = node.transformAttributes(config);
    const children = node.transformChildren(config);
    const id = generateHeadingID(children, attributes);
    // NOTE[chai]: false type error in the withMdoc parser
    // @ts-ignore
    return new Tag(this.render, { ...attributes, id }, children);
  },
};
