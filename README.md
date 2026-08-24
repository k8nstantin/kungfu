# 功夫 KungFu

**Fully automated version control.**

You write code. Versioning, sharing, visibility and reconciliation happen underneath you. There
are no commits to compose, no branches to manage, no merges to resolve, and nothing to memorise.

KungFu is not a re-engineered Git and shares nothing with it structurally. Git is snapshots,
Merkle trees and a commit graph, reconciled by textual heuristics that guess — and then ask a
human when they fail. **KungFu is a convergent mathematical structure:** a stream of attributed
operations over a CRDT, where concurrent edits combine by algebra rather than guesswork, and the
files on disk are a projection of that structure rather than the thing itself.

A different foundation, not a different interface over the same one.

---

## Why this exists

**A developer's job is to build a product. It is not to operate a version control system.**

Today, before someone can safely contribute to a codebase, they have to learn a data model that
has nothing to do with the product: the index, the staging area, refs, `HEAD`, detached `HEAD`,
tracking branches, rebase versus merge, the reflog, force-push and its consequences. That is a
genuine curriculum. It takes weeks to become comfortable and years to become confident.

**No development shop allocates time for that.** There is no sprint item called "learn source
control." Deadlines do not pause for it. And yet there is a standing presumption that you *should
just know it* — a substantial technical skill everyone is expected to have absorbed on their own
time, for work that produces nothing a user ever sees.

It hits hardest exactly where it should hurt least. Teams still on older tooling — SourceSafe,
TFS, Perforce, a network share — weigh the cost of migrating *plus* the cost of the learning
curve against the work already piled up, and correctly conclude it is a bad trade. So they stay
put. Not because the old tool is good, but because the new one demands a tax nobody budgeted.

### And the model was never built for machines either

Point an automated coding agent at a repository. It creates a branch. It works. It declares
itself **done** — and it isn't. The tests it was graded on pass, or it simply reports success, so
by its own measure the job is complete. The branch is left lying there. Nobody knows whether it
holds something valuable or something abandoned, and finding out costs a human an hour of reading.

Now run several agents at once. That is the normal configuration, not the extreme one, and it
produces a graveyard of branches that each claim to be finished, drift further from trunk every
hour, and will collide with one another the moment somebody tries to reconcile them.

The root cause: **the agent grades its own homework.** Git accepts a self-report — whatever was
pushed is what was meant, and "done" is whatever the author says it is. That was already generous
when the author was a person with a reputation to protect. Applied to something optimising against
a proxy for the goal rather than the goal itself, it fails completely.

So the tool now fails at both ends: **too much ceremony for the humans, too much trust for the
machines.**

There is an economic shift underneath this too. When producing code was the expensive part,
optimising for *distributing patches* — the problem Git was actually built to solve — made sense.
Producing code is no longer the expensive part. **Verification and integration are**, and those
are precisely what the current model leaves to human labour and good intentions.

### This is already solved everywhere else

Two people, one in Berlin and one in Los Angeles, open the same document. They both type. They see
each other's cursors move. Every change is versioned automatically. There are no conflicts and no
merge step. **Nobody has ever been sent on a training course to learn how to collaborate in a
shared document.** The mechanism is invisible to the people using it.

Source code never got this. KungFu is that, for code.

**This is not a better version control system. It is the removal of a discipline.** You do not get
an improved tool to operate — you get one fewer skill to have.

---

## What you get

### For the developer

| | |
|---|---|
| **Nothing to learn** | Start typing. There is no curriculum, no manual, and no concept you cannot explain in one sentence. |
| **You cannot lose work** | Everything is captured continuously. There is no unsaved state, nothing to stash, and no way to lose an afternoon to a bad command. |
| **You cannot break the build** | Not by discipline — structurally. Broken code cannot reach anyone else, because promotion requires a machine gate that never gets tired or rushed. |
| **No merge conflicts to resolve** | Concurrent edits combine by algebra. Where two people genuinely meant contradictory things, that surfaces as a rare, legible event with both intentions attached. |
| **No stale branch, ever** | Your work is continuously fed by everything accepted upstream. A week-old draft is as fresh as a minute-old one. |
| **Go back to any moment** | Including `last green` — the exact state when the test suite last passed. "The version from before lunch that worked" is a query, not an archaeology project. |
| **Reorganise fearlessly** | Moving code rewrites every reference automatically. Restructuring stops being a multi-day exercise nobody dares attempt. |
| **No status reporting** | No standup recitation, no ticket hygiene, no "what's the status of X." The work reports itself. |
| **Onboarding is one gesture** | Someone shares access. You start typing. No clone, no keys, no team memberships, no CI setup. |

