# Exact enrolled protection demand snapshots

Input d18afba336f48f8b982ffe83c5b609d1f903c9f7. WP-004/006/008/009;
REQ-001/002/004/005/007/009; SPEC-002/004/005/006/008;
IF-002/004/005/006/008; PKG-001/003. RUST/PROVIDER profiles.
Allowed: tig-store source-journal observation and read-only protection projection
through existing NodeStore/Tree and pure View ports; synthetic fixtures only.
Entry: exact journals, strict tree identities and explicit caller enrollment.
Exit: all registered sources are compared with admitted expected sources, all
current/pending/predecessor binding roots resolve under aggregate budgets, and
missing or changed coverage remains explicit. No persistent format migration.

The profile admits journal identities, source owners and branch/draft/job/retention
kinds, exact root interpretation and per-source lifecycle placement roles. All
four kinds need an explicit scope declaration, including intentionally empty
kinds. Place keys are assigned within this authority's namespace; project-local
roles must be resolved before combining demands. This collector neither resolves
provider bindings nor proves physical independence or availability. This is a trusted boundary declaration, not proof that unmanaged clients
are absent. Snapshot reads actual complete registered source sets rather than
mistaking a list of selected source queries for a complete inventory.

Only explicitly admitted flat binding trees are interpreted. Opaque/native context
roots require admitted extraction and remain gaps here. Do not recursively treat
an arbitrary metadata file object as its semantic dependencies. Binding-tree
nodes verify their hashes/lengths using existing Tree ports. Missing/corrupt roots
produce a gap and no partial demand for that root; other positive roots remain.
All roles and configured places contribute; duplicate bytes share placements.
Conflicting sizes refuse. Exact profile/checkpoint/provenance/demand identities
are deterministic and separate from physical byte availability or custody.
Origins include exact selected revisions or pending prepare references, so repeated
identical roots retain distinct source-selection provenance while bytes still share.

At most 16 journals, 1,024 enrolled sources, 4,096 roots, 500,000 aggregate binding
associations, 2,000,000 metadata reads and 512 MiB metadata read bytes; callers
supply finite tighter budgets and a 1..600,000 ms cooperative deadline. The
ordinary fixtures use 60 seconds; the 33,000-binding resource fixture uses 180.
Checks occur between and after provider calls, not by interrupting blocked IO.
Trees share the total read/byte budget, including
failed traversals. Growing demand rows remain caller-owned outside ordinary Git;
only bounded summaries serialize. Capture rechecks journal heads after tree IO;
changes add gaps. No SQL transaction spans metadata traversal. This is an exact
observed snapshot, not a lease or rollback/latest-authority proof.

Tree-node reads have the aggregate read/byte budget; journal replay retains its
existing per-journal command/byte bounds, with at most sixteen journals. This
projection does not enumerate local replica-retention grant files; provider guards
must independently enforce those obligations. Root-kind retention is not custody.
The pure View adapter retains its existing 1,024-claim/65,536-placement limits and
refuses larger projections without truncation. It always sets complete_claim_coverage=false. Fully resolved
scope does not prove effective claim-admission/removal fencing, unmanaged writer
exclusion or independent custody. Ensures may progress; removal stays held.
This prerequisite advances priority 2 but does not close complete protection
admission. The following executor work must add effective per-object admission
fences and guarded intentional-absence/retry semantics before any payload removal.

L0 validate/diff/fmt; L1 focused vectors, default/all-feature workspace tests and
strict Clippy; L2 actual temporary journals, current/pending/predecessor transitions,
unexpected/foreign/missing source cases, changed journal during metadata reads,
shared bytes, missing/corrupt root closure, restart and aggregate-budget refusal.
No live byte-store census, cache initialization, payload provider IO or runtime adoption.

Snapshot::copy_basis binds the observed demand/profile identities into the
existing orchestration Basis with the adapter's exact source-context digest. It
always carries an unfenced-source-admission gap; unresolved roots add a separate
gap. Actual temporary-store integration tests drive copy/retention with this
basis and reject reuse after claims change. No second source inventory is authored.

Snapshot identity changes with any captured journal head. This is provenance, not
an instruction to serialize unrelated jobs or replace their frozen input context.
Future admission fences must be scoped to overlapping object protection; a global
snapshot mutex or holding a journal transaction through IO is not an implementation
of that requirement.
