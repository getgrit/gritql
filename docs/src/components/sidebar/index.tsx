import cn from 'classnames';
import Link from 'next/link';

import { SidebarItem } from '@/custom-types/config';
import config from '@/statics/project';

import { makeSlug } from './helpers';

type Sidebar = {
  activeSlug: string;
};

type SidebarLink = {
  section: SidebarItem;
  activeSlug: Sidebar['activeSlug'];
  page: (typeof config.guides.sidebar)[0]['pages'][0];
};

const isString = (value: any): value is string => typeof value === 'string';

const SidebarLink = ({ activeSlug, page, section }: SidebarLink) => {
  const isComplex = !isString(page);
  let pageTitle: string;
  let slug: string;
  if (isComplex) {
    pageTitle = page.title;
    slug = page.path;
  } else {
    pageTitle = page;
    slug = makeSlug(section.title, page, '');
  }
  const active = slug === activeSlug;
  return (
    <Link href={slug} className={cn('text-base sm:text-sm px-4 transition')} passHref>
      <div
        className={cn('border-l px-3 py-1.5 transition  cursor-pointer', {
          'text-neutral-700 hover:text-neutral-900 border-neutral-200 hover:border-neutral-500 ':
            !active,
          'text-primary-500 border-primary-500 font-semibold': active,
        })}
      >
        {pageTitle}
      </div>
    </Link>
  );
};

export const SidebarList = ({ activeSlug }: Sidebar) => {
  return (
    <div className='flex flex-col flex-grow gap-8'>
      {config.guides.sidebar.map((section, sIndex) => (
        <div className='flex flex-col' key={`page-${sIndex}`}>
          {section.title && (
            <p className='text-base sm:text-sm font-semibold px-4 mb-2'>{section.title}</p>
          )}
          {section.pages.map((page, pIndex) => (
            <SidebarLink
              key={`page-${pIndex}`}
              page={page}
              section={section}
              activeSlug={activeSlug}
            />
          ))}
        </div>
      ))}
    </div>
  );
};

export const Sidebar = ({ activeSlug }: Sidebar) => {
  return (
    <div className='z-10 hidden md:block fixed inset-0 w-0 md:w-64 pt-20'>
      <div className='absolute w-full'>
        <div className='h-6 bg-gradient-to-b from-white to-white/0 ' />
      </div>
      <div className='overflow-y-auto h-full pb-10 pt-[49px] hidden-scrollbar pl-4'>
        <SidebarList activeSlug={activeSlug} />
      </div>
    </div>
  );
};
