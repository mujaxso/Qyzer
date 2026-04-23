import { useWorkspaceStore } from '@/features/workspace/stores/useWorkspaceStore';
import { cn } from '@/lib/utils';
import { useEffect, useState, useMemo } from 'react';
import { WorkspaceService } from '@/features/workspace/services/workspaceService';

interface StatusBarProps {
  className?: string;
}

/**
 * Simple language detection by file extension.
 * Used in the status bar to show the language mode without an extra bridge call.
 */
function detectLanguageExtension(path: string): string | undefined {
  const ext = path.split('.').pop()?.toLowerCase() ?? '';
  const known: Record<string, string> = {
    rs: 'Rust',
    ts: 'TypeScript',
    tsx: 'TypeScript JSX',
    js: 'JavaScript',
    jsx: 'JavaScript JSX',
    py: 'Python',
    go: 'Go',
    toml: 'TOML',
    yaml: 'YAML',
    yml: 'YAML',
    json: 'JSON',
    md: 'Markdown',
    html: 'HTML',
    css: 'CSS',
    scss: 'SCSS',
    sass: 'SASS',
    vue: 'Vue',
    c: 'C',
    cpp: 'C++',
    h: 'C Header',
    hpp: 'C++ Header',
    java: 'Java',
    lua: 'Lua',
    rb: 'Ruby',
    php: 'PHP',
    swift: 'Swift',
    kt: 'Kotlin',
  };
  return known[ext];
}

/** Data we need from the open file response (only metadata, not full content). */
interface FileMeta {
  largeFileMode: string;
  contentTruncated: boolean;
}

export function StatusBar({ className }: StatusBarProps) {
  const { currentWorkspace, isLoading, explorerUI } = useWorkspaceStore();
  const activeFilePath = explorerUI?.activeFilePath ?? null;
  const [fileMeta, setFileMeta] = useState<FileMeta | null>(null);

  // Fetch lightweight file metadata when the active file changes
  useEffect(() => {
    let cancelled = false;
    if (activeFilePath) {
      // openFile is lightweight enough; it reads the file on the Rust side
      // but only returns metadata and optionally truncated content.
      WorkspaceService.openFile({ path: activeFilePath }).then((resp) => {
        if (!cancelled) {
          setFileMeta({
            largeFileMode: resp.largeFileMode ?? 'Normal',
            contentTruncated: resp.contentTruncated ?? false,
          });
        }
      });
    } else {
      setFileMeta(null);
    }
    return () => {
      cancelled = true;
    };
  }, [activeFilePath]);

  // Derive presentation values from the active file path
  const fileName = useMemo(
    () => (activeFilePath ? activeFilePath.split(/[\\/]/).pop() ?? '—' : null),
    [activeFilePath]
  );

  const languageLabel = useMemo(
    () => (activeFilePath ? detectLanguageExtension(activeFilePath) ?? 'Plain Text' : null),
    [activeFilePath]
  );

  // Show a dedicated large‑file indicator only when the file is not normal
  const largeFileIndicator =
    fileMeta && fileMeta.largeFileMode !== 'Normal'
      ? `  ${fileMeta.largeFileMode === 'VeryLarge' ? 'Very Large' : 'Large'} File`
      : null;

  const truncationIndicator =
    fileMeta && fileMeta.contentTruncated ? '  (truncated)' : null;

  return (
    <div
      className={cn(
        'h-6 flex items-center justify-between px-3 text-[11px] font-sans leading-none',
        'text-primary',
        className
      )}
      style={{ backgroundColor: 'var(--status-bar-background)' }}
    >
      {/* ── Left side: workspace / operational state ── */}
      <div className="flex items-center space-x-3">
        <span className="font-medium">{currentWorkspace ? currentWorkspace.name : 'No workspace'}</span>
        {currentWorkspace && (
          <span className="text-primary/70 font-mono text-[10px]">
            {currentWorkspace.rootPath.split('/').pop()}
          </span>
        )}
        {isLoading && (
          <span className="text-accent font-medium">
            <span className="inline-block w-1.5 h-1.5 rounded-full bg-accent animate-pulse mr-1 align-middle" />
            Loading…
          </span>
        )}
      </div>

      {/* ── Right side: file/editor metadata ── */}
      <div className="flex items-center space-x-3">
        {activeFilePath && (
          <>
            {/* File name (no icon – text is enough) */}
            <span className="font-medium max-w-[180px] truncate" title={activeFilePath}>
              {fileName}
            </span>

            {/* Language mode */}
            <span className="text-primary/70">{languageLabel}</span>

            {/* Encoding & line endings – always useful when a file is open */}
            <span className="text-primary/70">UTF-8</span>
            <span className="text-primary/70">LF</span>

            {/* Large‑file / truncation indicators – only when relevant */}
            {(largeFileIndicator != null || truncationIndicator != null) && (
              <span className="text-yellow-500 font-medium">
                {largeFileIndicator ?? ''}
                {truncationIndicator ?? ''}
              </span>
            )}
          </>
        )}
      </div>
    </div>
  );
}
