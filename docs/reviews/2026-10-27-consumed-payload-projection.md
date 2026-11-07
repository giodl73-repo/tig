# Consumed payload projection producer review

Public summary: consumer identifiers generalized; original notes retained privately.

Entry81965c724085ff2a09126d84b9be1cf6e6c0e1ca. Pre-code controls, scope and
L0/L1/L2 are in specs/CONSUMED-PAYLOAD-PROJECTION.md. No dependencies or production
cache/media operations. This is producer-applied six-role review, not independent
or human acceptance. Owner: Tig producer.

- Identity: every admitted definition has an explicit unique exact-pin payload
  consumption policy. Requested selected/named stream bindings must be captured
  structurally, including role/group predicates. Same bytes in an uncaptured
  stream cannot satisfy admission. Qualify consumed streams in original order
  with slot/role/group/object. New domain-tagged projection and consumption
  identities deliberately differ from legacy identities.
- Git/adoption: no record format or authored inventory changes. Existing plan
  calls retain their policy/identity contract. Full graph vertices, edges,
  requirements and checkpoint/binding/admission remain unchanged. Captured raw
  controls remain required even when excluded from a producer's byte fingerprint.
- Recovery: explicit original projected-policy claim replay, with no self-admission
  or legacy fallback. Context/job requirements retain original controls through
  the existing metadata provider. Candidate finish/readback remains separate
  from owner adoption; no complete global claim coverage or byte custody follows.
- Privacy/provider: bounded slots/definitions/fields and exact original source
  lookup; errors contain categories, not payload values. Payload-policy Debug
  exposes only selected/count, not definition IDs or private slot labels.
- Placement: consumed identity is not retained demand. All graph capture
  requirements still apply. This narrows neither roots nor source/job ownership
  and cannot mark an ignored control as surplus or authorize eviction.
- Custody/producer: excluding raw control bytes requires a separately qualified
  producer semantic projection in fields/operation parameters. Tig cannot prove
  that extraction or actual execution. Native authoring fingerprints alone omit
  delivery/profile/request/runtime meaning. Actual consumer adoption stays open.

Initial evidence: five pure cases and one native Git/store/context/job/claim case
pass. Additional checks cover group changes, output-as-primary rejection and
legacy/projected replay in both directions. The initial fixture helper referenced
an unavailable error alias; corrected to the public tig_core::identity::Error
without a production API change. Final full regression/qualification follows.

Learning: object-set subset checks lose slot/role semantics when two streams share
one Object. Check selector membership structurally before hashing exact bound
stream tuples. Omission from a recipe is not omission from retention. A native
replay function must select its admission mode explicitly, rather than accepting
whatever mode makes an unsigned claim appear valid.

Disposition: pass_with_risk for this explicit mechanism pending final local and
exact-head hosted checks. Native scoped producer/context integration, generated
input freshness, all-family/history migration, global lifecycle/custody/eviction
and reversible consumer project cutover remain OPEN.

Final full workspace all-feature run passes555 entries, including all70 native
source entries and six new pure cases. The native fixture verifies both directions
of legacy/projected refusal and succeeds in each admitted mode without Git.
The empty-payload case still refuses an output before an extra source lookup.
All existing schema/variant/Git/LFS/provider/restart/500,000-binding tests remain
enabled and pass. This is full regression for this mechanism, not full readiness.

Final workspace all-feature/all-target warnings-denied Clippy, standalone Git/LFS
check and format pass. Builds use compact temporary D: scratch (two Cargo jobs,
debug symbols disabled, incremental off; assertions/tests unchanged). No Rust
production source changed after the full run; subsequent formatting affected test
layout only. Structural/role verification and exact-head hosted checks qualify
publication separately. No private source/runtime or cache bindings changed.
