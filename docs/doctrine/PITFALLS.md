# Tig pitfalls

Failure memory from scenario design review. Countermeasures are planned, not
implemented full-system fixes. See [mitigations](../scenarios/MITIGATIONS.md).

Publication refinement (WP-010/011/015): a target may retain revision A while
the source selects B or has no built selection. Do not overwrite B to make A's
receipt pass, infer a publication from a filename, or treat a different selected
subject as eviction authority. An admitted policy must choose the exact retained
stream and report its equality to current selection separately. Receipt binding
still establishes neither current remote state nor producer currency. Component
evidence and remaining acceptance belong to the publication-subject review.

## TIG-F-01: Membership mistaken for deletion permission

**Pattern:** A dropped label or branch is treated as proof that bytes are unused.

**Domain:** Claim reconciliation and eviction.

**Detection difficulty:** High when relying only on happy-path or set-level tests.

**Why it matters:** Data still needed by another consumer or retained release can be lost.

**Evidence:** T02/T03/RW04/LF06/SP10/LC07; designed only.

**Countermeasure:** M01: Union all admitted obligations, resolve new desired state durably, and revalidate current protection/custody before release.

**Owner:** placement-policy.

**Severity:** critical.

**Principles:** TIG-P-03/TIG-P-04.

**Invariants:** TIG-I-01/TIG-I-02.

**Status:** OPEN

## TIG-F-02: Logical savings mistaken for free disk space

**Pattern:** Logical sizes hide duplicate materializations, sparse allocation, staging or pack pinning.

**Domain:** Capacity and physical accounting.

**Detection difficulty:** High when relying only on happy-path or set-level tests.

**Why it matters:** A plan can exhaust the disk before it achieves any savings.

**Evidence:** T01/T15/CL09/GW14/RW03/SP01/SP08/SP09/LC09; designed only.

**Countermeasure:** M02: Budget peak allocated bytes and bounded scratch, reserve against concurrent writers, and measure completed physical reclaim.

Refinement: capacity aliases share one budget. Reconcile physically allocated
bytes with remaining unallocated reservations without double charging or a race
that counts neither. Idle worktrees do not reserve every possible job maximum.
Track materializations, operational metadata and unknown scratch separately from
new managed payloads; projected reclaim is not available capacity.

Bounded multi-leaf work must share one IO budget across all lookups and edits.
Reads that end in a conflict or failure still consumed work; resetting counters
per leaf or ignoring failed-route reads can evade the operation limit.

**Owner:** storage-custody.

**Severity:** major.

**Principles:** TIG-P-08.

**Invariants:** TIG-I-04/TIG-I-10.

**Status:** OPEN

## TIG-F-03: Derived catalog mistaken for durable protection

**Pattern:** Rebuilding an index discards uncommitted admission, unfinished intents or active-job protection.

**Domain:** Recovery and ingest.

**Detection difficulty:** High when relying only on happy-path or set-level tests.

**Why it matters:** Complete content can become falsely unreferenced after restart or backup restore.

**Evidence:** T14/CL05/GW03/GW04/GW05/CR01/CR02/CR03/CR04/CR10/SP05/LC01; designed only.

**Countermeasure:** M03: Persist protection and operation identities separately from indexes; recover/reconcile before any protection shrink.

A completion record may survive independently of intent after a partial restore.
Check its exact request/owner before recreating intent; never let retry rebind an
acknowledged operation. Missing acknowledged bytes are an integrity incident,
not an ordinary miss. Process restart, power loss and complete journal restore
are separate proof scopes; file sync alone cannot collapse them.

An intact local chain or minimum checkpoint does not prove that no newer unseen
acknowledgement exists. Backup restore needs admitted witness reconciliation.
Persisted sample replay is historical evidence, not renewed admission permission;
uncertain commit results must not leave a writable stale in-memory authority.

**Owner:** concurrency-recovery.

**Severity:** critical.

**Principles:** TIG-P-05.

