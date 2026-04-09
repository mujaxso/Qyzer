#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
FONTS_DIR="${PROJECT_ROOT}/assets/fonts"

echo "Downloading fonts to ${FONTS_DIR}..."
mkdir -p "${FONTS_DIR}"

# Download Noto Sans Regular (from Google Fonts GitHub release)
NOTO_SANS_URL="https://github.com/googlefonts/noto-fonts/raw/main/hinted/ttf/NotoSans/NotoSans-Regular.ttf"
# Download Noto Emoji Regular (from Google Fonts GitHub release)
NOTO_EMOJI_URL="https://github.com/googlefonts/noto-emoji/raw/main/fonts/NotoEmoji-Regular.ttf"

echo "Downloading Noto Sans Regular..."
if command -v curl &> /dev/null; then
    curl -L -o "${FONTS_DIR}/NotoSans-Regular.ttf" "${NOTO_SANS_URL}"
elif command -v wget &> /dev/null; then
    wget -O "${FONTS_DIR}/NotoSans-Regular.ttf" "${NOTO_SANS_URL}"
else
    echo "Error: Neither curl nor wget found. Please install one of them."
    exit 1
fi

echo "Downloading Noto Emoji Regular..."
if command -v curl &> /dev/null; then
    curl -L -o "${FONTS_DIR}/NotoEmoji-Regular.ttf" "${NOTO_EMOJI_URL}"
elif command -v wget &> /dev/null; then
    wget -O "${FONTS_DIR}/NotoEmoji-Regular.ttf" "${NOTO_EMOJI_URL}"
else
    echo "Error: Neither curl nor wget found. Please install one of them."
    exit 1
fi

# Verify the downloads
if [[ -f "${FONTS_DIR}/NotoSans-Regular.ttf" ]] && [[ -f "${FONTS_DIR}/NotoEmoji-Regular.ttf" ]]; then
    echo "Fonts downloaded successfully!"
    echo "Noto Sans Regular: ${FONTS_DIR}/NotoSans-Regular.ttf"
    echo "Noto Emoji Regular: ${FONTS_DIR}/NotoEmoji-Regular.ttf"
else
    echo "Error: Some font files failed to download."
    exit 1
fi
