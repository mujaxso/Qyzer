#!/bin/sh

# Simple run script for Zaroxi Desktop App
set -e

echo "Starting Zaroxi Desktop App..."

# Go to desktop directory
cd "$(dirname "$0")"

# Check if we're in the right place
if [ ! -f "package.json" ]; then
    echo "Error: package.json not found. Make sure you're in apps/desktop directory"
    exit 1
fi

# Kill any processes using port 1420
echo "Clearing port 1420..."
pkill -f "vite" 2>/dev/null || true
pkill -f "tauri dev" 2>/dev/null || true

# Also try to kill any process on port 1420 using lsof if available
if command -v lsof >/dev/null 2>&1; then
    lsof -ti:1420 2>/dev/null | xargs kill -9 2>/dev/null || true
fi

# Wait a moment
sleep 1

# Install dependencies if needed
if [ ! -d "node_modules" ]; then
    echo "Installing dependencies..."
    npm install
fi

# Run the app
echo "Starting development server..."
npm run tauri dev
