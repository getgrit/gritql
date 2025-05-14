import { WithChildren } from '@/custom-types/shared';
import { useFeatureFlag } from '@getgrit/shared';
import { FeatureFlag } from '@getgrit/universal';

type GateProps = WithChildren<{
  flag: FeatureFlag;
}>;

export const GateTag = (props: GateProps) => {
  const flag = useFeatureFlag(props.flag);
  if (!flag) {
    return null;
  }

  return <>{props.children}</>;
};
