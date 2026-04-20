#!/usr/bin/env bash

# Script to download JetBrains Mono Nerd Font .woff2 files for Zaroxi Studio
# Run this script from the project root directory

set -euo pipefail

# Configuration
FONT_DIR="apps/desktop/frontend/public/fonts"
NERD_FONTS_RAW="https://raw.githubusercontent.com/ryanoasis/nerd-fonts/master/patched-fonts/JetBrainsMono"

# Create fonts directory if it doesn't exist
mkdir -p "$FONT_DIR"

echo "Downloading JetBrains Mono Nerd Font .woff2 files to $FONT_DIR..."

# Define the font variants we need with their specific paths
declare -A FONT_VARIANTS=(
    ["Regular"]="Regular/complete/JetBrains%20Mono%20Regular%20Nerd%20Font%20Complete%20Mono.woff2"
    ["Bold"]="Bold/complete/JetBrains%20Mono%20Bold%20Nerd%20Font%20Complete%20Mono.woff2"
    ["Italic"]="Italic/complete/JetBrains%20Mono%20Italic%20Nerd%20Font%20Complete%20Mono.woff2"
    ["BoldItalic"]="BoldItalic/complete/JetBrains%20Mono%20Bold%20Italic%20Nerd%20Font%20Complete%20Mono.woff2"
)

# Download each variant
for variant in "${!FONT_VARIANTS[@]}"; do
    url_path="${FONT_VARIANTS[$variant]}"
    download_url="${NERD_FONTS_RAW}/${url_path}"
    output_file=""
    
    case "$variant" in
        "Regular")
            output_file="JetBrainsMonoNerdFont-Regular.woff2"
            ;;
        "Bold")
            output_file="JetBrainsMonoNerdFont-Bold.woff2"
            ;;
        "Italic")
            output_file="JetBrainsMonoNerdFont-Italic.woff2"
            ;;
        "BoldItalic")
            output_file="JetBrainsMonoNerdFont-BoldItalic.woff2"
            ;;
    esac
    
    echo "Downloading $variant variant..."
    echo "  URL: $download_url"
    
    if curl -L -o "$FONT_DIR/$output_file" "$download_url" --fail --progress-bar; then
        echo "  ✓ Downloaded $output_file"
        
        # Check if the file is valid (not empty and contains data)
        if [ ! -s "$FONT_DIR/$output_file" ]; then
            echo "  ✗ Error: Downloaded file is empty"
            rm -f "$FONT_DIR/$output_file"
        else
            # Check if it's a valid .woff2 file (starts with 'wOFF2' magic bytes)
            if head -c 5 "$FONT_DIR/$output_file" | grep -q 'wOFF2'; then
                echo "  ✓ Valid .woff2 file"
            else
                echo "  ⚠️  Warning: File may not be a valid .woff2"
                # Still keep it, might work anyway
            fi
        fi
    else
        echo "  ✗ Failed to download $variant"
        echo "  Trying alternative URL pattern..."
        
        # Try alternative URL pattern (without spaces encoded)
        alt_url_path=$(echo "$url_path" | sed 's/%20/ /g')
        alt_download_url="${NERD_FONTS_RAW}/${alt_url_path}"
        
        if curl -L -o "$FONT_DIR/$output_file" "$alt_download_url" --fail --silent --show-error; then
            echo "  ✓ Downloaded using alternative URL"
        else
            echo "  ✗ Failed with alternative URL as well"
        fi
    fi
done

# Verify we have the required files
echo ""
echo "Verifying downloaded fonts..."
REQUIRED_FILES=(
    "JetBrainsMonoNerdFont-Regular.woff2"
    "JetBrainsMonoNerdFont-Bold.woff2"
    "JetBrainsMonoNerdFont-Italic.woff2"
    "JetBrainsMonoNerdFont-BoldItalic.woff2"
)

all_present=true
for required_file in "${REQUIRED_FILES[@]}"; do
    if [ -f "$FONT_DIR/$required_file" ] && [ -s "$FONT_DIR/$required_file" ]; then
        file_size=$(stat -f%z "$FONT_DIR/$required_file" 2>/dev/null || stat -c%s "$FONT_DIR/$required_file" 2>/dev/null || echo "0")
        if [ "$file_size" -gt 1000 ]; then
            echo "  ✓ $required_file ($((file_size/1024)) KB)"
        else
            echo "  ✗ $required_file (file too small: ${file_size} bytes)"
            all_present=false
        fi
    else
        echo "  ✗ Missing: $required_file"
        all_present=false
    fi
done

if [ "$all_present" = true ]; then
    echo ""
    echo "✅ Success! All required .woff2 font files are present."
    echo "Fonts are ready in: $FONT_DIR"
    
    # Show file details
    echo ""
    echo "Font file details:"
    for required_file in "${REQUIRED_FILES[@]}"; do
        if [ -f "$FONT_DIR/$required_file" ]; then
            file_size=$(stat -f%z "$FONT_DIR/$required_file" 2>/dev/null || stat -c%s "$FONT_DIR/$required_file" 2>/dev/null || echo "0")
            echo "  - $required_file: $((file_size/1024)) KB"
        fi
    done
else
    echo ""
    echo "⚠️  Warning: Some font files are missing or invalid."
    echo "Current contents of fonts directory:"
    ls -la "$FONT_DIR/" 2>/dev/null || echo "  (fonts directory is empty)"
    echo ""
    echo "Alternative download options:"
    echo "1. Manual download from GitHub:"
    echo "   Visit: https://github.com/ryanoasis/nerd-fonts/tree/master/patched-fonts/JetBrainsMono"
    echo "   Navigate to the appropriate variant folder and download the .woff2 files"
    echo ""
    echo "2. Use the Nerd Fonts website:"
    echo "   Visit: https://www.nerdfonts.com/font-downloads"
    echo "   Download 'JetBrainsMono' and look for .woff2 files"
    echo ""
    echo "3. Convert .ttf to .woff2 (requires woff2 tools):"
    echo "   Install woff2 tools, then convert downloaded .ttf files"
fi

echo ""
echo "Font download process complete!"
