#!/usr/bin/env bash
set -e

echo "Installing Tree-sitter Markdown grammar..."

# Use project root if available, otherwise fallback to home
if [ -n "$QYZER_PROJECT_ROOT" ]; then
    PROJECT_ROOT="$QYZER_PROJECT_ROOT"
elif [ -n "$NEOTE_RUNTIME" ]; then
    PROJECT_ROOT="$(dirname "$(dirname "$NEOTE_RUNTIME")")"
else
    PROJECT_ROOT="$HOME/Work/qyzer"
fi

RUNTIME_DIR="$PROJECT_ROOT/runtime/treesitter"
mkdir -p "$RUNTIME_DIR/grammars/linux-x86_64"
mkdir -p "$RUNTIME_DIR/languages/markdown/queries"

echo "Using runtime directory: $RUNTIME_DIR"

# Download and compile
cd /tmp
rm -rf tree-sitter-markdown-build
mkdir -p tree-sitter-markdown-build
cd tree-sitter-markdown-build

echo "Downloading source code..."
# Try multiple methods
if command -v curl &> /dev/null; then
    curl -L -o source.zip https://github.com/tree-sitter/tree-sitter-markdown/archive/HEAD.zip
    unzip -q source.zip
    mv tree-sitter-markdown-* tree-sitter-markdown
elif command -v wget &> /dev/null; then
    wget -O source.zip https://github.com/tree-sitter/tree-sitter-markdown/archive/HEAD.zip
    unzip -q source.zip
    mv tree-sitter-markdown-* tree-sitter-markdown
elif command -v git &> /dev/null; then
    git clone --depth 1 https://github.com/tree-sitter/tree-sitter-markdown
else
    echo "Error: No download tool available (curl, wget, or git)"
    exit 1
fi

cd tree-sitter-markdown

echo "Building grammar..."
# Build with tree-sitter CLI if available
if command -v tree-sitter &> /dev/null; then
    tree-sitter generate
    tree-sitter build --release
    cp target/release/libtree-sitter-markdown.so "$RUNTIME_DIR/grammars/linux-x86_64/"
else
    # Manual build
    echo "Using manual compilation with cc..."
    if [ -f src/scanner.c ]; then
        cc -shared -fPIC -o libtree-sitter-markdown.so src/parser.c src/scanner.c
    else
        cc -shared -fPIC -o libtree-sitter-markdown.so src/parser.c
    fi
    cp libtree-sitter-markdown.so "$RUNTIME_DIR/grammars/linux-x86_64/"
fi

# Copy query files
if [ -d queries ]; then
    mkdir -p "$RUNTIME_DIR/languages/markdown/queries"
    cp queries/* "$RUNTIME_DIR/languages/markdown/queries/" 2>/dev/null || true
fi

echo "Markdown grammar installed successfully!"
echo "Library: $RUNTIME_DIR/grammars/linux-x86_64/libtree-sitter-markdown.so"
