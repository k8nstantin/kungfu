# Contributing to KungFu 🥋

**Welcome to the Dojo.** 

KungFu is a paradigm shift in version control, and we need architects, mathematicians, and engineers to help us evolve it. 

## 🧬 Our Philosophy
We are not building a Git wrapper. We are building a standalone, agent-native VCS.
- **We use the Biological Lexicon:** DNA, Mutations, Transcription, Natural Selection, Osmosis.
- **Trunk-Only:** We do not use branches. We use Mutations on a single DNA stream.
- **Mathematical Purity:** We rely on Loro CRDTs for conflict resolution.

## 🛠 Getting Started
1. **Toolchain:** Install Rust (latest stable).
2. **Clone:** `git clone git@github.com:k8nstantin/kungfu.git` (We use Git until KungFu can self-host).
3. **Build:** `cargo build`
4. **Test the Gateway:** `cargo run -- mcp`

## 🗺 Where to Help
Check the [GAP_ANALYSIS.md](./GAP_ANALYSIS.md) for the current technical debt and unimplemented features.

**Immediate needs:**
- Implementing the binary P2P Osmosis protocol (Phase 3).
- Hardening the VFS for binary file support.
- Building the first web-based "Intent Log" dashboard.

## 🤝 Rules of the Dojo
1. **Sign Your Work:** Every mutation must be cryptographically signed (Ed25519).
2. **Respect the DNA:** Mutations must survive Natural Selection (passing all tests) before integration.
3. **Intent over Diffs:** Explain the *Why* of your work in your express/intent calls.

**KungFu is evolving your code. Join us.**
