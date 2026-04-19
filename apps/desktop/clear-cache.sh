#!/bin/bash

echo "Clearing Tauri cache..."

# Remove Tauri build cache
rm -rf src-tauri/target 2>/dev/null || true

# Remove Vite cache
rm -rf node_modules/.vite 2>/dev/null || true

# Remove dist directory
rm -rf dist 2>/dev/null || true

echo "Cache cleared!"
echo ""
echo "Now run:"
echo "  ./run.sh"
echo "or"
echo "  npm run tauri dev"
