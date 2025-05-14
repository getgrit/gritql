import { useCallback, useEffect } from 'react';

type SetIsBottom = (bottom: boolean) => void;

const useScrollEnd = (setIsBottom: SetIsBottom) => {
  const onScroll = useCallback(() => {
    const scrollNode: Element = document.scrollingElement || document.documentElement;
    const scrollContainerEndPosition = Math.round(scrollNode.scrollTop + window.innerHeight);
    const scrollPosition = Math.round(scrollNode.scrollHeight);
    if (scrollPosition <= scrollContainerEndPosition) {
      setIsBottom(true);
    } else {
      setIsBottom(false);
    }
  }, [setIsBottom]);

  useEffect(() => {
    window.addEventListener('scroll', onScroll);
    return () => {
      window.removeEventListener('scroll', onScroll);
    };
  }, [onScroll]);
};

export default useScrollEnd;
