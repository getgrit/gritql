import { WrapperContainer } from '@/templates/wrapper';

export const ForbiddenRegistry: React.FC<{ error: React.ReactNode }> = ({ error }) => {
  return (
    <WrapperContainer details={undefined} frontmatter={{}}>
      <div className='relative pr-3'>
        <h1 id={'main-heading'}>Pattern registry</h1>
      </div>
      <div className='p-4 text-red-700 bg-red-100 rounded-md mt-2'>
        <p>{error}</p>
      </div>
    </WrapperContainer>
  );
};
