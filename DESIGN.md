# KungFu Architectural Blueprint

This document outlines the logical architecture of KungFu (kf), a next-generation version control system designed exclusively for the Agentic Era.

## 1. The Core Philosophy
*For the high-level vision and "Why" behind these choices, see the **[Brainstorming Archive](./BRAINSTORMING.md)**.*
- **Agent-First**: Git is snapshot-based and human-first. KungFu is operation-based and agent-first.
- **Flow over Snapshot**: Versioning is ambient. Every edit is a first-class CRDT operation that streams in real-time. 
- **Trunk-Only (No Branches)**: There are no divergent branches. Everything happens in the DNA. Mutations are ephemeral filters, not side-tracks.

## 2. Technical Foundation
- **CRDT Engine:** Powered by [Loro](https://loro.dev) for mathematical conflict resolution using Fugue (Text) and MovableTree (Filesystem).
- **Identity:** All operations and entities use **UUIDv7** for decentralized, time-sortable ID-generation.
- **Security:** Every operation is signed with **Ed25519** cryptographic keys.
- **Interface:** The **Model Context Protocol (MCP)** acts as the agent gateway.

## 3. The CLI Verbs (The Evolutionary Process)

| Command | legacy (Git) | Meaning |
| :--- | :--- | :--- |
| `kf mutate` | checkout -b | Start an experimental change. |
| `kf intent` | commit -m | Explain the "Why" of the work. |
| `kf expose` | merge / ci | Test the code; if it survives, it is permanent DNA. |
| `kf seed` | tag / release | Create a version to "plant" in production. |
| `kf osmose` | push / pull | Continuous, background sync with peers. |
| `kf timeline` | log | View the history of DNA evolution. |

## 4. Logical Architecture

### The Virtual File System (VFS)
Agents interact with a **Surgical VFS** that translates standard file operations into CRDT math:
- **Splice:** The primary edit mechanism (add/delete/replace at offset).
- **Annotate:** Tied to the `intent` command; groups operations for human review.

### The Sync Hierarchy
1. **Local Dojo:** Maintains the in-memory Loro document and exposes the MCP server.
2. **Central Dojo:** A high-speed **Hybrid Ledger**. It sequences operations via **SurrealDB** and archives the DNA sequence into **Apache Iceberg**.
3. **History Engine:** Visualizes the **Intent Timeline** using the Iceberg chronological record for replay and auditing.

## 5. Storage & Persistence

### Two-Tier Server Storage (Hybrid Ledger)

### The Disposable Server & Self-Healing Sync
KungFu eliminates DevOps complexity by treating the Central Dojo as a **Disposable Relay**. You do not need to deploy a highly-available, multi-region database cluster.

1. **Embedded Buffer:** The Rust server runs SurrealDB in embedded mode (In-Memory or RocksDB). It is purely a hot transit buffer.
2. **The Crash Scenario:** If the server container crashes, any operations in the hot buffer that haven't been micro-batched to Iceberg are destroyed.
3. **CRDT Gossip Recovery:** This is not a data loss event; it is a minor hiccup. When the server reboots, it loads the last known Version Vector from Iceberg. When agents reconnect, they perform a vector handshake. The server mathematically detects the gap, and the agents' local `kf` clients automatically re-transmit the missing operations.

By pushing the responsibility of state to the edges of the network (the agents) and the cold storage (Iceberg), the server requires zero database administration.

- **Hot (Real-time Buffer):** **SurrealDB** acts as the persistent high-speed buffer. Agents stream Loro binary deltas into SurrealDB, which utilizes Live Queries for sub-millisecond P2P broadcast.
- **Cold (Analytical Ledger):** **Apache Iceberg** (backed by S3/GCS) acts as the absolute Source of Truth and chronological history.
- **Micro-Batching:** A parameterized background process continuously drains SurrealDB and writes immutable Parquet files to the Iceberg table, enabling native time-travel and deep auditability.

### Local Persistence
- **WAL (Write-Ahead Log):** Operations are saved instantly to `.kungfu/ops.log`.
- **Materialized View:** The DNA is continuously **Transcribed** to the physical filesystem so legacy tools work unmodified.


## 9. Situational Awareness (The End of "Dark Work")

In Git, work happens in the dark. A developer or agent disappears onto a branch for days and returns with a massive, un-mergeable PR. KungFu eliminates "Dark Work" through real-time Liveness and Presence.

- **Spatial Presence (The Radar):** Agents and Humans constantly broadcast their focus via the WebSocket. The UI/CLI displays exactly who is reading or splicing which block of code in real-time. This prevents redundant work and allows actors to avoid 'hot' zones.
- **The Osmosis Stream (The Live Ticker):** The Dashboard visualizes the CRDT operation flow not as static files, but as a live ticker of events (e.g., `[10:42] Agent A spliced 14 lines in auth.go`). Humans achieve total God-mode situational awareness of the Swarm's activities.

### Symmetric Visibility (The Code is the Communication Bus)

### Universal Symmetric Awareness (Total Dojo Transparency)
Symmetric visibility extends beyond the text buffer. Humans and agents share a unified, real-time awareness of the entire Dojo state. 

- **Symmetric Cursors:** Humans see agent cursors; agents 'see' human cursors. This spatial awareness provides the primary context for collaboration.
- **Shared Metadata:** Any metadata attached to a VFS node (UUIDv7, kind, custom tags) is instantly visible to all actors. 
- **The Unified Reality:** There is no 'hidden' or 'local-only' state in the flow. If a human begins a mutation, the agents see the ripple. If an agent shifts its intent, the human sees the update. The entire architectural concept—DNA, Mutations, and the Organism—is a shared, transparent reality.

Because humans and agents are connected to the exact same continuous CRDT stream, the barrier between "Human Workspace" and "Machine Workspace" is eliminated. 

- **Bi-Directional Presence:** If a human opens `auth.go` in their IDE, their local `kf` client broadcasts their presence. Any agent operating in that file is instantly notified via the MCP server: *"Human operator has entered the file."*
- **Real-Time Intervention:** A human can type a comment directly into the code (e.g., `// Agent Alpha, pause the JWT validation, I am modifying the struct`). The agent receives this CRDT text splice instantly, processes the context, and adapts. 
- **The End of Chat Windows:** You do not need a detached Slack channel or terminal prompt to talk to the swarm. You communicate natively through the spatial geometry of the codebase. You are a peer programmer swimming in the exact same stream.

