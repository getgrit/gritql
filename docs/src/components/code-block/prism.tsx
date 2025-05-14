/*
 *
 * NOTE[chai]: Do note use this editor, use Monaco instead.
 *
 */

import cx from 'classnames';
import upperCase from 'lodash/upperCase';
import Highlight, { defaultProps, Language } from 'prism-react-renderer';
import { BiCodeBlock, BiTerminal } from 'react-icons/bi';

import { WithChildren } from '@/custom-types/shared';

import { CopyButton } from './buttons';

type HeadingProps = {
  language: Language;
  fileName?: string;
};

type MarkdocCodeFenceProps = WithChildren<HeadingProps>;

const Heading = ({ fileName, language }: HeadingProps) => {
  const iconClassName = 'text-neutral-400 w-4 h-4 self-center mr-1';
  const headingClassName = 'flex text-neutral-400 text-xs items-center';
  switch (language) {
    case 'bash':
      return (
        <span className={headingClassName}>
          <BiTerminal className={iconClassName} /> Terminal
        </span>
      );
      break;
    default:
      return (
        <span className={headingClassName}>
          <BiCodeBlock className={iconClassName} />
          {upperCase(language)} {fileName}
        </span>
      );
      break;
  }
};

export function PrismBlock(props: MarkdocCodeFenceProps) {
  const { children, fileName, language } = props;
  const code = (children as string).trim();

  return (
    <div className='bg-codeblock rounded-md overflow-hidden'>
      <div className='flex justify-between px-3 py-2 bg-black'>
        <Heading fileName={fileName} language={language} />
        <CopyButton data={code} />
      </div>
      <Highlight {...defaultProps} code={code} language={language}>
        {({ className, getLineProps, getTokenProps, tokens }) => (
          <pre className={cx(className, 'relative')}>
            {tokens.map((line, key) => (
              <div key={key} {...getLineProps({ key, line })} style={undefined}>
                {line.map((token, key) => (
                  <span key={key} {...getTokenProps({ key, token })} style={undefined} />
                ))}
              </div>
            ))}
          </pre>
        )}
      </Highlight>
    </div>
  );
}
