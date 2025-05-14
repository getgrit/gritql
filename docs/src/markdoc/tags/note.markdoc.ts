import { Note } from '@/components/note';

export const note = {
  attributes: {
    className: {
      type: String,
    },
    type: {
      matches: ['info', 'warning'],
      type: String,
      default: 'info',
    },
  },
  children: ['paragraph'],
  render: Note,
};
