# Frozen job resource contexts

Entry96177878c7db238958c20a5faf1eec9bcfd6f438. WP-004/006/008/009/010/011/012;
REQ-001/002/003/004/006/007/008/009/010, SPEC-003/004/005/007/008/010/011/012;
IF-002/004/005/008/010/011/012; PKG-003/004/006. Scope: optional native metadata
adapter, existing NodeStore/Tree/SourceJournal composition, fixtures and docs.
RUST/GIT/PROVIDER profiles; no cache creation, media IO, producer execution,
new ledger/provider, global lock, implicit journal transition or deletion.

A frozen production context protects declared source interpretation but omits
external producer/runtime pins and later output evidence. Add a generic job
context that references that existing context by hash, binds the exact original
recipe/operation and caller-admitted named runtime pins, and optionally adds an
exact original-plan checked unsigned output claim and named evidence pins.
Do not copy the whole source/input list into another editable manifest. The job
tree contains bounded added references; typed requirements are derived through
the already verified production context and claim. Runtime/evidence names use
LogicalPath. Runtime pins have at most 64 names; evidence is bounded by the
caller's total TreeLimits.max_entries, including all fixed, runtime, claim and
optional variant entries. Metadata/header/claim use existing strict ports.
Check safe sizes, contradictory hashes, context/plan/parameter alignment and
logical entry budgets before writes. Tree/provider IO budgets apply separately
and failed writes can leave unreferenced immutable prefixes. Freeze returns only
candidate metadata; explicit finish,
reopen/readback and owner prepare/adopt remain required before job handoff.

First restore the original production context under its external native admission.
Job restore then takes that admitted context/plan/runtime/evidence, checks
all exact tree/header/claim bytes, refuses missing/extra/substituted entries and
writes nothing. It does not infer opaque runtime dependency closure or trust an
unsigned claim as execution/approval/custody. Derived roles distinguish primary,
controls, interpretation, runtime, claimed output and evidence; raw deduplicated
logical size is not physical allocation or free-space admission. Generated cels
can be producer subjects and downstream primary inputs; compiled-master canonical
dependencies are still forbidden by production context admission.

Use the resulting Selection with the existing scoped SourceJournal. Test planned
and claimed-output handoff through actual metadata finish/reopen and journal
restart. Another source/main advancing must not release an independent job's old
requirements. Failure/pending keeps earlier protection; no timeout-based expiry,
release, deletion or invented complete claim coverage. Native managed execution,
job capacity/availability, live fencing, global sources, custody/retirement and
full legacy consumer adoption remain separate open obligations.

```text
exact source context + original recipe + declared runtime pins
                  |
       freeze small linked job context
                  |
       finish -> reopen/readback -> owner prepare/adopt
                  |
       independent job retains original requirements
                  |
       producer/byte publication (separate proof)
                  |
       freeze context + checked claim + evidence pins
                  |
       finish -> reopen/readback -> prepare/adopt claimed root
                  |
       earlier job root remains an owned predecessor
```

Primary/control/interpretation pins come from the admitted production context;
producer plus explicit runtime pins remain Runtime. Claim output payloads remain
ClaimedOutput, never automatically canonical primary inputs of that same job.
Output definition pins and raw claim are interpretation; named receipts are
Evidence. The same raw Object can have several roles and is counted once in the
derived logical total. This total excludes implicit storage overhead and does
not establish byte availability or transitive metadata-provider custody.

Content-identical attempts may share this immutable resource root and payloads.
The independently owned source/job identity and journal operations distinguish
the attempts; a resource root or unsigned claim is not an execution-event ID.
Do not add random attempt values to each payload just to prevent safe deduplication.

L0 fmt/locked tests/all-target-all-feature warnings-denied Clippy/repo and canonical
VTRACE. L1 mismatched context/plan/params/claim, missing/corrupt/substituted metadata,
unsafe/conflicting sizes, names/count/tree budgets and zero-write refusal. L2 real
temporary Git + SQLite finish/reopen + SourceJournal handoff/restart, with source
metadata unavailable, and shared-input/sparse job envelope behavior. Six producer
role lenses cover identity, placement, custody, concurrency, privacy and adoption.
Current-head Windows/Linux qualification required, not independent/human review.

## CI concurrency follow-up

Entry561cc1a98f6eac1fe58e3cfa849554da40d7853d, PR29 run40496316603 Windows
job122162546016: separate actual Git LFS clients failed with generic Capacity;
the Linux companion was cancelled by matrix fail-fast, not a separate code failure.
Other exact-head Linux push qualification passed. Existing admit/settle retry only
eight transient Busy/stale-probe attempts with identical exponential scheduling,
then incorrectly reports contention as Capacity. Two clients can supersede samples
or exhaust the short synchronized retry window without a full disk.

