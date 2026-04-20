#!/bin/bash
# zaroxi-refactor.sh

set -e

echo "Phase 1: Creating new crate directories..."
mkdir -p
crates/zaroxi-{ai-agent,core-ids,domain-{ai-context,editor,workspace},infra-{permissions,rpc,settings},lang-{lsp,syntax},ops-{file,pa
tch},protocol,service-{ai,workspace},theme}

echo "Phase 2: Moving existing crates..."
# Copy files (not moving to avoid data loss)
for src in "crates/ai/ai-agent" "domain/ai-context" "domain/editor-core" "domain/workspace-model" \
  "crates/infrastructure/permissions" "crates/infrastructure/rpc" "crates/infrastructure/settings" \
  "crates/language/lsp-client" "crates/language/syntax-core" \
  "crates/operations/file-ops" "crates/operations/patch-engine" "crates/theme"; do
  if [ -d "$src" ]; then
    dest_name=$(echo "$src" | sed '
            s|^crates/ai/ai-agent$|zaroxi-ai-agent|
            s|^domain/ai-context$|zaroxi-domain-ai-context|
            s|^domain/editor-core$|zaroxi-domain-editor|
            s|^domain/workspace-model$|zaroxi-domain-workspace|
            s|^crates/infrastructure/permissions$|zaroxi-infra-permissions|
            s|^crates/infrastructure/rpc$|zaroxi-infra-rpc|
            s|^crates/infrastructure/settings$|zaroxi-infra-settings|
            s|^crates/language/lsp-client$|zaroxi-lang-lsp|
            s|^crates/language/syntax-core$|zaroxi-lang-syntax|
            s|^crates/operations/file-ops$|zaroxi-ops-file|
            s|^crates/operations/patch-engine$|zaroxi-ops-patch|
            s|^crates/theme$|zaroxi-theme|
        ')
    echo "Copying $src to crates/$dest_name"
    cp -r "$src"/* "crates/$dest_name/" 2>/dev/null || true
  fi
done

echo "Phase 3: Updating Cargo.toml package names..."
# Update package names in each crate
for crate in crates/zaroxi-*; do
  if [ -f "$crate/Cargo.toml" ]; then
    old_name=$(basename "$crate")
    # Remove zaroxi- prefix for sed pattern
    base_name=${old_name#zaroxi-}
    original_name=$(echo "$base_name" | sed '
            s/-ai-agent/ai-agent/
            s/-domain-ai-context/ai-context/
            s/-domain-editor/editor-core/
            s/-domain-workspace/workspace-model/
            s/-infra-permissions/permissions/
            s/-infra-rpc/rpc/
            s/-infra-settings/settings/
            s/-lang-lsp/lsp-client/
            s/-lang-syntax/syntax-core/
            s/-ops-file/file-ops/
            s/-ops-patch/patch-engine/
            s/-theme/theme/
        ')

    # Update Cargo.toml
    sed -i "s/^name = \"$original_name\"/name = \"$old_name\"/" "$crate/Cargo.toml"

    # Add description if missing
    if ! grep -q "description =" "$crate/Cargo.toml"; then
      sed -i "/^name = /a description = \"$base_name for Zaroxi Studio\"" "$crate/Cargo.toml"
    fi
  fi
done

echo "Phase 4: Creating split crates..."
# Create zaroxi-core-ids from core-types/src/ids.rs
if [ -f "crates/core-types/src/ids.rs" ]; then
  cp "crates/core-types/src/ids.rs" "crates/zaroxi-core-ids/src/lib.rs"
fi

# Create zaroxi-protocol from the rest of core-types
if [ -d "crates/core-types" ]; then
  mkdir -p crates/zaroxi-protocol/src
  for file in commands.rs events.rs protocol.rs workspace.rs; do
    if [ -f "crates/core-types/src/$file" ]; then
      cp "crates/core-types/src/$file" "crates/zaroxi-protocol/src/"
    fi
  done
  echo 'pub mod commands;
pub mod events;
pub mod protocol;
pub mod workspace;' >crates/zaroxi-protocol/src/lib.rs
fi

echo "Phase 5: Setting up service crates..."
# Create service crates from services/
if [ -d "services/ai-daemon" ]; then
  mkdir -p apps/ai-daemon
  cp -r services/ai-daemon/* apps/ai-daemon/ 2>/dev/null || true
  # Update Cargo.toml
  sed -i 's/^name = ".*"/name = "zaroxi-ai-daemon"/' apps/ai-daemon/Cargo.toml 2>/dev/null || true
fi

if [ -d "services/workspace-daemon" ]; then
  mkdir -p apps/workspace-daemon
  cp -r services/workspace-daemon/* apps/workspace-daemon/ 2>/dev/null || true
  sed -i 's/^name = ".*"/name = "zaroxi-workspace-daemon"/' apps/workspace-daemon/Cargo.toml 2>/dev/null || true
fi

echo "Phase 6: Moving runtime..."
if [ -d "runtime" ]; then
  mkdir -p crates/zaroxi-lang-syntax/runtime
  cp -r runtime/* crates/zaroxi-lang-syntax/runtime/ 2>/dev/null || true
fi

echo "Refactoring complete!"
echo "Next steps:"
echo "1. Run the updated root Cargo.toml"
echo "2. Update imports in Rust files with: find . -name '*.rs' -exec sed -i 's/\\bcore_types\\b/zaroxi_protocol/g' {} \\;"
echo "3. Run 'cargo check --workspace' to verify"
