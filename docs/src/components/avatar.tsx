import { ComponentProps } from 'react';

import Image from 'next/image';

export function Avatar({ src }: { src: ComponentProps<typeof Image>['src'] }) {
  return (
    <span className='relative flex shrink-0 overflow-hidden rounded-full h-12 w-12'>
      <span className='flex h-full w-full items-center justify-center rounded-full bg-muted'>
        <Image src={src} alt='Profile picture of author' width='50' height='50' />
      </span>
    </span>
  );
}
