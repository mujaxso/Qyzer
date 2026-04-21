import { readdirSync, statSync, readFileSync, writeFileSync, existsSync } from 'fs';
import { join } from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = join(__filename, '..');

const nodeModulesBin = join(__dirname, '..', 'node_modules', '.bin');

function fixShebang(filePath) {
  try {
    const content = readFileSync(filePath, 'utf8');
    // Check if first line starts with #!
    if (content.startsWith('#!')) {
      // Fix the shebang to use /usr/bin/env node
      const lines = content.split('\n');
      // Only fix if it looks like a node script
      if (lines[0].includes('node')) {
        lines[0] = '#!/usr/bin/env node';
        const fixedContent = lines.join('\n');
        writeFileSync(filePath, fixedContent, 'utf8');
        console.log(`Fixed shebang in ${filePath}`);
      }
    }
  } catch (error) {
    console.error(`Error fixing ${filePath}:`, error.message);
  }
}

function walkDir(dir) {
  if (!existsSync(dir)) {
    console.log(`Directory ${dir} does not exist`);
    return;
  }
  
  const files = readdirSync(dir);
  for (const file of files) {
    const filePath = join(dir, file);
    try {
      const stat = statSync(filePath);
      
      if (stat.isFile() && !file.endsWith('.md') && !file.endsWith('.json')) {
        fixShebang(filePath);
      }
    } catch (error) {
      console.error(`Error processing ${filePath}:`, error.message);
    }
  }
}

console.log('Fixing shebangs in .bin files...');
walkDir(nodeModulesBin);
console.log('Done!');
