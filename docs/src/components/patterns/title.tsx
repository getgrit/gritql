import React from 'react';

import { DocPattern } from 'src/app/(doclike)/(default)/patterns/page';

import { PatternGitHubButton } from './buttons';
import { PatternLanguageButton } from './languages';

export const PatternTitle: React.FC<{
  pattern?: Pick<DocPattern, 'gitHubUrl' | 'language'>;
  children: any;
}> = (props) => {
  return (
    <div className='flex justify-between items-center'>
      <h1 id={'main-heading'}>{props.children}</h1>
      {props.pattern ? (
        <div className='flex gap-2 items-center'>
          <PatternLanguageButton size='lg' pattern={props.pattern} />
          <PatternGitHubButton size='lg' pattern={props.pattern} />
        </div>
      ) : null}
    </div>
  );
};
