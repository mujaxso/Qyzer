import { bridge } from '@/lib/bridge';

// Domain types (mirror Rust DTOs)
export interface OpenWorkspaceRequest {
  path: string;
}

export interface OpenWorkspaceResponse {
  workspaceId: string;
  rootPath: string;
  fileCount: number;
}

export interface ListDirectoryRequest {
  path: string;
}

export interface DirectoryEntryDto {
  path: string;
  name: string;
  isDir: boolean;
  fileType?: string;
  size?: number;
  modified?: string;
}

export interface OpenFileRequest {
  path: string;
}

export interface OpenFileResponse {
  content: string;
  language?: string;
}

export interface SaveFileRequest {
  path: string;
  content: string;
}

export interface OpenDialogResponse {
  selectedPath?: string;
}

// Explorer-specific types
export interface ExplorerTreeNode {
  id: string;
  path: string;
  name: string;
  isDir: boolean;
  fileType?: string;
  size?: number;
  modified?: string;
  children?: ExplorerTreeNode[];
  parentPath?: string;
}

export interface WorkspaceTreeRequest {
  workspaceId: string;
  rootPath: string;
}

export interface WorkspaceTreeResponse {
  workspaceId: string;
  rootPath: string;
  tree: ExplorerTreeNode[];
}

// Workspace events
export interface WorkspaceEvent {
  type: 'workspace_opened' | 'workspace_closed' | 'directory_changed';
  payload: unknown;
}

// Helper to detect Tauri environment
function isTauriEnvironment(): boolean {
  if (typeof window === 'undefined') return false;
  
  // Check for Tauri globals
  if (window.__TAURI__ !== undefined) return true;
  if ((window as any).__TAURI_INTERNALS__ !== undefined) return true;
  
  // Check user agent
  if (navigator.userAgent.includes('Tauri')) return true;
  
  // Try to detect by checking for Tauri-specific APIs
  try {
    // @ts-ignore
    if (window.__TAURI_IPC__ !== undefined) return true;
  } catch {}
  
  // Additional check for Tauri 2.0
  try {
    // @ts-ignore
    if (window.__TAURI__?.core !== undefined) return true;
  } catch {}
  
  return false;
}

/**
 * WorkspaceService - feature-specific business operations
 * 
 * This layer:
 * - Orchestrates multiple bridge calls
 * - Handles business logic that spans multiple operations
 * - Provides a clean API for containers/stores
 */
export class WorkspaceService {
  // Command operations
  static async openWorkspace(request: OpenWorkspaceRequest): Promise<OpenWorkspaceResponse> {
    // Check if we're in Tauri environment - use multiple detection methods
    const isTauri = 
      typeof window !== 'undefined' && 
      (window.__TAURI__ !== undefined || 
       (window as any).__TAURI_INTERNALS__ !== undefined ||
       navigator.userAgent.includes('Tauri'));
    
    if (!isTauri) {
      // Return mock workspace data for development
      return {
        workspaceId: 'mock-workspace-id-' + Date.now(),
        rootPath: request.path,
        fileCount: 42,
      };
    }
    
    try {
      const result = await bridge.invoke<any>('open_workspace', { request });
      
      // Handle both camelCase and snake_case
      const workspaceId = result.workspaceId || result.workspace_id;
      const rootPath = result.rootPath || result.root_path;
      const fileCount = result.fileCount || result.file_count;
      
      if (!workspaceId || !rootPath) {
        throw new Error('Invalid workspace response');
      }
      
      return {
        workspaceId,
        rootPath,
        fileCount: fileCount || 0,
      };
    } catch (error) {
      throw error;
    }
  }

  static async listDirectory(request: ListDirectoryRequest): Promise<DirectoryEntryDto[]> {
    return await bridge.invoke<DirectoryEntryDto[]>('list_directory', { request });
  }

  static async openFile(request: OpenFileRequest): Promise<OpenFileResponse> {
    return await bridge.invoke<OpenFileResponse>('open_file', { request });
  }

  static async saveFile(request: SaveFileRequest): Promise<void> {
    return await bridge.invoke<void>('save_file', { request });
  }

