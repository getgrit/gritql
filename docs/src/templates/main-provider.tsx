import React, { createContext, useContext, useState } from 'react';

type MainContextType = {
  isFirstTry: boolean;
  setIsFirstTry: React.Dispatch<React.SetStateAction<boolean>>;
};

const MainContext = createContext<MainContextType | undefined>(undefined);

export const MainProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [isFirstTry, setIsFirstTry] = useState(true);

  return (
    <MainContext.Provider value={{ isFirstTry, setIsFirstTry }}>{children}</MainContext.Provider>
  );
};

export const useMainContext = (): MainContextType => {
  const context = useContext(MainContext);
  if (context === undefined) {
    throw new Error('useMainContext must be used within a MainContextType');
  }
  return context;
};
