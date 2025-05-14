import { GateTag } from '@/components/gate';
import { FEATURE_FLAGS } from '@getgrit/universal';

export const gate = {
  attributes: {
    flag: {
      type: String,
      required: true,
      matches: Object.keys(FEATURE_FLAGS),
    },
  },
  render: GateTag,
};
