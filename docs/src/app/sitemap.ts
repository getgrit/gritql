import { MetadataRoute } from 'next';

import { getPatternsList } from '@/libs/patterns';

import project from '../statics/project';

export default async function sitemap(): Promise<MetadataRoute.Sitemap> {
  const baseUrl = 'https://docs.grit.io';
  const currentDate = new Date();

  const sitemapEntries: MetadataRoute.Sitemap = [];

  const addedUrls = new Set<string>();

  // Function to add a unique entry to the sitemap
  const addUniqueEntry = (path: string, priority: number) => {
    const canonicalPath = path.endsWith('/') ? path.slice(0, -1) : path;
    const fullUrl = `${baseUrl}${canonicalPath}`;
    if (!addedUrls.has(fullUrl) && !path.startsWith('http')) {
      sitemapEntries.push({
        url: fullUrl,
        lastModified: currentDate,
        changeFrequency: 'daily',
        priority,
      });
      addedUrls.add(fullUrl);
    }
  };

  // Add entries for each page in the sidebar
  project.guides.sidebar.forEach((section) => {
    section.pages.forEach((page) => {
      addUniqueEntry(page.path, 0.8);
    });
  });

  // Add entries for top links in the navbar
  project.navbar.topLinks.forEach((link) => {
    if (link.href.startsWith('/')) {
      addUniqueEntry(link.href, 0.7);
    }
  });

  // Add entries for all patterns
  const patterns = await getPatternsList();

  patterns.forEach((pattern) => {
    addUniqueEntry(`/patterns/library/${pattern.name}`, 0.6);
  });

  return sitemapEntries;
}
