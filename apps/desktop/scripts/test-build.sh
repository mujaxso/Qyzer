#!/bin/bash

# Test build script for Zaroxi Desktop
# Run from the zaroxi root directory

set -e

echo "Testing Zaroxi build..."

# Check if we're in the right directory
if [ ! -f "Cargo.toml" ] || [ ! -d "apps/desktop" ]; then
    echo "Error: Run this script from the zaroxi root directory"
    exit 1
fi

echo "1. Checking crate structure..."
./apps/desktop/scripts/create-missing-crates.sh

echo "2. Building workspace..."
if cargo build --workspace; then
    echo "✅ Workspace build successful!"
else
    echo "⚠️  Workspace build failed, trying desktop only..."
    cd apps/desktop/src-tauri
    if cargo build; then
        echo "✅ Desktop build successful!"
    else
        echo "❌ Build failed"
        exit 1
    fi
fi

echo ""
echo "✅ Test build completed!"
echo "You can now run the desktop app with:"
echo "  cd apps/desktop && npm run tauri dev"
