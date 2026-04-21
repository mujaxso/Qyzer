import { StatusBar } from './StatusBar';
import { CommandPalette } from './CommandPalette';
import { EditorContainer } from '@/features/editor/containers/EditorContainer';
import { ActivityRail } from '@/features/workbench/components/ActivityRail';
import { PanelHost } from '@/features/workbench/components/PanelHost';

export function AppShell() {
  return (
    <div className="flex flex-col h-screen bg-background text-foreground font-sans">
      <CommandPalette />
      
      <div className="flex flex-1 overflow-hidden">
        {/* Activity Rail */}
        <ActivityRail />
        
        {/* Side Panel */}
        <PanelHost />
        
        {/* Main Content */}
        <div className="flex-1 flex flex-col overflow-hidden">
          <div className="flex-1 overflow-hidden">
            <EditorContainer />
          </div>
        </div>
      </div>
      
      {/* Status Bar */}
      <StatusBar />
    </div>
  );
}
