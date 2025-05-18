import { Quote } from '@/components/quote';

export const pullquote = {
  attributes: {
    className: {
      type: String,
    },
    author: {
      type: String,
      required: true,
    },
    title: {
      type: String,
      required: true,
    },
    avatar: {
      type: String,
    },
  },
  children: ['paragraph'],
  render: Quote,
};
