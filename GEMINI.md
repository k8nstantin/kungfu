# GEMINI.md — KungFu Engineering Standards

This document defines the foundational mandates and architectural invariants for the KungFu (kf) project. Every contribution must align with these principles.

## Core Mandates

### 1. Operation-First, Snapshot-Never
KungFu does not store files; it stores a graph of operations. All data persistence must go through the Loro CRDT engine. The local filesystem is merely a "materialized view" of the CRDT state.

### 2. Agent-Native Design
The primary user of KungFu is an AI Agent. The CLI (`kf`) is for humans, but the MCP server (`fe mcp`) is the heartbeat of the system. 
- **The VFS Abstraction:** Agents MUST NOT interact with the physical disk (`os.File`). All file operations must route through `pkg/vfs`. 
- **Splice over Overwrite:** MCP tools must be built to use `vfs.Splice()`. Full file overwrites (`WriteFile`) are considered legacy compatibility fallbacks and should be avoided in agent prompts to preserve granular CRDT history.
- **Intent Logging:** Agents must call `vfs.Annotate()` before beginning a sequence of edits to tie the CRDT operations to their reasoning phase.

### 3. P2P Integrity
KungFu is decentralized by default. 
- Use mDNS for peer discovery.
- Use state-based reconciliation for sync (Automerge-like or Loro-native sync).
- Never require a central server for a `kf stance` to be productive.

### 4. Absolute Standalone System
KungFu DOES NOT integrate with Git. It is the end-to-end source of truth from development to deployment.
- Build servers must use `kf checkout` to materialize code for deployment.
- Git is considered a legacy human format and is explicitly not supported.

## Architectural Patterns

- **Loro Wrapper:** All interaction with the Rust `loro` library must be encapsulated in `internal/core`. Use a clean Go interface to hide the CGO complexity.
- **VFS Abstraction:** The `pkg/vfs` layer must allow agents to operate on a "shadow" version of the codebase that doesn't necessarily exist on the physical disk yet.
- **DAG Alignment:** While KungFu is standalone, it must be "DAG-aware," allowing operations to be tagged with UUUDs from an external orchestrator (like OpenPraxis).

## Coding Standards

- ** Go Idioms:** Follow standard Go practices (small packages, error wrapping, context propagation).
- ** CGO Safety:** Keep the CGO surface area as small as possible. Prefer passing byte slices (Loro updates) over complex struct pointers across the FFI boundary.
- **Testing:** Every CRDT operation must have a property-based test ensuring convergence across concurrent nodes.
### 6. Dynamic Snapshotting (Two-Tier Storage)
KungFu utilizes a "Hot/Cold" storage architecture.
-jP **Hot (The Buffer):** An append-only log capturing high-frequency agent operations locally.
-jP **Cold (The Durable State):** Immutable Loro snapshots exported to S3/GCS.
The frequency of these snapshots MUST be configurable via a `SnapshotPolicy` variable (Temporal, Event-Driven via DAG, or Operation-Threshold) to allow operators to balance durability vs. cloud costs.