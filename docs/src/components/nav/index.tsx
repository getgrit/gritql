import { useState } from 'react';

import cx from 'classnames';
import Link from 'next/link';
import { BiMenu } from 'react-icons/bi';

import { IconButton } from '@/components/buttons';
import { Dialog } from '@/components/dialog';
import { GritLogo } from '@/components/logo';
import { SidebarList } from '@/components/sidebar';
import { NavLink } from '@/custom-types/config';
import config from '@/statics/project';

type NavBarProps = {
  activeSlug: string;
};

type NavBarHead = {
  setIsOpen: (value: boolean) => void;
};

type NavBarLink = {
  tab: NavLink;
  activeSlug: NavBarProps['activeSlug'];
};

type NavBarDialog = {
  isOpen: boolean;
  activeSlug: NavBarProps['activeSlug'];
  setIsOpen: (value: boolean) => void;
};

const isActiveTab = (tab: NavLink, activeSlug: string) => {
  let referenceHref = tab.referenceHref || tab.href;
  if (referenceHref === '/') {
    return activeSlug === '' || activeSlug === '/';
  } else {
    return activeSlug?.startsWith(referenceHref);
  }
};

const Logo = () => (
  <Link href='/' passHref>
    <div className='select-none flex flex-row gap-4 items-center cursor-pointer'>
      <GritLogo className='text-neutral-900 h-10' />
      <b className='hidden sm:block text-md font-semibold flex-grow text-oxfordBlue'>
        Documentation
      </b>
    </div>
  </Link>
);

const NavBarHead = ({ setIsOpen }: NavBarHead) => (
  <div className='flex flex-row px-8 py-3 items-center gap-4'>
    <Logo />
    <div className='print:hidden flex-grow' />
    <div className='print:hidden flex flex-row gap-3 items-center'>
      <div className='hidden sm:flex flex-row items-center gap-5 font-medium text-sm transition'>
        {config.navbar.topLinks.map((link, lIndex) => (
          <Link key={`link-${lIndex}`} href={link.href}>
            <span className='whitespace-nowrap hover:text-neutral-900 text-neutral-600 cursor-pointer'>
              {link.title}
            </span>
          </Link>
        ))}
      </div>
    </div>
    <div className='flex sm:hidden h-full items-center flex-none'>
      <IconButton Icon={BiMenu} ariaLabel='Open menu' onClick={() => setIsOpen(true)} />
    </div>
  </div>
);

const NavBarLink = ({ activeSlug, tab }: NavBarLink) => {
  const isActive = isActiveTab(tab, activeSlug);
  return (
    <Link
      href={tab.href}
      className={cx('whitespace-nowrap border-b-2 text-sm font-medium transition cursor-pointer', {
        'border-primary-500 text-primary-500': isActive,
        'text-neutral-800 border-transparent hover:text-primary-500 ': !isActive,
      })}
    >
      {tab.title}
    </Link>
  );
};

const NavBarDialogLink = ({ activeSlug, tab }: NavBarLink) => {
  const isActive = isActiveTab(tab, activeSlug);
  return (
    <Link
      href={tab.href}
      className={cx(
        'whitespace-nowrap transition hover:text-neutral-900 border-neutral-200 hover:border-neutral-500 pl-4 cursor-pointer',
        {
          'text-neutral-700': !isActive,
          'text-primary-500 font-medium': isActive,
        },
      )}
    >
      {tab.title}
    </Link>
  );
};

const NavBarDialog = ({ activeSlug, isOpen = false, setIsOpen }: NavBarDialog) => (
  <Dialog isOpen={isOpen} onClose={() => setIsOpen(false)}>
    <div className='flex flex-row gap-4 px-4 pt-4 items-center'>
      <Logo />
    </div>
    <div className='flex flex-col gap-3 mb-12'>
      {config.navbar.tabs.map((tab, tIndex) => (
        <NavBarDialogLink key={`tab-${tIndex}`} tab={tab} activeSlug={activeSlug} />
      ))}
    </div>
    <div className='mb-[150px]'>
      <SidebarList activeSlug={activeSlug} />
    </div>
  </Dialog>
);

export const Navbar = ({ activeSlug }: NavBarProps) => {
  const [isOpen, setIsOpen] = useState(false);
  return (
    <header className='bg-neutral-50/80 flex flex-col backdrop-blur border-b border-neutral-100 h-[4rem]'>
      <NavBarHead setIsOpen={setIsOpen} />
      <div className='flex-grow' />
      <div className='flex flex-row w-full px-8 overflow-x-auto hidden-scrollbar gap-6'>
        {config.navbar.tabs.map((tab, tIndex) => (
          <NavBarLink key={`tab-${tIndex}`} tab={tab} activeSlug={activeSlug} />
        ))}
      </div>
      <NavBarDialog isOpen={isOpen} setIsOpen={setIsOpen} activeSlug={activeSlug} />
    </header>
  );
};
