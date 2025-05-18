import cx from 'classnames';
import { BiErrorCircle } from 'react-icons/bi';
import { FiInfo } from 'react-icons/fi';

import { WithChildren } from '@/custom-types/shared';

type NoteProps = WithChildren<{
  className?: string;
  type: 'info' | 'warning';
}>;

export const Note = ({ children, className, type }: NoteProps) => {
  const iconClass = 'w-10 h-7 my-4 mr-5 text-neutral';
  return (
    <div
      className={cx(className, 'flex note-tag pr-5 my-6 py-0 rounded-lg border px-6', {
        'bg-gray-50 border-gray-200': type === 'info',
        'bg-yellow-50 border-yellow-400': type === 'warning',
      })}
    >
      <div className='flex justify-center items-center'>
        {' '}
        {/* Added flexbox properties here */}
        {type === 'info' && <FiInfo className={iconClass} />}
        {type === 'warning' && <BiErrorCircle className={iconClass} />}
      </div>
      <div>{children}</div>
    </div>
  );
};