### For the team

- **You always know what everyone is doing.** No more "somebody took a branch and vanished for a
  week." Work is visible from the first keystroke to anyone who goes looking.
- **The 2,000-line surprise becomes impossible.** Not by policy — by construction. There was never
  a moment when the work was invisible, so review happens continuously rather than as a cold read
  at the end.
- **No branch graveyard.** There is nothing to abandon and nothing to clean up.
- **Review organises itself.** A shared queue, oldest first, owned by groups rather than
  individuals. No triage meeting, no assignment, no chasing, no single point of failure when
  someone is on holiday.
- **Bottlenecks become structural facts, not personal failings.** If work queues behind one
  person, that is instantly visible as an ownership problem with obvious remedies.
- **Many agents stop being chaos.** Ten actors produce ten continuously-fed drafts, not ten
  diverging branches, and none of them can declare its own work finished.

### For the company

- **Real transparency, without surveillance.** The true state of every piece of work is visible by
  inspection — pull-based, at draft granularity, peer-directed. Nothing is broadcast, scored or
  ranked.
- **Delivery flow becomes measurable without anyone reporting on it.** Review latency, where work
  stalls, where the same region gets rewritten repeatedly, where people keep colliding — which
  identifies the architecture that needs decoupling. System-level and aggregate.
- **Project management overhead collapses.** The board maintains itself, because every state is
  derived from real work rather than mirrored into tickets by hand.
- **You know exactly what is in production.** Per change, directly. Not reconstructed from tags
  and release branches.
- **Incident tracing is immediate.** The exact set of changes that reached production before an
  incident, with authors and reasoning attached.
- **Provable authorship.** Every operation is cryptographically signed — an audit trail that holds
  up when most of the code is machine-written.
- **Permissions finally fit reality.** Grant access to a single block or module rather than an
  entire repository. Contractors, sensitive code, regulated subsets and least-privilege for
  automated actors, all without splitting the codebase apart.
- **Repository sprawl stops.** Repos multiply largely *because* permissions are repo-shaped. Make
  permissions block-shaped and the reason to multiply them disappears.
- **The training cost goes to zero**, along with the hours currently spent on merge conflicts,
  branch administration and repository reconciliation.

---

## How it works

### Everything is a draft until you say otherwise

A half-written document is not a document — it is a **draft**. Every document tool has modelled
this forever and nobody ever needed a manual to understand it. You start writing: it's a draft.
You finish: it's ready.

Git has no such state. It has *commits*, which claim finality, and *branches*, which are
invisible. The tell is that "draft pull request" had to be invented at the hosting layer, because
the tool underneath could not express it.

```
edit ──▶ DRAFT ──▶ READY ──▶ [safe] ──▶ IN REVIEW ──▶ ACCEPTED ──▶ DEPLOYED
       (automatic)  (author)  (machine)   (locked)      (in the      dev │ qa │ prod
                                          (a peer)      codebase)
```

Five states, each explainable in one sentence to somebody who has never used the tool.

- **Draft** — the default state of every edit. You never create a draft; editing *is* drafting.
  Live, versioned, and visible to anyone who goes looking, but not yet part of what others build
  on.
- **Ready** — one gesture from the author: this thought is complete, put it in the queue.
- **In review** — frozen while a peer looks at it, because you cannot review something that is
  changing underneath you.
- **Accepted** — reviewed, and now part of the codebase everyone builds on.
- **Deployed** — running, and it records where: dev, qa, prod.

### Three gates, each owned by whoever holds the information

| gate | owner | question | why them |
|---|---|---|---|
| **Ready** | the author | is the *thought* complete? | code can compile and pass every test and still be half a thought — *"I added the function, I haven't wired it up yet."* No machine can infer that. |
| **Safe** | the machine | does it parse, build, pass? | mechanical, tireless, instant. A person should never carry this. |
| **Right** | a peer | is this the correct thing to do? | judgment, approach, design. Only a human can answer it. |

All three must pass. **Nobody is ever asked a question they cannot answer.**

