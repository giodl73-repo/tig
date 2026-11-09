# Tig controlled specification baseline

Input: 0a9a522, 13 pure handoff tests. `current` means observed experimental
behavior, not released compatibility. `target` is controlled design with open
implementation/validation. `unknown` decisions must be resolved or action-scoped
blocked. No old format or consumer safeguard is deprecated by this baseline.

Native interpretation archive extension to SPEC-005/007/010/011:
[NATIVE-METADATA-ARCHIVE](../../specs/NATIVE-METADATA-ARCHIVE.md), entry 0855e7a,
adds an experimental metadata-only candidate root and exact detached-source
restore. Existing source/job lifecycle and opaque native payload custody remain
open; no current wire format, source selection or runtime profile is replaced.

WP-001 update from input 1bbfcc6: SPEC-002 has a verified experimental pure
identity/tree implementation, specified in
[IDENTITY-AND-TREES](../../specs/IDENTITY-AND-TREES.md). Its broader representation
and provider contract remains target. Current workspace total: 154 test entries.
Released encoding, materialization and payload custody remain open. WP-002 adds
a verified pure subset of SPEC-006: isolated bindings, exact candidates and
three-way adoption plans. Durable ownership/Git coupling remain target. SPEC-003 now has a bounded local
publication subset under specs/PUBLICATION.md; process-restart tests do not close
provider-durable, complete restore coverage or resource/fence obligations.
SPEC-004/008 now have scoped local OS exclusion/pure capacity-accounting subsets
under SCOPED-COORDINATION.md. RESERVATION-JOURNAL.md adds a local transactional
command ledger under SPEC-003/004/005/008; physical admission, complete restore
authority and full retirement proof stay open. SPEC-009 now has an admitted
publication/conservative-file-footprint subset under ACCOUNTED-PUBLICATION.md;
full placement/removal/volume accounting is not verified.

| Specification Item | Class | Controlled behavior / limits | Parent requirements | Owner | Acceptance basis |
| --- | --- | --- | --- | --- | --- |
| SPEC-001 | current | Pure assigned-claim union, conservative candidate plans and snapshot transitions; no IO | REQ-002/003/004/005/009 | identity-namespace | 13 handoff tests and specs/HANDOFF-PROTOTYPE.md; caller assertions are not custody proof |
| SPEC-002 | target | Object identity, bounded immutable tree format, exact logical namespace and representation mapping | REQ-001/005/006 | identity-namespace | Canonical/hash/path vectors, malformed trees, bounded sparse edits |
| SPEC-003 | target | Durable ingest, publication acknowledgement levels and recoverable operation/claim ownership | REQ-003/004/009 | concurrency-recovery | Crash matrix, torn/stale journals and exact byte recovery before source retirement |
| SPEC-004 | target | Effective object/place reader/removal fences, job/draft protection and idempotent retries | REQ-002/003/004 | concurrency-recovery | Race/partition/expiry/ABA and unrelated-writer progress fixtures |
| SPEC-005 | target | Rebuildable shared catalog, scoped observation freshness, negative invalidation and bounded lookup | REQ-005/007 | concurrency-recovery | Warm/cold/miss/rebuild workloads; pinned budgets before acceptance |
| SPEC-006 | target | Base/private/sealed/Git-selected byte worktrees with tombstones and expected-selection adoption | REQ-001/004/009 | git-adoption | Same-path and delete/modify conflicts, partial staging and publication/index crash cases |
| SPEC-007 | target | Standard pointer extraction, admitted LFS transfer agent and explicit extra-copy accounting | REQ-006/008/009 | git-adoption | Pinned Git/LFS client versions, configured stores, denied access and prune coverage |
| SPEC-008 | target | Configurable proximity versus retained-copy/failure-domain constraints and source lifecycle | REQ-002/003/005/009 | placement-policy | Shared current/history/draft claims, archive offline and infeasible assignment cases |
| SPEC-009 | target | Bounded copy/verify/recheck/release transactions with measured physical outcome | REQ-003/004/005/006 | storage-custody | Local fake/real-temp providers, corrupt/partial/full disk and interruption fixtures |
| SPEC-010 | target | Optional typed contracts, graph/variant semantics and one authored reference authority | REQ-001/006/010 | identity-namespace | Pinned Art old/new vectors, complete native extraction and non-media typed fixture |
| SPEC-011 | target | Consumed projections, generic bindings/profiles, text/translation and build/result currency | REQ-005/010 | identity-namespace | Changed/unchanged projections, sparse variants, same-byte attempts and budget drift |
| SPEC-012 | target | Scoped evaluation/gates, triage and desired/observed delivery with trusted authority | REQ-006/009/010 | provider-privacy | Fake hosts, stale candidates, withdrawn authority, private issue/export and remote-drift fixtures |
| SPEC-013 | target | Optional remote providers, immutable transport shards and representation/key custody | REQ-003/005/006/008 | provider-privacy | Capability/privacy/retention fixtures; no seed-count backup inference |
| SPEC-014 | target | Read-only inspector and optional WASM view share native resolution, show gaps and avoid private build payloads | REQ-006/007/010 | git-adoption | Native/WASM vectors, exact commit scope and authenticated/missing-object cases |
| SPEC-015 | current | Structural scenario/doctrine/link checks plus Rust fmt/tests/Clippy and producer review records | REQ-006 | git-adoption | tools/validate.ps1, CI and pinned source checker hashes; not system acceptance |

