'use client';

import { MainLayoutContainer } from '@/components/layout';
import { Frontmatter } from '@/custom-types/frontmatter';
import { WithChildren } from '@/custom-types/shared';
import { useSidebarContext } from '@/hooks/sidebar';
import { doesPathHaveEditor } from '@/libs/dynamic';

import { WorkerWrapper } from './worker-wrapper';

export type MainContainerProps = WithChildren<{
  activeSlug: string;
  frontmatter: Frontmatter;
  details?: {
    gitHubUrl: string;
  };
}>;

const OVERVIEW_PAGES = ['/', '/blog', '/language/overview'];

export const MainContainer = ({
  activeSlug,
  children,
  frontmatter,
  details,
}: MainContainerProps) => {
  const isTutorial = doesPathHaveEditor(activeSlug);
  const { showEditorSidebar } = useSidebarContext();
  const showSidebar = isTutorial || !OVERVIEW_PAGES.includes(activeSlug);

  const main = (
    <MainLayoutContainer
      frontmatter={frontmatter}
      showSidebar={showSidebar}
      showEditorSidebar={showEditorSidebar}
      details={details}
    >
      {children}
    </MainLayoutContainer>
  );

  return <WorkerWrapper>{main}</WorkerWrapper>;
};
