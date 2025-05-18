import { MouseEventHandler } from 'react';

import cx from 'classnames';

import { WithChildren } from '@/custom-types/shared';

export const iconClassName = 'text-neutral-400 w-3 h-3';
const bgClass = 'bg-zinc-800';
export const buttonClassName =
  'rounded-md shadow-md focus:outline-none focus:ring-2 transition focus:ring-blue-700 hover:ring-2 hover:ring-blue-700';

export type BaseButtonProps = WithChildren<{
  onClick: MouseEventHandler<HTMLButtonElement> | undefined;
  size?: 'sm' | 'md' | 'lg';
  className?: string;
  disabled?: boolean;
}>;

const sizeClasses = {
  sm: 'p-2',
  md: 'p-2',
  lg: 'p-2',
};

export const computeButtonIconSize = (size: BaseButtonProps['size']) => {
  return size === 'lg' ? '2.25em' : undefined;
};

export const BaseButton = ({
  onClick,
  children,
  className,
  size = 'sm',
  disabled = false,
}: BaseButtonProps) => {
  return (
    <button
      onClick={onClick}
      disabled={disabled}
      className={cx(
        buttonClassName,
        sizeClasses[size],
        'text-zinc-100 text-xs',
        className,
        className?.includes('bg') ? '' : bgClass,
        disabled && 'opacity-50 cursor-not-allowed',
      )}
    >
      {children}
    </button>
  );
};
