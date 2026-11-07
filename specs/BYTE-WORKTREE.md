# Byte worktrees and Git selection

Status: target contract with an experimental pure binding-state subset under
WP-002 (checkpoint below). An explicit small-file Git proposal seam is documented
in [Git record proposals](GIT-RECORD-PROPOSALS.md). No mounted/durable overlay,
automatic Git adoption or released pointer format. Parent requirements:
TIG-R01/R04/R05/R09/R10.

## Four states, one version authority

| State | Owns | Authority |
| --- | --- | --- |
| Base | Exact immutable tree plus interpretation/namespace pins | Frozen starting view, never a moving current alias |
| Private changes | Durable put/remove bindings and isolated in-progress writes | Admitted worktree/job owner; not a published version |
| Sealed candidate | Immutable objects and resolved candidate bindings | Exact bytes/provenance, not approval, selection or a Git commit |
| Git selection | Authored leaf pointers/typed records in index and commits | Determines which candidate is adopted for that source view |

The worktree has a stable local identity independent of a branch name or directory
spelling. It records its base/source and durable admission. A new worktree shares
base objects and metadata; it need not materialize or copy every file. Shared
immutable nodes remain cache-managed objects, not owned editable files.

Conceptual read rule for logical path p:

```text
if a completed private put exists at p: use that candidate
else if p is covered by a removal entry: report absent
else: resolve p in the pinned immutable base
```

In-progress writable output is private to its producer and cannot masquerade as
a completed immutable candidate. A directory-level removal must explicitly cover
its subtree; recreation and descendant exceptions require defined overlay rules.
The first implementation may restrict removal to exact file paths and reject
unsupported directory operations rather than guess the result.

## Same filename, new bytes

```text
base V1                       job-private write
cel-a -> A1                   job-42/cel-b.png
cel-b -> B1                            |
cue   -> Q1                   close / hash / verify
                                       |
private completed changes: cel-b -> B2
                                       |
effective view:             A1 B2 Q1
                                       |
seal candidate V2           immutable snapshot/delta objects
                                       |
explicit adoption          update cel-b Git pointer / selected-result record
```

The producer receives a frozen read view and bounded private output area, ideally
on the configured destination place. It does not copy the whole base directory.
Plain filesystem tools may need explicit materialization; account for it and
enforce mutation isolation. A future virtual filesystem can expose the same view
transparently, but the first library API need not install a mount or daemon.

Sealing hashes actual completed bytes. If they equal B1, reuse the same byte
object; a distinct production attempt can retain its own evidence. Differing bytes
create B2 even if the pictures appear identical. Never mutate B1 to save space.

## One source of authored selection

Two supported authoring paths are planned:

1. Standard Git LFS pointers at existing tracked payload paths. Git LFS keeps its
   pointer/clean/smudge contract; Tig integrates access and retention around it.
2. Small native Tig records, optionally typed, for independently managed assets.
   They own authored properties/references and selected-result bindings. Type
   definitions remain separate from instances; exact result payloads live in
   immutable storage. Legacy .art adoption requires explicit parity/migration.

Native extension and encoding are not frozen. A .tig record is a naming proposal,
not a reason to replace existing LFS pointers or rename wire/hash domains.
Do not maintain both a handwritten LFS/record list and a second editable tree
membership list. Derive bundle trees from the admitted authoritative references.

Recommended merge granularity:

```text
Git scene-04/                 derived immutable release tree
  scene record ----+         scene definition and exact dependency pins
  cel-a record ----+-------> A1
  cel-b record ----+-------> B2
  cue record ------+-------> Q1
```

Changing Cel B updates its record. A scene that references its stable logical ID
need not churn unless authored scene intent changes. A new resolved build view
pins the adopted exact selections; historical views stay frozen. This requires
the optional typed layer's resolver/fingerprint rules, not just byte placement.
An ordinary LFS repository instead changes the appropriate leaf pointer.

Generated aggregate roots are checkpoints, not mandatory hand-edited global
records. If a project chooses root-only authoring, it accepts tree-level merge
requirements and must not be promised conflict-free leaf edits without that code.

## Rename, remove and discard

| Change | Logical effect | Byte effect |
| --- | --- | --- |
| Put new bytes at existing path | Completed overlay replaces base binding | New immutable object if bytes differ |
| Rename path | Remove old binding and add new binding to same object | No payload transfer solely for the rename |
| Remove path | Explicit removal hides base binding | No immediate object deletion |
| Discard private change | Authorized local overlay cleanup returns that path to base | Shared objects/other claims untouched |

Tig does not automatically rewrite references inside arbitrary native files when
a path is renamed. Stable typed IDs can survive path changes; path-based consumers
need a declared adapter/refactor. Reject traversal, unsupported file/link types,
case/normalization collisions and file/directory prefix conflicts before IO.

## Two branches or jobs edit the same base

For base binding B1, left and right selections follow ordinary three-way intent:

| Left | Right | Intended result |
| --- | --- | --- |
| B1 unchanged | B2 | B2 |
| B2 | B1 unchanged | B2 |
| B2 | B2 | B2 with both required provenance/decisions reconciled |
| B2 | B3 | Explicit conflict/adoption; do not pick latest completion time |
| removed | B2 | Delete/modify conflict |
| removed | removed | Removed binding |

