import React from 'react';

import cx from 'classnames';

type IconProps = {
  className: string;
};

type ButtonProps = {
  ariaLabel: string;
  onClick: () => void;
  className?: string;
  Icon: React.ComponentType<IconProps>;
};

export const IconButton = ({ ariaLabel, Icon, onClick, className }: ButtonProps) => {
  return (
    <button
      className={cx('block cursor-pointer p-1.5 rounded-md hover:bg-black/5 transition', className)}
      aria-label={ariaLabel}
      onClick={() => onClick()}
    >
      <Icon className='w-5 h-5 text-neutral-900' />
    </button>
  );
};
