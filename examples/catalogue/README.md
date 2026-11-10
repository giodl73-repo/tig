# Learn Tig through one sample catalogue

Delivery contract: [TEACHING-CATALOGUE](../../specs/TEACHING-CATALOGUE.md).
The first runnable rehearsal covers seven ordinary lifecycle checkpoints using
real Git, source journals and provider operations. The catalogue's **Learn Tig ·
sample lessons** view reads their actual sealed reports. The complete teaching
course, failure rehearsals and final-retirement/lifetime lessons remain in development.

Configure external build and temporary roots using
[development setup](../../docs/DEVELOPMENT.md), then choose a **new absolute output
directory** outside every Git checkout. Replace `ABSOLUTE_NEW_OUTPUT` below with
that path; its parent must exist.

```text
cargo run -p tig-git --features schema --bin tig-sample --locked -- rehearse-ordinary ABSOLUTE_NEW_OUTPUT
cargo run -p tig-git --features schema --bin tig-catalogue --locked -- check ABSOLUTE_NEW_OUTPUT/catalogue.json
cargo run -p tig-git --features schema --bin tig-catalogue --locked -- serve ABSOLUTE_NEW_OUTPUT/catalogue.json 8801
```

The rehearsal commits v1, branches to fix the data, merges v2 and removes the
merged fix branch while an old job still uses v1, ends that job, reverts, and
returns to v2. It checks every checkpoint
against the independent oracle before acknowledging it. Existing output roots
are refused; repeat in another new directory. No viewer starts automatically.
The runner uses an isolated Git configuration, an empty template and its own
fixture identity.

For the local Git exchange lesson, choose another fresh root and run:

```text
cargo run -p tig-git --features schema --bin tig-sample --locked -- rehearse-exchange ABSOLUTE_NEW_OUTPUT
```

This includes the ordinary seven checkpoints, then adds **publish v2**, **pull
v1** and **pull return v2** to the same catalogue lessons. Open its generated
catalogue with the check/serve commands above. Everything runs against a new
local bare remote and observer checkout inside that root; no account or network
endpoint is used. `exchange.json` records actual commit identities and the dirty
pull refusal; the CLI result pins its exact hash and size.

| Command / trigger | Checked result | Tig copy evidence |
| --- | --- | --- |
| `git push`, `git clone --no-local --no-hardlinks` | Observer selects the published commit and small records; selected payload seeds are absent from its tracked tree | Existing F/D/M copies remain; this checkpoint streams zero payload bytes |
| `git fetch --no-tags` | Remote-tracking ref advances; observer HEAD remains unchanged | Fetch itself requests no Tig action |
| `git pull --ff-only` with an overlapping local edit | Git refuses because the edit would be overwritten; edit, HEAD and all managed copies remain unchanged | No adoption or placement call runs for this failed pull |
| Repair the exact fixture edit, then `git pull --ff-only` | Observer selects the exact v1 record | Explicit fixture source adoption/reconciliation brings original to F+M and fixed to D+M |
| Push/fetch/pull the return to v2 | Observer selects exact v2 | Explicit reconciliation returns fixed to F+M and original to D+M |

The observer is a metadata reader, not an enrolled asset writer/job. There is no
automatic pull hook or general live-source adoption. The bare Git remote is not
a payload backup. Remote byte publication, offline backup and complete command/
worktree/failure lessons remain separate course obligations.

In the catalogue, choose **Learn Tig · sample lessons**. Use the lesson selector
or Previous/Next to compare each object's earlier recorded copies, the plan's
desired roles and the freshly verified copies recorded after that step. Follow
the original through **merge v2 job active**, **job ended** and **revert v1**:
F+M becomes D+M, then F+M again, while its hash stays identical. These are recorded
results; they do not claim current custody. Missing evidence is unavailable,
and an absent observation stays unknown. The viewer cannot run storage actions.
Choose **Inspect sample catalogue** to visit the actual works; returning to the
lessons keeps your selection. This control appears only with explicit teaching
enrollment. Report bytes are verified against the enrollment's pinned trace.