Current numeric limits: View permits at most 1,024 claims and 65,536 assigned
placements; plan accepts at most 65,536 supplied verified-placement assertions;
opaque ID tokens are at most 128 bytes; u64 size/generation overflow rejects.
These are prototype limits, not final scalable API or throughput guarantees.

Unknown decisions: released encoding/digest domains, directory overlay semantics,
durable backend and removal-fence primitive, expiry/clock model, provider/client
matrix, key management and exact performance/admission budgets. Resolve them in
bounded discovery/contract packages. Until resolved, affected actions remain
unsupported rather than inheriting an unsafe permissive default.

## Stage review

Producer-applied six-role lenses: pass_with_risk. S-01: broad no-loss prose could
be mistaken for implemented storage safety. Resolved by one precise current
mechanism row, separate target rows and explicit limits/unknowns. Every REQ maps
to current partial and/or target behavior; no full requirement is closed.

S-02 native fixture clarification: specs/NATIVE-RECORDS.md and the two design-1
JSON Schemas control a proposed instance/definition envelope for private metadata
fixtures. They use exact raw-byte pins, not released semantic revision identity.
SPEC-010/011/012 remain target: mixed stream choices obey pinned definitions,
source-valid/output-stale merges can catch up later, and ready-output/delivery
gates must not misrepresent old renders as current. No runtime parser, scheduler
or complete native renderer is implemented by the fixture proposal.

SPEC-008/009 now have an explicit local sampling subset under EVID-016 and
LOCAL-CAPACITY-PROBE.md. It binds store/ledger instances and uses fresh actual
user-available-space samples. Native volume/quota authority and full enforcement
remain target obligations; no full spec is closed.

SPEC-003/004/005/009 gain a private-attempt cleanup subset under EVID-017 and
PRIVATE-ATTEMPT-CLEANUP.md. Durable cancellation precedes reviewed partial removal;
remaining metadata is charged before disposition. This is explicitly authorized
unacknowledged discard, not proof of final-object custody/eviction or full reclaim.

SPEC-003/004/005/009 gain live exact-replica verification and declared-domain
coverage under EVID-018/REPLICA-VERIFICATION.md. Saved observations remain
historical. This does not close durable retention, physical independence,
authoritative restore, complete claims or final-removal revalidation.

SPEC-003/004/005/008/009 gain bounded budgeted local retention under EVID-019 and
DURABLE-RETENTION.md. Design-4 stores enforce immutable owner-scoped claims across
normal process restart. Native independence, authoritative restore, owned release,
complete claims and final-object eviction remain target obligations.

SPEC-003/004/005/008/009 gain exact owned local retention handoff under EVID-020
and RETENTION-HANDOFF.md. Marker-before-retirement preserves custody through
restart/onward moves; full ownership/recovery and payload eviction stay open.
EVID-021 records actual consumer adoption requirements without promoting small
mechanism fixtures to scale or native-production evidence.

## Download-only LFS compatibility checkpoint

