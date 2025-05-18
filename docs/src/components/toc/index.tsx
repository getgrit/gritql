'use client';

import { createContext, useCallback, useContext, useEffect, useMemo, useState } from 'react';

import cx from 'classnames';
import { debounce } from 'lodash';
import { FiExternalLink } from 'react-icons/fi';

import { WithChildren } from '@/custom-types/shared';

import { HeadingItem, useHeadingsData } from './lib';
import { getClassName } from './lib';
import { useIntersectionObserver } from './observer';

type TOCContextType = {
  currentHash: string | undefined;
  activeId: string;
  isParentToActive: (id: string) => boolean;
};

const TOCContext = createContext<TOCContextType | null>(null);

const getParents = (
  entries: HeadingItem[],
  id: string,
  saveIds: string[],
): string[] | undefined => {
  if (!entries) {
    return undefined;
  }

  if (entries.find((e) => e.id === id)) {
    return saveIds;
  }

  for (const entry of entries) {
    const parents = getParents(entry.items, id, [...saveIds, entry.id])?.filter(Boolean);
    if (parents) {
      return parents;
    }
  }

  return undefined;
};

type TOCProviderProps = WithChildren<{
  entries: any;
  activeId: string;
}>;

const TOCProvider = ({ activeId, children, entries }: TOCProviderProps) => {
  const [currentHash, setCurrentHash] = useState<string>();

  useEffect(() => {
    const handleHashChange = () => {
      setCurrentHash(window.location.hash);
    };
    window.addEventListener('hashchange', handleHashChange, false);
    handleHashChange();
    return () => {
      window.removeEventListener('hashchange', handleHashChange, false);
    };
  }, []);

  const isParentToActive = useCallback(
    (id: string) => {
      return !!getParents(entries, activeId, [])?.includes(id);
    },
    [activeId, entries],
  );

  const value = {
    activeId,
    currentHash,
    isParentToActive,
  };

  return <TOCContext.Provider value={value}>{children}</TOCContext.Provider>;
};

const useTOC = () => {
  const context = useContext(TOCContext);
  if (!context) {
    throw new Error('useTOC must be used within a TOCProvider');
  }
  return context;
};

type WithIndentationProps = WithChildren<{
  id: string;
  depth: number;
  isActive: boolean;
}>;

const WithIndentation = ({ children, depth, id, isActive }: WithIndentationProps) => {
  let cs = {
    className: '',
  };
  if (depth > 0) {
    const item = isActive ? 'lineActive' : 'lineInactive';
    cs = getClassName(item, item);
  }
  return (
    <div id={id} className='flex flex-row'>
      {depth > 0 && <div className='w-4 flex-none' />}
      <div className={cx(`${cs.className} flex-grow`, { 'pl-2': depth > 0 })}>{children}</div>
    </div>
  );
};

type ItemProps = WithChildren<{
  id: string;
  title: string;
  href: string;
  className?: string;
  depth?: number;
}>;

const Item = ({ children, className, depth = 0, href, id, title }: ItemProps) => {
  const { activeId } = useTOC();
  const shouldHighlight = activeId === id;
  const baseItem = depth === 0 ? 'topItem' : 'item';
  const fallbackBaseItem = 'item';

  const cs = getClassName(
    baseItem + (shouldHighlight ? 'Active' : 'Inactive'),
    fallbackBaseItem + (shouldHighlight ? 'Active' : 'Inactive'),
    cx('block', className, {
      'cursor-pointer': href,
    }),
  );

  return (
    <WithIndentation id={`toc-link-${href}`} depth={depth} isActive={shouldHighlight}>
      <div className='flex flex-col'>
        {title && (
          <a className={cs.className} href={href}>
            {title}
          </a>
        )}
        {children}
      </div>
    </WithIndentation>
  );
};

type TreeItemProps = {
  item: HeadingItem;
  depth?: number;
};

const TreeItem = ({ depth = 0, item }: TreeItemProps) => {
  if (item.items?.length > 0) {
    return (
      <Item id={item.id} title={item.title} href={item.href} depth={depth || 0}>
        {item.items.map((childItem) => (
          <TreeItem key={`${item.id}-${childItem.id}`} item={childItem} depth={(depth || 0) + 1} />
        ))}
      </Item>
    );
  } else {
    return <Item id={item.id} title={item.title} href={item.href} depth={depth || 0} />;
  }
};

type TreeProps = {
  entries: HeadingItem[];
  activeId: string;
  expandAll?: boolean;
};

const Tree = ({ activeId, entries = [] }: TreeProps) => {
  return (
    <TOCProvider entries={entries} activeId={activeId}>
      <div className='flex flex-col gap-1 overflow-hidden hidden-scrollbar'>
        {entries?.map((entry) => (
          <TreeItem key={entry.id} item={entry} />
        ))}
      </div>
    </TOCProvider>
  );
};

export const TOC: React.FC<{ details?: { gitHubUrl: string } }> = (props) => {
  const [activeId, setActiveId] = useState('');
  const { nestedHeadings } = useHeadingsData();

  useEffect(() => {
    const contentId = window.location.hash.substring(1);
    const mainHeading = document.getElementById(contentId);
    if (mainHeading) mainHeading.scrollIntoView();
  }, []);

  const updateHistory = (hash: string) => {
    if (history.replaceState) {
      history.replaceState(null, '', `#${hash}`);
    } else {
      location.hash = `#${activeId}`;
    }
  };

  // eslint-disable-next-line react-hooks/exhaustive-deps
  const handleActiveId = useMemo(() => debounce(updateHistory, 500), []);
  useIntersectionObserver(setActiveId);

  useEffect(() => {
    if (!activeId) return;
    if (activeId === 'main-heading') return;
    handleActiveId(activeId);
  }, [activeId, handleActiveId]);

  if (!nestedHeadings || nestedHeadings.length === 0) {
    return null;
  }

  return (
    <div className='sticky z-20 top-24 overflow-y-auto hidden lg:block'>
      <h4 className='font-bold mt-4'>On this page</h4>
      <Tree entries={nestedHeadings} activeId={activeId} />
      <div className='mt-3 space-y-2 border-t border-gray-200 pt-5 text-sm text-gray-900 dark:border-gray-300'></div>
      {props.details && (
        <a
          href={props.details.gitHubUrl}
          className={
            getClassName('itemInactive', 'item', 'flex justify-between content-center').className
          }
        >
          <div>Edit on GitHub</div>
          <div>
            <FiExternalLink size='1.1rem' />
          </div>
        </a>
      )}
    </div>
  );
};
