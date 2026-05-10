# KungFu (kf) 🥋

**"KungFu is evolving your code."**

KungFu is a next-generation, agent-native version control system (VCS) powered by the **Loro CRDT engine**. It replaces the rigid, human-first paradigm of Git with a fluid, operation-based system where AI agents and humans collaborate in a continuous, conflict-free stream.

---

## 🛑 Why not Git? (The Problem)

Git was built in 2005 for humans emailing patches to each other. It is a phenomenal tool for human administration, but it is fundamentally broken for the Agentic Era:

1. **Snapshots are Lossy:** Git tracks the *result* (diffs), not the *intent* (operations). If an agent rewrites 50 lines to fix a typo, Git just sees 50 lines changed.
2. **Merge Conflicts are Fatal:** If two agents touch the same file, Git throws a `<<<< HEAD` conflict. Agents cannot parse these reliably, leading to catastrophic hallucination loops.
3. **The "Human Tax":** Staging areas, commit messages, and complex rebases force the agent to act like a database administrator.
4. **Open Shell Danger:** Forcing agents to use `bash` to run Git commands is slow, error-prone, and a massive security risk.

## 🌊 Why KungFu? (The Solution)

KungFu abandons the "Snapshot" model for the **Mathematical Flow Model**.

1. **Agent-Native Interface (MCP):** Agents do not use bash or Git commands. They connect to the local KungFu server via the Model Context Protocol (MCP) and issue surgical `Splice` and `Move` commands.
2. **Conflict-Free Concurrency:** Powered by Loro's `MovableTree` and `Fugue` algorithms, KungFu resolves concurrent edits mathematically. Merge conflicts are effectively obsolete.
3. **The Intent Timeline:** KungFu groups high-frequency agent operations into semantic *Intents*. You don't review raw diffs; you review the agent's reasoning.
4. **Branchless Architecture:** There are no branches. Everything is a "Mutation" on the main DNA sequence.

---

## 🧬 The Biological Metaphor

KungFu treats your codebase not as a machine to be built, but as an organism to be evolved.

- **Dojo:** The workspace (Local or Remote).
- **The DNA:** The absolute source of truth. The mathematical log of all Operations and Intents.
- **The Organism:** The physical codebase (The Trunk). The living result of executing the DNA.
- **Mutation:** An isolated deviation from the DNA for agent experimentation.
- **Transcription:** Projecting the CRDT DNA into physical files on your SSD so legacy tools (compilers/linters) still work.
- **Seed:** A hard, immutable cryptographic checkpoint used for deployment to CI/CD.

## 🔄 The Evolutionary Loop (How it works)

1. **Mutate:** An agent (or human) starts a new task. KungFu isolates a mutation vector. (`kf mutate`)
2. **Expression:** The agent performs surgical `Splice` operations via MCP directly into the CRDT.
3. **Intent:** The agent explains the reasoning behind the operations. (`kf intent`)
4. **Expose:** The mutation is exposed to the environment (the test suite). (`kf expose`)
5. **Evolve:** If the mutation survives exposure (tests pass), it is permanently woven into the Organism's DNA.
6. **Osmosis:** The new DNA is shared via background P2P sync with all peers. (`kf osmose`)

---

## 🚀 Getting Started

KungFu is a standalone Rust binary. It runs entirely locally.

### 1. Initialize the Dojo
```bash
kf init
```
*This generates your Ed25519 identity, creates the `.kungfu` database, and establishes the local Write-Ahead Log (WAL).*

### 2. Start the Agent Gateway
```bash
kf mcp
```
*This spins up a local Axum server on `127.0.0.1:8766`. Hook Claude Code, Gemini, or OpenPraxis to this port. They will automatically discover the `kungfu_splice`, `kungfu_read`, and `kungfu_create` tools.*

### 3. Materialize the Code
Once your agents have evolved the DNA, project it onto your physical disk to run your compiler or tests:
```bash
kf transcribe ./src
```

---

## 🤝 The Open Source Vision

We are building KungFu in the open because the Agentic Era needs a universal, decentralized foundation. Eventually, the KungFu repository itself will be hosted on KungFu. Until then, we use Git.

**Join the Dojo. Help us evolve the future.**

*License: Apache 2.0*
