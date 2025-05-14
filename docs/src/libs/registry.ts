import { notFound } from 'next/navigation';

import { Repo } from '@getgrit/universal';

const trustedRepos = [
  ['github.com', 'fabian-hiller'],
  ['github.com', 'getgrit'],
  ['github.com', 'cloudflare'],
  ['github.com', 'e2b-dev'],
] as const;

export function isRepoTrusted(repo: Repo) {
  return trustedRepos.some(
    (trusted) => trusted[0] === repo.host && repo.full_name.startsWith(trusted[1]),
  );
}

export function isUrlTrusted(url: string) {
  return trustedRepos.some((trusted) =>
    url.startsWith(`https://raw.githubusercontent.com/${trusted[1]}`),
  );
}

export async function fetchRemotePattern(remoteUrl: string) {
  try {
    const request = await fetch(remoteUrl);
    if (!request.ok) {
      notFound();
      return { error: `Remote doc not found: ${remoteUrl}` };
    }

    const markdown = await request.text();

    return { markdown };
  } catch (error) {
    return {
      error: `Error fetching remote doc: ${JSON.stringify(error)}`,
    };
  }
}
