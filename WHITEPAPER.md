# 🥋 KungFu: Architecture of Infinite Engineering
**The Dawn of the Intelligent VCS**
**Deep-Dive: Intelligent Version Control, Dual-Tier AI, and Symmetric Visibility**
*Author: Constantin Alexander*

## Executive Summary
Version control has historically been a record-keeping exercise. In the Agentic Era, it must become a cognitive layer. KungFu re-envisions version control by merging mathematical convergence (Loro Rust CRDTs) with embedded intelligence (Gemma) and industrial-scale analytical ledgers (Apache Iceberg). This document details the tools and architectural tiers that transform the "Dark Work" of Git into the "Universal Symmetric Awareness" of KungFu.

---

## 1. The Surgical Toolset: Verbs of Evolution
KungFu abandons the 140+ administrative commands of Git. It replaces them with a streamlined, biologically-inspired toolset designed for machine-native speed and human-native reasoning.

### The Surgical Virtual File System (VFS)
The VFS is the "Brain" of the local client. It decouples **File Identity** from **Filesystem Pathing**.
*   **UUIDv7 Routing:** In KungFu, a file is not a string (e.g., src/main.go). It is a 128-bit time-sortable UUIDv7. The path is merely mutable metadata. 
*   **Conflict-Free Restructuring:** Because the VFS routes edits to UUIDs, an agent can rename /internal to /pkg while three other agents are performing character-level splices inside those files. The math routes the edits to the correct UUID regardless of the path change. 
*   **The Verb (kf transcribe):** To allow agents to run legacy compilers (`go build`, `npm test`) without breaking the human's main IDE, KungFu projects the CRDT state to a hidden physical "Execution Scratchpad." To prevent SSD burnout from high-frequency agent splices, `transcribe` utilizes **Incremental Memory-Mapping (mmap)**. It does not rewrite whole files; it flushes only the exact byte-deltas to the physical disk. Furthermore, it is **state-aware**, aggressively tombstoning physical ghost-files if an agent deletes a node in the CRDT.

### The Agent Gateway (MCP)
KungFu provides a native **Model Context Protocol (MCP)** server. This is the bridge that allows Claude, Gemini, and local models to "live" inside the VCS. 
*   **kungfu_patch:** Unlike traditional tools that force agents to overwrite files, patch allows agents to provide a "Find" and "Replace" block. The Rust engine calculates the mathematical offsets, protecting the agent from the character-counting errors that plague LLMs.

---

## 2. Gemma: The VCS with a Neural Network
KungFu is the first VCS that is semantically aware of the data it stores. We achieve this by embedding Google's open-weights **Gemma** model in two distinct architectural tiers.

### Local Tier: The Proactive Peer (Edge Intelligence)
Sitting on the developer's machine, a lightweight Gemma model monitors the live CRDT stream.
*   **The Immune System (Human-Gated):** Gemma performs an AST validation on every merge/expose execution. While CRDTs guarantee mathematical convergence, they do not guarantee logical correctness. If a concurrent rename breaks the AST, Gemma acts as the immune system by synthesizing a patch. **Crucially, this patch is never applied silently.** It is quarantined as a "Proposed Mutation" requiring explicit human approval, ensuring the AI never quietly weaves logical bugs into the DNA.
*   **Asynchronous Vigilance:** Gemma proactively looks for items humans and agents miss: **security vulnerabilities, hardcoded secrets, and violations of coding standards.** 
*   **The Not Annoying Promise:** Gemma does not block the terminal or throw pop-ups. It uses the **Loro Cursor** system to drop silent, spatial markers in the code. A developer sees a glowing indicator on a line; clicking it reveals Gemma's reasoning (e.g., Warning: This string looks like a production API key).

### Central Tier: The Iceberg Oracle (Cloud Intelligence)
A heavier Gemma model sits atop the **Apache Iceberg Ledger**, which archives the entire history of the enterprise's code evolution.
*   **Predictive Analytics:** By analyzing millions of historical CRDT operations, the central Gemma predicts future bottlenecks. It can flag areas of high cognitive collision or regression probability.
*   **Natural Language Discovery:** Humans no longer search history; they interview it. A manager asks: "Show me the evolution of the auth system." Gemma queries the Iceberg catalog, summarizes the Intent Log, and replays the relevant splices.

---

## 3. Radical Transparency: The End of "Dark Work"
Git is opaque; KungFu is a Glass Box. We achieve this through **Universal Symmetric Awareness**.

### Symmetric Visibility (The Radar)
In Git, branches are Dark Work—invisible until a PR is opened. KungFu eliminates this through a real-time **Presence Radar**.
*   **The Live Ticker:** The Central Dojo (via SurrealDB Live Queries) broadcasts a high-speed ticker of all activity. 
*   **Visual Presence:** When a human opens a file in the KungFu Dashboard, they see the agents in real-time. Agent cursors glow in different colors based on their current **Intent Status** (Reasoning, Splicing, or Testing).
*   **Collision Avoidance:** Because every actor is visible, the Congestion and Accidents of Git are replaced by **Fluid Avoidance**. Agents see a human's cursor and automatically route their edits to other functions to avoid cognitive collision.

### The Immutable Ledger (Transparency & Trust)
Every operation in KungFu is **Signed and Sorted**.
*   **Ed25519 Trust:** No code enters the DNA unless it is signed by a verified cryptographic key. This provides an indisputable audit trail.
*   **Tiered Decentralized Time Travel:** To preserve Git's legendary offline resilience, KungFu utilizes a **Tiered History** model. The local `kf` client maintains a high-performance **RocksDB Fossil Cache** of recent DNA evolution. This allows for sub-millisecond, offline time-travel and rollback. Only deep, multi-year archival queries require a network connection to the central **Apache Iceberg Oracle**, ensuring the developer can work from an airplane without losing visibility into the project's past.

---

## 4. The Data Lake: Iceberg as the Backbone
We utilize **Apache Iceberg** not just as a backup, but as the primary analytical engine.
*   **The ACID Commit:** Every micro-batch from the SurrealDB buffer is committed to the Iceberg Ledger via an ACID transaction. This ensures the Metadata Catalog is the ultimate source of truth.
*   **Standard SQL Interface:** Because the codebase is stored in a structured, columnar format (Parquet), engineering teams can use standard Data Science tools (Python, Snowflake, BigQuery) to analyze their own development velocity.

---

## Conclusion: Evolving Code over Managing Files
Git was a filing system for an era where humans typed slowly. **KungFu is an intelligent nervous system** for an era where machines and humans evolve code together at lightspeed. By combining the mathematical purity of CRDTs with the proactive intelligence of Gemma and the analytical power of Iceberg, we have reclaimed the thousands of hours stolen by legacy administration. 

**KungFu is the infrastructure for the next century of software engineering.**
