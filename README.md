# 功夫 KungFu (kf) 🥋

> *"Kung Fu (功夫): A skill achieved through hard work, repetition, and practice."*

**Git is dead. Long live KungFu.**

Git was built in 2005 for humans emailing static patches to the Linux kernel mailing list. It is a phenomenal tool for human administration, but it is fundamentally broken for the Agentic Era. When you have a swarm of autonomous AI agents generating 50 lines of code every 3 seconds, asking them to stop, stage, write a commit message, and resolve `<<<< HEAD` merge conflicts is like putting a horse engine in a Tesla.

KungFu is a next-generation, agent-native version control system (VCS). It replaces the rigid, human-first snapshot paradigm of Git with a fluid, operation-based CRDT engine where AI agents and humans collaborate in a continuous, conflict-free stream.

**We are not managing files. We are evolving code.**

> **"Git forces machines to act like humans; KungFu allows humans to orchestrate machines."**

**KungFu never sleeps.**

---



---

## 🌌 The Invisible Force: Git is Matter, KungFu is Dark Matter

**Git is standard matter.** It is observable, static, and heavy. It deals with the visible artifacts of coding: files, snapshots, and commits. It is what we see after the work is done.

**KungFu is dark matter.** It is the invisible, high-frequency stream that actually holds the universe together. It captures the 95% of engineering that Git misses: the sub-second vibrations of thought, the surgical splices of intent, and the continuous flow of evolution that happens *between* the snapshots. 

**Automation over Administration.** 
Because KungFu handles the mathematical complexity of state resolution invisibly, the "Human Tax" is eliminated. You do not need to memorize a myriad of complex CLI commands, flag combinations, or disaster-recovery scenarios (like detached heads or broken rebases). You just write code. KungFu does the rest.

While Git records history, KungFu powers existence.


## 🆚 The Core Difference: Streaming vs. Batching

When developers ask, *"How is this different from Git?"* The answer lies in mathematics and physics. 

**Git is a Batch Processing system built for humans.**
You write code for hours. You stop. You navigate a myriad of complex CLI commands to bundle it all up into a `commit`. You push that massive batch to a server. Git was never designed for the high-frequency speed of AI agents; if two agents push a batch at the same time, the system breaks (Merge Conflict) and forces a human to manually resolve the collision.

**KungFu is a Continuous Streaming system built for agents (with humans in the loop).**
There are no batches. There are no complex commands to memorize. As an agent types, every single character deletion or insertion is streamed in real-time as a microscopic **CRDT** (Conflict-free Replicated Data Type) operation over a WebSocket. Because the edits are streamed instantly and applied via pure mathematics (Fugue/MovableTree algorithms), they weave together perfectly. **You cannot have a merge conflict if you never merge.**


---

## ⚡️ The KungFu Advantage

> *"Git forces machines to act like humans. KungFu allows humans to orchestrate machines."*

1. **Speed (Zero-Cost Edits):** Agents don't rewrite 2,000-line files to change a variable. They send a 50-byte `Splice` JSON payload over a local WebSocket. The math updates in memory in under a millisecond.
2. **Scale (Infinite Concurrency):** If 5 agents edit the same file in Git, the system crashes with `<<<< HEAD`. In KungFu, 1,000 agents can edit the exact same file simultaneously. The CRDT math guarantees a perfect, conflict-free merge.
3. **Safety (Cryptographic Identity):** Git authorship can be spoofed trivially. In KungFu, every single character typed is signed by an Ed25519 Private Key. You can cryptographically prove exactly which agent wrote which line.
4. **Semantics (The "Why"):** You don't review a 400-line diff of `+` and `-` symbols. You review the **Intent Log**. You see: *"Intent: Refactoring Auth"* ➔ Linked to 12 Splice Operations. You review reasoning, not just raw text.
5. **Unstoppable Productivity (No Blocking Gates):** Traditional CI/CD is a passive system of myriad gates designed to block work. KungFu is a high-productivity platform for those who never stop writing. There is no rebasing or waiting for approvals. You `Mutate` and `Expose`. The flow never stops.
6. **Chronology (UUIDv7 Time-Travel):** Git relies on chaotic pointer graphs and server timestamps. KungFu assigns a **UUIDv7** to every single file, mutation, and operation. This guarantees that every action is decentralized yet perfectly time-sortable. The UI acts as a DVR, allowing you to slide through the codebase's evolution exactly as it happened, without needing a central clock.
7. **AI-Powered Active Intelligence (The Self-Aware Codebase):** Git is dumb storage; it just holds text. Because KungFu processes a live stream of operations, the system itself can infer the code. It can automatically generate documentation, flag semantic collisions, and provide deep analytics on how the codebase is evolving over time.




---

## 🥊 KungFu is Anti-Git

We aren't just "better" than Git. We are the antithesis of Git. KungFu is a fundamental rejection of the 20-year-old premises that hold back modern engineering:

- **We reject Snapshots.** Snapshots are dead history. We embrace **Flow**.
- **We reject Branches.** Branches are isolationist. We embrace **Extreme Trunk-Based Evolution**, where all Mutations happen on a single, continuous DNA stream.
- **We reject Manual Merging.** Manual merging is a human tax. We embrace **Mathematical Convergence**.
- **We reject the Staging Area.** Staging is friction. We embrace **Real-time Streaming**.

Git was designed for a world of slow, human-to-human patch-mailing. KungFu is designed for a world of high-frequency, machine-to-machine evolution.


