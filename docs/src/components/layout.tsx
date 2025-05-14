import React from 'react';

import cx from 'classnames';

import BlogHeader from '@/components/blog/header';
import { Subscribe } from '@/components/blog/subscribe';
import { StandaloneEditor } from '@/components/editor/standalone-editor';
import { TOC } from '@/components/toc';
import { Frontmatter, isBlogFrontmatter } from '@/custom-types/frontmatter';
import { WithChildren } from '@/custom-types/shared';

export type LayoutContainerProps = WithChildren<{
  frontmatter: Frontmatter;
  details?: {
    gitHubUrl: string;
  };
  showEditorSidebar: boolean;
  showSidebar: boolean;
}>;

export const MainLayoutContainer = ({
  children,
  frontmatter,
  details,
  showEditorSidebar,
  showSidebar,
}: LayoutContainerProps) => {
  return (
    <div className='relative mb-32'>
      <div className='antialiased w-full md:pl-64'>
        <div
          className={cx(
            'overflow-wrap break-words antialiased prose prose-neutral w-full pb-12 mx-auto items-start',
            {
              'flex max-w-none': showEditorSidebar,
              'grid grid-cols-1 lg:grid-cols-4 max-w-[1000px]': !showEditorSidebar,
            },
          )}
        >
          <div
            className={cx('pt-4 px-8', {
              'lg:col-span-3 w-full': !showEditorSidebar,
              'w-full md:w-1/2': showEditorSidebar,
            })}
          >
            {isBlogFrontmatter(frontmatter) ? (
              <BlogHeader frontmatter={frontmatter} />
            ) : (
              frontmatter.title && <h1 id='main-heading'>{frontmatter.title}</h1>
            )}
            {children}
            {isBlogFrontmatter(frontmatter) && (
              <div className='mt-8'>
                <Subscribe cta='Subscribe to updates' />
              </div>
            )}
          </div>
          {showSidebar &&
            (showEditorSidebar ? (
              <div
                className={cx(
                  'animate-slideIn fixed md:sticky w-full md:w-1/2 top-[100px] h-[80vh] md:h-[calc(100vh-128px)] pr-2 pl-2 md:pl-0 lg:pr-5 z-20',
                )}
              >
                <StandaloneEditor />
              </div>
            ) : (
              <>
                <TOC details={details} />
              </>
            ))}
        </div>
      </div>
    </div>
  );
};
