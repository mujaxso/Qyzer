#!/usr/bin/env bash

# Fix line endings for shell scripts
# Run this if you get "bad interpreter: No such file or directory" errors

echo "Fixing line endings for shell scripts..."

for script in *.sh; do
    if [ -f "$script" ]; then
        # Check if file has CRLF line endings
        if file "$script" | grep -q "CRLF" || grep -q $'\r' "$script"; then
            echo "Fixing $script..."
            # Remove carriage returns
            sed -i 's/\r$//' "$script"
            echo "  ✓ Fixed line endings"
        else
            echo "  $script already has Unix line endings"
        fi
    fi
done

echo ""
echo "✅ Line endings fixed!"
echo "Now try running: ./run.sh"
