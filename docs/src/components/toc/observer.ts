import { useEffect, useRef, useState } from 'react';

import { minBy } from 'lodash';

import useScrollEnd from '@/hooks/use-scroll-end';

import { getHeadingsSelector } from './lib';

type HeadingElementsMap = {
  [key: string]: IntersectionObserverEntry;
};

export const useIntersectionObserver = (setActiveId: (id: string) => void) => {
  const [headingElements, setHeadingElements] = useState<Element[]>([]);
  const headingElementsRef = useRef<HeadingElementsMap>({});
  const [isBottom, setIsBottom] = useState(false);
  useScrollEnd(setIsBottom);

  useEffect(() => {
    const updateHeadings = () => {
      const elements = Array.from(document.querySelectorAll(getHeadingsSelector()));
      setHeadingElements(elements);
    };
    updateHeadings();
    const observer = new MutationObserver(updateHeadings);
    observer.observe(document.body, {
      attributes: true,
      childList: true,
      subtree: true,
    });
    return () => {
      observer.disconnect();
    };
  }, []);

  useEffect(() => {
    const onChange = (headings: IntersectionObserverEntry[]) => {
      headingElementsRef.current = headings.reduce((map: HeadingElementsMap, headingElement) => {
        map[headingElement.target.id] = headingElement;
        return map;
      }, headingElementsRef.current);

      const visibleHeadings: IntersectionObserverEntry[] = [];
      const domHeadingIds = (headingElements || []).map((h) => h.id);
      Object.keys(headingElementsRef.current)
        .filter((key) => domHeadingIds.includes(key))
        .forEach((key) => {
          const headingElement = headingElementsRef.current[key];
          if (headingElement?.isIntersecting) {
            visibleHeadings.push(headingElement);
          }
        });

      // NOTE[chai]: always highlight last item when at the bottom.
      if (isBottom && visibleHeadings.length > 0) {
        const [lastHeading] = visibleHeadings.slice(-1);
        if (!lastHeading) return;
        setActiveId(lastHeading.target.id);
        return;
      }
      if (visibleHeadings.length === 1) {
        setActiveId(visibleHeadings[0]!.target.id);
        return;
      }
      if (visibleHeadings.length > 1) {
        const heading = minBy(visibleHeadings, (h) => {
          return headingElements.findIndex((heading) => heading.id === h.target.id);
        });
        if (!heading) return;
        setActiveId(heading.target.id);
        return;
      }
    };

    const observer = new IntersectionObserver(onChange, {
      root: document.querySelector('[data-scroll-root]'),
      rootMargin: '0px',
      threshold: 0.9,
    });
    headingElements.forEach((element) => observer.observe(element));
    return () => {
      observer.disconnect();
    };
  }, [setActiveId, headingElements, isBottom]);
};
