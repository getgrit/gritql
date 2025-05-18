import { useEffect, useState } from 'react';

export type Theme = { [key: string]: string };
export const defaultTheme: Theme = {
  heading: 'pt-1 font-medium text-sm text-neutral-900',
  item: 'pt-1 text-sm no-underline',
  itemActive: 'pt-1 font-bold text-sm text-neutral-900 no-underline',
  itemInactive:
    'pt-1 font-normal text-sm text-neutral-500 no-underline hover:text-neutral-900 hover:underline',
  lineActive: '',
  lineInactive: '',
};

export type HeadingItem = {
  id: string;
  title: string;
  href: string;
  items: HeadingItem[];
};

export const getHeadingsSelector = () => {
  const indices = [1, 2, 3];
  return indices.map((i) => `h${i}`).join(', ');
};

export const getNestedHeadings = (headingElements: HTMLElement[]) => {
  const nestedHeadings: HeadingItem[] = [];
  headingElements.forEach((heading) => {
    const { id, innerText: title } = heading;
    if (heading.nodeName === 'H2') {
      nestedHeadings.push({ href: `#${id}`, id: id, items: [], title: title });
    } else if (heading.nodeName === 'H3' && nestedHeadings.length > 0) {
      nestedHeadings[nestedHeadings.length - 1]?.items.push({
        href: `#${id}`,
        id: id,
        items: [],
        title: title,
      });
    }
  });
  return nestedHeadings;
};

export const useHeadingsData = () => {
  const [nestedHeadings, setNestedHeadings] = useState<HeadingItem[]>([]);

  useEffect(() => {
    const updateHeadings = () => {
      const headingElements = Array.from(document.querySelectorAll(getHeadingsSelector()));
      const newNestedHeadings = getNestedHeadings(headingElements as HTMLElement[]);
      setNestedHeadings(newNestedHeadings);
    };
    updateHeadings();

    const observer = new MutationObserver(updateHeadings);
    observer.observe(document.body as Node, {
      attributes: true,
      childList: true,
      subtree: true,
    });

    return () => {
      observer.disconnect();
    };
  }, []);

  return { nestedHeadings };
};

export const getClassName = (item: string, fallbackItem: string, className?: string) => {
  const themeItem = getThemeItem(item, fallbackItem);
  return {
    className: `${className} ${themeItem}`,
  };
};

const getThemeItem = (item: string, fallbackItem: string) => {
  if (item in defaultTheme) {
    return defaultTheme[item];
  } else {
    return defaultTheme[fallbackItem];
  }
};
