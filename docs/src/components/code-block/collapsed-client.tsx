'use client';

import { useEffect } from 'react';

import { FaPlay } from 'react-icons/fa';

import { useSidebarContext } from '@/hooks/sidebar';
import { useStandaloneEditor } from '@getgrit/editor';

import { TryButton } from './buttons';

export const CollapsedEditorPlaceholder: React.FC<{ pattern: string }> = ({ pattern }) => {
  const { setPattern, pattern: contextPattern } = useStandaloneEditor();
  const { setShowEditorSidebar } = useSidebarContext();

  useEffect(() => {
    if (!contextPattern || contextPattern === '') {
      setPattern(pattern);
    }
  }, [pattern, contextPattern, setPattern]);

  const onTry = () => {
    setPattern(pattern);
    setShowEditorSidebar(true);
  };

  return (
    <TryButton
      onClick={onTry}
      className='tracking-wide px-4 py-3 mt-4 text-sm font-semibold tracking-normal leading-4 rounded-md'
    >
      <FaPlay size={7} /> Try it on your code
    </TryButton>
  );
};
