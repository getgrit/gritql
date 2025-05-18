import { notFound } from 'next/navigation';

import { ForbiddenRegistry } from '@/components/forbidden';
import { fetchRemotePattern, isRepoTrusted } from '@/libs/registry';
import { WrapperContainer } from '@/templates/wrapper';
import { makeRepo } from '../../../universal';

import { MarkdownPatternPage } from '../../patterns/library/[pattern]/render';

type PageProps = {
  params: {
    path: string[];
  };
};

function parseProps(props: PageProps) {
  const path = props.params.path;
  if (path.length < 4) {
    return null;
  }

  const hostName = path[0]!;
  const repoName = path.slice(1, path.length - 1).join('/');
  const pattern = path[path.length - 1]!;

  const repo = makeRepo(repoName, hostName);

  return {
    repo,
    pattern,
  };
}

export default async function PreviewPage(props: PageProps) {
  const parsed = parseProps(props);
  if (!parsed) {
    notFound();
    return null;
  }

  const { repo, pattern } = parsed;
  if (!isRepoTrusted(repo)) {
    return (
      <ForbiddenRegistry
        error={
          <>
            Sorry, <code>{repo.full_name}</code> is not trusted yet.
          </>
        }
      />
    );
    return null;
  }

  const remoteUrl = `https://raw.githubusercontent.com/${repo.full_name}/main/.grit/patterns/${pattern}.md`;

  const content = await fetchRemotePattern(remoteUrl);
  if ('error' in content) {
    return <ForbiddenRegistry error={content.error} />;
  }

  const { markdown } = content;

  const gitHubUrl = `https://github.com/${repo.full_name}/blob/main/.grit/patterns/${pattern}.md`;
  const details = {
    gitHubUrl,
    name: pattern,
    repo: {
      fullName: repo.full_name,
      host: repo.host,
    },
  };

  return (
    <>
      <WrapperContainer details={details} frontmatter={{}}>
        <MarkdownPatternPage markdown={markdown} patternInfo={details} />
      </WrapperContainer>
    </>
  );
}