Input c68d7fad00b7fa6da528255f9b242a42e4dcb048; WP-005, REQ-006/008/009,
SPEC-005/007, IF-005/008/009, PKG-004. Controlled scope and entry/exit:
specs/LFS-CACHE-INTEROP.md. EVID-022 adds 12 Windows component entries;
EVID-023 adds three actual-client temporary-repo fixtures. Windows total 261;
Linux adds a Unix cross-filesystem entry, conditional on /dev/shm availability.
L0 structure/fmt/VTRACE; L1 locked workspace tests/all-target Clippy; L2 current
and historical checkout, two simultaneous clients, corruption and repaired retry.
No private caches or consumer configuration changed. Full WP-005, requirements,
validation, staged/ref adoption, upload and complete accounting remain OPEN.

## Exact Git source checkpoint

Input 07cfa97ce60a0d74f5923aaa96cdad8aad0e994b; WP-005, REQ-001/006/008/009,
SPEC-006/007, IF-008/009, PKG-004. GIT-SOURCE-EXTRACTION.md controls scope.
EVID-024 adds 16 temporary Git fixtures; EVID-025 connects exact source selection
through a complete OID tree to actual LFS transfer authorization. L0 structure/
fmt/VTRACE; L1 locked workspace/Clippy; L2 partial/conflicted staging, linked/bare
repos, missing/corrupt Git objects, disabled promisor hydration and historical
checkout. Windows total 278; Linux adds its Unix cross-filesystem entry. Complete
coverage is limited to explicit prefixes. Durable old/new claim handoff, global
attribute-aware extraction, large metadata scale and native adoption stay OPEN.

## Persistent source-root checkpoint

Input 5411e2be0521aef0e3722e508f31ff6355b8a66f; WP-008/009, REQ-002/004/007/009,
SPEC-005/006/007/008, IF-004/006/008, PKG-003/004. SOURCE-CLAIM-JOURNAL.md
controls scope and L0/L1/L2: structure/fmt/VTRACE, locked tests/Clippy, actual Git
root handoff plus six child exits. EVID-026 adds 19 journal entries including
helper; EVID-027 adds two Git/context fixtures. Full command cap replay executes,
not inventory-scale proof. Total 299 Windows / 300 Linux entries. Owner/revision
transitions preserve old/new roots; exact retries remain historical. Returned
obligations deny global completeness. Root closure, full source/job coverage,
final-removal fencing, restore authority, compaction and native adoption stay OPEN.

## Persistent metadata and scale checkpoint

Input 29e21c7673c4a50acade32228612ed849ac0ca3f; WP-001/008/009,
REQ-001/005/007/009, SPEC-002/005/006/008, IF-002/005/008/009, PKG-002/003/004.
PERSISTENT-METADATA.md controls L0 structure/fmt/VTRACE, L1 locked tests/Clippy,
and L2 persistent 500,000-binding synthetic scale plus actual LFS integration.
EVID-028: 14 provider entries including helper/three child exits. EVID-029: one
scale case, 13 edits adding 391 nodes / 212,992 database bytes, old root retained.
EVID-030: two core/integration cases. Total 316 Windows / 317 Linux entries.
Bounds are explicit; large-tree refusal is Budget. Full custody/admission,
compaction, complete native graph/source/job coverage and cutover stay OPEN.

Frozen native context extends SPEC-005/008/010/011 under
[NATIVE-PRODUCTION-CONTEXT](../../specs/NATIVE-PRODUCTION-CONTEXT.md), entry
e7d2bde. It preserves exact source/policy/roots/descriptor and typed requirements
through existing metadata ports and source-journal selections; no old native
format, global protection or payload-retirement contract is superseded.

Declared consumed-input recipes extend SPEC-010/011 under
[NATIVE-PRODUCTION-RECIPES](../../specs/NATIVE-PRODUCTION-RECIPES.md), entry e504cf3.
The admitted pure projection is separate from full provenance and never declares
output freshness, producer execution, semantic completeness or storage authority.

Produced correspondence extends SPEC-005/010/011/012 under
[PRODUCED-CORRESPONDENCE](../../specs/PRODUCED-CORRESPONDENCE.md), entry77a7e14.
Bounded unsigned claims and exact native context replay establish declared
metadata correspondence, without producer execution or storage authority.

Variant interpretation retention extends SPEC-005/010/011 under
[NATIVE-VARIANT-RETENTION](../../specs/NATIVE-VARIANT-RETENTION.md), entry1095b6f.
One asset-side linked packet captures independently admitted per-record choices;
optional job version2 links it without changing legacy version1 calls/bytes.
Actual producer projection, all-family/global protection and custody remain open.

