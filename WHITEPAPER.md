# 🥋 KungFu: An Agent-Native VCS Architecture
*Author: Constantin Alexander* | *Version: 1.1*

## 🔬 Abstract: The Concurrency Bottleneck
Git was architected for manual, batch-oriented human workflows. In the Agentic Era—where autonomous swarms generate code concurrently at millisecond frequencies—Git is a mechanical bottleneck. Its reliance on semantically blind snapshots, isolated branches, and heuristic merging leads to severe repository congestion. 

**KungFu** is a version control system built on modern primitives. By orchestrating **Rust, Loro CRDTs, SurrealDB, Apache Iceberg, and Google’s Gemma**, KungFu replaces the rigid snapshot with a fluid, mathematically convergent data structure. 

---

## 1. The Core Engine: Mathematical Certainty
KungFu replaces heuristic merging (guessing how text fits together) with Algebraic certainty.

### The Virtual File System (VFS)
The VFS serves as the interface between the CRDT graph and the physical world.
*   **Identity-First Routing (UUIDv7):** In Git, a file's identity is its string path. In KungFu, a file is an immutable 128-bit UUIDv7; its path is mutable metadata. An agent can rename a root directory while ten others concurrently edit its contents. The VFS routes the edits to the UUID, resolving structural conflicts automatically.
*   **The Execution Scratchpad:** Agents require physical files for compilers (`go build`, `npm test`). KungFu continuously projects a "Ghost State" into isolated directories (`.kungfu/workspaces/<agent-id>/`). This shields the human's primary environment while granting agents 100% compatibility with standard build tools.
*   **Incremental I/O (mmap):** To prevent SSD burnout from high-frequency agent splices, the `transcribe` mechanism utilizes memory-mapping. We flush only specific byte-deltas to the hardware, maintaining high throughput for the physical projection.

### The Agent Gateway (MCP)
*   **kungfu_patch:** LLMs frequently miscalculate raw character offsets. `patch` accepts semantic "Find/Replace" blocks. The Rust engine calculates the exact offsets mathematically against the Loro state, ensuring an agent's intent is not corrupted by a token-counting error.

---

## 2. Embedded Intelligence: Semantic Awareness
KungFu embeds open-weights **Gemma** models to introduce semantic awareness to the data layer.

### Edge Intelligence (The Immune System)
A lightweight Gemma instance runs locally, providing a verification layer:
*   **AST-Aware Validation:** CRDTs guarantee mathematical convergence, but not syntactic logic. If a concurrent rename breaks a function call, Gemma analyzes the resulting Abstract Syntax Tree (AST) during the `kf expose` phase and synthesizes a repair patch.
*   **Human-Gated Application:** AI-synthesized patches are quarantined as "Proposals." The AI acts as an auditor; the human operator remains the final arbiter of truth.
*   **Continuous Scanning:** Gemma proactively flags hardcoded secrets and security flaws as they are typed, dropping non-blocking spatial cursors in the IDE rather than interrupting the terminal flow.

### Central Intelligence (The Iceberg Oracle)
A larger Gemma model sits atop the **Apache Iceberg Ledger** for historical analysis:
*   **Semantic Querying:** Operators can query history naturally (e.g., *"Explain the evolution of the Auth system and the introduction of the JWT dependency"*). Gemma queries the Iceberg catalog and replays the sequence of logged Intents.
*   **Telemetry:** Iceberg exposes precise metrics: **Cognitive Collision Rates** (where actors overlap), **Mutation Survival Rates** (prompt efficiency), and **Surgical Velocity** (true engineering throughput).

---

## 3. Symmetric Awareness: Replacing "Dark Work"
In Git, unmerged branches obscure work-in-progress. KungFu utilizes a real-time presence layer.

*   **Live Presence:** Through SurrealDB Live Queries, the active focus of every human and agent is broadcast. Participants see spatial cursors indicating where others are currently operating.
*   **Spatial Communication:** Agents and humans communicate by dropping cursors containing JSON metadata directly onto the code, utilizing the codebase's geometry as the communication bus.
*   **Tiered History (RocksDB):** To maintain offline resilience, the local `kf` client maintains a RocksDB cache of recent evolution, enabling sub-millisecond offline time-travel. Only deep archival queries route to the central Iceberg ledger.

---

## 4. The Stack: High-Fidelity Primitives
This architecture relies entirely on the capabilities of modern open-source technologies:
1.  **Rust:** Provides memory-safe, multi-threaded performance for the MCP server and CRDT engine.
2.  **Loro CRDTs:** Implements the `Fugue` and `MovableTree` algorithms for conflict-free, tombstone-efficient synchronization.
3.  **SurrealDB:** Acts as an embedded, high-speed buffer utilizing Live Queries for zero-friction P2P broadcasting.
4.  **Apache Iceberg:** Provides ACID transactions, schema evolution, and time-travel for the immutable historical ledger.
5.  **Gemma:** Supplies the open-weights intelligence required for local AST validation and central data-lake analytics.

---

## 🤝 Collaboration
We are replacing the administrative bottleneck of version control with an architecture designed for high-frequency, concurrent agentic development.

We invite engineers and AI researchers to review the architecture, examine the code, and collaborate in the Dojo. 🥋