Git collapses all three into a single act — the pull request — and hands the whole bundle to
humans, *including the machine's share*: "did you run the tests?", "please rebase on main", "fix
the conflicts."

### Nothing can declare itself finished

**`ready` is a request, not a verdict.** It means *"I believe this is complete, put it in the
queue."* It puts nothing into the codebase. The machine still has to agree that it builds and
passes; a person still has to agree that it is the right thing to do. **A self-report of "done" is
structurally irrelevant** — which is exactly the assumption that collapses when the thing
reporting is optimising against a proxy for the goal.

Everything else follows without extra machinery:

- **There are no branches to abandon.** Work either promotes or it doesn't; there is no side-track
  for it to rot on.
- **Abandoned work surfaces instead of accumulating.** A draft untouched for three days appears in
  the queue with its age attached, because age is intrinsic to every identifier.
- **Many actors at once is the normal case**, not the pathological one.
- **Each actor is scoped and provable** — its own cryptographic identity, a grant covering only
  what it needs, and every operation attributable.

The same three gates that stop a rushed human from breaking the build stop a confident machine
from claiming it didn't.

### The block replaces commit, branch and pull request

A **block** is the set of code regions one draft touched, across however many files. It is
*derived, never declared* — you do not create a block, it is the footprint of your work. The
system records exactly which characters changed and who changed them; matching that against the
parse tree tells it which functions and declarations they belong to.

- **Precise, not file-shaped.** Function-level granularity, so two people working in different
  functions of the same file never collide.
- **Spans files naturally.** A feature touching five files is one block, because it was one draft.
- **Reviewer highlighting is a query, not a feature.** Git has to *reconstruct* a diff by comparing
  snapshots and inferring what moved. Here the change set is recorded and exact — and a reviewer
  can replay the intermediate steps to see *how* the author got there, which no diff can show.

---

## What disappears

**Gone — the machine's job now**

| | |
|---|---|
| staging / choosing what to include | nothing to choose; everything is captured |
| commit / bundling and naming a batch | automatic |
| branch create, switch, delete, track | unnecessary |
| merge, rebase | automatic convergence |
| push, pull, fetch, remotes | continuous background sync |
| stash | meaningless — nothing is ever unsaved |
| reset, revert, cherry-pick, reflog archaeology | replaced by *"put it back how it was"* |
| ignore-file curation | inferred |
| pull-request mechanics | review is a view, not an artifact you assemble |

**Kept — because they are genuinely human decisions**

- **"This is good, ship it."** Approval.
- **"This state matters."** A release worth a name.
- **Adjudicating a real collision**, when two people actually meant contradictory things.

Thirteen learned concepts removed, three kept. **That is the product.**

---

## Why there are no branches

Branches do two protective jobs. Separate them and each has a better owner.

| what is being protected | branch's version | here |
|---|---|---|
| **production**, from broken code | branch + PR + merge discipline | **the deployment pipeline** — which already does this, better, with real gates |
| **other developers**, from your broken work-in-progress | branch isolation — which also *hides* you | **draft isolation** — the same protection without the invisibility |

Branches were never what kept production safe. **dev → qa → prod** is the real gate; it has tests,
canaries and rollback, and it does not care how the code got there. Every company already runs one
and already trusts it.

So `ready` is not a new concept to learn — it is the on-ramp to machinery the team already uses.
Flip the bit and your work enters the pipeline it was always going to enter, without a branch, a
PR, a rebase or a merge to get there.

**And drafts cannot rot.** A branch decays because the world moves and it does not. A draft is
*fed*: everything accepted upstream flows into it automatically while you work. It is continuously
rebased by construction. Stale branches, reconciliation debt and the merge-day catastrophe stop
being problems to manage and stop being *categories*.

**Mandatory review is only painful when the unit is large and the reviewer is cold.** This makes
the unit small and the reviewer warm. Proven at scale: Google requires review on every change
across roughly 25,000 developers on a single trunk, and it works precisely *because* changes are
small and integration is continuous.

---

## Why there are no directories

When you work in a shared document you do not think about directories. You think about the
document.

Directories serve two jobs today, and they need separating:

- **Human navigation** — a crude search index. Not needed. Search, recency and presence are
  strictly better at it.
- **Machine resolution** — compilers genuinely need a layout, and some languages encode paths in
  the source text itself (`use crate::auth::verify`, `import './utils'`).

