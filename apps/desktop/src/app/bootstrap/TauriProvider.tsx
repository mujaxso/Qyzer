import { ReactNode, useEffect } from 'react';

interface TauriProviderProps {
  children: ReactNode;
}

export function TauriProvider({ children }: TauriProviderProps) {
  useEffect(() => {
    // Initialize Tauri-specific setup here
    console.log('Tauri provider mounted');
    
    return () => {
      // Cleanup
    };
  }, []);

  return <>{children}</>;
}
