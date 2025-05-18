import 'server-only';

import { gql } from 'graphql-request';
import { omit, pick } from 'lodash';
import { isObject } from 'lodash';
import { unstable_cache } from 'next/cache';

import config from '@/statics/config';

export const PATTERNS_CACHE_KEY = 'patterns';

export interface ResolvedGritPattern {
  __typename: 'ResolvedGritPattern';
  body: string;
  engine: BaseEngineKind;
  language: Language | 'UNIVERSAL';
  name: string;
  level?: string;
  title: string;
  description?: string;
  tags?: string[];
  body?: string;
  samples?: {
    input: string;
    output: string;
  }[];
  path?: string;
  raw?: {
    format: 'markdown' | 'grit';
    content: string;
  } | null;
  localName: string;
}


export interface EnhancedPattern extends ResolvedGritPattern {
  gitHubUrl: string;
}

const getAllPatterns: () => Promise<EnhancedPattern[]> = async () => {
  const endpoint = config.NEXT_PUBLIC_GRAPHQL_URL;

  // grit-ignore custom_no_console_log
  console.log(`FETCHING REMOTE PATTERNS from ${endpoint}...`);
  try {
    // const client = new ProvoloneServerClient({ endpoint });
    // const query = gql`
    //   query Patterns {
    //     raw_standard_library {
    //       language
    //       data
    //     }
    //   }
    // `;
    // const result = await client.client.request<{
    //   raw_standard_library: {
    //     language: Language;
    //     data: EnhancedPattern[];
    //   }[];
    // }>(query);

    // if (!isObject(result) || !('raw_standard_library' in result)) {
    //   throw new Error('invalid result');
    // }

    // const allPatterns = result.raw_standard_library.flatMap((p) => p.data);

    return [];
  } catch (e) {
    console.error('failed to fetch dynamic patterns', e);
    return [];
  }
};

export const getStdlib = unstable_cache(
  async () => {
    const allPatterns = await getAllPatterns();

    const limitedPatterns = allPatterns.map((p) => {
      return pick(p, ['name', 'body', 'language']);
    });
    return limitedPatterns;
  },
  ['remote_patterns', 'stdlib'],
  {
    tags: [PATTERNS_CACHE_KEY, 'all', 'stdlib'],
    revalidate: false,
  },
);

export const getPatternsList = unstable_cache(
  async () => {
    const allPatterns = await getAllPatterns();
    const filteredPatterns = allPatterns
      .filter((p) => !p.tags?.includes('hidden'))
      .filter((p) => !p.localName.startsWith('_'))
      .filter((p) => p.title && p.title.length > 0);

    const limitedPatterns = filteredPatterns.map((p) => {
      return omit(p, 'raw');
    });
    return limitedPatterns;
  },
  ['remote_patterns', 'stdlib', 'all'],
  {
    tags: [PATTERNS_CACHE_KEY, 'all', 'display'],
    revalidate: false,
  },
);

export const getRemotePattern = unstable_cache(
  async (name: string) => {
    const allPatterns = await getAllPatterns();
    return allPatterns.find((p) => p.name === name);
  },
  ['remote_patterns', 'display', 'single'],
  { tags: [PATTERNS_CACHE_KEY, 'single', 'pattern'] },
);
