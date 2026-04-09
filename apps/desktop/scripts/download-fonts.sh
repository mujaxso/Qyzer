#!/usr/bin/env bash
set -euo pipefail

# Get the directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# Go up one level to the desktop directory
DESKTOP_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
# Go up another level to the project root (if needed)
PROJECT_ROOT="$(cd "${DESKTOP_DIR}/.." && pwd)"
FONTS_DIR="${DESKTOP_DIR}/assets/fonts"

echo "Script directory: ${SCRIPT_DIR}"
echo "Desktop directory: ${DESKTOP_DIR}"
echo "Project root: ${PROJECT_ROOT}"
echo "Fonts directory: ${FONTS_DIR}"

echo "Downloading fonts to ${FONTS_DIR}..."
mkdir -p "${FONTS_DIR}"

# Remove placeholder files if they exist
rm -f "${FONTS_DIR}/NotoSans-Regular.ttf"
rm -f "${FONTS_DIR}/NotoEmoji-Regular.ttf"
rm -f "${FONTS_DIR}/NotoColorEmoji.ttf"

# Download Noto Sans Regular from a reliable source
# Using the official Google Fonts GitHub repository
NOTO_SANS_URL="https://github.com/googlefonts/noto-fonts/raw/main/hinted/ttf/NotoSans/NotoSans-Regular.ttf"

# Download Noto Color Emoji (which includes emoji support)
# Note: NotoEmoji-Regular.ttf might not exist, so we'll use NotoColorEmoji.ttf
NOTO_EMOJI_URL="https://github.com/googlefonts/noto-emoji/raw/main/fonts/NotoColorEmoji.ttf"

echo "Downloading Noto Sans Regular..."
if command -v curl &> /dev/null; then
    if curl -L -o "${FONTS_DIR}/NotoSans-Regular.ttf" "${NOTO_SANS_URL}"; then
        echo "✓ Noto Sans downloaded successfully"
    else
        echo "✗ Failed to download Noto Sans"
        exit 1
    fi
elif command -v wget &> /dev/null; then
    if wget -O "${FONTS_DIR}/NotoSans-Regular.ttf" "${NOTO_SANS_URL}"; then
        echo "✓ Noto Sans downloaded successfully"
    else
        echo "✗ Failed to download Noto Sans"
        exit 1
    fi
else
    echo "Error: Neither curl nor wget found. Please install one of them."
    exit 1
fi

echo "Downloading Noto Color Emoji..."
if command -v curl &> /dev/null; then
    if curl -L -o "${FONTS_DIR}/NotoColorEmoji.ttf" "${NOTO_EMOJI_URL}"; then
        echo "✓ Noto Color Emoji downloaded successfully"
        # Also create a symlink for the expected name
        cd "${FONTS_DIR}"
        ln -sf "NotoColorEmoji.ttf" "NotoEmoji-Regular.ttf"
        cd - > /dev/null
    else
        echo "✗ Failed to download Noto Color Emoji"
        exit 1
    fi
elif command -v wget &> /dev/null; then
    if wget -O "${FONTS_DIR}/NotoColorEmoji.ttf" "${NOTO_EMOJI_URL}"; then
        echo "✓ Noto Color Emoji downloaded successfully"
        # Also create a symlink for the expected name
        cd "${FONTS_DIR}"
        ln -sf "NotoColorEmoji.ttf" "NotoEmoji-Regular.ttf"
        cd - > /dev/null
    else
        echo "✗ Failed to download Noto Color Emoji"
        exit 1
    fi
else
    echo "Error: Neither curl nor wget found. Please install one of them."
    exit 1
fi

# Verify the downloads
if [[ -f "${FONTS_DIR}/NotoSans-Regular.ttf" ]] && [[ -f "${FONTS_DIR}/NotoColorEmoji.ttf" ]]; then
    echo ""
    echo "✅ Fonts downloaded successfully!"
    echo "   Noto Sans Regular: ${FONTS_DIR}/NotoSans-Regular.ttf"
    echo "   Noto Color Emoji: ${FONTS_DIR}/NotoColorEmoji.ttf"
    echo ""
    echo "To rebuild the application with the new fonts:"
    echo "  cd ${DESKTOP_DIR}"
    echo "  cargo build --bin desktop"
else
    echo "❌ Error: Some font files failed to download."
    exit 1
fi
