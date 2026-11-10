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
a payload backup. Remote byte publication, real hardware failures and the complete
command/failure course remain separate obligations.

For linked worktree and index lessons, choose another fresh output root:

```text
cargo run -p tig-git --features schema --bin tig-sample --locked -- rehearse-worktree ABSOLUTE_NEW_OUTPUT
```

Its twelve recorded checkpoints include the ordinary seven, then **worktree open
v1**, **index staged v2**, **index unstaged v1**, **worktree closed job active**
and **worktree job ended**. The primary checkout stays at v2 throughout.

| Command / trigger | Checked Git/source result | Verified placement |
| --- | --- | --- |
| `git worktree add --detach` | Separate review checkout/index at v1; explicit fixture draft and job admitted | Original returns to F+M; current fixed version and shared input remain F+M |
| `git add`, `git diff --cached`, `git diff`, `git status` | Review index selects v2; a later working-file edit selects v1 and yields `MM`; captured index/native snapshot still selects v2 | Old job preserves original nearby across draft changes |
| Dirty `git worktree remove` | Refuses without force; exact edited file and managed copies remain | No claim withdrawal or placement call for the refusal |
| `git restore --staged --source=HEAD` | Review index returns to v1; old captured v2 snapshot stays exact; primary index/HEAD remain unchanged | Explicit index-draft adoption keeps the independent old job protected |
| Clean `git worktree remove`, then draft withdrawal | Removes only the verified fresh child checkout | Original remains F+M because the job is still active |
| Explicit job end and fresh reconciliation | Job obligation ends separately | Original demotes to D+M; fixed version and shared input remain F+M |

`worktree.json` pins actual commit, index and native-context identities, status/
diff results and dirty-removal preservation. Its hash/size is in the CLI result.
The catalogue's **Associated Git commit** is a reference for the lesson; separate
admitted index/draft/job snapshots also contribute placement demand. `git add`
stages a small record here. No hook discovers/enrolls real worktrees or uploads
payloads. Worktree removal does not delete retained asset bytes. General native
writer integration and complete command/failure/retirement lessons remain open.

For backup unavailability, choose another fresh output root:

```text
cargo run -p tig-git --features schema --bin tig-sample --locked -- rehearse-backup-outage ABSOLUTE_NEW_OUTPUT
```

This produces nine recorded checkpoints. M's configured path becomes unavailable
during merge; then the old job ends while M remains unavailable. **Merge backup
offline** and **job ended backup offline** show actual held SDK reports. The
working copies remain; useful publication into history can proceed, but old F
cannot be evicted without fresh retained-custody verification. **Backup catchup**
restores that same provider and retries the same plan/source checkpoint, with
seed reads disabled. Ordinary demotion finishes only after current verification.

| Recorded state | Original | Fixed / shared | What remains unknown |
| --- | --- | --- | --- |
| Merge, M unavailable | F verified | F verified | M's current coverage |
| Old job ends, M still unavailable | F and D verified; F still protected | F verified | M's current coverage |
| M returns; same current plan retried | D+M verified | F+M verified | No unknown roles in this scoped observation |

Known copies and **Unknown · Backup / M** appear together in the table; partial
counts include only known copies. The next checkpoint's Before column preserves
the earlier unknown coverage. Unavailable does not mean absent, and a previous
backup receipt does not establish current custody. `backup.json` and the CLI
result pin plan/source identities and zero seed-read attempts during recovery.
This models configured-path unavailability by moving only a verified child
directory inside the fresh root; it does not qualify actual hardware failure.
No real drive is disconnected and no existing cache is changed.

For a same-byte rename, choose another fresh root and run:

```text
cargo run -p tig-git --features schema --bin tig-sample --locked -- rehearse-rename ABSOLUTE_NEW_OUTPUT
```

Eight recorded steps include the ordinary seven and a final Git rename of the
small authored record. A conflicting destination refuses first, preserving the
index, HEAD, authored bytes and managed copies. Successful `git mv` preserves
the record's Git blob and asset IDs; explicit adoption changes source context.
Actual SDK reconciliation streams zero bytes and records no provider effects.
`rename.json` pins both commits, paths, record object and unchanged copy snapshots.
Provider-owned payload paths are independent of this authored Git path.

For a protection change between planning and applying, use another fresh root:

```text
cargo run -p tig-git --features schema --bin tig-sample --locked -- rehearse-stale-plan ABSOLUTE_NEW_OUTPUT
```

This records ten checkpoints. After the old job ends, the fixture plans demotion,
then explicitly admits a new job before applying that plan. The actual SDK returns
Stale before storage effects. The rejected step shows the old plan's desired D+M
beside unchanged observed F+M, with both planned and newly observed basis IDs.
Its receipt is a separate actual SDK status observation, not a successful apply.

| Recorded step | Original's copies | Reason |
| --- | --- | --- |
| Stale plan rejected | F+M | New job invalidated the old plan before effects |
| Fresh plan, job active | F+M | Current protection requires the working copy |
| Fresh plan, job ended | D+M | Separately ending the job permits ordinary demotion |

`stale.json` pins the real error, old/fresh plan IDs, changed source/basis IDs and
equal before/after copy snapshots. A later equivalent desired set does not revive
an earlier plan's authority. The new job is explicit fixture admission; this does
not qualify arbitrary native writers, crash recovery or final deletion.

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
seeds, two works, twenty-three specified checkpoints and seven baseline counterexamples.
The ordinary runner executes seven; exchange executes ten; worktree executes
twelve; backup-outage executes nine including held/catchup stages; stale-plan
executes ten including rejected/fresh-plan stages; rename executes eight with
unchanged bytes and zero new provider effects. Exchange/worktree
add checked dirty-operation counterexamples. Final retirement remains
a target-only checkpoint. Each mode produces its own bounded rehearsal, with at
most sixteen reports per book. A complete recorded course still needs bounded
chapters or paging alongside the remaining lessons. Hashes
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
