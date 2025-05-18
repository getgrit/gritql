import React, { useCallback } from 'react';

type Props = React.PropsWithChildren<{
}>;

export const AnalyticsProvider = ({ children, ...props }: Props) => {

  return <>{children}</>;
};

export const useAnalytics = () => {


  const capture = useCallback((event: string) => {}, []);

  return {
    analytics,
    identify: (userId: string) => {
    },
    capture,
    deidentify: () => {
    },
  };
}; 