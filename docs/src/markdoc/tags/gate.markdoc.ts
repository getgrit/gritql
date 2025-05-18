import { GateTag } from '@/components/gate';

export const gate = {
  attributes: {
    flag: {
      type: String,
      required: true,
    },
  },
  render: GateTag,
};
