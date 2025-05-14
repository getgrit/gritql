import { WrapperContainer } from '@/templates/wrapper';

export default async function DefaultLayout({ children }: { children: React.ReactNode }) {
  return <WrapperContainer frontmatter={{}}>{children}</WrapperContainer>;
}
