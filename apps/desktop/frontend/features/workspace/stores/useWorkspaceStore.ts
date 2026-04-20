import { create } from 'zustand';
import { devtools, persist } from 'zustand/middleware';
import { WorkspaceService, type DirectoryEntryDto } from '../services/workspaceService';

// UI-only state types
export interface WorkspaceUI {
  id: string;
  name: string;
  rootPath: string;
}

export interface WorkspaceStoreState {
  // UI state only
  currentWorkspace: WorkspaceUI | null;
  currentDirectory: string | null;
  fileTree: DirectoryEntryDto[];
  isLoading: boolean;
  error: string | null;
  
  // UI actions (no business logic)
  setCurrentWorkspace: (workspace: WorkspaceUI | null) => void;
  setCurrentDirectory: (path: string | null) => void;
  setFileTree: (tree: DirectoryEntryDto[]) => void;
  setLoading: (loading: boolean) => void;
  setError: (error: string | null) => void;
}

/**
 * WorkspaceStore - UI state management only
 * 
 * This store:
 * - Manages UI state (what's selected, loading states, errors)
 * - Does NOT contain business logic
 * - Is updated by feature containers/services
 */
export const useWorkspaceStore = create<WorkspaceStoreState>()(
  devtools(
    persist(
      (set) => ({
        currentWorkspace: null,
        currentDirectory: null,
        fileTree: [],
        isLoading: false,
        error: null,
        
        // Pure UI state setters
        setCurrentWorkspace: (workspace) => set({ currentWorkspace: workspace }),
        setCurrentDirectory: (path) => set({ currentDirectory: path }),
        setFileTree: (tree) => set({ fileTree: tree }),
        setLoading: (loading) => set({ isLoading: loading }),
        setError: (error) => set({ error }),
      }),
      {
        name: 'workspace-ui-storage',
        partialize: (state) => ({
          // Only persist UI state that makes sense across sessions
          currentWorkspace: state.currentWorkspace,
        }),
      }
    )
  )
);