  static async openFileDialog(): Promise<OpenDialogResponse> {
    const isTauri = isTauriEnvironment();
    
    if (!isTauri) {
      // For development, return a mock path
      const mockPath = '/Users/developer/projects/test-workspace';
      return { selectedPath: mockPath };
    }
    
    try {
      const result = await bridge.invoke<any>('open_file_dialog');
      
      // Handle both camelCase and snake_case
      const selectedPath = result.selectedPath || result.selected_path;
      
      return { selectedPath };
    } catch (error) {
      throw error;
    }
  }

  // Explorer-specific operations
  static async getWorkspaceTree(request: WorkspaceTreeRequest): Promise<WorkspaceTreeResponse> {
    // Check if we're in Tauri environment - use multiple detection methods
    const isTauri = 
      typeof window !== 'undefined' && 
      (window.__TAURI__ !== undefined || 
       (window as any).__TAURI_INTERNALS__ !== undefined ||
       navigator.userAgent.includes('Tauri'));
    
    if (!isTauri) {
      // Return mock tree data for development
      const mockTree: ExplorerTreeNode[] = [
        {
          id: `${request.rootPath}/file1.rs`,
          path: `${request.rootPath}/file1.rs`,
          name: 'file1.rs',
          isDir: false,
          fileType: 'rs',
          size: 1234,
          modified: new Date().toISOString(),
          children: undefined,
          parentPath: request.rootPath,
        },
        {
          id: `${request.rootPath}/src`,
          path: `${request.rootPath}/src`,
          name: 'src',
          isDir: true,
          fileType: undefined,
          size: undefined,
          modified: new Date().toISOString(),
          children: [],
          parentPath: request.rootPath,
        },
        {
          id: `${request.rootPath}/Cargo.toml`,
          path: `${request.rootPath}/Cargo.toml`,
          name: 'Cargo.toml',
          isDir: false,
          fileType: 'toml',
          size: 567,
          modified: new Date().toISOString(),
          children: undefined,
          parentPath: request.rootPath,
        },
      ];
      return {
        workspaceId: request.workspaceId,
        rootPath: request.rootPath,
        tree: mockTree,
      };
    }
    
    try {
      // The Rust command expects a WorkspaceTreeRequest with camelCase fields
      // due to #[serde(rename_all = "camelCase")]
      // We pass it as a single request object
      const result = await bridge.invoke<WorkspaceTreeResponse>('get_workspace_tree', {
        request: {
          workspaceId: request.workspaceId,
          rootPath: request.rootPath
        }
      });
      return result;
    } catch (error: any) {
      // Extract error message from BridgeError
      const errorMessage = error?.message || error?.toString() || 'Unknown error building workspace tree';
      
      // Try to get more details
      let detailedMessage = 'Unknown error loading workspace tree';
      if (typeof error === 'string') {
        detailedMessage = error;
      } else if (error?.message) {
        detailedMessage = error.message;
      } else if (error?.toString) {
        detailedMessage = error.toString();
      }
      
      throw new Error(`Failed to load workspace tree: ${detailedMessage}`);
    }
  }

  static async loadDirectoryChildren(path: string): Promise<DirectoryEntryDto[]> {
    return await this.listDirectory({ path });
  }

  // Event subscriptions
  static onWorkspaceOpened(handler: (workspaceId: string, rootPath: string) => void) {
    return bridge.listen<{ workspaceId: string; rootPath: string }>('workspace:opened', (event) => {
      handler(event.workspaceId, event.rootPath);
    });
  }

  static onDirectoryChanged(handler: (path: string) => void) {
    return bridge.listen<{ path: string }>('workspace:directory_changed', (event) => {
      handler(event.path);
    });
  }

  // Business operations (combine multiple commands)
  static async openWorkspaceAndLoadTree(
    path: string
  ): Promise<{ workspace: OpenWorkspaceResponse; tree: WorkspaceTreeResponse }> {
    const workspace = await this.openWorkspace({ path });
    const tree = await this.getWorkspaceTree({
      workspaceId: workspace.workspaceId,
      rootPath: workspace.rootPath
    });
    
    return { workspace, tree };
  }

  static async openFileInEditor(path: string): Promise<OpenFileResponse> {
    return await this.openFile({ path });
  }
}
