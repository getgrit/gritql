import { createContext, PropsWithChildren, useContext, useState } from 'react';

export const SidebarContext = createContext({
  showEditorSidebar: false,
  setShowEditorSidebar: (_show: boolean) => {},
});

export const SidebarProvider: React.FC<PropsWithChildren<{}>> = ({ children }) => {
  const [showEditorSidebar, setShowEditorSidebar] = useState(false);

  return (
    <SidebarContext.Provider value={{ showEditorSidebar, setShowEditorSidebar }}>
      {children}
    </SidebarContext.Provider>
  );
};

export const useSidebarContext = () => {
  return useContext(SidebarContext);
};
