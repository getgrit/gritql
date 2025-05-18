import React, { createContext, useContext } from 'react';
import { checkFeatureFlag, FeatureFlag } from '@getgrit/universal';

// Create a context for the feature flag provider
export const FeatureFlagContext = createContext<(flag: FeatureFlag) => boolean>(() => false);

// Create a context provider component
export const GritFeatureFlagProvider: React.FC<React.PropsWithChildren<{}>> = ({ children }) => {
  const useFeatureFlag = (flag: FeatureFlag) => {
    const localFlag = checkFeatureFlag(flag);
    if (localFlag) {
      return true;
    }
    try {
      return false;
    } catch (e) {
      return false;
    }
  };

  return (
    <FeatureFlagContext.Provider value={useFeatureFlag}>{children}</FeatureFlagContext.Provider>
  );
};

// Create a hook to use the feature flag
export const useFeatureFlag = (flag: FeatureFlag) => {
  const checkFlag = useContext(FeatureFlagContext);
  return checkFlag(flag);
}; 