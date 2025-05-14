const normalizePath = (path: string) => {
  return path.toLowerCase().replace(/\s+/g, '-');
};

const getPagePathString = (titleInfo: string | string[]) => {
  if (typeof titleInfo === 'string') {
    return titleInfo;
  } else if (titleInfo.length > 1) {
    return titleInfo[1];
  }
  return '';
};

export const makeSlug = (
  section: string | undefined,
  title: string | string[],
  basePath: string,
) => {
  const path = getPagePathString(title);
  return normalizePath(`${basePath}/${section ? section + '/' : ''}${path}`).replace(/\/$/, '');
};
