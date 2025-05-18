import cx from 'classnames';
import { BiCode, BiCodeBlock, BiTerminal } from 'react-icons/bi';
import { AiFillThunderbolt } from 'react-icons/ai';
import upperCase from 'lodash/upperCase';
import { CSSProperties } from 'react';
import React from 'react';

type HeadingProps = {
  fileName?: string;
  title: string;
  iconClassName?: string;
  headingClassName?: string;
  iconStyle?: CSSProperties;
  headingStyle?: CSSProperties;
};

export const Heading = ({
  fileName,
  title,
  iconClassName,
  headingClassName,
  iconStyle,
  headingStyle,
}: HeadingProps) => {
  switch (title) {
    case 'bash':
      return (
        <span className={headingClassName} style={headingStyle}>
          <BiTerminal className={iconClassName} style={iconStyle} /> Terminal
        </span>
      );
    case 'diff':
      return (
        <div
          className={cx(headingClassName, 'justify-between')}
          style={{ ...headingStyle, justifyContent: 'space-between' }}
        >
          <span>
            <BiCode className={iconClassName} style={iconStyle} /> INPUT
          </span>
          <span>
            OUTPUT <AiFillThunderbolt className={iconClassName} style={iconStyle} />
          </span>
        </div>
      );
    default:
      const ifTitle = fileName?.split('.').at(-1) !== title && upperCase(title);
      return (
        <span className={headingClassName} style={headingStyle}>
          <BiCodeBlock className={iconClassName} style={iconStyle} />
          {ifTitle || ''} {fileName}
        </span>
      );
  }
};

interface SnippetHeadingProps {
  title: string;
}

export const SnippetHeading: React.FC<SnippetHeadingProps> = ({ title }) => {
  return (
    <div className="text-sm font-medium text-neutral-200">
      {title}
    </div>
  );
};
