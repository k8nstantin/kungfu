# KungFu (kf)

*The Standalone Version Control System for the Agentic Era.*

KungFu is a next-generation, agent-native version control  system (VCS) powered by the Loro CRDT engine. It is designed to replace the rigid, human-first paradigm of Git with a fluid, operation-based system where AI agents and humans collaborate in a continuous, conflict-free stream.

## Why KungFu?

Traditional VCS (Git) treats code as a series of static snapshots. This forces agents to stop, stage, and commit, leading to brittle workflows and complex merge conflicts. 

**KungFu treats code as a continuous flow of intent.**

- **Agent-First Mechanism:** Agents stream surgical edits (`splice`) directly into the CRDT graph via MCP. No more `git add` errors or broken commit messages.
- **Mathematical Harmony:** Powered by Loro's `MovableTree` and `Fugue` algorithms, KungFu resolves concurrent edits mathematically. Merge conflicts are effectively obsolete.
- **The Intent Timeline:** Instead of a list of opaque hashes, KungFu maintains a semantic log of *Intents*. You see the evolution of the codebase through the reasoning of the agents that built it.
- **Flexible Topology:** Works as a pure P2P system for local speed, or connects to a **Central Dojio** (Remote Server) for team-wide orchestration and human-in-the-loop review.

## The Metaphor

- **Dojo:** The workspace (Local or Remote).
- **Stance:** A specific task or feature vector (The "Branch").
- **Flow:** The continuous background sync of CRDT operations.
- **Strike:** A hard, immutable checkpoint exported to legacy Git for external compatibility.

## Architecture

___
cud/kf/             CLI entry point & MCP Server
internal/
  core/             Loro CRDT wrapper & History (Intent) engine
  sync/             Streaming transport (WebSockets/P2P)
  mcp/              MCP Server (The agent gateway)
pkg/
  vfs/              Virtual File System (The abstraction)
___

## Tech Stack

- **Language:** Go (Backend/CLI)
- **CRDT Engine:** [Loro](https://loro.dev) (Rust via CGO/FFI)
- **Interface:** MCP (Model Context Protocol)

---
*KungFu is a standalone research project designed to power autonomous engineering at scale.*
