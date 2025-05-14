import type { Metadata, ResolvingMetadata } from 'next';
import { notFound } from 'next/navigation';

import { parsePlainText, renderPlainText } from '@/libs/markdown';
import { getPatternsList, getRemotePattern } from '@/libs/patterns';
import { WrapperContainer } from '@/templates/wrapper';
import { utils } from '@getgrit/api';

import { MarkdownPatternPage } from './render';

type PageProps = {
  params: {
    pattern: string;
  };
};

async function getPattern(props: PageProps) {
  const patternName = props.params.pattern;
  const pattern = await getRemotePattern(patternName);
  if (!pattern) {
    notFound();
  }

  return pattern;
}

export async function generateStaticParams() {
  const patterns = await getPatternsList();

  return patterns.map((pattern) => ({
    pattern: pattern.name,
  }));
}

export async function generateMetadata(
  props: PageProps,
  _parent: ResolvingMetadata,
): Promise<Metadata> {
  const pattern = await getPattern(props);

  return {
    title: `${utils.getPatternTitle(pattern)}`,
    openGraph: {
      title: utils.getPatternTitle(pattern),
      description: renderPlainText(parsePlainText(utils.getPatternDescription(pattern))),
      type: 'website',
      url: `https://docs.grit.io/patterns/library/${pattern.name}`,
      siteName: 'Grit',
    },
  };
}

export default async function PatternPage(props: PageProps) {
  const pattern = await getPattern(props);

  const markdown =
    pattern.raw && pattern.raw.format === 'markdown'
      ? pattern.raw.content
      : utils.getPatternDescription(pattern);

  return (
    <>
      <WrapperContainer
        details={pattern.gitHubUrl ? { gitHubUrl: pattern.gitHubUrl } : undefined}
        frontmatter={{}}
      >
        <MarkdownPatternPage markdown={markdown} patternInfo={pattern} />
      </WrapperContainer>
    </>
  );
}
