export default function DebugPanel() {
  return (
    <div className="p-4 h-full flex flex-col">
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h3 className="font-medium">Debug</h3>
          <span className="text-xs px-2 py-1 bg-muted rounded">Ready</span>
        </div>
        
        <div className="space-y-3">
          <div className="p-3 border border-border rounded-lg">
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm font-medium">Breakpoints</span>
              <span className="text-xs bg-accent text-accent-foreground px-2 py-0.5 rounded">2</span>
            </div>
            <div className="space-y-1">
              <div className="flex items-center justify-between text-xs">
                <span className="text-muted-foreground">main.rs:42</span>
                <button className="text-destructive hover:text-destructive/80">×</button>
              </div>
              <div className="flex items-center justify-between text-xs">
                <span className="text-muted-foreground">editor.rs:128</span>
                <button className="text-destructive hover:text-destructive/80">×</button>
              </div>
            </div>
          </div>
          
          <div className="space-y-2">
            <h4 className="text-sm font-medium">Debug Controls</h4>
            <div className="grid grid-cols-4 gap-2">
              <button className="p-2 text-xs border border-border rounded hover:bg-hover-bg flex flex-col items-center">
                <span className="text-lg">▶</span>
                <span>Start</span>
              </button>
              <button className="p-2 text-xs border border-border rounded hover:bg-hover-bg flex flex-col items-center">
                <span className="text-lg">⏸</span>
                <span>Pause</span>
              </button>
              <button className="p-2 text-xs border border-border rounded hover:bg-hover-bg flex flex-col items-center">
                <span className="text-lg">⏭</span>
                <span>Step</span>
              </button>
              <button className="p-2 text-xs border border-border rounded hover:bg-hover-bg flex flex-col items-center">
                <span className="text-lg">⏹</span>
                <span>Stop</span>
              </button>
            </div>
          </div>
        </div>
        
        <div className="border-t border-border pt-4">
          <h4 className="text-sm font-medium mb-2">Call Stack</h4>
          <div className="space-y-1">
            <div className="text-xs p-2 bg-muted rounded">
              main() - main.rs:15
            </div>
            <div className="text-xs p-2 hover:bg-hover-bg rounded cursor-pointer">
              init_editor() - editor.rs:42
            </div>
          </div>
        </div>
      </div>
      
      <div className="mt-auto pt-4 border-t border-border">
        <p className="text-xs text-muted-foreground text-center">
          Debugger ready. Set breakpoints and start debugging.
        </p>
      </div>
    </div>
  );
}
