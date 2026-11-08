# Managed local replica retirement and explicit reacquisition

Input fcb7009f82b725e16e35eb1f2d1a8deaebe328e1;
WP-003/004/006/007/008/009;
REQ-001/002/003/004/005/006/007/009; SPEC-002/003/004/005/006/008/009;
IF-002/003/004/005/006/008/012; PKG-002/003; RUST/PROVIDER profiles.
Allowed: new synthetic local managed stores, existing publication/ledger/retention
and all-namespace fences, actual scoped replica unlink and explicit reacquisition.
Entry: fixed store protection roster, mandatory namespace admission, scoped readers
and durable retained replicas. Exit: intent before unlink, exact-incarnation replay,
independent retained custody through removal, process-restart recovery and real
restore. No final-copy deletion, migration of existing roots or live adoption.

## Frozen execution contract

Explicit fresh local design-7 stores require a protection binding and bounded
per-object incarnation history. Old readers reject the marker. Design-5/6 bytes
and behavior stay unchanged. Managed publication uses the existing accounted path;
bare generic mutation refuses before allocating an anchor. Publication intent and
completion bind their exact incarnation. A missing acknowledged active object is
still an integrity incident. A completed authorized retirement produces distinct
authorized absence; old publish/recover cannot recreate it or attach old receipts
to a new same-hash placement. Immutable bytes retain the same content identity.

Each object has at most sixteen immutable births, retirement intents and retirement
completions/abort certificates. Cancellation advances an action slot while preserving
the physical incarnation and original publication identities; reacquisition advances
both. At most 65 exact-object files are inspected. Names and records are strictly
bounded/validated; malformed, incomplete
or exhausted histories refuse new destructive work. No prefix is reset or forgotten.
Normal operations visit only that object's bounded metadata, not a store census.
Continuation/metadata expiry remains a separate required lifetime milestone.
Known missing anchors/history are not silently re-created. An interrupted initial
unacknowledged bootstrap can require explicit owner recovery rather than treating
an unexplained existing anchor as a fresh store/object.

Replica retirement binds owner/operation, actual enrollment, exact publication and
incarnation, admitted source context, survivor grants/domain enrollment and required
coverage. Explicit authorization is scoped to that immutable plan; it is not final-
copy deletion authority. Metadata allocation through the existing capacity journal
precedes writes and reserves intent/completion/recovery space. Provider object and
all namespace subject guards remain held through fresh source verification, keeper
revalidation and unlink. Required independent retained domains survive the step;
aliases, stale/missing/changed custody and any surviving source demand hold removal.
No SQL transaction spans tree or byte IO; unrelated objects remain concurrent.

Versioned plans bind the capacity authority and sort keepers by store identity.
Execution compares actual private-profile context and domain enrollments, not
domain assertions supplied solely by a portable plan. A scope token is not
authentication or human approval; trusted operator/policy execution and private
profiles remain the boundary. Publication acknowledgment does not create a permanent
proximity claim: clients must admit their required use.

Intent is immutable and synced before unlink. Interrupted intent blocks new
publication until exact recovery. If bytes remain, retry repeats all destructive
eligibility/custody checks. If exact intent exists and bytes are already absent,
recovery can acknowledge that past effect without another unlink. A completed
old operation replays only its historical outcome, including after reacquisition;
it must never touch a later incarnation. Changed plan/job under the same operation
refuses. Cancellation/failed preconditions preserve bytes and recoverable debt.
A settled journal with missing terminal provider evidence refuses repeat destructive
IO. Pending retirement with bytes present permits guarded reads. Owner cancellation
verifies those bytes, records abort before successor metadata, and preserves old
publication receipts. It cannot cancel already absent bytes; old aborted operations
cannot become fresh deletion permission when demand changes again.

Explicit new-operation reacquisition binds the exact retired receipt and next
birth. It admits metadata and payload capacity before their effects, uses existing
streamed hash/length verification and no-clobber publication, and cannot reuse a
spent publication ID. Crash/retry resumes the same birth and allocation. New
publication-attempt bindings are validated before birth, including name collisions.
New desired
demand may precede reacquisition; source admission is not a byte-availability ack.
No automatic resurrection follows from Git revert or old retry.

Report namespace unlink and logical bytes separately from the pre-unlink file-data
allocation and historical retry. No predicted reclaim credits capacity. Provider
snapshots/trash/aliases and concurrent volume changes prevent interpreting unlink
as secure erasure or measured whole-volume free space. Fresh capacity observation
remains separate. Durability is local cooperative process restart only; latest-
authority/full-restore, power-loss, hostile FS, dynamic epoch handoff and private
consumer admission remain unqualified. The 8 MiB reference limit remains.

L0 validate/diff/fmt and six producer lenses; L1 locked focused/default/all-feature
tests, strict Clippy and standalone Git; L2 actual temporary unlink/restore,
reader/claim/retention/alias refusals, wrong authorization/operation/instance,
same-hash new incarnation versus old retries, exhausted/corrupt history, missing
custody and real child exits before/after unlink/completion/reacquisition.
Implementation and verification/validation/trace/review close separately. These
fixtures do not qualify bulk final deletion, automatic Git actions or live stores.
