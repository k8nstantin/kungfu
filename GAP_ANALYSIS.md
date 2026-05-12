# KungFu Gap Analysis (Rolling Document)

This document tracks identified architectural blind spots, unimplemented critical paths, and technical debt.

## Active Gaps

### GAP-005: The "Osmosis" Delta Disconnect
*   **Status:** 🟡 PARTIAL (Alpha)
*   **Module:** `src/core/vfs.rs`, `src/sync/osmosis.rs`
*   **The Flaw:** The VFS `express()` method returns an `OperationTrace` (metadata), but fails to extract the raw Loro binary update bytes required for P2P transmission.
*   **The Fix:** Call `doc.export(ExportMode::Updates { from: vv })` immediately after splicing to capture the mathematical delta.

## Closed Gaps (Verified)

- **GAP-001:** Agent Read Tool (`kungfu_read`) - ✅ CLOSED
- **GAP-002:** VFS Metadata Tracking (UUIDv7 Names) - ✅ CLOSED
- **GAP-003:** Write-Ahead Log (WAL) Append - ✅ CLOSED
- **GAP-004:** Non-blocking Disk I/O (Async Mutex) - ✅ CLOSED

### GAP-006: Mirror Mode (Asynchronous Git Parity)
*   **Status:** 🔴 OPEN
*   **The Flaw:** We lack a safety net for human developers. If KungFu corrupts its internal state, work is lost.
*   **The Fix:** Implement an optional `--mirror-git` flag. Every `kf intent` or `kf expose` should automatically trigger a Git commit on the physical disk to provide a fallback audit trail.

### GAP-007: The "Ingestion" Problem
*   **Status:** 🔴 OPEN
*   **The Flaw:** We cannot easily move existing projects into KungFu.
*   **The Fix:** Implement `kf ingest [path]`. This tool must recursively read a physical directory and initialize the Loro `MovableTree` and `Text` objects.

### GAP-008: Mathematical vs. Syntactic Consistency
*   **Status:** 🔴 OPEN
*   **The Flaw:** CRDTs guarantee mathematical convergence but not valid code structure. Concurrent edits might result in un-compilable code.
*   **The Fix:** Implement "AST-Aware Selection." The `kf expose` phase must include a syntax check (via tree-sitter or native compilers) to ensure the mutation is valid.

### GAP-009: Large File / Junk Support (`.kfignore`)
*   **Status:** 🔴 OPEN
*   **The Flaw:** Splicing large binaries or junk files (node_modules) into the DNA will bloat the CRDT and choke the network.
*   **The Fix:** Implement a `.kfignore` mechanism to explicitly exclude paths from entering the Loro document.

### GAP-010: Hybrid Ledger Implementation (SurrealDB + Iceberg)
*   **Status:** 🔴 OPEN
*   **The Flaw:** We lack the server-side infrastructure to buffer and archive the DNA stream.
*   **The Fix:** Implement the Central Dojo server using SurrealDB for real-time Live Query broadcasting and Apache Iceberg for long-term Parquet-based operational storage.

### GAP-011: Idempotent VFS Transcription (Conflict Resolution)
*   **Status:** 🔴 OPEN
*   **The Flaw:** Transcription currently assumes physical disk state matches the CRDT intent. Disconnected edits might cause 'Ghost File' collisions.
*   **The Fix:** Make `transcribe` strictly idempotent; it must aggressively force the physical disk to match the Loro `MovableTree`, ignoring local un-synced physical changes.

### GAP-012: Iceberg Schema Evolution
*   **Status:** 🔴 OPEN
*   **The Flaw:** New metadata added to agent operations will break the fixed Parquet schema in Iceberg.
*   **The Fix:** The Micro-Batcher must utilize the Iceberg Schema Evolution API to dynamically add columns to the Parquet/Iceberg table before flushing new metadata keys.

### GAP-013: Ed25519 Key Revocation
*   **Status:** 🔴 OPEN
*   **The Flaw:** We lack a mechanism to revoke stolen cryptographic keys in a decentralized P2P network.
*   **The Fix:** Implement a 'Revocation Ledger' table in Iceberg. The SurrealDB ingest layer must reject operations from keys listed as revoked.

### GAP-014: Dashboard Hot/Cold Transition Flicker
*   **Status:** 🔴 OPEN
*   **The Flaw:** During the micro-batch flush from SurrealDB to Iceberg, data might temporarily disappear from queries during the catalog commit window.
*   **The Fix:** The React UI must implement optimistic merging of Hot (Surreal) and Cold (Iceberg) data, using UUIDv7s to de-duplicate and bridge the flush window.
