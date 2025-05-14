import { ForbiddenRegistry } from '@/components/forbidden';
import { fetchRemotePattern, isUrlTrusted } from '@/libs/registry';
import { WrapperContainer } from '@/templates/wrapper';

import { MarkdownPatternPage } from '../library/[pattern]/render';

type PageProps = {
  params: {};
  searchParams: {
    url: string;
  };
};

async function getRemoteDoc(props: PageProps) {
  const remoteUrl = props.searchParams.url;
  if (!remoteUrl) {
    return {
      error: 'No remote URL provided, please add `?url=...` to the URL.',
    };
  }

  if (!isUrlTrusted(remoteUrl)) {
    // TODO: decide if we can relax this concern.
    return {
      error: 'For safety, only URLs from the Grit stdlib are allowed',
    };
  }

  return fetchRemotePattern(remoteUrl);
}

export default async function PreviewPage(props: PageProps) {
  const content = await getRemoteDoc(props);
  if ('error' in content) {
    return <ForbiddenRegistry error={content.error ?? 'Unknown error'} />;
  }

  const { markdown } = content;

  return (
    <>
      <WrapperContainer details={undefined} frontmatter={{}}>
        <div className='relative pr-3'>
          <h1 id={'main-heading'}>Preview pattern</h1>
        </div>
        <MarkdownPatternPage markdown={markdown} />
      </WrapperContainer>
    </>
  );
}