Only the second is real, and projection handles it: **the file layout is generated when a compiler
looks at it.** Paths are derived metadata, not identity. Every existing tool — compiler, linter,
language server, test runner, formatter — sees ordinary files on disk and works unmodified.

**Which makes reorganising the codebase free.** Because paths are derived and the system holds both
a parse tree and the full edit history, moving code is a *semantic* operation that rewrites every
reference automatically. Today restructuring a large codebase is a frightening multi-day exercise
that breaks every import and gets repaired by hand — so teams do not do it, and the structure
calcifies around decisions made years ago. Here it is a machine operation.

---

## Why there is one codebase

No branches, no repos. One codebase, with blocks flowing through it at their own pace.

**Permissions are how this survives at company scale.** Access works the way document sharing
works — you share with named people:

| document role | here |
|---|---|
| viewer | can read |
| commenter | **can review and approve, but not edit** — precisely the reviewer role |
| editor | can draft |

Stronger than document sharing, because every actor has a cryptographic identity (Ed25519):
authorship is provable rather than a self-declared string. Individuals are the primitive; groups
are a convenience layer over them.

### Permission granularity is the unlock

Git has exactly one access boundary: the entire repository. You cannot say *"this contractor may
edit the payment UI but not the payment logic."* The choice is the whole repo or nothing, and there
is no third option.

Here a grant can target a single block, document or draft. Going from *whole repo or nothing* to
*per block* is a categorical improvement and it needs no qualification.

- **Contractors and external collaborators** — grant exactly the module they work on.
- **Sensitive code inside a shared codebase** — auth, crypto, payments, PII handling can live
  alongside everything else with narrow access, instead of being exiled to a separate repo with a
  separate pipeline and duplicated tooling.
- **Open-sourcing part of a codebase becomes a permission change**, not a repo extraction plus a
  permanent sync job.
- **Least privilege for automated actors.** An agent with repository access has *everything* —
  unrelated systems, secrets, all of it. Here each one gets a grant scoped to the blocks it needs.

Default is inherited, exceptions are narrow — nobody hand-configures a million code units. That is
the pattern that leaves Google with under 1% of their codebase restricted.

Two honest notes. **The build needs broader read access than any human** — if your code depends on
a module you cannot see, the compiler still needs it — so machine identity and human identity are
separate. And **revocation stops future access, not existing copies**, identical to the situation
with any clone-based system today.

---

## Visibility: the end of not knowing

In Git, someone takes a branch and disappears for a week. The work is invisible right up until it
lands, and then it lands badly. An entire industry of process — standups, PR etiquette, "please
keep PRs small" — exists to compensate for a tool that structurally cannot show you what is
happening while it happens.

Here, *"what is Molly working on?"* is answered by going and looking.

**Visibility is pull, not push.** That distinction is the whole design:

> *"What's Molly working on? Let me go look."* — a colleague walking over to a desk.
> *"Here is a live feed of everything Molly typed."* — a panopticon.

Nothing is broadcast, nothing is scored, nothing is ranked. The state is simply **not hidden** from
a teammate with a reason to look — exactly how a shared document behaves. A document does not
notify your manager that you rewrote a paragraph; it is just openable.

**And the draft is the unit of visibility, not the keystroke.** Others see that a draft exists,
where it lives and what it is about. That solves "vanished for a week" — the actual problem —
without exposing the interior thrash of somebody's thinking.

Design rules, enforced architecturally rather than by policy:

- Visibility is **pull-based, draft-granular and peer-directed** by default. Anything push-based,
  keystroke-granular or hierarchy-directed is opt-in at most.
- **Local history is private by default, with an explicit publish gate.** "Everything is captured"
  must never mean "everything is visible."
- **Secret scanning at capture time, plus a real expunge primitive.** Nobody wants the API key they
  briefly hardcoded living in permanent history. Designing the redaction path in from the start is
  cheap; retrofitting it into an append-only log is a rewrite.

---

## History: autosave, not recording

This is a safety net for undo, not a surveillance record. Framing matters more than mechanism
here — the same automatic capture reads as *"autosave in your editor"* or as *"a keylogger"*
depending entirely on how it is presented and what it is for.

Four tiers, of which only the top two are ever visible:

1. **Operations** — every captured edit. The storage substrate; never surfaced.
2. **Transactions** — coalesced on a ~300 ms idle window, with breaks on save, focus change and
   undo boundary. The granularity of undo.
