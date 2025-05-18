import { CollapsedCodeBlock } from '@/components/code-block/collapsed';

export const collapsedpattern = {
  render: CollapsedCodeBlock,
  children: ['fence'],
  attributes: { className: { type: String } },
};
