import React from 'react';

import cx from 'classnames';
import Link from 'next/link';

type IconProps = {
  className: string;
};

export type CardProps = {
  title: string;
  text: string;
  color: 'blue' | 'green' | 'yellow';
  href: string;
  Icon?: React.ComponentType<IconProps>;
};

export const Card = ({ color, href, Icon, text, title }: CardProps) => {
  return (
    <Link className='no-underline' href={href} rel='noreferrer' passHref>
      <div className='group cursor-pointer'>
        <div
          className={cx(
            'flex h-32 items-center justify-center rounded-md mb-6 group-hover:opacity-80 group-hover:shadow-lg transition transform group-hover:-translate-y-px duration-500',
            {
              'bg-gradient-to-r from-primary-600 to-primary-400': color === 'blue',
              'bg-gradient-to-r from-yellow-500 to-yellow-400': color === 'yellow',
              'bg-gradient-to-r from-emerald-500 to-emerald-400': color === 'green',
            },
          )}
        >
          {Icon && <Icon className='w-7 h-7 text-white' />}
        </div>
        <p
          className={cx('text-xs uppercase font-semibold mb-2', {
            'text-blue-500': color === 'blue',
            'text-emerald-500': color === 'yellow',
          })}
        >
          {title}
        </p>
        <p className='text-sm text-neutral-500 p-0 m-0'>{text}</p>
      </div>
    </Link>
  );
};
