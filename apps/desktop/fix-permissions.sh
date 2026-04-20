#!/usr/bin/env bash

# Fix permissions script for Zaroxi Desktop
# If this script is not executable, run: chmod +x fix-permissions.sh

echo "Fixing script permissions..."

# First, make this script executable if it's not
if [ ! -x "$0" ]; then
    echo "Making this script executable..."
    chmod +x "$0"
fi

# Fix line endings and make scripts executable
for script in run.sh start.sh setup.sh build.sh fix-permissions.sh clear-cache.sh; do
    if [ -f "$script" ]; then
        # Fix line endings if needed
        if file "$script" | grep -q "CRLF"; then
            echo "Fixing line endings for $script..."
            dos2unix "$script" 2>/dev/null || sed -i 's/\r$//' "$script"
        fi
        chmod +x "$script"
        echo "✓ Made $script executable"
    fi
done

# Make check-setup.js executable if it exists
if [ -f "check-setup.js" ]; then
    chmod +x check-setup.js
    echo "✓ Made check-setup.js executable"
fi

echo ""
echo "✅ Permissions fixed!"
echo ""
echo "Now you can run:"
echo "  ./run.sh      # Start development"
echo "  ./start.sh    # Alternative start"
echo "  ./setup.sh    # Install dependencies"
echo "  ./build.sh    # Build for production"
echo ""
echo "If you still get 'permission denied', try:"
echo "  chmod +x *.sh"
echo "  chmod +x check-setup.js"
echo ""
echo "If you get 'bad interpreter' errors, try:"
echo "  sed -i 's/\r$//' *.sh"
