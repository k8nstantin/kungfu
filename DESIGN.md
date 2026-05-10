# KungFu Architectural Blueprint

This document outlines the logical architecture of KungFu (kf), a next-generation version control system designed explusively for the Agentic Era.

## 1. The Core Philosophy
- **Agent-First**: Git is snapshot-based and human-first. KungFu is operation-based and agent-first.
- **Flow over Snapshot**: Versioning is ambient. Every edit is a first-class CRDT operation that streams in real-time. 

## 2. Technical Foundation
- **CRDT Engine:** Powered by [Loro](https://loro.dev) for mathematical conflict resolution using Fugue (Text) and MovableTree (Filesystem).
- **Identity:** All operaations and entities use __UUIDv7__ for decentralized, time-sortable ID-generation.
- **Interface:** The __Model Context Protocol (MCP)__ acts as the gateway, allowing agents to edit the CRDT graph via surgical splices.

## 3. Logical Architecture

### The Virtual File System (VFS)
Agents interact with a __Surgical VFS__ that translates standard file operations into CRDT math:
- **Splice:** The primary edit mechanism (add/delete/replace at offset).
- **Annotate:** Ties a sequence of operations to a high-level __Intent__ (e.g., "Refactoring logger") for human review.

### The Sync Hierarchy
1. **Local Dojo:** Runs on the developer laptop. Maintains the in-memory Loro document and exposes the MCP server.
2. **Central Dojo:** A high-speed __Blob Relay__. It sequences operations from multiple peers and broadcasts them conflict-free.
3. **History Engine:** Visualizes the __Intent Timeline__, allowing humans to replay, approve, or revert agent work at the intent level.

## 4. Storage & Persistence


### Two-Tier Server Storage
- **Hot (Transactional):** Append-Only Log on fast disk for instant agent writes.
- **Cold (Durable):** S3/GCS buckets holding immutable Loro snapshots.
- **Snapshot Policy:** Variable control (Temporal, Event-Driven, or Threshold-based).

### Local Persistence
- **WAL (Write-Ahead Log)():** Local operations are saved instantly to `.kungfu/ops.log`.
- **Materialized View:** The CRDT state is continuously flushed to the physical filesystem (e.g., `main.go`) so legacy IDEs and compilers work unmodified.

## 5. Network & Authentication (The "Flow" Protocol)

KungFu abandons manual `push/pull` over HTTP/SSH in favor of a continuous, streaming, and cryptographically secure mesh.


### The Transport Layer
- **WebSockets / gRPC:** Clients maintain long-lived, bi-directional connections to the Central Dojo.
- **Binary Payloads  Bincode):** We do not send JSON diffs. Clients stream highly compressed, native Loro binary updates (`[u8]`) for sub-millisecond sync.
- **Version Vector Exchange:** Connections begin with a state vector comparison, ensuring nodes only transmit the deltas the peer is missing.


### Cryptographic Identity (Ed25519)
In Git, authorship is a manual string (`user.name`). In runaway agent swarms, trust must be mathematical.
- **Keypairs:** Every KingFu client (agent or human) generates an Ed25519 keypair (get, `auth gen`). The Public Key is the absolute identity.
- **Signed Operations:** The Central Dojo does not trust the connection; it trusts the payload. Every `OperationTrace` sent over the wire is signed by the actor's Private Key.
- **Anfi-Amnesia Audit:** Because every CRDT splice is cryptographically tied to an Agent's ephemeral key, or orchestrator's Key, the Action Log provides indisputable, non-repudiable proof of who wrote what code.