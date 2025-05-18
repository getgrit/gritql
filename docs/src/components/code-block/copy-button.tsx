import { CSSProperties, useState } from 'react';
import { BiCheck, BiCopy } from 'react-icons/bi';

type CopyButtonProps = {
  data: string;
  buttonClassName?: string;
  iconClassName?: string;
  buttonStyle?: CSSProperties;
  iconStyle?: CSSProperties;
};

export const CopyButton = ({
  data,
  buttonClassName,
  iconClassName,
  buttonStyle,
  iconStyle,
}: CopyButtonProps) => {
  const [isCopied, setIsCopied] = useState(false);
  return (
    <button
      className={buttonClassName}
      style={buttonStyle}
      onClick={() => {
        navigator.clipboard.writeText(data);
        setIsCopied(true);
        setTimeout(() => setIsCopied(false), 1000);
      }}
    >
      <CopyIcon isCopied={isCopied} className={iconClassName} style={iconStyle} />
    </button>
  );
};

const CopyIcon = ({
  isCopied,
  className,
  style,
}: {
  isCopied: boolean;
  className?: string;
  style?: CSSProperties;
}) => {
  return isCopied ? (
    <BiCheck className={className} style={style} />
  ) : (
    <BiCopy className={className} style={style} />
  );
}; 