**Invariants:** TIG-I-05/TIG-I-06.

**Status:** OPEN

## TIG-F-04: Stale observation mistaken for a removal fence

**Pattern:** A planner checks once, then removes while a reader or new claim arrives.

**Domain:** Concurrency and active reads.

**Detection difficulty:** High when relying only on happy-path or set-level tests.

**Why it matters:** A valid consumer can lose its only usable copy.

**Evidence:** T12/T13/CL04/CL06/CR05/RW01/RW02/RW07/RW09; designed only.

**Countermeasure:** M04: Use object/place-scoped acquire/read/remove fencing, durable operation IDs and generation-aware retries; preserve completed useful copies.

End owned guards with explicit unlock before handle close: duplicated/inherited
handles can otherwise delay release. Never unlink an anchor to force progress;
independently acquired guards and durable claims retain their own obligations.

**Owner:** concurrency-recovery.

**Severity:** critical.

**Principles:** TIG-P-06.

**Invariants:** TIG-I-03/TIG-I-07.

**Status:** OPEN

## TIG-F-05: Expiry mistaken for permission

**Pattern:** Wall-clock expiry or an expired lease is used as automatic proof that jobs and retention have ended.

**Domain:** Authority and clocks.

**Detection difficulty:** High when relying only on happy-path or set-level tests.

**Why it matters:** Clock skew, lost renewal or revoked authority can create unsafe removal.

**Evidence:** T24/CR06/CR07/RW06/RW10; designed only.

**Countermeasure:** M05: Bind expiry to an admitted owner and clock/renewal policy; revalidate active-use authority and other obligations before release.

**Owner:** placement-policy.

**Severity:** critical.

**Principles:** TIG-P-03/TIG-P-06.

**Invariants:** TIG-I-01/TIG-I-07/TIG-I-11.

**Status:** OPEN

## TIG-F-06: Path or alias mistaken for immutable identity

**Pattern:** A reused path, case-colliding name or changed mount is assumed to denote the same object/place.

**Domain:** Namespace and place enrollment.

**Detection difficulty:** High when relying only on happy-path or set-level tests.

**Why it matters:** Wrong content can satisfy a request or one device can count as a different backup.

**Evidence:** T04/T05/CR09/SP06/BN03/BN05/LC10; designed only.

**Countermeasure:** M06: Resolve exact snapshot/path under declared normalization, pin stable place/object identities, preserve explicit removal entries in overlays, and revalidate mount identity before actions.

**Owner:** identity-namespace.

**Severity:** major.

**Principles:** TIG-P-02/TIG-P-04.

**Invariants:** TIG-I-08/TIG-I-09.

**Status:** OPEN

## TIG-F-07: Availability hint mistaken for verified custody

**Pattern:** An old row, successful download or peer count stands in for current independent retained bytes.

**Domain:** Cache indexes and observations.

**Detection difficulty:** High when relying only on happy-path or set-level tests.

**Why it matters:** Corrupt, offline or same-domain replicas can falsely authorize eviction.

**Evidence:** T07/T08/T09/T10/T23/CL02/CL03/GW12/SP07/BN07/BN08/LC02/LC05; designed only.

**Countermeasure:** M07: Carry integrity, coverage, freshness, retention and failure-domain provenance; treat offline/unobserved states explicitly and recheck before removal.

**Owner:** storage-custody.

**Severity:** critical.

**Principles:** TIG-P-01/TIG-P-04.

**Invariants:** TIG-I-01/TIG-I-02/TIG-I-06.

**Status:** OPEN

## TIG-F-08: LFS operation mistaken for lifecycle ownership

**Pattern:** An OID request or local completion implies a branch, cloud upload, transformed identity or safe pruning.

**Domain:** Git LFS protocol and materialization.

**Detection difficulty:** High when relying only on happy-path or set-level tests.

**Why it matters:** Tig can misclassify new objects, report false upload success or hide worktree copies.

