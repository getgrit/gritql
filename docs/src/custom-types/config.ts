import config from '@/statics/project';

/*
 *
 * Types: Project Config
 * Export any config related types you need within the project from here.
 *
 */

export type SidebarItem = (typeof config.guides.sidebar)[number];

export type NavLink = {
  title: string;
  href: string;
  referenceHref?: string;
};
