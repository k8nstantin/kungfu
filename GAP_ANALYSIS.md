# KungFu Gap Analysis (Rolling Document)

This document tracks identified architectural blind spots, unimplemented critical paths, and technical debt. As a research-grade project, identifying what we *haven't* solved is as important as what we have.

## Active Gaps

### GAP-001: The Agent is "Blind" (Missing Read Tool)
*   **Status:** 🔴 OPEN
*   **Module:** `src/mcp/server.rs`
*   **The Flaw:** The MCP server exposes `kungfu_splice` for writing, but no tool for reading. Agents cannot see the current CRDT text state to calculate splice offsets.
*   **The Fix:** Implement `kungfu_read`. It must traverse the Loro `MovableTree` via path string, return the `LoroText` content, and provide absolute character counts to aid the LLM in offset calculation.

### GAP-002: File Creation is Nameless (VFS Metadata)
*   **Status:** 🔴 OPEN
*   **Module:** `src/core/vfs.rs`
*   **The Flaw:** `OpType::Create` creates a node in the Loro `MovableTree`, but fails to initialize the associated metadata map with a `name` or `kind` (file/dir). 
*   **The Impact:** `kf transcribe` crashes or outputs "unnamed" because path resolution fails.

### GAP-003: The "Hot Log" is Fake (WAL Not Implemented)
*   **Status:** 🔴 OPEN
*   **Module:** `src/mcp/server.rs`, `src/cli/init.rs`
*   **The Flaw:** `kf init` touches `.kungfu/ops.log`, but the MCP server never appends to it. Instead, the server triggers a full `snapshot.loro` export on every keystroke.
*   **The Impact:** Destroys disk IOPS. Violates the "Two-Tier Storage" architecture.
*   **The Fix:** Serialize the `OperationTrace` and append to `ops.log`. Snapshotting should be decoupled into a background or event-driven process.

### GAP-004: Async Deadlock Risk (Disk I/O within Mutex)
*   **Status:** 🔴 OPEN
*   **Module:** `src/mcp/server.rs`
*   **The Flaw:** The `axum` JSON-RPC handler locks the shared `Dojo` state, applies the CRDT math, and then blocks on synchronous disk I/O (`fs::write`) *before* releasing the lock.
*   **The Impact:** Concurrent agents will bottleneck. 
*   **The Fix:** Disk I/O must happen outside the critical section. Update the CRDT in RAM, drop the lock, and spawn a `tokio::task` to handle persistence.

### GAP-005: The "Osmosis" Delta Disconnect
*   **Status:** 🔴 OPEN
*   **Module:** `src/core/vfs.rs`, `src/sync/osmosis.rs`
*   **The Flaw:** The VFS `express()` method returns an `OperationTrace` (metadata), but fails to extract the raw Loro binary update bytes required for P2P transmission.
*   **The Fix:** Call `doc.export(ExportMode::Updates { from: vv })` immediately after splicing to capture the mathematical delta, and attach it to the `OperationTrace` or `OsmosisUpdate` payload.

---
*Note: Resolve these gaps before declaring Phase 3 (Central Dojo) complete.*
