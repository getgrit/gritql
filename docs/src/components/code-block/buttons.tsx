import cx from 'classnames';
import { BiX } from 'react-icons/bi';

import {
  BaseButton,
  BaseButtonProps,
  buttonClassName,
  iconClassName,
} from '@/components/buttons/base-button';
import { useSidebarContext } from '@/hooks/sidebar';
import { useMainContext } from '@/templates/main-provider';
import { CopyButton as BaseCopyButton } from '@getgrit/shared';

type CopyButtonProps = {
  data: string;
};

export const CopyButton = ({ data }: CopyButtonProps) => {
  return <BaseCopyButton buttonClassName={`${buttonClassName} p-2`} {...{ data, iconClassName }} />;
};

export const TryButton = ({
  children,
  className,
  onClick,
}: BaseButtonProps & { onClick: NonNullable<BaseButtonProps['onClick']> }) => {
  const { isFirstTry, setIsFirstTry } = useMainContext();

  return (
    <button
      onClick={(e) => {
        if (isFirstTry) setIsFirstTry(false);
        onClick(e);
      }}
      className={cx('gradient-border', className, {
        'pulse-animation': isFirstTry,
      })}
    >
      <div className='relative z-10 flex items-center gap-1.5 px-3 py-1.5 text-xs font-semibold text-neutral-300'>
        {children}
      </div>
    </button>
  );
};

export const CloseButton = () => {
  const { setShowEditorSidebar } = useSidebarContext();

  return (
    <BaseButton
      onClick={() => {
        setShowEditorSidebar(false);
      }}
    >
      <BiX />
    </BaseButton>
  );
};