**Evidence:** CL07/CL08/GW01/GW06/GW07/GW11/LF01/LF02/LF04/LF05/LC04; designed only.

**Countermeasure:** M08: Keep standard pointer/client semantics; derive lifecycle from exact admitted Git views, verify destination completion, and account for each LFS/materialized place.

**Owner:** git-adoption.

**Severity:** major.

**Principles:** TIG-P-07/TIG-P-08.

**Invariants:** TIG-I-06/TIG-I-11/TIG-I-12.

**Status:** OPEN

## TIG-F-09: Provider convenience mistaken for trusted authority

**Pattern:** A repository, torrent or shared catalog selects arbitrary paths, execution or private data access.

**Domain:** Adapters, privacy and permissions.

**Detection difficulty:** High when relying only on happy-path or set-level tests.

**Why it matters:** Credentials, private names or payloads can leak; unsupported capabilities can masquerade as safety.

**Evidence:** T16/T21/T25/GW02/RW05/RW08/LF03/LF10/BN06; designed only.

**Countermeasure:** M09: Use trusted local enrollment, capability/access checks, redacted observations and explicit authorized operations; reject executable/path/provider substitutions.

**Owner:** provider-privacy.

**Severity:** critical.

**Principles:** TIG-P-07.

**Invariants:** TIG-I-08/TIG-I-11.

**Status:** OPEN

## TIG-F-10: Content addressing mistaken for bounded work

**Pattern:** A valid-looking root expands into unbounded, cyclic or repeatedly scanned metadata.

**Domain:** Indexes and graph traversal.

**Detection difficulty:** High when relying only on happy-path or set-level tests.

**Why it matters:** Lookup can consume excessive memory/IO and routine work can become a cache census.

**Evidence:** T17/CL01/BN01/BN02; designed only.

**Countermeasure:** M10: Enforce explicit node/edge/depth/byte/read budgets, exact node validation and bounded stable partitions; report incomplete coverage on exhaustion.

**Owner:** identity-namespace.

**Severity:** major.

**Principles:** TIG-P-04/TIG-P-08.

**Invariants:** TIG-I-02/TIG-I-10.

**Status:** OPEN

## TIG-F-11: Logical objects mistaken for physical representations

**Pattern:** An object digest is confused with a pack/torrent root, or key rotation silently changes its logical identity.

**Domain:** Packs, encryption and transfer shards.

**Detection difficulty:** High when relying only on happy-path or set-level tests.

**Why it matters:** Reuse, integrity and physical reclamation calculations become false.

**Evidence:** T18/T22/GW13/SP02/SP03/SP04/BN09; designed only.

**Countermeasure:** M11: Pin object-to-representation maps, exact keys/capabilities and immutable shard manifests; budget copy/verify/retire compaction separately.

**Owner:** provider-privacy.

**Severity:** major.

**Principles:** TIG-P-02/TIG-P-08.

**Invariants:** TIG-I-04/TIG-I-09/TIG-I-10.

**Status:** OPEN

## TIG-F-12: Events mistaken for authoritative desired state

**Pattern:** A missed hook, renamed branch or later writer replaces obligations without a complete trusted checkpoint.

**Domain:** Branch transitions and policy.

**Detection difficulty:** High when relying only on happy-path or set-level tests.

**Why it matters:** Claims can disappear or policy can be weakened by an untrusted candidate.

**Evidence:** T06/T11/T19/T20/CL10/GW08/GW09/GW10/GW15/CR08/LF07/LF08/LF09/BN04/BN10/LC03/LC06/LC08; designed only.

**Countermeasure:** M12: Reconcile exact source checkpoints under protected policy, preserve old/new coverage during handoff, and retire only owner-authorized claim generations. Preserve independent authored leaf changes; root-only authoring requires an explicit tested tree merge.

**Owner:** git-adoption.

**Severity:** major.

**Principles:** TIG-P-03/TIG-P-05.

