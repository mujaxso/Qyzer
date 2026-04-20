import { useEffect } from 'react';
import { ExplorerTree } from '../components/ExplorerTree';
import { useWorkspaceStore } from '../stores/useWorkspaceStore';
import { WorkspaceService } from '../services/workspaceService';

export function ExplorerContainer() {
  const {
    workspaceTree,
    explorerUI,
    isLoading,
    toggleExpanded,
    setSelectedPath,
    setActiveFilePath,
    setWorkspaceTree,
    setLoading,
    setError,
  } = useWorkspaceStore();

  const handleNodeClick = async (node: any) => {
    setSelectedPath(node.path);
    
    if (node.isDir) {
      toggleExpanded(node.path);
    } else {
      // Open file in editor
      setActiveFilePath(node.path);
      try {
        await WorkspaceService.openFileInEditor(node.path);
      } catch (error) {
        setError(error instanceof Error ? error.message : 'Failed to open file');
        console.error('Failed to open file:', error);
      }
    }
  };

  const handleLoadChildren = async (path: string) => {
    setLoading(true);
    try {
      const children = await WorkspaceService.loadDirectoryChildren(path);
      // Update the tree with loaded children
      // This is a simplified implementation - in a real app, we'd update the specific node
      console.log('Loaded children for', path, children);
    } catch (error) {
      setError(error instanceof Error ? error.message : 'Failed to load directory children');
      console.error('Failed to load directory children:', error);
    } finally {
      setLoading(false);
    }
  };

  if (isLoading && workspaceTree.length === 0) {
    return (
      <div className="p-4">
        <div className="space-y-2">
          {Array.from({ length: 5 }).map((_, i) => (
            <div key={i} className="h-6 bg-muted rounded animate-pulse"></div>
          ))}
        </div>
      </div>
    );
  }

  if (workspaceTree.length === 0) {
    return (
      <div className="p-8 text-center text-muted-foreground">
        <p>No files found in workspace.</p>
      </div>
    );
  }

  return (
    <div className="py-2">
      <ExplorerTree
        nodes={workspaceTree}
        expandedPaths={explorerUI.expandedPaths}
        selectedPath={explorerUI.selectedPath}
        activeFilePath={explorerUI.activeFilePath}
        onNodeClick={handleNodeClick}
        onLoadChildren={handleLoadChildren}
      />
    </div>
  );
}
