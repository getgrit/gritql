import { Collapse as CollapseTag } from '@/components/collapse';

export const collapse = {
  attributes: {
    boxed: {
      type: Boolean,
    },
    className: {
      type: String,
    },
    title: {
      type: String,
    },
  },
  render: CollapseTag,
};