## 📜 Table of Contents
1. [The Paradigm Shift: KungFu vs. Git](#-the-paradigm-shift-kungfu-vs-git)
2. [Why We Built It in Rust](#-the-engine-why-rust)
3. [The CRDT Magic (Loro)](#-the-crdt-magic-loro)
4. [The Surgical VFS](#-the-surgical-vfs)
5. [The Evolutionary Loop](#-the-evolutionary-loop-how-it-works)
6. [Getting Started](#-getting-started)

---

## ⚔️ The Paradigm Shift: KungFu vs. Git

| Feature | Git (The Legacy) | 功夫 KungFu (Agent-Native) |
| :--- | :--- | :--- |
| **Data Structure** | Static Snapshots of Files | Continuous Graph of Operations (CRDT) |
| **Collaboration** | Manual `push` / `pull` / `fetch` | Continuous background streaming (Osmosis) |
| **Conflict Resolution** | Heuristic text-guessing (Manual fixes required) | Pure Algebra. Automatic weave via Fugue/MovableTree |
| **Topology** | Millions of divergent branches | A single, continuous Trunk (The DNA) |
| **The "Branch"** | Physical copies of the filesystem (`checkout -b`) | Branchless. A logical filter (Mutation) on the Trunk |
| **File System** | Direct manipulation of physical disk | **Surgical VFS:** Agents edit a mathematical tree in memory |
| **Agent Interface** | Error-prone Bash commands | Native MCP Server (JSON-RPC `Splice` commands) |
| **Testing / CI** | PR-based webhooks | **Natural Selection:** Mutations tested against the environment |
| **Security/Identity**| Unverified string (`user.name`) | Cryptographic Identity (Ed25519) per agent/human |
| **History View** | Opaque hashes and commit messages | A semantic timeline of **Intents** and Agent Reasoning |
| **Philosophy** | Mechanical Construction | Organic Evolution |

---

## 🦀 The Engine: Why Rust?

KungFu is built entirely in **Rust**. Why? Because agents don't type; they blast. 

When you have a swarm of agents generating code, the system needs to process thousands of character-level edits per second, lock-free.
* **Fearless Concurrency:** We use `tokio` to handle hundreds of agent WebSocket connections simultaneously.
* **Memory Safety:** We are manipulating the actual byte-arrays of the codebase in memory. Rust guarantees we don't corrupt the project source code.
* **Zero-Cost Bincode:** We don't send heavy JSON diffs over the network. We send highly compressed, native Rust `Bincode` byte streams. It takes less than 50ms for an agent in Tokyo to see a character typed by an agent in New York.

---

## 🧠 The CRDT Magic (Loro)

The heart of KungFu is the **[Loro](https://loro.dev)** library (a high-performance CRDT engine). In KungFu, files don't technically exist. The codebase is a living mathematical graph.

We use two specific Loro algorithms to make merge conflicts mathematically impossible:
* **Fugue (The Text Algorithm):** If Agent A and Agent B edit the exact same line of code at the exact same millisecond, Git throws a conflict and gives up. Loro uses Fugue to mathematically weave the two strings together into a valid state. 
* **MovableTree (The Filesystem):** In Git, if you rename a folder while someone else edits a file inside it, the repo explodes. Loro natively models the filesystem as a `MovableTree`. Every file has a UUIDv7. You can move `/src` to `/lib`, and the agent editing `auth.go` inside it doesn't even notice. 

---

## 🔪 The Surgical VFS

AI agents are trained to use tools like `read_file` and `edit_file`. If we force them to learn "CRDT Graph Math," they hallucinate. 

So, we built the **Surgical Virtual File System (VFS)**.
When an agent connects to the KungFu MCP Server and says *"I want to edit main.go"*, the Rust server intercepts that command. It traverses the Loro `MovableTree`, finds the UUIDv7 for `main.go`, and executes a mathematical `Splice` on the CRDT text object. 

The agent thinks it's hacking in a bash terminal. In reality, it is streaming pure math into a conflict-free DNA sequence. 

---

## 🧬 The Evolutionary Loop (How it works)

Because of this architecture, **KungFu has no branches.** Everything happens on a single, continuous timeline (The DNA). 

1. **Mutate:** An agent (or human) starts a new task. KungFu isolates a mutation vector. (`kf mutate`)
2. **Expression:** The agent performs surgical `Splice` operations via MCP directly into the CRDT.
3. **Intent:** The agent explains the reasoning behind the operations. (`kf intent`)
4. **Expose:** The mutation is exposed to the environment (the test suite). (`kf expose`)
5. **Evolve:** If the mutation survives exposure (tests pass), it is permanently woven into the Organism's DNA.
6. **Osmose:** The new DNA is shared via background P2P sync with all peers. (`kf osmose`)

---

## 🚀 Getting Started

KungFu is a standalone Rust binary. 

### 1. Initialize the Dojo
```bash
kf init
```
*Generates your Ed25519 identity, creates the `.kungfu` database, and establishes the local Write-Ahead Log (WAL).*

### 2. Start the Agent Gateway
```bash
kf mcp
```
*Spins up a local Axum server on `127.0.0.1:8766`. Hook Claude Code, Gemini, or OpenPraxis to this port to expose the `kungfu_splice`, `kungfu_read`, and `kungfu_create` tools.*

### 3. Materialize the Code
Project the CRDT DNA onto your physical disk to run your compiler or tests:
```bash
kf transcribe ./src
```

---
**Join the Dojo. Help us evolve the future.**
*License: Apache 2.0*
