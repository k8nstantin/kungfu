# Convergence: KungFu and Scion

*Recorded 2026-10-14.*

On 2026-10-13 Anthropic published
[Redesigning source control at Anthropic](https://www.anthropic.com/engineering/redesigning-source-control),
describing **Scion**, the system that replaced git for their monorepo. Much of its diagnosis, and
several of its design choices, match what this repository has said publicly since 2026-05-10.

This note records where the two converged, the evidence for when, and — just as deliberately —
what is not being claimed.

---

## The claim, stated precisely

**KungFu reached the same diagnosis independently and published it five months earlier.**

The two dates are not the same kind of evidence.

| | first public | verifiable by |
|---|---|---|
| **KungFu** | 2026-05-10 | GitHub's server timestamps, available to anyone |
| **Scion** | 2026-10-13 | the article's publication date |

Scion's article describes internal work before that: a chart beginning in January 2026 and an
ingestion prototype written "earlier in the year." That is Anthropic's own account of private work,
and it cannot be checked from outside. This note makes no claim about when Scion began internally.
It claims only what the public record shows: KungFu was public first, by five months, and could not
have drawn on Scion, which was not public.

That is the more useful fact anyway. A system built and run at Anthropic's scale arriving at the
same conclusions is evidence the conclusions are right.

---

## The evidence

### Server-timestamped

These timestamps are set by GitHub, not by the author, and cannot be backdated.

| what | when (UTC) | relevance |
|---|---|---|
| Repository `k8nstantin/kungfu` created, public | 2026-05-10 17:39:15 | the anchor |
| Issues #1–#5 | 2026-05-10 20:54–20:55 | central sync server, binary files, intent dashboard |
| PR #8 opened and merged | 2026-05-11 20:15 / 20:32 | agent coordination through the shared code |
| Issue #9 | 2026-05-12 12:31:47 | `kf ingest`: importing existing git projects |
| Issue #11 | 2026-05-12 16:28:41 | disposable server; cloud object storage as the source of truth |
| Issue #13 | 2026-05-17 11:31:26 | block-level convergence over parsed code |

GitHub records an edit timestamp on issue bodies. None of the above has ever been edited.

### Commit-dated

Commit dates are set by the author, and history was rewritten on 2026-06-26 (`eef8ae5`) to purge a
committed key, so the current hashes postdate that rewrite. These are supporting evidence; the
server timestamps above are the anchor.

| idea | first appears | date |
|---|---|---|
| Agent-first, operation-based, a write-ahead log instead of snapshots | `b3102e2` | 2026-05-10 |
| Trunk-only, no branches | `6238e14` | 2026-05-10 |
| One monorepo; *"agents do not clone"* | `27d2482` | 2026-05-10 |
| Access control on logical boundaries inside the monorepo | `27d2482` | 2026-05-10 |
| A continuous verification gate replacing branch-based safety | `27d2482` | 2026-05-10 |
| *"Agents write, humans review"* | `829e664` | 2026-05-10 |
| Hot buffer plus cloud object storage as the durable ledger | `004168a` | 2026-05-12 |
| Lock contention on branch refs as the failure point | `f063969` | 2026-05-14 |
| Tiered history: recent locally, deep history fetched centrally | `033fac9` | 2026-05-14 |

---

## Where they converged

| | KungFu | Scion |
|---|---|---|
| **The diagnosis** | Git was built for humans exchanging patches and offline trust; agents break its assumptions. | Git was built for distributed, offline clients; nearly all of theirs are always-online agents and jobs. |
| **The bottleneck** | Lock contention on branch refs, repository congestion. | No global merge lock; commits are lock-free; a ref update is a single compare-and-swap. |
| **Trunk** | Trunk-only. Named releases kept. | Refs discouraged except main and release branches. |
| **One repository** | Native monorepo; agents work without cloning. | One monorepo; source control exposed as an API so jobs need no local copy. |
| **Changes, not snapshots** | An attributed operation stream with a write-ahead log. | A changeset is a write-ahead-log entry holding changes relative to its parent, with no tree pointer. |
| **Derived state** | Blocks, paths, review state, deploy state and `last green` are derived, never declared. | Blame, path history and graph indices are derived data, invisible to clients. |
| **Topology** | Per-machine daemon, central hot path, cloud object storage underneath. | Per-node daemon, per-region cache, authoritative server on a database plus cloud storage. |
| **Access inside one repo** | Narrow grants for sensitive code instead of exiling it to a separate repository. | Per-directory access control to protect sensitive code within the monorepo. |
| **Review** | The reviewer judges whether the change is the right thing to do. | Review judges the intent of a change rather than every line. |
| **Safety** | Branches were never the safety mechanism; the pipeline is. | No global merge-queue lock; post-merge and pre-deploy automated testing. |
| **Migration** | `kf ingest` (issue #9); replaying real history is the first piece of work. | A git-to-Scion ingestion pipeline. |
| **Restraint** | The clean cut (`db74a9c`) and the third law: a feature that only makes sense in a story about the future of software is not core. | Much of the engineers' work was telling the model no when it over-engineered for hypothetical concerns. |

---

## Where they did not

These are as informative as the agreements.

- **Online versus local-first.** Scion assumes clients are always online: a thin client, a bounded
  cache, authority on the server. KungFu's local daemon owns the state and converges offline.
- **The unit of change.** Scion keeps commits and merges, made cheap and moved server-side. KungFu
  removes them in favour of operations and derived blocks.
- **Conflicts.** Scion lets the model repair them. KungFu proposed exactly that early on
  (`c349feb`, 2026-05-14) and then rejected it: model-based resolution is a suggestion, not a gate.
  Convergence may produce garbage inside a draft; garbage cannot promote.
- **Virtual filesystems.** Scion materialises files on demand. KungFu rejected virtual filesystems
  (`3c271c4`, 2026-05-14), on the grounds that the only one that works runs on a managed fleet —
  which is what Scion's clients, mostly CI and agent jobs, are.
- **When the gate runs.** KungFu's machine gate runs before work is accepted. Scion's post
  says they could not make a global merge-queue lock work, and tests after the merge instead.
- **When work becomes visible.** In Scion, from the moment of commit. In KungFu, from the first
  keystroke, as a draft.
- **Scope.** Scion says plainly that it is not a general git replacement and hopes its ideas flow
  back into git. KungFu is meant for everyone, including teams still on SourceSafe.

---

## What neither invented

Most of the shared mechanisms predate both projects, and claiming them would be wrong.

- **Google's Piper and CitC** — one monorepo, trunk-based development, per-directory permissions,
  virtual working copies. KungFu's README already cites it.
- **Meta's Mononoke and Sapling** — server-side changesets stored relative to their parents, and
  derived data such as blame computed on the server.

What is newer is the *reason*: that agents, not humans, are now the primary client of source
control, and that this changes which of git's assumptions still hold. That is the part both
projects arrived at in 2026.

---

## What it means for KungFu

1. **The diagnosis is corroborated.** A team with Anthropic's resources and a ten-fold rise in
   merge volume reached the same conclusions about git, trunks and monorepos.
2. **"The pipeline is the real gate" is corroborated.** They did not use a global
   merge-queue lock, and rely on post-merge and pre-deploy testing instead.
3. **"You cannot break the build" is challenged.** Two drafts can each pass the machine gate
   against trunk and fail together. Re-validating every draft against a moving trunk is a merge
   queue — the thing Scion's post says they could not make work. The answer to dev, qa and prod
   on one trunk has to account for this.
4. **Release pointers survived.** Even the most trunk-committed design kept release branches.
5. **The open question is still open.** The post describes no CRDT, so it says nothing about
   convergence at repository scale. The replay measurement remains KungFu's to do.
