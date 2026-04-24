import type { IconName } from '@/components/ui/Icon';

const EXTENSION_TO_ICON: Record<string, IconName> = {
  ts: 'file-typescript',
  tsx: 'file-reactts',
  js: 'file-javascript',
  jsx: 'file-reactjs',
  json: 'file-json',
  rs: 'file-rust',
  toml: 'file-toml',
  md: 'file-markdown',
  css: 'file-css',
  scss: 'file-sass',
  html: 'file-html',
  py: 'file-python',
  go: 'file-go',
  rb: 'file-ruby',
  java: 'file-java',
  c: 'file-c',
  cpp: 'file-cpp',
  h: 'file-c',
  hpp: 'file-cpp',
  yaml: 'file-yaml',
  yml: 'file-yaml',
  lock: 'file-color',
  gitignore: 'file-git',
  env: 'file',
  // fallback
};

export function getLanguageIcon(path: string): IconName {
  const name = path.split(/[/\\]/).pop() || '';
  const ext = name.includes('.') ? name.split('.').pop()!.toLowerCase() : '';
  // also handle well‑known filenames
  if (name === 'package.json') return 'file-json';
  const result = EXTENSION_TO_ICON[ext];
  return result ?? 'file';
}