**Invariants:** TIG-I-02/TIG-I-03/TIG-I-11.

**Status:** OPEN

### Accounted publication refinement (TIG-F-02 / M02)

A file-data footprint is not volume growth. Name its basis and account separately
for metadata, ledger, external writers and shared aliases; conservative overcharge
is permitted until a coherent fresh probe. Settled evidence verifies an existing
result; it cannot finance rebuilding lost bytes. An interrupted private partial
retains in-flight capacity and cannot spawn another full copy automatically.
EVID-015 tests these bounded local mitigations; whole-device safety stays OPEN.

### Sampling refinement (TIG-F-02 / TIG-F-06)

Free space must be scoped to an admitted capacity authority and sampled after a
new ticket, using caller-available bytes. Exact operation replay is historical
proof, not a fresh sample. Validate history independently of whether a first
capacity observation exists. Explicit label/instance enrollment does not prove
native volume/quota identity or prevent independent double enrollment. EVID-016
exercises the bounded sampler; native mapping remains an admission obligation.

### Private cleanup refinement (TIG-F-01 / TIG-F-04 / TIG-F-05)

Unacknowledged staging is not automatically disposable by age or lost labels.
Require exact reviewed discard authority and durable cancellation before removal.
A writer must recheck cancellation after acquiring its attempt fence. Missing
files count as an idempotent retry only after the matching cancellation exists.
Retain binding/intent evidence and charge remaining metadata; partial removal is
not final-object eviction or proof of volume reclaim. EVID-017 exercises this
bounded mitigation; full custody/retirement and restore authority stay OPEN.

### Replica evidence refinement (TIG-F-07 / TIG-F-09)

Exact completed bytes plus a held guard prove scoped live integrity, not long-term
retention or physical independence. Saved observations carry historical provenance;
only fresh in-process sessions participate in declared-domain coverage. Duplicate
store aliases cannot inflate counts. Failed revalidation invalidates evidence
without unlocking it. EVID-018 tests this subset; durable custody/eviction stays OPEN.

### Persistent retention refinement (TIG-F-03 / TIG-F-05 / TIG-F-07)

Install a durable claim before acknowledging retained custody; session lifetime
or timeout is not its lifetime. Reserve metadata before writes, bind owner and
operation, and check scoped claims under the same object fence as exclusive
access. Installed-but-unsettled claims still protect bytes. Missing acknowledged
records are incidents, never silently recreated with spent budget. EVID-019 tests
normal restart protection; owned release and authoritative restore stay OPEN.

### Handoff history refinement (TIG-F-03 / TIG-F-04 / TIG-F-07)

Hold a verified successor guard while retiring the exact owned source claim.
Record handoff before metadata removal; missing metadata alone is not a retry.
Released grants cannot count as successors or resurrect from saved receipts.
Recovery of a committed handoff does not repeat its decision after custody moves
onward. EVID-020 tests these local mitigations; payload-removal and restore authority
remain OPEN. EVID-021 prevents small fixtures being mistaken for native scale proof.

### Native reference refinement (TIG-F-01 / TIG-F-09 / TIG-F-10)

A parent file hash alone does not identify snapshot-selected dependencies. Pin
frozen results separately; materialize exact referenced revisions in a declared
binding identity. A retained candidate inventory is not selected build closure,
and a selected result manifest is not automatically all primary payloads.
Admit interpretation externally, preserve gaps, bound callback responses and
edge attempts, and return only scoped unreferenced candidates. EVID-033 tests
these pure mitigations; global ref counts, custody and eviction remain OPEN.
### Windows prefix refinement (TIG-F-09)

An extended drive prefix can report absolute before it is a usable directory
lookup target. Require a complete drive root at enrollment, and start ancestor
opens only once RootDir is present. EVID-046 checks normal/canonical aliases,
three layouts and retained protection; accepting canonical spelling must never
be implemented by weakening sharing or ancestor guards.

### Job snapshot refinement (TIG-F-01 / TIG-F-09 / TIG-F-10)