Extend allowed scope to the existing tig-git cache/protocol adapter and controlled
concurrency fixture (WP-004/006/009, REQ-003/004/007/008/009, SPEC-003/004/007/009,
IF-004/005/007/009/012, PKG-003/004). Use bounded capped op-specific backoff and
more transient attempts, always re-probe/re-sample rejected observations. Preserve
all capacity/headroom/freshness/corruption/ownership checks; no global locks, broad
exception retries, test command retries or weaker admission. Exhausted transient
contention must report Busy distinctly from actual capacity/policy rejection.
L1: deterministic >8 transient conflicts then success, bounded delay, immediate
non-transient refusal and exhaustion error. L2: all six actual LFS client cases
and concurrent independent clients; exact-head cross-platform CI requalification.
Do not restart the still-running push workflow merely for an observation timeout.

## Native authority-identity follow-up

Entryfd23a1801920b859e70d64a6cf7447c806b46c6c. Native job review found that
source/owner/revision/root may coincide in separately created journals. Expose
SourceJournal.identity as the already verified stable genesis digest so consumers
can bind capabilities and job fences to actual journal authority, not caller
namespace labels or mutable whole-journal checkpoints. No new authority or
transition is created; identity alone is not a current-state/retirement fence.

## Complete output evidence budget

Entry7da43d6ca60c7f0c1821e7f7b73b21ca1cd03dc1. WP-004/008/010/011;
REQ-002/003/006/007/010; SPEC-004/005/010/011; IF-004/005/008/010/011/012;
PKG-003/004/006. Allowed scope: native job evidence entry admission, equivalent
prefix conflict checking, synthetic metadata tests and bounded control records.
Entry: existing exact context/recipe/claim admission. Exit: complete declared
evidence fits the caller's total tree-entry budget, or refuses before NodeStore
IO; runtime's independent 64-name bound remains. No payload/provider activity,
cache creation, transfer, global lock, custody or deletion is added.

A checked delivery can contain hundreds of stems, review frames or other output
objects. Truncating that list to 64 would omit resource obligations. Apply the
existing explicit tree-entry budget to the whole envelope, including four fixed
entries, runtime names, claim, evidence and any retained variant entry. Checked
arithmetic and name-conflict validation precede metadata reads/writes. Ancestor
lookup is bounded by LogicalPath's depth instead of comparing every name pair.
Every evidence object retains its Evidence role; it does not become a canonical
primary input or prove complete global source coverage.

L0: repo validation, fmt and warnings-denied all-feature/all-target Clippy. L1:
1,000 evidence names at the exact total-entry boundary; one fewer entry refuses
before freeze/restore IO; prefix conflicts and the runtime bound still refuse.
L2: real temporary Git context/claim, immutable freeze/readback, deduplicated
role membership and sparse replacement preserving the prior root. Exact-head
Windows/Linux CI and downstream consumer qualification remain separate gates.
Allowed scope adds this read-only accessor and one temporary-journal regression
(WP-008/009, IF-005/008, PKG-003/004). Reopen preserves identity; independently
created same-project/epoch journals differ. Exact-head CI must requalify.

## Pre-execution composition dependencies

Entry4e80e0116e36567d1e1f10dbc6c6acf742e2f192. WP-004/008/009/010/011;
REQ-001/002/003/004/006/007/009/010; SPEC-002/004/005/008/010/011;
IF-002/004/005/008/010/011; PKG-003/004/006. Allowed scope: optional native job
resource admission, immutable envelope/role derivation, existing metadata/journal
fixtures and bounded control records. No new provider/ledger, media semantics,
cache activity, automatic ownership change or global lock.

Entry: exact admitted production context, original plan/runtime and independent
caller resolution of composition/evidence pins. Exit: those exact pre-execution
dependencies survive freeze/finish/readback and independently owned journal
transitions, or admission refuses before metadata IO. RUST/GIT/PROVIDER profiles:
L0 fmt/locked warnings-denied all-feature Clippy/repo/VTRACE; L1 exact-map, names,
size/overflow/whole-entry bounds, zero-IO refusal, legacy identity and variants;
L2 real temporary Git, metadata reopen and journal restart/shared dependency cases.

Add an opt-in Dependencies port with named composition objects and input evidence.
Explicit APIs accept it alongside an optional independently replayed variant
packet. Existing Admission and freeze/restore APIs remain source-compatible.
Composition has its own resource role; input evidence remains Evidence. Neither
becomes Primary, Runtime or the job's newly ClaimedOutput. Derive these maps from
authoritative asset/receipt resolution; do not create a second authored inventory.

Nonempty dependencies use header experimental-3, with exact counts and disjoint
composition/input-evidence tree namespaces. Empty dependencies retain identical
experimental-1/2 roots, including the variant case. Include all names in the same
total entry budget before any metadata IO. Names are prefix-safe within their
namespace; all pins share contradictory-size/safe-integer checks. Cross-role
byte aliases remain valid and logical sizes deduplicate immutable objects.

Restore requires the exact external dependency admission; missing, extra, renamed,
reclassified or substituted pins refuse. Optional output claims/evidence and
variant packets may coexist without resetting budgets. Job roots and scoped
source ownership protect the admitted references before execution; metadata-only
membership establishes no byte availability, semantic/producer currency,
independent custody, complete global claims or removal authority.

These are resource links, not an automatic recipe extractor. The consumer must
also bind actually consumed component subjects/claims in its admitted operation
parameters/consumption policy. Adding or changing evidence can change a job root
without changing a producer recipe; do not infer output freshness from that root.
