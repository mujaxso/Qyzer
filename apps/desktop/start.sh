#!/bin/bash

echo "Zaroxi Desktop - Alternative Start Script"
echo "=========================================="
echo ""

# Check if we're in the right directory
if [ ! -f "package.json" ]; then
    echo "ERROR: Not in apps/desktop directory!"
    echo "Current directory: $(pwd)"
    echo "Please run: cd apps/desktop"
    exit 1
fi

echo "1. Checking npm dependencies..."
if [ ! -d "node_modules" ]; then
    echo "   Installing npm dependencies..."
    npm install
    if [ $? -ne 0 ]; then
        echo "   ERROR: npm install failed"
        exit 1
    fi
    echo "   ✓ npm dependencies installed"
else
    echo "   ✓ npm dependencies already installed"
fi

echo ""
echo "2. Checking Rust dependencies..."
if [ ! -d "../../target" ]; then
    echo "   Building Rust workspace..."
    cd ../..
    cargo build --workspace
    if [ $? -ne 0 ]; then
        echo "   ERROR: cargo build failed"
        exit 1
    fi
    cd apps/desktop
    echo "   ✓ Rust dependencies built"
else
    echo "   ✓ Rust dependencies already built"
fi

echo ""
echo "3. Starting Zaroxi Desktop..."
echo "   Frontend: http://localhost:1420"
echo "   Press Ctrl+C to stop"
echo ""
npm run tauri dev
