# KungFu (kf)

**The Standalone Version Control System for the Agentic Era.**

"KungFu is evolving your code."

KungFu is a next-generation, agent-native version control system (VCS) powered by the **Loro CRDT engine**. It replaces the rigid, human-first paradigm of Git with a fluid, operation-based system where AI agents and humans collaborate in a continuous, conflict-free stream.

## The Evolutionary Loop

1. **Mutate:** An agent (or human) starts a new task. KungFu creates a **Mutation** (an isolated CRDT filter).
2. **Expression:** The agent performs surgical **Splice** operations. These are recorded as **DNA** segments.
3. **Transcription:** KungFu continuously **Transcribes** the mutation to the local disk so the agent can run tests.
4. **Natural Selection:** The agent (or human) calls `kf integrate`. The system subjects the Mutation to the environment (the test suite).
5. **Evolution:** If the Mutation survives (tests pass), it is woven into the **Organism**. If it fails, the mutation is discarded or refined.
6. **Osmosis:** The new DNA is instantly shared via **Osmosis** with all other peers in the Dojo.

## The Metaphor

- **Dojo:** The workspace (Local or Remote).
- **The DNA:** The Intent Log & Blueprint. The absolute source of truth.
- **The Organism:** The Trunk / Codebase. The living result of executing the DNA.
- **Mutation:** A Stance / Edit. An isolated deviation from the DNA.
- **Transcription:** Checkout / Materialization. Projecting DNA into physical files.
- **Fossil:** A hard, immutable cryptographic checkpoint used for deployment.

---
*KungFu is a standalone research project designed to power autonomous engineering at scale.*
