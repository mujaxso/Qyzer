# Neote Architecture

Neote is an AI-first IDE built as a modular Rust workspace. This document outlines the high-level architecture and component relationships.

## Overview

Neote follows a client-server architecture with a desktop frontend, multiple background services, and a collection of shared libraries (crates). The system is designed to be extensible, with clear separation between UI, business logic, and AI capabilities.

## Workspace Structure

### Applications (`apps/`)
- **desktop**: The main desktop application built with a native UI framework (to be chosen). Contains UI modules for editor, sidebar, chat, terminal, and layout management.

### Services (`services/`)
- **workspace-daemon**: Manages workspace state, file system operations, Git integration, terminal sessions, and background tasks.
- **ai-daemon**: Orchestrates AI operations including provider routing, streaming responses, quota management, and request routing.

### Core Crates (`crates/`)
- **core-types**: Shared data structures for IDs, events, commands, and protocol definitions.
- **editor-buffer**: Text editing primitives: buffer management, edits, positions, selections, and undo/redo.
- **workspace-model**: Models for workspace state, file trees, open editors, project graphs, and snapshots.
- **lsp-client**: Language Server Protocol client with session management, transport, capabilities, and diagnostics.
- **ai-context**: AI context collection, ranking, packing, and prompt construction.
- **ai-agent**: AI agent planning, execution, tool usage, patch generation, and verification.
- **patch-engine**: Diff generation, patch application, and preview functionality.
- **rpc**: Remote Procedure Call framework with client/server, framing, and message handling.
- **settings**: Configuration model and loader.
- **permissions**: Policy and grants for security and access control.

## Frontend Platform Strategy

Zaroxi Studio adopts a hybrid frontend architecture that combines native Rust UI (Iced) for performance‑sensitive editor surfaces with webview‑based (Tauri) windows for previews and design canvases. This approach preserves the investment in the existing Iced codebase while enabling rich, HTML‑based simulations of mobile, desktop, and website outputs.

**Key decisions:**

1. **Desktop shell** – The main window (file tree, editor, terminal) remains an Iced application (`apps/desktop`), ensuring native rendering performance and tight integration with the Rust core.
2. **Preview shell** – A separate Tauri application (`apps/preview`) provides webview windows for device simulations, visual designers, and live previews of AI‑generated experiences.
3. **Communication** – Both shells communicate with the same Rust business‑logic crates and background services via the existing RPC framework (`crates/rpc`).
4. **Separation of concerns** – UI‑specific code is kept out of the core crates; the `preview‑engine` crate encapsulates simulation logic, device models, and the local HTTP server that serves preview content.

This hybrid model allows Zaroxi AI to generate experiences that can be immediately previewed in a realistic environment (mobile, web, desktop) while retaining the responsive, native feel of a professional code editor.

## Component Relationships

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   Desktop App   │◄──►│  Workspace Model │◄──►│  Editor Buffer  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
         │                        │                        │
         ▼                        ▼                        ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│   UI Modules    │    │   LSP Client     │    │   Patch Engine  │
│  (layout, chat, │    │                  │    │                 │
│  editor, etc.)  │    └──────────────────┘    └─────────────────┘
└─────────────────┘            │                        │
         │                     ▼                        ▼
         │            ┌──────────────────┐    ┌─────────────────┐
         └───────────►│   AI Context     │◄──►│     AI Agent    │
                      └──────────────────┘    └─────────────────┘
         ▲                     │                        │
         │                     ▼                        ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Preview Shell  │    │   AI Daemon      │    │  RPC Framework  │
│   (Tauri/web)   │    └──────────────────┘    └─────────────────┘
└─────────────────┘            │                        │
         ▲                     ▼                        ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│ Preview Engine  │◄──►│ Workspace Daemon │◄──►│    Settings     │
│ (simulation,    │    └──────────────────┘    └─────────────────┘
│  device models) │
└─────────────────┘
```

**Preview Subsystem:** The Preview Engine crate (`crates/preview-engine`) contains the logic for simulating mobile devices, desktop frames, and browser environments. It runs a local HTTP server that serves the content to be previewed, which is displayed in the Tauri-based Preview Shell. The Desktop App (Iced) communicates with the Preview Engine via RPC to update previews in real time as the user edits code or design specifications.

## Anti‑Patterns to Avoid

- **Rewriting the UI from scratch** – Do not discard the existing Iced codebase; it provides a performant, native editor experience that would be costly to rebuild.
- **Tight coupling between UI and core logic** – Keep UI‑specific code out of `editor‑core`, `workspace‑model`, and other business‑logic crates.
- **Direct webview embedding inside Iced** – This is technically complex and unstable; instead use separate windows managed by Tauri.
- **Blocking the main thread with preview updates** – All communication between the desktop app and the preview engine must be asynchronous (via RPC) to keep the editor responsive.
- **Premature abstraction** – Build the preview infrastructure for concrete use cases first (mobile, web, desktop simulation) before attempting a generic “everything preview” system.

## Communication Patterns

1. **Desktop ↔ Services**: RPC over local sockets or message passing
2. **Inter-crate**: Direct Rust dependencies for compile-time safety
3. **AI Operations**: Async streaming via the AI daemon
4. **LSP**: JSON-RPC over stdio or sockets via the lsp-client crate

## Key Design Principles

1. **Modularity**: Each crate has a single, well-defined responsibility
2. **Extensibility**: Plugin system for language support and AI providers
3. **Performance**: Native Rust implementation with async/await where appropriate
4. **Security**: Permission system for sensitive operations
5. **Observability**: Built-in tracing and logging throughout

## Extension Points

- **Language Support**: Via LSP client and extension crates
- **AI Providers**: Pluggable through the AI daemon
- **UI Themes**: Through the desktop application's theming system
- **Tools**: Extensible toolset for the AI agent

## Development Workflow

1. **Build**: `cargo build --workspace`
2. **Test**: `cargo test --workspace`
3. **Format**: `cargo fmt --all`
4. **Lint**: `cargo clippy --workspace --all-targets`
5. **Run**: Individual binaries can be run directly (e.g., `cargo run -p desktop`)

## Incremental Migration Plan

1. **Phase 1 (Current)** – Keep the existing Iced desktop app unchanged. Introduce the `preview‑engine` crate as a library, with a simple local HTTP server that can serve static HTML. No integration with the UI yet.
2. **Phase 2** – Create a separate Tauri application (`apps/preview`) that embeds a webview and connects to the preview‑engine via RPC. Provide basic window management and device‑frame rendering. The desktop app can launch this preview window via a command.
3. **Phase 3** – Implement real‑time synchronization: as the user edits code, the desktop app sends updates through RPC to the preview‑engine, which refreshes the webview. Add mobile and desktop viewport presets.
4. **Phase 4** – Optionally migrate non‑editor UI panels (AI chat, asset library, design canvas) to webview‑based windows, leveraging the same preview infrastructure. Keep the core editor in Iced.

This plan ensures that each step is self‑contained, does not break existing functionality, and progressively builds towards the hybrid architecture.

## Future Considerations

- **Multi-platform Support**: Windows, macOS, Linux
- **Cloud Sync**: Optional synchronization of workspace state
- **Collaboration**: Real-time collaborative editing
- **Mobile**: Potential mobile companion applications

This architecture is designed to evolve as Neote grows, maintaining flexibility while providing a solid foundation for an AI-first development experience.