A source-only context misses producer/runtime and result obligations. Reference
that context in a bounded immutable job envelope; derive its inputs and optional
checked claim outputs instead of copying another editable inventory. Journal
source/operation identities distinguish attempts even when roots/bytes deduplicate.
EVID-047 tests independent peer jobs, main advancement, claimed-output handoff and
reopen. These are positive declared obligations, never complete absence/custody or
permission to remove. Prefix checking must consider nonadjacent sorted names;
lexical neighbors alone miss a,a-b,a/b file/directory conflicts.

### Contention refinement (TIG-F-04 / TIG-F-09)

A short synchronized retry window can reject valid concurrent work; reporting all
ledger failures as Capacity hides the distinction from a full disk. Bound and
stagger only transient retries, always refresh superseded observations, and report
Busy separately from capacity/policy and state-integrity failures. EVID-047 adds
nine-conflict recovery and real parallel LFS checks; no global lock is a remedy.

### Git proposal refinement (TIG-F-03 / TIG-F-06 / TIG-F-12)

A retained producer result is not a permission to overwrite today's record. Bind
the proposed replacement to exact base bytes, create a separate Git commit/ref,
and use Git's normal three-way application. Keep branch/index/worktree changes
out of proposal creation. A proposal ref must use create-only, no-dereference CAS;
a symbolic ref must not redirect writes into a branch. Disable configured ref
transaction hooks for this metadata-only operation. Proposal refs protect small
Git metadata, not media custody. Source/job roots remain retention obligations
until separate reconciled handoff. EVID-048 exercises this bounded seam; complete
source/claim lifecycle and provider safety remain OPEN.

### Catch-up refinement (TIG-F-06 / TIG-F-09 / TIG-F-12)

An unavailable desired plan is blocked, not an unbuilt output. Existing selected
bytes without provenance are unproven, not automatically stale. Freeze one source
checkpoint per assessment, reject duplicate keys and keep foreign claims blocked.
A frozen exact input intentionally ignores later logical-ID revisions; fixtures
for catch-up must distinguish that from snapshot-selected authoring. EVID-050
covers the scoped assessment; no global discovery, execution or custody follows.

### Native evidence-location refinement (TIG-F-06 / TIG-F-09 / TIG-F-12)

Keep evidence discovery in the exact selected record. A parallel editable claim
registry can drift from output selection. Explicit literal metadata keys identify
the admitted location; once an evidence namespace is present, partial/null pins
are malformed rather than missing history. Read exact retained claim bytes and
replay original semantics separately from desired producer changes. EVID-051
qualifies read-only synthetic native triage, not evidence authenticity or custody.

### Schema interpreter refinement (TIG-F-03 / TIG-F-06 / TIG-F-12)

A schema byte pin does not prove its assertions ran. Bind explicit dialect and
engine policy to a successful check; refuse declared-dialect disagreement instead
of silently forcing an interpretation. Disable retrievers independently of Cargo
feature defaults. A runtime regex failure, including beneath negation, must not
become success. Defaults/unknown annotations are not executed authoring or native
rules. EVID-052 tests this optional authored-schema seam, not a general CPU sandbox
or complete asset semantics. Recheck retained records and any expanded snapshot.

### Local contract refinement (TIG-F-03 / TIG-F-06 / TIG-F-12)

A legacy or empty rule map has unknown meaning, not a successful no-op contract.
Require a versioned declaration and explicit admitted dialect before executing
local assertions. Keep authored-only and authored+local checks distinguishable.
In Draft202012, items applies after prefixItems; shared constraints for every
stream must be expressed in each prefix or in an independent allOf/items schema.
EVID-053's first fixture caught that authored-rule omission; correcting the schema
preserves engine semantics. Contract authors need negative cases for every
intended assertion. Local success still proves no external/native/custody facts.

### Variant context refinement (TIG-F-03 / TIG-F-06 / TIG-F-09)

