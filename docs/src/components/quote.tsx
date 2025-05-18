import cx from 'classnames';

import { WithChildren } from '@/custom-types/shared';

import { Avatar } from './avatar';

type QuoteProps = WithChildren<{
  className?: string;
  author: string;
  title: string;
  avatar?: string;
}>;

export const Quote = ({ children, className, author, title, avatar }: QuoteProps) => {
  return (
    <div
      className={cx(
        className,
        'flex flex-col items-center justify-center p-5 my-6 mx-auto text-center',
        {
          'text-gray-800': author,
          'text-gray-600': title,
        },
      )}
    >
      <blockquote>{children}</blockquote>
      <div className='flex items-center space-x-4'>
        {avatar && <Avatar src={avatar} />}
        <div className='flex flex-col'>
          <p className='mt-4 mb-0 text-xl font-semibold'>{author}</p>
          <p className='mt-0 text-sm text-gray-500'>{title}</p>
        </div>
      </div>
    </div>
  );
};
