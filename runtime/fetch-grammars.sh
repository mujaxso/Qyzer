#!/usr/bin/env bash
set -euo pipefail

# Script to fetch Tree‑sitter grammar shared libraries for Neote runtime.
# Currently supports Linux‑x86_64 (`.so`) only; extend for other platforms as needed.

RUNTIME_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/treesitter"
GRAMMAR_DIR="${RUNTIME_ROOT}/grammars/linux-x86_64"
mkdir -p "${GRAMMAR_DIR}"

# Rust grammar (release v0.20.3)
RUST_URL="https://github.com/tree-sitter/tree-sitter-rust/releases/download/v0.20.3/libtree-sitter-rust.so"
echo "Fetching Rust grammar…"
curl -Lf "${RUST_URL}" -o "${GRAMMAR_DIR}/libtree-sitter-rust.so" 2>/dev/null || {
    echo "Failed to download Rust grammar. Attempting to build from source…"
    if command -v tree-sitter >/dev/null 2>&1; then
        TS_DIR="$(mktemp -d)"
        git clone --depth 1 https://github.com/tree-sitter/tree-sitter-rust "${TS_DIR}"
        (cd "${TS_DIR}" && tree-sitter generate && tree-sitter build)
        mv "${TS_DIR}/libtree-sitter-rust.so" "${GRAMMAR_DIR}/"
        rm -rf "${TS_DIR}"
    else
        echo "tree‑sitter CLI not installed. Please install it and re‑run this script."
        exit 1
    fi
}

# TOML grammar (release v0.20.0)
TOML_URL="https://github.com/tree-sitter/tree-sitter-toml/releases/download/v0.20.0/libtree-sitter-toml.so"
echo "Fetching TOML grammar…"
curl -Lf "${TOML_URL}" -o "${GRAMMAR_DIR}/libtree-sitter-toml.so" 2>/dev/null || {
    echo "Failed to download TOML grammar. Attempting to build from source…"
    if command -v tree-sitter >/dev/null 2>&1; then
        TS_DIR="$(mktemp -d)"
        git clone --depth 1 https://github.com/tree-sitter/tree-sitter-toml "${TS_DIR}"
        (cd "${TS_DIR}" && tree-sitter generate && tree-sitter build)
        mv "${TS_DIR}/libtree-sitter-toml.so" "${GRAMMAR_DIR}/"
        rm -rf "${TS_DIR}"
    else
        echo "tree‑sitter CLI not installed. Please install it and re‑run this script."
        exit 1
    fi
}

echo "Grammars have been placed in ${GRAMMAR_DIR}"
ls -la "${GRAMMAR_DIR}/"
