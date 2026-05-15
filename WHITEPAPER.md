# 🥋 KungFu: An Agent-Native VCS Architecture
*Author: Constantin Alexander* | *Version: 1.2*

## Table of Contents
1. [Abstract: The Concurrency Bottleneck](#abstract-the-concurrency-bottleneck)
2. [The Core Engine: Mathematical Certainty](#1-the-core-engine-mathematical-certainty)
3. [Embedded Intelligence: Semantic Awareness](#2-embedded-intelligence-semantic-awareness)
4. [Symmetric Awareness: Concurrency & State](#3-symmetric-awareness-concurrency--state)
5. [The Stack: High-Fidelity Primitives](#4-the-stack-high-fidelity-primitives)
6. [Collaboration](#collaboration)

---

## Abstract: The Concurrency Bottleneck
Git was architected for manual, batch-oriented human workflows based on Directed Acyclic Graphs (DAGs) of static snapshots. In the Agentic Era—where autonomous AI swarms generate code concurrently at millisecond frequencies—Git acts as a mechanical bottleneck. Its reliance on semantically blind snapshots, isolated branches, and heuristic text-merging leads to severe repository congestion and high conflict resolution overhead. 

**KungFu** is a version control system designed from first principles for machine-native concurrency. By orchestrating Rust, Loro CRDTs (Conflict-free Replicated Data Types), SurrealDB, Apache Iceberg, and Google’s Gemma models, KungFu replaces the rigid snapshot paradigm with a fluid, mathematically convergent data structure optimized for low-latency state synchronization.

---

## 1. The Core Engine: Mathematical Certainty
KungFu replaces heuristic merging (which relies on line-matching algorithms like diff3) with strict Algebraic certainty provided by CRDTs.

### The Virtual File System (VFS)
The VFS serves as the local interface between the in-memory CRDT graph and the physical computing environment.
*   **Identity-First Routing (UUIDv7):** Traditional VCS tracks entities by string paths (e.g., `src/main.go`). KungFu tracks entities via immutable 128-bit UUIDv7 identifiers. The path is treated as mutable metadata attached to the UUID. This structural decoupling allows one agent to rename a root directory while multiple other agents concurrently execute character-level splices inside its child files. The VFS routes the text splices to the UUIDs, resolving filesystem tree conflicts natively via Loro's `MovableTree` algorithm without risk of cyclic dependencies.
*   **The Execution Scratchpad:** Autonomous agents require physical files to execute language servers (LSP), compilers (`go build`), and test suites. KungFu isolates this process by projecting a "Ghost State" into isolated physical directories (`.kungfu/workspaces/<agent-id>/`). This architecture provides agents with 100% compatibility with standard POSIX build tools while shielding the human operator's primary IDE from unstable, intermediate compilation states.
*   **Incremental I/O (Memory-Mapping):** To prevent SSD burnout caused by writing physical files during high-frequency agent splices, the `transcribe` mechanism utilizes OS-level memory-mapping (`mmap`). By maintaining a memory-mapped projection of the CRDT state, KungFu relies on the operating system's page-fault mechanisms to flush only the specific modified byte-deltas to the hardware. This maintains high physical projection throughput with minimal write amplification.

### The Agent Gateway (MCP)
KungFu integrates a native Model Context Protocol (MCP) server, operating as an asynchronous JSON-RPC gateway for AI agents.
*   **Semantic Patching (`kungfu_patch`):** Large Language Models frequently miscalculate raw character offsets due to tokenization discrepancies. The `kungfu_patch` tool accepts semantic "Find/Replace" code blocks. The Rust engine performs a deterministic string search against the materialized Loro state to calculate the exact byte offsets mathematically. This ensures an agent's intent translates accurately into a CRDT splice without introducing token-counting corruption.

---

## 2. Embedded Intelligence: Semantic Awareness
CRDTs guarantee that concurrent operations will mathematically converge into a uniform state across all peers, but they do not guarantee that the resulting state is syntactically valid code. KungFu embeds open-weights **Gemma** models to introduce semantic and syntactic awareness to the data layer.

### Edge Intelligence (The Immune System)
A lightweight, quantized Gemma instance runs locally within the `kf` client environment to provide an asynchronous verification layer:
*   **AST-Aware Validation:** If a concurrent rename operation mathematically merges with a function call modification, the resulting code may fail to compile. During the `kf expose` phase, the system generates an Abstract Syntax Tree (AST) using `tree-sitter`. If parsing fails, the local Gemma model analyzes the broken AST state and synthesizes a repair patch.
*   **Human-Gated Application:** To maintain rigorous security and logic control, AI-synthesized patches are never applied silently to the CRDT. They are quarantined as "Proposals." The AI functions strictly as a continuous auditor; the human operator remains the final arbiter of state validation.
*   **Continuous Scanning:** Gemma proactively analyzes the incoming CRDT stream for hardcoded credentials, security vulnerabilities, and stylistic deviations. It flags these by dropping non-blocking spatial cursors in the code, alerting human operators without interrupting terminal execution flows.

### Central Intelligence (The Iceberg Oracle)
A higher-parameter Gemma model operates centrally, interfacing directly with the Apache Iceberg data lake for deep historical analysis:
*   **Semantic Querying:** Engineering operators can query historical repository states using natural language. Gemma translates these requests into optimized SQL queries against the Iceberg catalog, summarizing the sequence of logged Intents that led to specific architectural changes.
*   **Telemetry Analytics:** The integration allows for the extraction of precise engineering metrics: **Cognitive Collision Rates** (frequency of concurrent actor overlap on semantic blocks), **Mutation Survival Rates** (efficiency metrics for agent prompts passing CI), and **Surgical Velocity** (throughput measured in precise mathematical splices rather than arbitrary lines-of-code).

---

## 3. Symmetric Awareness: Concurrency & State
In Git, unmerged branches obscure active work-in-progress, leading to redundant effort and delayed integration. KungFu replaces branches with a real-time, symmetric presence layer.

*   **Live Presence via WebSockets:** Utilizing SurrealDB Live Queries, the active file focus of every human and agent is continuously broadcast to the network. Participants view spatial cursors in their clients, indicating the exact character offsets where concurrent actors are operating.
*   **Spatial Communication:** Agents and humans communicate by dropping persistent Loro Cursors containing JSON metadata directly onto specific code blocks. Because Loro Cursors bind to the underlying CRDT PeerID/Counter rather than string indices, the metadata floats dynamically as text is inserted or deleted above it. This utilizes the codebase's spatial geometry as the primary communication bus.
*   **Tiered History & Offline Resilience:** To maintain the offline operational capabilities inherent to decentralized VCS, the local `kf` client manages a highly compressed RocksDB cache of recent repository evolution. This permits sub-millisecond offline time-travel and rollback operations. Only deep, multi-year archival queries route to the central Iceberg ledger.

---

## 4. The Stack: High-Fidelity Primitives
The KungFu architecture achieves this performance by strictly orchestrating purpose-built, modern open-source technologies:

1.  **Rust:** Provides the foundational memory safety and multi-threaded performance required for the MCP server and concurrent CRDT evaluation without Garbage Collection latency.
2.  **Loro CRDTs:** Provides the `Fugue` algorithm for text interleaving and `MovableTree` for hierarchical filesystem management, achieving conflict-free, tombstone-efficient synchronization.
3.  **SurrealDB:** Deployed as an embedded, in-memory/RocksDB data store. It serves as a high-speed operational buffer, utilizing native `Live Queries` over WebSockets for zero-friction P2P broadcasting, rendering the central server effectively stateless and highly disposable.
4.  **Apache Iceberg:** Acts as the "Cold Ledger." A parameterized background process micro-batches operations from SurrealDB and executes ACID transactions against the Iceberg Metadata Catalog. This provides schema evolution, Multi-Version Concurrency Control (MVCC), and time-travel querying over immutable Parquet files.
5.  **Gemma:** Supplies the open-weights intelligence layer, enabling local AST validation and central data-lake analytics without reliance on rate-limited, closed-source APIs.

---

## Collaboration
KungFu replaces the administrative overhead of legacy version control with an architecture designed strictly for high-frequency, concurrent, agentic development.

We invite systems engineers, mathematicians, and AI researchers to review the architecture, examine the implementations, and collaborate.
