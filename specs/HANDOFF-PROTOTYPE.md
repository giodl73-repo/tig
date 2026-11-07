# Pure branch-to-main handoff prototype

Status: experimental Rust mechanism, not a released wire format or storage engine.
Scope: TIG-R02/R03/R04/R05/R09; selected T01-T03/T08/T12/T24 and CL06/GW12
mechanisms. Full scenarios and requirements remain open.

## Minimal contract

`Object` is a caller-supplied SHA-256 value and byte size, not a hash computed by
this crate. `Placement` pairs it with an opaque place ID. `Claim` is an opaque ID
and a deduplicated assigned placement set; role constraints must already have
been resolved by an admitted policy. IDs are bounded tokens, not parsed filesystem
paths. A `View` has a generation, asserted claim-coverage flag and claims.

`View::transition` projects one replacement/retirement batch against an expected
generation. It returns a new view or an error without mutating the old view.
Unknown retirements, duplicate/conflicting batch IDs, duplicate claim IDs,
inconsistent sizes for one digest, size/generation overflow and budgets reject.
The caller owns authentication, trusted source/coverage, durable commit, retry
identity and a real compare-and-swap. This pure function is not durable atomicity
or a distributed fence. Generation is a fixture snapshot token, not a global lock.

`View::plan` compares the union of required placements with a supplied set of
claimed verified copies. It returns acquisitions, retained placements, held
surplus and release candidates. It counts each object once per place, regardless
of how many claims use it. Logical required bytes are not measured allocated
bytes, scratch admission or a custody-domain count.

A surplus copy becomes a candidate only when claim coverage is asserted complete,
there is a remaining retention obligation for its object, and all assigned
required copies of that object are supplied as verified. Missing obligations,
unknown coverage or missing required copies hold it. This is deliberately
conservative. Unregistered objects need an admitted archive/retention policy;
they are not deleted as garbage.

Candidates never authorize deletion. No independent-custody evaluation,
observation integrity/freshness, reader fencing, expiry, constraint solver,
capacity scheduling, native extraction or filesystem/provider action exists.

## Concrete handoff walkthrough

| Point | Logical claims | Physical effect proposed by the pure model |
| --- | --- | --- |
| Main V1 | current uses A+B at working | Existing copies retained |
| Branch draft | scratch uses A+C at working | Acquire C only; A is shared |
| Merge accepted | current uses A+C; history retains B; scratch retired in same desired projection | Reuse C; request B at history; hold B at working until supplied history verification |
| History satisfied | Same desired claims | B at working becomes a release candidate |
| Other branch/job uses B at working | Additional independent claim | Keep B at working even though main no longer uses it |

Scratch is a lifecycle claim/bundle, not a mandatory physical place. A separate
scratch disk can be explicitly configured; the fixture then holds its copy until
required working placement is present. Retiring scratch before its desired-state
handoff is durable remains forbidden in a real backend.

## Art adapter readiness

Art passes exact selected result/variant objects, resolved bundle membership,
admitted placement policy and root coverage. Tig does not reconstruct a semantic
closure from filenames. A schematic Art record with `selected: null` may describe
planning but cannot supply a playable object. Empty core inputs do not prove that
native embedded references have been extracted completely.

Before a real consumer handoff, add a selected-result-to-object/bundle adapter,
exact byte sizes and representations, native-reference coverage, stable claim
owner/source identity and trusted observations. Missing pieces remain explicit
planning gaps; they must never produce empty demand or removal permission. This
prototype tests the receiving placement algebra, not an actual Art integration.

## Evidence and limits

Thirteen tests in [handoff.rs](../crates/tig-core/tests/handoff.rs) execute the
pure mechanism, including shared branch/job protection, archive pending, separate
scratch place, stale transitions, unknown coverage, unregistered objects, size
conflicts and reversion. See [verification](../docs/vtrace/VERIFICATION.md).
The shared-object test also models 1,000 current objects with five replacements:
only five new working placements and five history placements are requested, then
only the five replaced working copies become candidates. This measures set
behavior, not bounded-index performance, scratch use or real reclaimed space.
All 110 broad design cases remain designed; these tests do not close them.

Next: freeze trusted claim-source/recovery contract and fake-observation/receipt
semantics; evaluate backend fit; exercise a fake journal/provider before real IO.