Do not duplicate payload pointers in a variant index: name native stream slots.
Validate the entire project context before consumed-axis projection; otherwise a
misspelled unused dimension disappears. Separate admitted current project space
from an asset's pinned space so adding an unused language does not rewrite neutral
assets. Missing consumed variants still refuse. Preserve full request context as
separate pinned provenance and keep only projected meaning in recipe parameters.
Interpretation/control documents are not primary media; declare/retain their
objects through existing context mechanisms rather than assuming index presence
establishes byte custody or complete job coverage. EVID-054 covers scoped cases.

### Variant retention refinement (TIG-F-03 / TIG-F-06 / TIG-F-12)

A temporary selected stream is not enough to replay why a job chose it. Retain
the index, original/current spaces, full context and externally admitted policy
in an immutable packet linked from the job. Verify coverage per source record:
two owners sharing one audio Object still need two checked witnesses. Payload
deduplication must not conceal missing interpretation. Match each covered node's
primary set exactly to its selection; accepting all language streams defeats
projection. EVID-055 checks these scoped rules, not global protection or eviction.

### Captured versus consumed refinement (TIG-F-03 / TIG-F-06 / TIG-F-09)

A retained control need not be raw-hashed into every producer recipe, but ignoring
its bytes requires qualified semantic extraction elsewhere. Keep full graph
requirements/provenance. Never authorize an uncaptured slot because its Object
matches another slot: role/group/name are part of the admission boundary. Preserve
ordered consumed streams. Explicit original projected-policy replay must not
silently try legacy admission or vice versa. EVID-056 is mechanism evidence, not
complete native extraction, media recoverability or safe eviction.

### Progress versus present bytes refinement (TIG-F-03 / TIG-F-06)

A copied file can precede its logged acknowledgement; an acknowledged file can
later change. Never interpret unlogged completion as absence or logged completion
as present custody. Bind explicit progress inspection to the exact plan and
reverify actual destinations on retry. A torn tail is incomplete evidence, not
a selected file or automatic cleanup target. EVID-069 exercises this boundary;
full disk/power-loss behavior and independent custody remain separate gates.

### A free-space number is not capacity admission (TIG-F-03 / TIG-F-06)

Sampling a convenient ancestor can measure a different filesystem from the
destination. Require explicit root enrollment, check destination ancestry and
refuse unavailable or unexpected volumes. Keep required thresholds, logical
copies, consumer scratch estimates and observed free space separate. No probe
reserves blocks, accounts every allocation or fences unrelated writers. EVID-070
adds scoped preflight evidence; durable admission and quotas stay separate.

### A private filename is not ownership (TIG-F-01 / TIG-F-03 / TIG-F-06)

A matching partial name can belong to another plan, a live writer or an unrelated
creator. List only explicit bounded destination scopes; do not follow links or
infer activity/ownership from names or age. EVID-074 reports unassociated metadata
and unknown physical reclaim, preserving bytes through retry. Actual disposal
still requires authenticated attribution and fresh scoped authority/fencing.

Cleanup guards must be armed only after exclusive creation succeeds. A proposed
random filename does not become owned when `create_new` refuses; dropping a
pre-armed path guard can remove somebody else's existing file. Force a collision
in a synthetic fixture and check the original path and bytes remain. Successful
creation must still clean its own unacknowledged partial on ordinary copy failure.

### Retirement replay and provider restore refinement

A settled allocation does not permit repeating physical removal when provider
intent/completion evidence is missing. Hold the restored prefix for recovery;
do not interpret a missing terminal record as an unstarted action. An old deletion
binds an exact placement incarnation, not every future copy of the same hash.
Cancellation before unlink preserves bytes/publication identity and leaves a
terminal abort; explicit reacquisition advances incarnation. EVID-088 checks actual
unlink, same-hash restore, partial provider-prefix loss and child exits. Full
authority/backup restore, epoch continuation and receipt expiry remain separate.
