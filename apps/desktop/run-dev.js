import { spawn } from 'child_process';
import { fileURLToPath } from 'url';
import { dirname, join } from 'path';

const __filename = fileURLToPath(import.meta.url);
const __dirname = dirname(__filename);

const vitePath = join(__dirname, 'node_modules', 'vite', 'bin', 'vite.js');

const child = spawn('node', [vitePath, '--port', '1420'], {
  stdio: 'inherit',
  shell: true
});

child.on('error', (err) => {
  console.error('Failed to start vite:', err);
});

child.on('exit', (code) => {
  console.log(`Vite exited with code ${code}`);
});
