'use client';

import Link from 'next/link';

import { Avatar } from '@/components/avatar';
import { BlogFrontmatter, normalizedBlogFrontmatter } from '@/custom-types/frontmatter';

export default function BlogHeader(props: { frontmatter: BlogFrontmatter }) {
  const frontmatter = normalizedBlogFrontmatter(props.frontmatter);
  return (
    <>
      <Link
        href='/blog'
        className='flex items-center space-x-2 mb-2 no-underline text-gray-500 hover:text-blue-500 hover:underline'
      >
        <svg
          xmlns='http://www.w3.org/2000/svg'
          fill='none'
          viewBox='0 0 24 24'
          strokeWidth='1.5'
          stroke='currentColor'
          className='w-6 h-6'
        >
          <path
            strokeLinecap='round'
            strokeLinejoin='round'
            d='M10.5 19.5L3 12m0 0l7.5-7.5M3 12h18'
          />
        </svg>
        <span className='ml-2'>Back to Blog</span>
      </Link>
      <article className='flex flex-col space-y-4'>
        <h1 className='text-4xl font-bold text-gray-800 dark:text-gray-200'>{frontmatter.title}</h1>
        <div className='flex items-center space-x-4'>
          <Avatar src='/morgante.jpeg' />
          <div className='flex flex-col'>
            <span className='text-lg font-medium text-gray-700 dark:text-gray-300'>
              Morgante Pell
            </span>
            <span className='text-sm text-gray-500 dark:text-gray-400'>
              {frontmatter.date.toLocaleDateString('en-US', {
                year: 'numeric',
                month: 'long',
                day: 'numeric',
              })}
            </span>
          </div>
        </div>
      </article>
    </>
  );
}
