#!/usr/bin/env bash

# Setup script for Zaroxi Desktop App
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

echo "Setting up Zaroxi Desktop App..."

# Check if we found the right directories
if [ ! -f "$DESKTOP_DIR/package.json" ]; then
    echo "Error: Could not find apps/desktop/package.json"
    echo "Make sure you're in the zaroxi repository"
    exit 1
fi

cd "$DESKTOP_DIR"

# Install npm dependencies
echo "Installing npm dependencies..."
if ! npm install; then
    echo "npm install failed. Please check your Node.js installation."
    exit 1
fi

# Check for missing crates
echo "Checking for missing crates..."
cd "$ZAROXI_ROOT"

# Create zaroxi-core-ids if missing
if [ ! -f "crates/zaroxi-core-ids/Cargo.toml" ]; then
    echo "Creating missing zaroxi-core-ids crate..."
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
    echo "Created zaroxi-core-ids crate"
fi

echo "Setup complete!"
echo ""
echo "To start the app in development mode:"
echo "  ./run.sh"
echo "  or"
echo "  npm run tauri dev"
echo ""
echo "For frontend-only development:"
echo "  npm run dev"
echo ""
echo "Note: If you encounter build errors, the run.sh script will handle missing crates."