3. **Sessions** — grouped by **activity gap, not fixed interval.** A session boundary is *"the
   human stopped,"* which is how people index their own memory. "Before lunch" is literally an
   activity gap. This is the default history view.
4. **Milestones** — the sparse spine you actually navigate.

### `last green`

*"Get me the version from before lunch that worked"* is answerable exactly, and not by guessing
from the diff. **Tag the tree state at the moment the test suite last exited zero.** That is a
fact, observed for free by watching build and test exit codes. Same for dev-server restarts and
explicit releases.

Navigation follows the design that demonstrably needs no training:

- **The entry point is a timestamp in the chrome**, not a menu item. No new noun to learn.
- **One toggle collapses history to the milestone spine** — the single control that makes an
  unbounded history navigable.
- **Named versions are capped**, which forces names to mean milestones rather than labels on
  everything.
- **Diff-on-select is the default view**, colour-coded per author.
- **Restore is non-destructive** and creates a new version, so exploring history is fearless.
- **Checkpoint before *and* after** any automatic reconciliation, so one action undoes it.
- **Rewind operates on the whole codebase**, not one file. "Take me back to before lunch" is
  inherently a tree-level question.

---

## The review queue

One view of every block and where it sits: awaiting review, in review and for how long, accepted,
deployed to dev/qa/prod.

**It is a work queue, not a scoreboard**, and that distinction decides whether the tool survives
contact with the people who have to install it. It shows *the state of work*, never *the activity
of people*. "Here is what is waiting, oldest first, grab one" is shared infrastructure nobody
objects to. "Here is how long Molly has been sitting on things" is the framing that gets a tool
banned.

**The queue prioritises itself.** The oldest lock is, by construction, the most urgent thing in the
organisation — it is blocking a person right now. Sort by age and correct priority falls out for
free. No triage meeting, no priority labels, nobody assigning anything. Age is intrinsic rather
than bookkept: every operation, block and state transition carries a time-sortable UUIDv7, so "how
long has this been in review" is arithmetic on the identifier itself.

### The lock, and its valuable side effect

Review requires a stable target, so a block freezes while it is being reviewed. This is the cheap
replacement for a commit: a commit freezes the entire tree at a point in time, while this freezes
only the reviewed regions for only the duration of review. The author keeps working, on a different
draft.

**And it converts review latency into a cost that is felt immediately.** Today a slow review costs
you a branch that quietly rots, paid weeks later as a merge conflict. Here a slow review holds a
lock somebody can see. That puts the pain exactly where the fix is. A pull request can be ignored
indefinitely because ignoring it costs nothing visible; a pending review here has a blocked person
attached to it. **Review latency becomes the system's key metric**, and the most useful number the
tool produces.

### Review is owned by groups, never individuals

Which removes the single point of failure — vacation, timezone, sole expert — and turns review from
*an assignment pushed at someone* into *a pull from a queue*. Combined with the self-prioritising
order, that is a self-organising review system **with no assignment step at all.** When a group is
genuinely over capacity, it surfaces as a group-capacity problem with obvious remedies rather than
as one person's backlog.

**Escalation is configurable, and escalating to a human is the correct terminal action.** The
tempting alternative — auto-accepting an unreviewed block to clear a stale lock — would make the
system lie about its own guarantee. If reviews are not happening, that is information leadership
needs, not an inconvenience to be approved away.

### A project board that needs no project management

Because every state is *derived from real work* rather than reported by a person, this is a Kanban
board that maintains itself. **Nobody moves a card. The cards move because the work moved.**

Ticket systems exist largely because the true state of work is invisible, so humans manually mirror
it into tickets — and that mirror is always stale and frequently wrong. Here the board *is* the
work.

This is also the honest version of measurement: **flow becomes visible without anyone reporting on
it.** Queue health, review latency, where work stalls, where the same region gets rewritten
repeatedly, where people collide. System-level and aggregate — not individual ranking. Activity is
measurable; "productivity" is not, and every proxy metric ever adopted got gamed and then misled.

---

## Deployment state is an attribute of the block

*"Which commit is actually in prod?"* is a perennial archaeological question answered with tags,
release branches and hope. Here each block records where it has reached:

- *"Is my change live?"* — answerable per block, directly.
- *"What is in production right now?"* — a filter, not an investigation.
- **Incident tracing** — the exact set of blocks that reached prod before an incident, with authors
  and reasoning attached.