Consumed payload policy extends SPEC-005/010/011 under
[CONSUMED-PAYLOAD-PROJECTION](../../specs/CONSUMED-PAYLOAD-PROJECTION.md),
entry81965c7. A distinct opt-in identity hashes an explicit structural subset of
captured payloads; full requirements/provenance and legacy calls stay unchanged.
Native claim replay requires independent original policy, not inferred downgrade.

The explicit retirement operator/agent interface extends SPEC-002/003/004/005/
006/007/008/009/013 under [RETIREMENT-AGENT-CLI](../../specs/RETIREMENT-AGENT-CLI.md),
input3e300ca. Canonical portable plans never replace private policy admission;
status reports historical state separately from fresh byte evidence. Existing
copy CLI schemas remain compatible. Final deletion, native/provider adoption and
automatic Git lifecycle execution retain separate gates.

The optional S3 adapter extends SPEC-002/003/004/007/008/009/013 under
[S3-PUBLICATION](../../specs/S3-PUBLICATION.md), input3fd9918. Conditional upload and
full readback reuse existing guarded local source and accounted witness ports.
Private access is excluded from portable plans; verified remote availability is
explicitly separate from retained custody, durability and removal authority.

The optional standalone LFS upload extends SPEC-002/003/004/005/007/008/009/013
under [LFS-UPLOAD](../../specs/LFS-UPLOAD.md), input9b8a689. It preserves exact
source bytes and default download compatibility while adding Basic Batch/PUT/
verify/readback and existing accounted witnesses. Private action credentials are
explicitly scoped; no automatic discovery/helper or implicit Batch-token forwarding.
Real server/custody/large-file qualification and Git event integration remain open.

Committed-ref lifecycle selection extends SPEC-002/003/004/005/006/007/008/009/013
under [GIT-LIFECYCLE-RECONCILIATION](../../specs/GIT-LIFECYCLE-RECONCILIATION.md),
inputd81cabe. Current selection admission reuses existing source event encodings;
new experimental plan/profile/status documents remain distinct. Atomic adoption
is desired protection, never byte/custody/convergence acknowledgement. Old-reader
qualification uses a pinned pre-pair artifact separately from ordinary tests.
The full Git lifecycle/provider execution and trigger boundary remain open.

The bounded local placement composition extends those same controls under
[PLACEMENT-RECONCILIATION](../../specs/PLACEMENT-RECONCILIATION.md), inputa678381.
Readonly requirements and explicit operator pages reuse existing event, accounted
publication, retention/handoff and managed incarnation/removal contracts. No new
journal encoding exists. Current private enrollment/page/report documents remain
experimental; page convergence is not full selection convergence. EVID-093 scopes
temporary-store verification separately from external trigger, native/provider/
large-file/lifetime, final-deletion and metadata-retirement requirements.

Optional trusted GitHub intake extends the same source/provider controls under
[GITHUB-TRIGGER](../../specs/GITHUB-TRIGGER.md), input1c253ea. EVID-094 separates
signed nonempty synthetic placement from actual issuer/foreground empty-scope
compatibility; neither grants live enrollment or final-copy authority.

Explicit staged-index protection extends SPEC-002/004/005/006/007/008/009 under
[INDEX-LIFECYCLE](../../specs/INDEX-LIFECYCLE.md), input195bd0a. Object-set draft
roots admit known conflict alternatives without relaxing merged extraction.
Existing source journal bytes and placement/removal contracts remain unchanged;
new index plan/profile/status documents are experimental and separately versioned.

[SOURCE-SETTLEMENT](../../specs/SOURCE-SETTLEMENT.md), input9c2a29f, extends the
same source/Git/placement controls with bounded transactional revision predicates,
exact local ref absence and owned current-to-predecessor withdrawal. Event bytes
remain unchanged; historical receipt, current source/Git and custody are distinct.

[RETAINED-SUBSET](../../specs/RETAINED-SUBSET.md), input63f2c96, extends
SPEC-002/004/005/006/008/009/014 with owner-authorized exact partial historical
root settlement. Existing tree edits and Enroll/Prepare/Adopt/Withdraw/Retire
encodings preserve the unselected set under the same immutable policy class.
New experimental plan/profile/status records grant no payload deletion authority.
