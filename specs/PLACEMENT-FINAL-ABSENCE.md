# Completed final absence in ordinary placement

Input81ecc27eba6289f2639174c7872be3c53583f5ee (PR54, local focused qualification;
broad/hosted checks remain running at entry). Full PRODUCT_PLAN stays active.
This contract precedes runtime changes in feat/placement-final-absence.

WP-003/004/006/007/008/009/013/015/016;
REQ-001/002/003/004/005/006/007/009/010;
SPEC-002/003/004/005/006/008/009/013/014;
IF-002/003/004/005/006/008/010/011/012/013;
PKG-002/003/004/006/007/008/009. RUST/GIT/PROVIDER/INTEGRATION profiles.

Allowed: a narrowly scoped private placement observation/helper and no-effect
handling for an unrequired cache whose exact current birth has a validated,
completed, settled final deletion and physically absent payload. Reuse existing
LocalStore lifecycle validation and final-provider status/capacity/job checks.
Use existing AuthorizedAbsence observation; add synthetic SDK counterexamples
and compatibility regressions. No new public execution authority, cache engine,
provider/event/journal format, dependency, automatic reacquisition, global lock,
live root, provider migration, expiry or claim-release/removal path.

Entry: read source/placement/final-provider/reacquisition/status/capacity contracts,
engineering router/pulse, pitfalls and six role checklists. Current placement's
recipe returns FinalRetirement for every final_state. In a three-role profile,
explicit reacquisition into M can therefore leave an unused, deleted D copy
holding global convergence even when current F/M requirements are satisfied.
Distinguish read-only knowledge of absence from permission to create/delete bytes.

Exit: after separately authorized exact-byte new M acquisition and ordinary F
restoration, an unrequired completed-absent D contributes AuthorizedAbsence and
no storage effect. Actual required current copies can converge without creating
an unwanted D copy or clearing its final tombstone. Required final-deleted stores
still report FinalRetirement until their own explicit final reacquisition.
Unknown/offline/corrupt/incomplete/unsettled or unexpectedly present states hold.
Old deletion retries remain historical and cannot remove new births.

## Proof boundary

A candidate may recognize completed absence only through the admitted store's
validated current lifecycle head, its exact immutable final provider plan,
matching capacity identity and original job/allocation, completed Deleted phase,
released grants and settled metadata charge. Independently inspect current payload
presence. No absent path alone, old terminal file alone, spent operation, declared
label or claim withdrawal is sufficient. Reuse the final status API; do not
reimplement private final-operation hash recipes or ledger binding derivations.

No clock sample or fresh final-loss authorization is needed to read a completed
historical outcome. That read grants no removal/claim-release permission. Preserve
all currently enforced provider/root/generation/policy/job/capacity checks and
known independent rollback minima. This bounded result does not solve whole-store
restore/latest-authority adoption, foreign peers or metadata expiry.

Active final intent, partial grant release, unlink before terminal, terminal
before settled accounting, cancellation/present bytes, corrupt/missing records,
wrong ledger/job, stale known prefix and provider unavailability must remain held.
A final terminal that claims deletion while bytes are present is recovery-required,
never a fresh deletion opportunity. A new explicit birth replaces current-head
observation normally; do not project an old terminal over it.

Observation may report AuthorizedAbsence for a completed deleted birth. It is
satisfied only when that row does not require a copy there. Required cache or
retained floors do not count absence as available or retained custody. Existing
activate/publication paths keep their FinalRetirement guard. Skipping an already
absent unrequired cache performs zero publication/retention/handoff/retirement,
source mutation or ledger append; it does not require eviction permission.
Revalidate final status for each actual observation; do not memoize hash-wide
absence across a later birth. Scoped provider guards remain effective. A racing
new copy can make status nonconverged; a no-effect skip can never unlink it.

## Independent fixture and regression matrix

Use public deterministic bytes and three explicitly created enrolled synthetic
providers with the same admitted namespace and policy. Complete final deletion
of an old historical D copy and the old M copy under existing exact authority.
Select the same object again: ordinary enabled copy/reacquisition still cannot
cross M's final fence, even if a public seed is available. Then explicitly create
and retain a new M birth. With seed access disabled, ordinary reconciliation may
restore F using actual verified M bytes and must leave D absent. Independently
read literal F/M bytes, confirm D absence and verify exact new retention.

Check status is read-only by comparing source/capacity/provider state before and
after independent inspection. Repeat the same plan with zero streamed bytes and
zero new effects. Replay the old deletion(s), including after their old window:
zero new loss; current F/M bytes/claims/checkpoints unchanged. No new D birth,
new D grant, source mutation, hidden inventory reset or tombstone deletion.

Counterexamples must cover required D demand, missing seed before explicit M
acquisition, partial final stages, physically present terminal, corrupt/missing
proof, wrong job/capacity, unsettled accounting, stale known prefixes and a fresh
same-hash D birth. Existing legacy/ordinary retirement and final-capable old-writer
refusal remain unchanged. Include actual process/reopen coverage at existing
final-provider boundaries where a terminal/settlement distinction matters.

L0: contract/source pins, file/link/structure/fmt/diff. L1: locked focused SDK
positive/refusal/interruption cases and strict Clippy. L2: actual synthetic SDK
copy/final/reacquire/replay with independent bytes/checkpoints; all existing15
teaching modes' cases and shipped controls; both workspace/500k/strict profiles;
standalone Git; fresh persisted artifact; exact-head Windows/Linux/secrets and
pinned baseline writer/source compatibility. No live/browser/human acceptance.

Apply all six producer lenses; record target/control identity, severity, owner,
disposition and evidence. Close implementation, verification, synthetic validation,
trace and producer review separately. Complete joined public teaching export,
provider/current authority/large-file/lifetime/metadata expiry and full product/
human acceptance remain OPEN. This slice cannot waive those requirements.
