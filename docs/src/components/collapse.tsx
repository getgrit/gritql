'use client';

import { useCallback, useRef, useState } from 'react';

import cx from 'classnames';

import { DownIcon } from '@/components/icons/down';
import { WithChildren } from '@/custom-types/shared';

type CollapseProps = WithChildren<{
  title: string;
  boxed?: boolean;
  className?: string;
}>;

export const Collapse = ({ boxed = false, children, className, title }: CollapseProps) => {
  const contentRef = useRef<HTMLDivElement>(null);
  const [state, setState] = useState({ height: 0, open: false });

  const toggleOpen = useCallback(() => {
    setState((s) => {
      if (s.open) {
        return { height: 0, open: false };
      } else {
        const containerHeight = contentRef?.current?.scrollHeight;
        if (typeof containerHeight === 'number') {
          return { height: containerHeight, open: true };
        }
        return s;
      }
    });
  }, []);

  return (
    <div
      className={cx(
        'flex flex-col rounded-lg',
        {
          'border border-neutral-200 px-4': boxed,
        },
        className,
      )}
    >
      <div
        className='flex flex-row items-center gap-4 cursor-pointer py-3 group text-neutral-900 not-prose'
        onClick={toggleOpen}
      >
        <h4
          className={cx(
            'max-w-none prose prose-neutral font-semibold select-none transition group-hover:text-neutral-500',
            {
              'flex-grow': boxed,
              'text-md': !boxed,
            },
          )}
        >
          {title}
        </h4>
        <DownIcon
          className={cx('w-6 h-6 flex-none transform duration-300 text-neutral-600 transition', {
            'rotate-180': state.open,
          })}
        />
      </div>
      <div
        ref={contentRef}
        style={{ height: state.height }}
        className='overflow-hidden transition-all duration-200 prose prose-neutral max-w-none'
      >
        <div className='mb-4'>{children}</div>
      </div>
    </div>
  );
};