- **Rollback scoped to a block**, because the block is the unit that was reviewed and accepted.

Deployments are batched even though blocks are granular: a deploy picks up everything accepted at
that moment, and a block depending on an earlier one cannot ship without it. The block still
records where it got to.

---

## Two parts: the core, and the add-ons

This matters for how the project is built and how it is adopted.

### The core

**Self-contained. Depends on nothing external. No git, no node, no service.** Starting a new
project, you use KungFu immediately and load none of the add-ons.

- capture edits (editor plugin + filesystem watcher)
- converge (CRDT)
- draft → ready → review → accepted
- automatic history and navigation
- projection to disk, so every existing tool works unmodified
- presence, pull-based
- the review queue

### The add-ons

Each one touches exactly one external thing, and each one can be absent.

| add-on | what it does |
|---|---|
| **migration** | index existing repositories, consolidate them into one codebase, keep them in sync during transition |
| **git bridge** | mirror to and from git while a team crosses over |
| **deployment** | pipeline hooks and deployed-state tracking |
| **agent interface** | typed tools for automated actors |
| **escalation workflow** | configurable policy engine for review ageing |
| **analytics** | anything beyond the built-in queue view |
| **encryption** | per-block cryptographic confidentiality, if ever needed |

**This is the structural defence against scope creep.** An earlier version of this project accrued
a dozen external dependencies before it could save a file, because every integration was treated as
architecture. The rule now: *the core depends on nothing outside itself; every integration is a
module that can be absent.*

---

## Architecture

### Capture

Two layers, both required.

**Editor plugins** stream edits over local IPC to one long-running daemon that owns the state.
Build order is chosen by leverage: a **VS Code extension** first, because one artifact covers VS
Code, Cursor, Windsurf, VSCodium and every other fork; then JetBrains; then Neovim. A language
server mode covers the long tail (Helix, Emacs, Sublime, Xcode) but is never the primary path —
there the client chooses the sync mode, incremental sync has known correctness bugs in the wild,
there is no cursor notification, and you still need a plugin to launch the server anyway.

**A filesystem watcher** underneath, always. It is the only layer that sees automated tools,
terminal commands, formatters, code generators and editors with no plugin installed. It must handle
save amplification, event coalescing, queue overflow and atomic-rename inode churn.

**No virtual filesystem.** Microsoft abandoned theirs and now recommends sparse checkout with real
files; Meta maintains three divergent backends with documented reliability problems; Google's is
Linux-only on a managed fleet; and third-party macOS filesystem extensions are currently broken on
the latest OS with developers blocked waiting on Apple. Real files plus a watcher is where everyone
converges, including the teams most capable of doing otherwise.

The reconciliation rule: **the daemon's state is the source of truth; the filesystem is a
projection.** On a watcher event, diff disk against the materialised state and discard anything
already explained by buffered plugin edits.

Honest fidelity: with a plugin, per-edit. Supported editor without a plugin, per-save. Automated
tool or script, per-write. For reference, Google ran a decade of save-granularity history for
25,000 developers and nobody complained.

### Foundations

| what | choice | why |
|---|---|---|
| convergence engine | **Loro** (Rust, MIT) | the only CRDT with a real **movable tree** — moving or renaming a directory concurrently with edits inside it is *the* defining operation of source control. Automerge has no move operation at all; the alternatives are text-only. Also ships history truncation as a real feature. |
| transport, NAT traversal, relays, device identity | **iroh** (Apache-2.0) | removes the largest single chunk of infrastructure work |
| wire protocol, room multiplexing | **`loro-dev/protocol`** (MIT) | official Rust client and server |
| presence and live cursors | **Loro `EphemeralStore`** | per-key last-write-wins with TTL and **partial updates** — whole-state broadcast does not scale to hundreds of file cursors |
| structural understanding | **tree-sitter** | incremental reparse with error recovery, so half-typed code still parses; changed-range queries answer "which function did this edit touch" |
| git interoperability (add-on) | **gitoxide** | |
| local storage | **redb**, moving to **fjall** when blob volume demands key-value separation | |
| identity | **Ed25519** per actor; **UUIDv7** for every operation, block and transition | provable authorship; time-sortable identifiers make age free |

The mathematics underneath, since it is the substrate the whole system stands on:

- **Fugue** — the text-ordering algorithm, with provably maximal non-interleaving. Two people
  writing into the same region concurrently do not get their words shuffled into nonsense.
- **Eg-walker** (Gentle & Kleppmann) — what makes decade-long histories tractable: invoke the merge
  algorithm *only* to merge, then discard its state. Never store it, never transmit it. In steady
  state you hold the current content, not the accumulated machinery of how it got there. Because
  code is overwhelmingly edited sequentially, most of a codebase's history is never transformed at
  all.
- **A highly-available move operation for replicated trees** (Kleppmann et al.) — moving or
  renaming a directory concurrently with edits inside it, without the structure breaking. The
  defining operation of source control, and the reason the engine choice was forced rather than
  chosen.

### Sharding is a hard constraint

Loro's internal counters are 32-bit — roughly 2 billion operations per peer per document. **The
codebase must never be one document.** Per-file documents plus one tree document puts you three
orders of magnitude under the ceiling.

That in turn creates a many-document sync problem, and it is the largest concrete engineering gap
in the design: the published work built for exactly this case is abandoned at alpha. **This layer
will have to be built.**

One trap to guard from day one: concurrent creation of the same key converges to one visible value
and *silently hides the other*. In filesystem terms — two developers create `src/foo.rs` offline
and one file quietly disappears. The mergeable-container variants exist for precisely this, and
using them is discipline rather than an option.

---

## Migration (add-on)

Existing repositories are the hard case. New projects need none of this.

**Two things with two different quality bars.** Greenfield is native, trivial, and must be
flawless — it is the whole promise. Legacy migration is a one-time bridge: the hardest thing in the
system, and allowed rough edges because nobody lives on a bridge.

Company repositories are a genuine mess — dozens or hundreds of them, no coherent picture of how
they relate, version skew between internal packages, no atomic cross-repo change, N build systems
and N review queues. The migration add-on does three separately valuable things:

1. **Assessment** — the real dependency graph across every repository, duplicated and vendored
   code, circular dependencies, dead repos. Most organisations do not know this about themselves,
   and it is worth having *even if they never migrate.*
2. **Consolidation** — one codebase, all history preserved, one dependency graph, atomic cross-repo
   changes possible for the first time. Native here rather than a migration project, because there
   are no branches and one codebase by design.
3. **Continuous sync** — the transition path, with no flag day.

**The rule that keeps sync safe: one writable side at a time, per imported repository.** A repo is
owned either by git or by KungFu; the other side is a read-only mirror. Ownership moves repo by
repo, team by team. Simultaneous bidirectional writing means two sources of truth and reconciling
two models forever — that is where projects like this die.

Outbound export is **one commit per accepted block**, which makes the git mirror *better* than
hand-written history: every commit is atomic, green, attributed to both author and reviewer, and
described.

Open decisions: layout policy for consolidation, whether submodules get inlined or kept as
references, and a content-addressed store for binary and large-file content, which no convergence
engine provides.

---

## What we don't claim

Being specific here matters more than being impressive.

**Convergence is not correctness.** Automatic convergence guarantees every machine agrees on the
same text. It does *not* guarantee the text is a correct program. Two people concurrently editing
the same function converge deterministically — to something that may be nonsense. Published
research is blunt about this: an attempt to merge a live program heap this way produced truncated
lists and cycles while the merge engine worked perfectly, and the authors' own conclusion was
*"we don't have a solution yet."*

**Our answer is not to solve it at the merge layer.** Everyone attempting that is stuck, because it
requires inferring intent from text. Convergence is *allowed* to produce garbage, because
convergence happens inside a draft and a draft is isolated by definition. **Garbage cannot
promote**, because promotion requires the safe gate. We never need the merge to be correct — we
need broken output to be unable to reach anyone else, and that is a decidable, mechanical property.

Where two people genuinely meant contradictory things, that surfaces as a rare, legible event with
both intentions attached, and a human decides. A merge conflict is *information*; the goal is to
make it rare and meaningful, not to pretend it cannot happen.

**Structural merge is not the safe automation it appears to be.** The best available measurement
found AST-based merge tools buy roughly eight percentage points of extra auto-resolution and pay for
it with three to sixteen times more *silently incorrect* merges — and that a tool which merely
merges import statements better outperformed them. For an automatic system, "silently wrong" is
undetected corruption, so structure is used here as a **detector and classifier, never a resolver.**

