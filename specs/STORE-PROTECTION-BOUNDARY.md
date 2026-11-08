# Store-bound protection assessment

Input 83d3ec950e3145f0d0a8b358ff4480dd943aafe8.
WP-003/004/006/008/009; REQ-001/002/003/004/005/006/007/009;
SPEC-002/003/004/005/006/008/009; IF-002/003/004/005/006/008/012;
PKG-002/003. RUST/PROVIDER profiles. Allowed scope: explicit synthetic local
stores, existing guarded namespaces/provider fences and bounded assessment.
Entry: mandatory per-subject namespace admission and existing local read/retention
guards. Exit: exact store/roster/generation admission, every enrolled namespace
assessed under held fences, local readers/retention respected, unknown coverage
refused, unrelated subjects concurrent. No physical removal or consumer adoption.

## Binding and admission

An explicitly created local design-6 store pins an immutable protection binding:
nonzero enrollment generation, admitted external-writer/pruner boundary identity,
and 1..16 distinct guarded namespace authority identities, each with 1..16 distinct
namespace-local place keys assigned to this provider. Multiple place aliases and
multiple projects can protect the same physical copy. Required placement is the
union after each namespace's admitted precedence. A namespace not in the roster
has no implicit mapping to this provider; it cannot be silently substituted for
an enrolled member. The boundary identity is a trusted enrollment assertion, not
authentication, unmanaged-client discovery or proof about raw filesystem actors.

Legacy design-5 store bytes/behavior remain unchanged. Older readers reject the
new marker. A new reader checks the exact schema/binding and cached instance on
every provider guard. No existing root is migrated or scanned. Binding order is
normalized; duplicates, empty rosters, zero generations and excess bounds refuse
before creating roots. Portable Enrollment binds store instance, generation and
canonical binding identity; deserializing it never creates a live assessment.

All concrete callers must use the store's admitted roster, not a supplied subset
or a caller assertion of complete coverage. Assessment requires exactly those
guarded namespace instances, validates each place against actual immutable policy,
then acquires all subject fences in authority order and the existing local object's
exclusive guard. No new object/attempt/retention anchor is created for inspection.
Missing anchors or missing/unavailable namespaces refuse without repair. Local
retention records, active readers, publishers and retention installers conflict
through the existing provider fence. No SQL transaction spans metadata traversal.

Every namespace is resolved with one shared root/binding/association/read/byte/time
budget, including alias mappings; a second namespace does not reset those limits.
Missing/corrupt flat-root closure, size conflicts and stale/replaced markers refuse.
The owned assessment keeps every namespace and provider guard alive; it is neither
Clone nor serializable. A bounded summary lists exact checked authorities and
checkpoints. Revalidation checks actual store/namespace markers and enrollment;
unrelated journal heads may advance because all relevant subject changes remain
fenced. No global cache lock, census, tree inventory in Git or second cache engine.

## Deliberately open execution gates

The first binding is immutable. Changing namespace membership/policy/epoch requires
an explicit acknowledged handoff protocol, including old-executor fencing and
surviving demands; this slice offers no marker-replacement or migration API.
Immutable binding is a safe initial enrollment, not an implementation of handoff.
Full restore with an unchanged old marker still needs latest-authority recovery.

Store-bound absence does not establish independently retained custody, authorized
absence/incarnation replay, durable deletion intent or final-deletion permission.
The assessment reports removal authority false. These gates remain necessary
before physical eviction; batch final deletion and metadata forgetting retain
their separate roadmap contracts. Provider contract/acknowledgment remains local
process-restart only, 8 MiB reference objects; no power-loss or hostile-FS claim.

L0: validate/diff/fmt, reviewed hashes and all six producer lenses. L1: focused
multi-namespace/alias/invalid/stale/budget/legacy vectors, locked default/all-feature
workspace tests, strict all-target Clippy and standalone Git independence. L2:
actual temporary store/journals and reader/claim contention, process exit/restart,
missing authority/anchor, wrong provider instance and marker replacement, unrelated
progress and aggregate budgets. Keep verification/validation/trace/review separate
from implementation and distinguish local tests from hosted Windows/Linux checks.
