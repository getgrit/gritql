export type BlogFrontmatter = {
  title: string;
  // For blog posts, the date is required, in format YYYY-MM-DD.
  date: string;
  variant: 'blog';
};

export type Frontmatter =
  | {
      title?: string;
    }
  | BlogFrontmatter;

export function isBlogFrontmatter(frontmatter: Frontmatter): frontmatter is BlogFrontmatter {
  return 'variant' in frontmatter && frontmatter.variant === 'blog';
}

/**
 * Parses the frontmatter and returns a normalized version of it (date is a date)
 */
export function normalizedBlogFrontmatter(frontmatter: BlogFrontmatter) {
  return {
    ...frontmatter,
    date: new Date(frontmatter.date),
  };
}
