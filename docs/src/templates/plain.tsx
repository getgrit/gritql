import { GritIcon } from '@/components/logo';
import { Navbar } from '@/components/nav';
import { Sidebar } from '@/components/sidebar';
import { WithChildren } from '@/custom-types/shared';
import { SidebarProvider } from '@/hooks/sidebar';

import { MainProvider } from './main-provider';

export const meta = {
  name: 'Plain',
};

export type TemplateProps = WithChildren<{
  path: string;
  layout: 'docs' | 'full';
}>;

const Footer = ({}) => (
  <div className='bg-neutral-50/80 backdrop-blur py-10 border-t border-neutral-100'>
    <GritIcon className='text-neutral-300 w-10 h-10 mx-auto' />
  </div>
);

export const Template = ({ children, path, layout }: TemplateProps) => {
  return (
    <MainProvider>
      <div className='fixed top-0 left-0 w-full z-50'>
        <Navbar activeSlug={path} />
      </div>
      <div className='relative min-h-screen py-24'>
        <SidebarProvider>
          {layout === 'docs' ? <Sidebar activeSlug={path} /> : null}
          {children}
        </SidebarProvider>
      </div>
      <Footer />
    </MainProvider>
  );
};
