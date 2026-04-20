#!/usr/bin/env bash

# Zaroxi Desktop - Alternative Start Script
# Can be run from anywhere within the zaroxi repository

set -e

# Find the zaroxi root directory
find_zaroxi_root() {
    local dir="$PWD"
    while [ "$dir" != "/" ]; do
        if [ -f "$dir/Cargo.toml" ] && [ -d "$dir/apps/desktop" ]; then
            echo "$dir"
            return 0
        fi
        dir="$(dirname "$dir")"
    done
    return 1
}

ZAROXI_ROOT="$(find_zaroxi_root 2>/dev/null || echo "$PWD")"
DESKTOP_DIR="$ZAROXI_ROOT/apps/desktop"

echo "Zaroxi Desktop - Alternative Start Script"
echo "=========================================="
echo ""

# Check if we found the right directories
if [ ! -f "$DESKTOP_DIR/package.json" ]; then
    echo "ERROR: Could not find apps/desktop/package.json"
    echo "Make sure you're in the zaroxi repository"
    exit 1
fi

cd "$DESKTOP_DIR"

echo "1. Checking npm dependencies..."
if [ ! -d "node_modules" ]; then
    echo "   Installing npm dependencies..."
    if ! npm install; then
        echo "   ERROR: npm install failed"
        exit 1
    fi
    echo "   ✓ npm dependencies installed"
else
    echo "   ✓ npm dependencies already installed"
fi

echo ""
echo "2. Checking Rust dependencies..."
if [ ! -d "$ZAROXI_ROOT/target" ]; then
    echo "   Building Rust workspace..."
    cd "$ZAROXI_ROOT"
    
    # Check for missing zaroxi-core-ids crate
    if [ ! -f "crates/zaroxi-core-ids/Cargo.toml" ]; then
        echo "   Creating missing zaroxi-core-ids crate..."
        mkdir -p crates/zaroxi-core-ids/src
        cat > crates/zaroxi-core-ids/Cargo.toml << 'EOF'
[package]
name = "zaroxi-core-ids"
version = "0.1.0"
edition = "2024"
license = "MIT"
description = "Strongly-typed identifiers for Zaroxi Studio"

[dependencies]
uuid = { workspace = true, features = ["v4", "serde"] }
serde = { workspace = true, features = ["derive"] }
thiserror = { workspace = true }
EOF
        cat > crates/zaroxi-core-ids/src/lib.rs << 'EOF'
//! Identifier types for Zaroxi Studio.
//!
//! Defines strongly-typed identifiers for various entities in the system
//! (documents, users, sessions, etc.) to prevent mixing different ID types.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A strongly-typed buffer identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BufferId(pub Uuid);

/// A strongly-typed workspace identifier  
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct WorkspaceId(pub Uuid);

impl BufferId {
    /// Create a new unique buffer ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl WorkspaceId {
    /// Create a new unique workspace ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for BufferId {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for WorkspaceId {
    fn default() -> Self {
        Self::new()
    }
}
EOF
        echo "   ✓ Created zaroxi-core-ids crate"
    fi
    
    if ! cargo build --workspace; then
        echo "   WARNING: Workspace build failed, trying desktop only..."
        cd "$DESKTOP_DIR/src-tauri"
        if ! cargo build; then
            echo "   ERROR: Desktop build also failed"
            exit 1
        fi
        echo "   ✓ Desktop built successfully"
    else
        echo "   ✓ Rust dependencies built"
    fi
    cd "$DESKTOP_DIR"
else
    echo "   ✓ Rust dependencies already built"
fi

echo ""
echo "3. Starting Zaroxi Desktop..."
echo "   Frontend: http://localhost:1420"
echo "   Press Ctrl+C to stop"
echo ""
npm run tauri dev
