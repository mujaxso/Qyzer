import { useEffect, useRef } from 'react';
import { cn } from '@/lib/utils';

interface CodeEditorProps {
  initialValue: string;
  onChange: (value: string) => void;
  language?: string;
  readOnly?: boolean;
  className?: string;
}

export function CodeEditor({
  initialValue,
  onChange,
  language = 'plaintext',
  readOnly = false,
  className,
}: CodeEditorProps) {
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (textareaRef.current) {
      // Simple auto-resize
      textareaRef.current.style.height = 'auto';
      textareaRef.current.style.height = `${textareaRef.current.scrollHeight}px`;
    }
  }, [initialValue]);

  const handleChange = (e: React.ChangeEvent<HTMLTextAreaElement>) => {
    onChange(e.target.value);
    
    // Auto-resize
    e.target.style.height = 'auto';
    e.target.style.height = `${e.target.scrollHeight}px`;
  };

  return (
    <div className={cn('relative h-full', className)}>
      <div className="absolute inset-0 bg-editor font-mono text-sm">
        <textarea
          ref={textareaRef}
          value={initialValue}
          onChange={handleChange}
          readOnly={readOnly}
          className="w-full h-full bg-transparent text-editor-foreground resize-none outline-none p-4 leading-relaxed whitespace-pre overflow-auto scrollbar-ide"
          spellCheck="false"
          style={{ tabSize: 2 }}
        />
      </div>
    </div>
  );
}
