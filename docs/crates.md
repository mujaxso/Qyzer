# Neote Crates

This document provides an overview of the Rust crates that make up the Neote workspace, their responsibilities, and dependencies.

## Overview

Neote is organized as a Cargo workspace with multiple crates, each with a specific responsibility. This modular architecture enables clear separation of concerns, independent development, and efficient compilation.

## Core Crates

### `core-types`
**Purpose**: Shared data structures and type definitions used across the entire system.
**Key Modules**:
- `ids`: Strongly-typed identifiers for documents, users, sessions, etc.
- `events`: Event definitions for system communication and state tracking.
- `commands`: Command structures for editor operations, AI tasks, and workspace actions.
- `protocol`: Communication protocol definitions for inter-component messaging.
**Dependencies**: `serde`, `uuid`

### `editor-buffer`
**Purpose**: Core text editing functionality for the IDE.
**Key Modules**:
- `buffer`: Text storage and basic manipulation operations.
- `edit`: Insert, delete, and replace operations with transaction support.
- `position`: Line/column positioning and navigation.
- `selection`: Text selection ranges and operations.
- `undo`: Undo/redo functionality with transaction tracking.
**Dependencies**: `serde`, `thiserror`

### `workspace-model`
**Purpose**: Modeling and managing workspace state.
**Key Modules**:
- `workspace`: Central workspace state container and high-level operations.
- `file_tree`: Hierarchical file and directory representation.
- `open_editors`: Tracking of currently open files and their state.
- `project_graph`: Dependency relationships between projects and modules.
- `snapshots`: Workspace state snapshots for undo/redo and persistence.
**Dependencies**: `serde`, `uuid`

## AI & Intelligence Crates

### `ai-context`
**Purpose**: Gathering, organizing, and preparing context for AI operations.
**Key Modules**:
- `collector`: Collects relevant information from various sources.
- `ranking`: Scores and prioritizes collected context based on relevance.
- `packing`: Organizes and compresses context to fit model constraints.
- `prompt`: Constructs structured prompts from context and task descriptions.
**Dependencies**: `serde`, `serde_json`

### `ai-agent`
**Purpose**: Orchestrating AI-driven tasks and operations.
**Key Modules**:
- `planner`: Breaks down high-level goals into actionable steps.
- `executor`: Executes planned tasks and manages state.
- `tools`: Interface and implementations for AI-accessible tools.
- `patch`: Generation and management of AI-created patches.
- `verify`: Validation of AI operation results for correctness and safety.
**Dependencies**: `serde`, `serde_json`, `anyhow`, `tokio`

## Infrastructure Crates

### `lsp-client`
**Purpose**: Language Server Protocol client implementation.
**Key Modules**:
- `manager`: Orchestrates multiple LSP sessions and lifecycle.
- `session`: Individual LSP session management.
- `transport`: Underlying communication channel handling.
- `capabilities`: Client/server capability negotiation.
- `diagnostics`: Processing and management of LSP diagnostics.
**Dependencies**: `serde`, `serde_json`, `tokio`, `anyhow`

### `patch-engine`
**Purpose**: Generating and applying changes to code and data.
**Key Modules**:
- `diff`: Generating differences between versions.
- `apply`: Applying patches with conflict detection.
- `preview`: Previewing patches before application.
**Dependencies**: `serde`, `anyhow`

### `rpc`
**Purpose**: Remote Procedure Call framework for inter-process communication.
**Key Modules**:
- `client`: RPC client implementation for outgoing requests.
- `server`: RPC server implementation for incoming requests.
- `framing`: Message framing over byte streams.
- `messages`: RPC message structure definitions.
**Dependencies**: `serde`, `serde_json`, `tokio`, `anyhow`

## Configuration & Security

### `settings`
**Purpose**: Configuration management for user and system settings.
**Key Modules**:
- `model`: Settings structure and validation rules.
- `loader`: Loading settings from various sources.
**Dependencies**: `serde`, `serde_json`, `anyhow`

### `permissions`
**Purpose**: Access control and security policies.
**Key Modules**:
- `policy`: Security policy and rule definitions.
- `grants`: Permission granting and revocation.
**Dependencies**: `serde`, `uuid`

## Applications & Services

### `desktop` (Application)
**Purpose**: Main desktop application with native UI.
**Key Modules**: UI modules for layout, editor, sidebar, chat, and terminal.
**Dependencies**: `tokio`, `tracing`, `anyhow`

### `workspace-daemon` (Service)
**Purpose**: Background service for workspace management.
**Responsibilities**: File operations, Git integration, terminal sessions, background tasks.
**Dependencies**: `tokio`, `tracing`, `anyhow`

### `ai-daemon` (Service)
**Purpose**: Background service for AI operations.
**Responsibilities**: Provider routing, streaming responses, quota management, request routing.
**Dependencies**: `tokio`, `tracing`, `anyhow`, `serde`, `serde_json`

## Development Guidelines

### Adding a New Crate
1. Create directory under `crates/` with appropriate name
2. Add `Cargo.toml` with workspace inheritance
3. Add minimal dependencies (prefer workspace dependencies)
4. Create `src/lib.rs` with module declarations
5. Add crate to workspace members in root `Cargo.toml`

### Dependency Management
- Use workspace dependencies when possible: `serde = { workspace = true }`
- Keep dependencies minimal and focused
- Document the purpose of each dependency
- Regular security audits via `cargo audit`

### Testing Strategy
- Unit tests within each crate
- Integration tests in `tests/` directory
- Property-based testing for core algorithms
- Mock external services for reliable tests

## Build & Development

```bash
# Build all crates
cargo build --workspace

# Test all crates
cargo test --workspace

# Check formatting
cargo fmt --all -- --check

# Run clippy
cargo clippy --workspace --all-targets -- -D warnings
```

This crate architecture provides a solid foundation for Neote's development while maintaining flexibility for future growth and specialization.