Different independently authored leaf paths can merge independently under Git.
Tig must not claim Git automatically merges opaque binary content. Typed records
may need semantic reconciliation when metadata, approval and selected-result pins
refer to different subjects. Never attach a previous subject's pass to new bytes.

Within one local view, each job carries the expected base/selection for its write
set. Adoption compares those expectations at the relevant paths. A stale result
remains a protected candidate, not the newly selected result. Disjoint write sets
should not require a global cache lock. Cross-path atomic changes need an explicit
transaction scope; the pure prototype's global fixture generation is not that
scalable worktree protocol.

## Crash recovery and Git coupling

```text
admit/protect -> private write -> verify/seal -> propose record edits
      |                                |                |
      +-------------- durable recovery ----------------+
                                                       |
                                       explicit Git index/commit adoption
                                                       |
                                   durable source-claim reconciliation
```

There is no atomic transaction spanning file publication, Tig's journal, Git's
index and remote push. If sealing succeeds but record adoption fails, retain a
recoverable candidate. If Git adoption succeeds before Tig observes it, preserve
admission/old protection until exact-source catch-up completes. Retrying the same
operation must identify its prior result; conflicting reuse must reject.

Protect new objects before branch pointers can name them. Removing or resetting
a Git pointer cannot authorize deleting a user's dirty file, another worktree's
input or the last retained copy. Explicit user intent is required for destructive
private-worktree cleanup; cache eviction remains separately fenced and verified.

## Validation before implementation

Existing designed cases provide related obligations: T04/T05 for names, CL04-CL06
for publication/read races, GW05-GW09 for staging/dirty views, RW01/RW02/RW09 for
fencing/retries and LC01/LC06/LC08/LC09 for production/adoption/reuse. They do not
constitute executed byte-worktree tests. Required new executable fixtures include
overlay fallback/tombstones, independent writes, binary/delete-modify conflicts,
same-byte reuse, stale job adoption and each publication/index crash boundary.
Keep these linked to F01/F03/F04/F06/F08/F10/F12 and M01/M03/M04/M06/M08/M10/M12.

## WP-002 pure implementation checkpoint

Input: main 792578e3cb7a28f2b048eb2376e6cddb2da740cd, verified pure WP-001.
Controls: REQ-001/004/009, SPEC-006, ARCH-004, IF-007/008, DES-006,
PKG-001/005, PROFILE-002 (RUST). Scope: tig-core worktree module and synthetic
public-API tests; broader provider/engine integration remains planned. Entry:
design-1 trees and exact-leaf overlay semantics frozen. Exit: shared base,
isolated puts/removals, no-op reuse, exact candidates, disjoint adoption and
same-leaf/delete-modify conflicts exercised without provider IO.

`ByteWorktree::open` stores a base pin without reading/copying its tree.
`put`/`remove` stage at most 1,024 exact-file intents. Prefix-overlapping intents
reject before mutation. Directory operations/file-directory conversions are
unsupported: staging does not validate base shape, so `seal` rejects them.
Reads return staged pins/tombstones or fall back to base. `discard` drops one
private intent without deleting any object.

`seal` resolves touched base bindings, removes no-op changes and derives a tree
plus exact before/after bindings. It preserves the private view/base on success
and failure; repeated sealing has the same root. This is a **binding seal**, not
durable byte publication: callers supply completed object pins. Hashing,
storing, verifying and protecting actual payloads remain WP-003/004/007 duties.
A pin is not proof bytes exist. Separate production attempts can share one raw
object identity without merging their provenance or approval.

`plan_adoption` compares changed leaves against a current tree. An unchanged
target adopts ours; identical content is already applied; different content or
delete/modify yields structured conflicts. File/directory conflicts have a
shape kind. A conflict result has no partial executable plan and writes zero
metadata. Missing/corrupt metadata and exhausted budgets are errors, not absence.
Resolve explicitly against the new current view, then seal/plan again; no
timestamp winner or binary-content merge is provided.

`AdoptionPlan::apply` requires its exact expected target root. Changed views
must replan; disjoint changes still reconcile. It returns a tree value, without
publishing a shared ref or changing Git. Real scoped generation/index fencing
is required around publication; this compare alone is not concurrent execution
safety. No global cache lock is introduced.

Tree IO limits cover the whole seal/plan, including reads ending in shape
conflicts. Counters are charged at the metadata port, not reset per leaf. These
limits do not sandbox provider allocation. Sparse touched-route checks do not
certify whole metadata closure, payload bytes or custody; complete admission
needs full validation/verified-root authority. Empty intent sets preserve pins
without validating the complete tree. Durable worktree/job ownership is future
engine state, not supplied by this cloneable in-memory value.

L0: structure/fmt/whitespace. L1: 22 new worktree tests plus 39 regressions,
locked workspace tests and all-target Clippy with warnings denied. L2: public
API callers exercise in-memory/fault ports, the two-day scene example, generic
dataset edit commutativity, stale-plan revalidation and failed-seal retry.
Actual Git/index crash protocols, materialization, space reservation, replica
movement and eviction remain unverified; VAL-001 remains planned. The 110 full
workflow scenarios are still designed, not newly executed by these tests.

Commands: `cargo test --test worktree --locked` (22),
`cargo test --workspace --locked` (61 at the WP-002 checkpoint; current total in
README). Evidence and producer role review
are recorded under docs/vtrace/ with exact foundation-targets.json source pins.
