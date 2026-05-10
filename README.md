# KungFu (kf)

**The Standalone Version Control System for the Agentic Era.**

"KungFu is evolving your code."

KungFu is a next-generation, agent-native version control system (VCS) powered by the **Loro CRDT engine**. It replaces the rigid, human-first paradigm of Git with a fluid, operation-based system where AI agents and humans collaborate in a continuous, conflict-free stream.

## The Evolutionary Loop

1. **Mutate:** An agent (or human) starts a new task by isolating a mutation vector. (`kf mutate`)
2. **Express:** The agent writes the code via surgical Splice operations into the CRDT. 
3. **Intent:** The agent (or human) explains the reasoning behind the operations. (`kf intent`)
4. **Expose:** The mutation is exposed to the environment (the test suite). (`kf expose`)
5. **Evolve:** If the mutation survives exposure (tests pass), it is woven into the Organism DNA.
6. **Osmose:** The new DNA is shared via background P2P sync with all peers. (`kf osmose`)

## The Metaphor

- **Dojo:** The workspace (Local or Remote).
- **The DNA:** The Intent Log & Blueprint. The absolute source of truth.
- **The Organism:** The Trunk / Codebase. The living result of executing the DNA.
- **Mutation:** An isolated deviation from the DNA for experimentation.
- **Transcription:** Checkout / Materialization. Projecting DNA into physical files.
- **Seed:** A hard, immutable cryptographic checkpoint used for deployment.

---
*KungFu is a standalone research project designed to power autonomous engineering at scale.*
