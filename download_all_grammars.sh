#!/usr/bin/env bash
set -e

# Script to download and install ALL Tree-sitter grammars
# for Qyzer Studio

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$SCRIPT_DIR"

echo "Installing ALL Tree-sitter grammars for Qyzer Studio..."
echo ""

# Check for required tools
for cmd in curl unzip cc cargo; do
    if ! command -v $cmd &> /dev/null; then
        echo "Error: $cmd is required but not installed."
        exit 1
    fi
done

# Create runtime directory structure
RUNTIME_DIR="$PROJECT_ROOT/runtime/treesitter"
mkdir -p "$RUNTIME_DIR/grammars"
mkdir -p "$RUNTIME_DIR/languages"

echo "Runtime directory: $RUNTIME_DIR"

# Build and run the download-grammars tool
cd "$PROJECT_ROOT"

echo "Building download-grammars tool..."
cargo build --bin download-grammars --manifest-path crates/syntax-core/Cargo.toml

echo ""
echo "Installing ALL available grammars..."
echo "This may take a while..."

cargo run --bin download-grammars --manifest-path crates/syntax-core/Cargo.toml -- install-all

echo ""
echo "Done! All grammars have been installed."
echo "You can now use syntax highlighting for:"
echo "  - Rust (.rs)"
echo "  - TOML (.toml)"
echo "  - Markdown (.md, .markdown)"
echo "  - JavaScript (.js)"
echo "  - Python (.py)"
echo "  - JSON (.json)"
echo "  - HTML (.html)"
echo "  - CSS (.css)"
echo "  - Go (.go)"
echo "  - C (.c)"
echo "  - C++ (.cpp)"
echo "  - Java (.java)"
echo "  - Bash (.sh)"
echo "  - TypeScript (.ts)"
