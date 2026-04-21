import { create } from 'zustand';
import { devtools } from 'zustand/middleware';

export type PanelId = 'explorer' | 'search' | 'git' | 'debug' | 'assistant' | 'settings';

export interface PanelState {
  // Active panel ID
  activePanel: PanelId | null;
  // Whether the side panel is visible
  isPanelVisible: boolean;
  // Panel width (for future resizing)
  panelWidth: number;
}

export interface PanelActions {
  // Activate a panel (makes it visible if not already)
  activatePanel: (panelId: PanelId) => void;
  // Toggle panel visibility (collapse/expand)
  togglePanelVisibility: () => void;
  // Close the panel
  closePanel: () => void;
  // Set panel width
  setPanelWidth: (width: number) => void;
  // Toggle a specific panel (if already active, close it)
  togglePanel: (panelId: PanelId) => void;
}

const DEFAULT_PANEL_WIDTH = 280;

export const useWorkbenchStore = create<PanelState & PanelActions>()(
  devtools(
    (set, get) => ({
      activePanel: 'explorer',
      isPanelVisible: true,
      panelWidth: DEFAULT_PANEL_WIDTH,

      activatePanel: (panelId) => {
        const { activePanel, isPanelVisible } = get();
        if (activePanel === panelId && isPanelVisible) {
          // Already active and visible, do nothing
          return;
        }
        set({
          activePanel: panelId,
          isPanelVisible: true,
        });
      },

      togglePanelVisibility: () => {
        set((state) => ({
          isPanelVisible: !state.isPanelVisible,
        }));
      },

      closePanel: () => {
        set({
          isPanelVisible: false,
          activePanel: null,
        });
      },

      setPanelWidth: (width) => {
        set({ panelWidth: Math.max(200, Math.min(600, width)) });
      },

      togglePanel: (panelId) => {
        const { activePanel, isPanelVisible } = get();
        if (activePanel === panelId && isPanelVisible) {
          // Toggle off
          set({ isPanelVisible: false });
        } else {
          // Activate this panel
          set({
            activePanel: panelId,
            isPanelVisible: true,
          });
        }
      },
    }),
    {
      name: 'workbench-store',
    }
  )
);
