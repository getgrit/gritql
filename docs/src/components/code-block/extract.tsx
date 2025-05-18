import { FenceProps } from './monaco';

export function extractCodeString(children: FenceProps | FenceProps[] | string): string {
  if (typeof children === 'string') {
    return children;
  }

  if ('props' in children) {
    return extractCodeString(children.props.children);
  }

  return children
    .map((child) => {
      if (typeof child === 'string') {
        return child;
      }
      return extractCodeString(child.props.children);
    })
    .join('\n');
}
