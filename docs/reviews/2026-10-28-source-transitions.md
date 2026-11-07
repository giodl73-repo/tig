# Explicit source transition preview and application

Public summary: consumer identifiers generalized; original notes retained privately.

Entry 2746e7334099db1d7c4c7a9e69d9e75c52492c58. WP-008/009/015;
REQ-002/004/007/009, SPEC-005/006/008, IF-004/006/008, PKG-003.
Scope: reusable read-only command preview plus explicit CLI plan/apply of existing
prepare/adopt/abort/withdraw/retire metadata transitions. No payload operation,
source discovery, ownership inference, daemon, runtime install or hook creation.

Preview must use the same canonical request, idempotency, ownership, revision and
growth checks as apply, in a coherent read transaction. Proposed state is not a
committed receipt. A recorded request returns its original historical receipt.
Apply consumes a bounded private plan with exact request hash and observed
checkpoint, revalidates current state, saves a new private acknowledgment request
before mutation, and then uses the existing journal transaction. Never hold a
database transaction across output-file IO. A failed receipt publication can be
reconciled using that saved request; unknown commands are not automatically run.

Plan freshness is source-CAS and trusted-prefix based, not a global journal lock:
unrelated sources may progress. The CLI is an explicit trusted-caller interface;
owner strings and self-hashes are not authentication. Retirement withdraws one
source obligation only, never establishes complete demand or byte custody.

L0 structure/fmt/locked all-target all-feature Clippy. L1 preview read-only,
historical retry, changed/unknown/stale/owner/budget refusals, plan tamper and
no-overwrite/private output checks. L2 full source-journal regression, synthetic
CLI lifecycle with another source retaining old roots, interrupted acknowledgment
and hosted Windows/Linux. Producer concurrency/recovery, provider/privacy and
Git/adoption review; independent/human acceptance and full consumer adoption open.

Focused outcome: two API cases pass (1.04 seconds), three initial CLI cases pass
(4.19 seconds), followed by the complete six CLI cases (3.35 seconds). The latter
adds deterministic busy refusal with saved-but-uncommitted intent, committed
prepare with failed receipt hard-link publication and exact recovery, and rollback
to a state accepted by the original floor but rejected by the observed plan floor.
The lifecycle exercises all five command kinds, main replacing A with B while a
branch retains A, explicit predecessor retirement, exact retry and acknowledgment
from the automatically saved request. Proposed-state tampering refuses before
intent; repeated exact recorded requests remain historical. All existing four
inspector/acknowledgment CLI cases pass. Locked all-target/all-feature workspace
Clippy passes (final 0.50 seconds). Full workspace regression is running.

Producer disposition: pass_with_risk, metadata transitions only. Concurrency/
recovery confirms short transactions, source CAS, no global-head lock, and intent
versus completion distinction. Provider/privacy confirms bounded strict profiles,
private results, no-overwrite outputs and cleaned owned temps on handled errors.
Git/adoption confirms Git/ref decisions and byte/root-closure protection remain
caller responsibilities. High/open, integration owner: full job/source coverage,
compaction, independent custody, guarded eviction, hooks and runtime adoption.

Actual private consumer preview at candidate 9574f8a: the completed consumer project
rehearsal source is revision 4 at journal sequence 12. An explicit withdraw
preview proposes revision 5, current root becoming a second predecessor. No
transition was applied. Database/marker SHA-256 and authority file count are
identical before/after, and executable hash is unchanged. Private request/plan/
proof remain outside Git. This proves the real metadata preview path, not a
decision to withdraw the source, actual handoff completion or cache retirement.

The first full workspace run stopped in an existing retention coverage test:
its source replica was verified before a second durable fixture was created,
so the five-second observation correctly became stale under parallel setup.
Move verification after setup in that test and the analogous invalidation test.
Production replica freshness/guards are unchanged. All three focused retained
coverage/invalidation tests pass (3.48 seconds), with final Clippy passing
(2.67 seconds). A fresh full workspace run is in progress after this test-only
correction; the earlier failed run is not counted as qualification.
