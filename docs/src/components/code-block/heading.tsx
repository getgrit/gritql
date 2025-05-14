import { Heading } from '@getgrit/shared';

type HeadingProps = {
  fileName?: string;
  title: string;
};

export const SnippetHeading = ({ fileName, title }: HeadingProps) => {
  const iconClassName = 'text-gray-400 w-4 h-4 self-center mr-1 inline-block';
  const headingClassName = 'flex text-gray-400 w-full text-xs items-center';
  return (
    <Heading
      fileName={fileName}
      title={title}
      iconClassName={iconClassName}
      headingClassName={headingClassName}
    />
  );
};
