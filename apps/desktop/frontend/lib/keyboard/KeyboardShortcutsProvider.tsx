import { ReactNode, useEffect } from 'react';
import { useWorkbenchStore } from '@/features/workbench/store/workbenchStore';

interface KeyboardShortcutsProviderProps {
  children: ReactNode;
}

function KeyboardShortcutsHandler({ children }: KeyboardShortcutsProviderProps) {
  const { togglePanelVisibility, activatePanel } = useWorkbenchStore();

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Global keyboard shortcuts
      switch (e.key) {
        case 'b':
          if ((e.ctrlKey || e.metaKey) && e.shiftKey) {
            e.preventDefault();
            togglePanelVisibility();
          }
          break;
        case ',':
          if (e.ctrlKey || e.metaKey) {
            e.preventDefault();
            activatePanel('settings');
          }
          break;
        case '1':
          if (e.ctrlKey || e.metaKey) {
            e.preventDefault();
            activatePanel('explorer');
          }
          break;
        case '2':
          if (e.ctrlKey || e.metaKey) {
            e.preventDefault();
            activatePanel('search');
          }
          break;
        case '3':
          if (e.ctrlKey || e.metaKey) {
            e.preventDefault();
            activatePanel('git');
          }
          break;
        case '4':
          if (e.ctrlKey || e.metaKey) {
            e.preventDefault();
            activatePanel('debug');
          }
          break;
        case '5':
          if (e.ctrlKey || e.metaKey) {
            e.preventDefault();
            activatePanel('assistant');
          }
          break;
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [togglePanelVisibility, activatePanel]);

  return <>{children}</>;
}

export function KeyboardShortcutsProvider({ children }: KeyboardShortcutsProviderProps) {
  return <KeyboardShortcutsHandler children={children} />;
}
