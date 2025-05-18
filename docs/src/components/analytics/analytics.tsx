import { AnalyticsBrowser } from '@segment/analytics-next';
import React, { useCallback } from 'react';

const AnalyticsContext = React.createContext<AnalyticsBrowser>(undefined!);

type Props = React.PropsWithChildren<{
  writeKey: string;
}>;

export const AnalyticsProvider = ({ children, writeKey, ...props }: Props) => {
  const analytics = React.useMemo(() => AnalyticsBrowser.load({ writeKey }), [writeKey]);

  return <AnalyticsContext.Provider value={analytics}>{children}</AnalyticsContext.Provider>;
};

export const useAnalytics = () => {
  const analytics = React.useContext(AnalyticsContext);
  if (!analytics) {
    throw new Error('Context used outside of its Provider!');
  }

  const capture = useCallback((event: string) => {}, []);

  return {
    analytics,
    identify: (userId: string) => {
      analytics.identify(userId);
    },
    capture,
    deidentify: () => {
      analytics.reset();
    },
  };
}; 