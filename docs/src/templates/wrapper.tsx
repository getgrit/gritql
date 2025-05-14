'use client';

import { usePathname } from 'next/navigation';

import { MainContainer, MainContainerProps } from './container';

export const WrapperContainer: React.FC<
  React.PropsWithChildren<Pick<MainContainerProps, 'details' | 'frontmatter'>>
> = ({ children, details, frontmatter }) => {
  const pathname = usePathname();
  return (
    <MainContainer activeSlug={pathname ?? ''} frontmatter={frontmatter} details={details}>
      {children}
    </MainContainer>
  );
};
