# KungFu Architectural Blueprint

This document outlines the logical architecture of KungFu (kf), a next-generation version control system designed exclusively for the Agentic Era.

## 1. The Core Philosophy
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
2. **Central Dojo:** A high-speed **Blob Relay** that sequences and broadcasts operations.
3. **History Engine:** Visualizes the **Intent Timeline** for human review and replay.

## 5. Storage & Persistence

### Two-Tier Server Storage (Cloud-Agnostic)
- **Hot (Transactional):** Append-Only Log (WAL) on fast disk for instant agent writes.
- **Cold (Durable):** GCS/S3 buckets holding immutable Loro snapshots.
- **Policy:** Configurable (Temporal, Event-Driven, or Threshold-based).

### Local Persistence
- **WAL (Write-Ahead Log):** Operations are saved instantly to `.kungfu/ops.log`.
- **Materialized View:** The DNA is continuously **Transcribed** to the physical filesystem so legacy tools work unmodified.
