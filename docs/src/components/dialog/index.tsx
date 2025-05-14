import { Fragment } from 'react';
import { useEffect, useRef } from 'react';

import { Dialog as HUIDialog, Transition } from '@headlessui/react';
import { clearAllBodyScrollLocks, disableBodyScroll } from 'body-scroll-lock';
import { BiX } from 'react-icons/bi';

import { IconButton } from '@/components/buttons';
import { WithChildren } from '@/custom-types/shared';

type DialogProps = WithChildren<{
  isOpen: boolean;
  onClose: () => void;
}>;

export const Dialog = ({ children, isOpen, onClose }: DialogProps) => {
  const animations = {
    enter: 'ease-out duration-300',
    enterFrom: 'opacity-0 translate-x-[-400px]',
    enterTo: 'opacity-100 translate-x-0',
    leave: 'ease-in duration-200',
    leaveFrom: 'opacity-100 translate-x-0',
    leaveTo: 'opacity-0 translate-x-[-400px]',
  };
  return (
    <Transition show={isOpen} as={Fragment}>
      <HUIDialog open={isOpen} onClose={onClose}>
        <Transition.Child
          as={Fragment}
          enter='ease-out duration-300'
          enterFrom='opacity-0'
          enterTo='opacity-100'
          leave='ease-in duration-200'
          leaveFrom='opacity-100'
          leaveTo='opacity-0'
        >
          <div className='z-50 fixed inset-0 bg-black/20 backdrop-blur-md' />
        </Transition.Child>
        <Transition.Child as={Fragment} {...animations}>
          <div className='z-50 fixed inset-y-0 left-0 w-4/5 max-w-[400px] top-0 right-0 origin-left'>
            <HUIDialog.Panel className='relative w-full bg-white h-screen'>
              <div className='absolute z-50 right-4 top-4'>
                <IconButton Icon={BiX} onClick={onClose} ariaLabel='Close menu' />
              </div>
              <ScrollContainer>{children}</ScrollContainer>
            </HUIDialog.Panel>
          </div>
        </Transition.Child>
      </HUIDialog>
    </Transition>
  );
};

const ScrollContainer = ({ children }: WithChildren<{}>) => {
  const ref = useRef<HTMLDivElement>(null);

  useEffect(() => {
    if (!ref.current) return;
    disableBodyScroll(ref.current);

    return () => {
      clearAllBodyScrollLocks();
    };
  }, []);

  return (
    <div ref={ref} className='flex flex-col h-screen overflow-y-auto hidden-scrollbar'>
      {children}
    </div>
  );
};