Open **Studies** and inspect the two works, then the managed
objects and named history stream. The final view shows current copies; earlier
Git snapshots do not turn today's storage observation into a past observation.
`trace.json` points to the actual per-checkpoint reports in `reports/`, containing
source commits, plans, receipts and verified locations. In particular, compare
`merge-v2-job-active.json`, `job-ended.json` and `revert-v1.json` to follow the old
object from working to history and back without changing its identity.

These small directories model three **synthetic admitted failure domains**. Real
copies and hashes are checked, but independent hardware backup is unproven and
labelled as such in every report. The sample's selected/primary/history adapter
is fixture-specific; it does not qualify arbitrary native asset semantics.
The fix branch can propose a released record before main accepts it; that proposal
does not change main's adopted selection. Drafts have their own explicit protection.

The same assets carry through the course: a shared input used by two works and
an asset with an original and a fixed version. Working, history and backup are
configuration roles. Every lesson will show Git references, exact object identity,
Tig's plan and verified physical copies together.

[The public input/oracle](lifecycle.design-1.json) freezes three exact UTF-8 byte
seeds, two works, eleven specified checkpoints and seven counterexamples. The
ordinary runner executes seven; exchange executes ten. Final retirement remains
a target-only checkpoint. Hashes
and byte sizes identify real generated inputs. Expected copies are literal
independent assertions for the existing executor; the file is not executed
evidence. Runtime qualification must produce and check the matching observations.

Follow three identities: **S** is the shared reference input, **O** the original
series, and **N** its fixed version. The fixture checks these copy sets:

| Checkpoint | Working / F | History / D | Backup / M |
| --- | --- | --- | --- |
| v1 released | S, O | empty | S, O |
| v2 merged; old job active | S, O, N | empty | S, O, N |
| Old job ends | S, N | O | S, O, N |
| Revert to v1 | S, O | N | S, O, N |
| Return to v2 | S, N | O | S, O, N |

S is used by both works but occupies one copy in each required cache. Ending the
old job frees its working placement; it does not remove the retained backup.
The backup keeps growing across replacements until a separately authorized
final-retirement lesson removes eligible old managed copies.

| Lesson | Git activity | Tig result to demonstrate | Counterexample / visible explanation |
| --- | --- | --- | --- |
| 01 Start | init / clone | Create explicit isolated sample enrollment; no implicit discovery | Opening a project cannot start transfers or removal |
| 02 Add | add / status / diff | Admit new bytes and show draft versus committed selection | An uncommitted change cannot replace the committed view |
| 03 Commit | commit | Pin exact references and preserve old immutable bytes | A commit is not evidence of a backup |
| 04 Branch | branch / worktree / switch | Share unchanged bytes; add independent working obligations | Dropping one branch cannot release another job's bytes |
| 05 Publish | push / fetch / pull | Publish missing bytes and report verified completion separately | Missing/failed backup leaves placement pending |
| 06 Release | merge | Fixed version goes to working+backup; inactive old bytes go to history+backup | Shared inputs stay nearby; an active job keeps old bytes nearby |
| 07 Finish job | inspect source obligations | Job-end permits a fresh demotion plan; destination protection precedes eviction | Stale plans cannot override a new claim |
| 08 Return | checkout / revert | Resolve exact old bytes and reacquire only missing nearby copies | An old path cannot silently resolve to today's object |
| 09 Explain | log / show / diff | Inspect version references, shared objects and actual copy receipts | Historical outcomes are distinct from current observations |
| 10 Remove branch | branch -d | Retire owned branch obligations and recompute placement | Hard retention and other owners remain protected |
| 11 Reclaim | inspect eligible cached copies | Evict eligible replicas while retaining required custody | Offline/corrupt backup and incomplete coverage hold removal |
| 12 Retire | rm plus a separate final-retirement decision | Select a block/category/age, disclose exact last-copy loss, authorize and execute | Shared/job-protected objects remain excluded; cancellation cannot undo completed loss |
| 13 Bound history | inspect retention / expiry | Remove authored records and eventually expire detailed operational metadata | Stale restored clients still cannot replay old deletion authority |

Each lesson needs one command sequence, an expected-results table, a reset into
a new fresh root, a machine-checked scenario and its corresponding catalogue
view. The corpus is an independent oracle for existing adapters; it must not
grow a second solver or cache implementation. Simulated time and target-only
capabilities must be visibly labelled.