**Automatic conflict resolution by language model is not viable as an invisible gate.** Current
benchmarks top out under 60% exact match. That is a suggestion, not a gate.

**Repository-scale convergence is unproven, and we intend to prove it.** The measured per-operation
costs are strong — roughly 0.5 to 1.1 bytes per keystroke on disk, memory tracking the working set
rather than the history, a real 1.66-million-operation document loading in about a millisecond, and
reconstructed traces from real repository history (947,000 events across 194 authors on a single
file) merging in under 10 milliseconds. Extrapolated, fifty engineers over ten years lands in the
low gigabytes. **But no published benchmark exists for a CRDT over an entire repository.** The
extrapolation is arithmetic on single-file traces, and every published trace is human keystrokes —
nobody has measured machine-generated edit patterns against any of these engines.

**Read-restriction is access control, not encryption.** Code references code, so signatures and call
sites leak structure. That covers the overwhelming majority of real needs and is strictly better
than a model whose only boundary is the whole repository — but it is not cryptographic
confidentiality and will not be described as such.

---

## Status

**Clean slate, by design.** The concept and architecture are documented here; the implementation
starts from nothing.

An earlier prototype existed — roughly 1,200 lines of Rust with a working convergence-backed edit
engine and an agent gateway — but it predated this design and did not reflect it. Rather than
carry code shaped by a different model, it has been archived and removed. It remains fully
recoverable for reference:

```
git checkout archive/prototype-v0
```

**The first piece of work is the measurement that de-risks everything else:** replay five to ten
years of a real repository's history into per-file documents and measure size, memory, load time
and merge cost, including machine-generated edit patterns. Roughly two weeks. It either validates
the foundation or kills the idea before a product is built on top of it — and the code written for
it becomes the migration add-on.

---

## Contributing

**Contributions are wanted, and the architecture is deliberately shaped to make them possible.**
Because the core is small and every integration is an add-on that can be absent, most useful work
can happen independently without coordinating with everything else.

Good places to start, roughly in order of how much they unblock:

- **The replay measurement.** Ingest a real repository's history into per-file Loro documents and
  publish the numbers. This is the single highest-value contribution available, it needs no product
  decisions, and nobody in the world has published it.
- **Editor capture plugins.** The VS Code extension has the most leverage by far — one artifact
  covers VS Code, Cursor, Windsurf and every fork. JetBrains and Neovim next. These are
  self-contained: stream edit events over local IPC and you are done.
- **The many-document sync layer.** The known gap. Per-file sharding is mandatory, which creates a
  multi-document sync problem whose best prior implementation is abandoned at alpha.
- **The filesystem watcher and reconciliation.** Unglamorous, essential, and full of real edge
  cases: save amplification, coalescing, queue overflow, atomic-rename churn.
- **Projection and reference rewriting.** Materialise the tree for compilers; rewrite references
  when code moves. This is what makes reorganisation free.
- **`last green` detection.** Watch build and test exit codes and tag the tree state. Small, and it
  delivers one of the most useful features in the product.
- **Migration add-on.** Assessment first — dependency graphs and duplication reports across many
  repositories are valuable output even for organisations that never migrate.

Before proposing a feature, three tests it has to pass — see **The three laws** below. The most
useful one to apply early: *every concept must be fully explainable in one sentence to somebody who
has never used the tool.*

---

## The three laws

Every decision gets tested against these.

1. **Costs surface where they are created.** Every pain in the incumbent model is a deferred cost
   that only becomes visible once it is expensive to fix. Move it forward to where it is cheap and
   somebody can still act on it.
2. **When something turns out to be policy, ship the mechanism and let the organisation own the
   policy.** How long a review may sit, who reviews, who gets escalated to — those are company
   decisions. The product exposes state, age and thresholds. If reviews are not happening, that is
   an organisational problem and the correct behaviour is to make it visible, not to solve it.
3. **If a feature only makes sense inside a story about the future of software, it is not core. If
   it makes a developer's Tuesday better, it is.**

And the design test that governs the entire interface: **every concept must be fully explainable in
one sentence to someone who has never used the tool.** *"It's a draft until you mark it ready"*
passes. *"Detached HEAD"* does not. That is the operational definition of *no manual*, and it is the
veto on every future feature.

---

*License: Apache 2.0*
