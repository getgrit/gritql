import kebabCase from 'lodash/kebabCase';

export const assertUnreachable = (_x: never) => {
  throw new Error("Didn't expect to get here");
};

export const generateHeadingID = (children: any, attributes: any) => {
  if (attributes.id && typeof attributes.id === 'string') {
    return attributes.id;
  }
  return kebabCase(
    children.map((child: any) => {
      if (typeof child === 'string') return child;
      return JSON.stringify(child.children);
    }),
  );
};
