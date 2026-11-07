# Controlled interfaces

Full target interfaces remain proposals. Bounded experimental core/store/Git/asset
subsets are implemented and scoped in EVIDENCE.md; none is a released complete
product contract. Versioning/canonicalization/path/error/limit rules
must be frozen with vectors before any new persistent format is accepted.

| ID | Boundary / owner | Inputs -> outputs and safety semantics | Compatibility / proof |
| --- | --- | --- | --- |
| IF-001 | Pure handoff / PKG-001 | Assigned claims + alleged verified placements -> exact plan; no IO/removal authority | Current public Rust types; 13 tests; no released wire contract |
| IF-002 | Object/tree / PKG-001 | Digest/size + bounded snapshot/path -> exact object or scoped error | Domain-separated native identities; raw blob OID mapping explicit; malformed/budget vectors |
| IF-003 | Ingest/publication / PKG-002/003 | Expected identity, source, admitted resources and operation ID -> persistence-level acknowledgement | Idempotent retry versus conflicting payload; flush/verification and crash matrix |
| IF-004 | Owned claims/job/read lease / PKG-003 | Owner/source/scope/generation/clock assumptions -> durable protection/fenced action | No expiry-only deletion; authority/renewal/ABA and recovery fixtures |
| IF-005 | Observation/catalog / PKG-002/003 | Exact object/place/provider generation, evidence scope -> usable/unknown/denied/unavailable view | Presence, freshness, retention and failure domain distinct; no proof from name alone |
| IF-006 | Reconciliation / PKG-003 | Exact desired/observed checkpoint and policy/budgets -> plan, pending state and retry journal | Fresh final removal checks; peak scratch/allocated outcomes and full-disk recovery |
| IF-007 | Byte worktree / PKG-005 | Base/private write set + expected selections -> sealed candidates/proposed Git edits | Tombstones, dirty/private data, same-leaf conflict and crash-between-steps tests |
| IF-008 | Git source / PKG-004 | Trusted enrollment + exact commit/tree/index/worktree scope -> pointer/claim view with coverage | No arbitrary checkout hooks; conflicted index and source-identity fixtures |
| IF-009 | Git LFS / PKG-004 | Supported standard pointer and upstream transfer messages -> exact verified access/completion | Pinned Git/LFS client versions; unsupported extensions reject; no implicit cloud success |
| IF-010 | Typed assets / PKG-006 | Definition/record/variant/native source pins -> exact semantic/result/dependency view | Preserve Art identity/test vectors before import; no duplicate authored reference list |
| IF-011 | Evaluation/build/delivery / PKG-006/007 | Consumed subject/producer/policy + admitted operation -> produced evidence or observed target state | No score implies authority; source edits/withdrawals and remote drift/retry fixtures |
| IF-012 | Provider/transport / PKG-002/007 | Explicit roots/capabilities/credentials/representation maps -> bounded operations and evidence | Cloud/local/LFS/BitTorrent capabilities evaluated separately; deletion may be unsupported |
| IF-013 | Application/UI / PKG-008 | Explicit source/view and read/execute scope -> explanation or admitted operation | Native CLI/catalogue subsets; full WASM parity and private-data boundaries remain open |

## Compatibility and failure contract

Unsupported schema/extension/capability, missing source, denied access, stale
generation, ambiguous authority and resource exhaustion remain distinct errors.
Reject conflicting digest sizes. Imports never choose arbitrary filesystem paths.
No plain hash implies access permission, independent custody or current approval.

Native design-1 intake is experimental under specs/NATIVE-INTAKE.md; do not rename
legacy .art/schema domains or replace standard LFS pointers as branding. Git selection and Tig publication
do not share one transaction; durable candidate protection bridges them.

## Stage review

Producer-applied six-role lenses: pass_with_risk. IF-01: a single success flag
could confuse sealing, local durability and remote replication. Resolved by
explicit acknowledgement/evidence levels and interface-specific proof. Released
schemas/provider capability matrix remain entry gates for their implementation.

## Authored authority versus file presence

Directory presence is an observation, not semantic selection or current/history
authority. In native typed mode, a small Git record is created when a logical
asset is declared, potentially before any payload exists. It owns parameters and
the adopted result binding. A sealed job output is only a candidate until that
record's expected-selection adoption succeeds. Its .tig extension/syntax is
still proposed. Standard Git LFS pointers remain the alternate direct byte path.

```text
Git record/pointer -> exact selected object -> Tig logical working view
                                             -> indexed eligible physical copy

Git lifecycle + admitted rule -> owned placement/retention claims
                              -> transfer/verification/release plan
```

Source context is explicit: dirty working records, staged index, branch commit
and PR merge candidate are different views. Filename equality or an image left
in a staging directory cannot replace a selected binding. A place rule governs
where required copies belong; it does not choose a more recent render's bytes.

IF-02 follow-up review: clarified the one authored selection source and declaration
before production. No new wire format or directory-presence-as-authority rule was
introduced. Native record adoption and LFS generation still need their fixtures.

IF-03 concurrency follow-up: workers author independent Git worktree copies of
the same stable leaf record. Textual merge success is insufficient if it pairs
changed controlling parameters with a result produced from other inputs. Verify
the selected result's consumed fingerprint against the adopted record/context;
same-leaf competing results need explicit adoption. Original byte objects remain
immutable and protected under the union of outstanding claims.

IF-04 instance/definition and merge-policy clarification: native .tig files are
asset instances, not type schemas. Separate exact definition pins govern field
shape, stream groups/order/roles and admitted semantic input/output rules.
The design-1 fixture envelopes in specs/schemas are proposals, not a released
runtime parser. See specs/NATIVE-RECORDS.md for raw-byte fixture pin semantics.

Merge resolution may explicitly take three streams from one branch and two from
another. Validate declared constraints on that actual candidate before accepting
it; do not impose a common production attempt or infer invalidity from unusual
mixing. A trusted local driver can surface semantic conflicts, and required
server/merge-queue checks enforce target policy if local hooks are bypassed.

Compare desired consumed inputs with recorded production provenance to derive
freshness. A valid source merge may retain an out-of-date last-produced episode:
source consistency and ready-output/delivery gates are separate. Projects may
merge fixes now and explicitly run catch-up builds later. The batch freezes one
exact chosen source view; new edits do not change a running job's inputs. Queue
coalescing before admission does not promise to undo renders already started.

This follow-up preserves truthful history and user-resolved bindings while
requiring exact schema/semantic validation at the configured transition. No
native format release, automatic render scheduler or merge driver is implemented.
