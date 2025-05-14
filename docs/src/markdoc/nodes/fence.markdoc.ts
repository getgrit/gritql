import { ServerMonacoBlock } from '@/components/code-block/server-block';

export const fence = {
  attributes: {
    content: { type: String },
    fileName: {
      type: String,
      required: false,
    },
    short: {
      type: Boolean,
      default: false,
    },
    language: {
      type: String,
    },
    snippet: {
      default: false,
      type: Boolean,
    },
    isFence: {
      default: true,
      type: Boolean,
    },
    /**
     * If true, this fence is the first in a pair of before/after snippets
     */
    firstInPair: {
      default: false,
      type: Boolean,
    },
    secondInPair: {
      default: false,
      type: Boolean,
    },
  },
  render: ServerMonacoBlock,
};
