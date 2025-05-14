'use client';

import React from 'react';

import cx from 'classnames';
import { usePathname } from 'next/navigation';

import { WithChildren } from '@/custom-types/shared';
import config from '@/statics/config';

type HeadingNode = WithChildren<{
  id: string;
  className?: string;
  level: number;
}>;

type HeadingLevels = 'h1' | 'h2' | 'h3' | 'h4' | 'h5' | 'h6';

interface HeadingProps extends React.HTMLAttributes<HTMLHeadingElement> {
  level: HeadingLevels;
  className: string;
  id: string;
}

const HeadingComponent = ({ children, className, id, level }: HeadingProps) => {
  const Heading = ({ ...props }: React.HTMLAttributes<HTMLHeadingElement>) =>
    React.createElement(level, props, children);
  return (
    <Heading tabIndex={-1} id={id} className={className}>
      {children}
    </Heading>
  );
};

export function Heading({ children, className, id = '', level = 1 }: HeadingNode) {
  const headingLevel = `h${level}` as HeadingLevels;
  // NOTE: The reason we need absolute URL is because algolia search requres it.
  const appURL = config.DOCS_APP_URL;
  const path = usePathname();

  const copyLink = (_e: React.MouseEvent<HTMLAnchorElement>) => {
    const link = `${appURL}${path}#${id}`;
    navigator.clipboard.writeText(link);
  };

  const Link = (
    <a
      id={`anchor-${id}`}
      href={`${appURL}${path}#${id}`}
      onClick={copyLink}
      className={cx(
        'p-0 m-0 inline no-underline focus:outline-none focus:ring-0',
        `heading-level-${headingLevel}`,
      )}
    >
      <HeadingComponent
        id={id}
        level={headingLevel}
        className={cx(
          'heading after:content-["_#"] after:transition-opacity after:font-bold after:text-blue-700 after:ease-in-out after:opacity-0 after:hover:opacity-100',
          className,
        )}
      >
        {children}
      </HeadingComponent>
    </a>
  );

  return Link;
}
