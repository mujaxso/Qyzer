export default function SourceControlPanel() {
  return (
    <div className="p-4 h-full flex flex-col">
      <div className="space-y-4">
        <div className="flex items-center justify-between">
          <h3 className="font-medium">Source Control</h3>
          <span className="text-xs px-2 py-1 bg-muted rounded">main</span>
        </div>
        
        <div className="space-y-3">
          <div className="p-3 border border-border rounded-lg">
            <div className="flex items-center justify-between mb-2">
              <span className="text-sm font-medium">Changes</span>
              <span className="text-xs bg-accent text-accent-foreground px-2 py-0.5 rounded">3</span>
            </div>
            <div className="space-y-2">
              <div className="flex items-center justify-between text-xs">
                <span className="text-muted-foreground">Modified</span>
                <span>2 files</span>
              </div>
              <div className="flex items-center justify-between text-xs">
                <span className="text-muted-foreground">Untracked</span>
                <span>1 file</span>
              </div>
            </div>
          </div>
          
          <div className="space-y-2">
            <h4 className="text-sm font-medium">Commit Message</h4>
            <textarea
              placeholder="Enter commit message..."
              className="w-full h-20 px-3 py-2 text-sm border border-input rounded-lg bg-background focus:outline-none focus:ring-2 focus:ring-primary resize-none"
            />
            <div className="flex gap-2">
              <button className="flex-1 px-3 py-2 text-sm bg-primary text-primary-foreground rounded hover:bg-primary/90">
                Commit
              </button>
              <button className="px-3 py-2 text-sm border border-border rounded hover:bg-hover-bg">
                ...
              </button>
            </div>
          </div>
        </div>
        
        <div className="border-t border-border pt-4">
          <h4 className="text-sm font-medium mb-2">Repository Actions</h4>
          <div className="grid grid-cols-2 gap-2">
            <button className="p-2 text-xs border border-border rounded hover:bg-hover-bg">
              Pull
            </button>
            <button className="p-2 text-xs border border-border rounded hover:bg-hover-bg">
              Push
            </button>
            <button className="p-2 text-xs border border-border rounded hover:bg-hover-bg">
              Fetch
            </button>
            <button className="p-2 text-xs border border-border rounded hover:bg-hover-bg">
              Branch
            </button>
          </div>
        </div>
      </div>
      
      <div className="mt-auto pt-4 border-t border-border">
        <p className="text-xs text-muted-foreground text-center">
          Git integration powered by zaroxi
        </p>
      </div>
    </div>
  );
}
