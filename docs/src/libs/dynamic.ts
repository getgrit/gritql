export function doesPathHaveEditor(path: string) {
  return (
    path.includes('/tutorial') ||
    path.includes('/playground') ||
    path.includes('/patterns/library') ||
    path.includes('/patterns/preview') ||
    path.startsWith('/registry')
  );
}
