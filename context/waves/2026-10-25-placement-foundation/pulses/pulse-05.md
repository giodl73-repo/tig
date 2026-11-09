# Pulse 05: Providers and optional typed-asset adoption

Goal: Audited providers, Art-donor parity and optional typed-asset integration;
no automatic bulk conversion. Work packages and integration boundaries are in
docs/vtrace/WORK_PACKAGES.md and INTEGRATION_PLAN.md.

Status: in progress. Native intake/graphs, contexts, history and local catalogue
slices are implemented; complete typed/provider/consumer adoption remains open.
See docs/ARCHITECTURE.md for current scope. Private consumer checkpoints and
runtime bindings remain owner-managed; docs/CONSUMER-ADOPTION.md defines generic
gates. Registration and component tests do not close production or custody.

Validation: pwsh -NoProfile -File tools/validate.ps1; git diff --check.
For implementation changes, run fmt/locked Rust tests/Clippy and exact focused
failure fixtures before closing the pulse. Update docs/vtrace/REVIEW.md with
producer/independent method and exact evidence scope.

## Native intake progress

WP-010 adds bounded pure design-1 envelope/definition binding and explicit
read-only tig-record inspection/slot lookup (EVID-031/032, 15 tests). Model,
graph, variant parity and complete native/REEL semantics remain OPEN. Private
consumer project conversion fixtures are maintained in that artifact repository, not Tig.

## Native graph progress

EVID-033 adds 34 tests for caller-admitted declared native references, logical
snapshot selection, frozen pins, typed edges/cycles, payload meaning and scoped
primary change candidates. A cel edit need not touch parent files; graph binding
identity tracks its resolution. Concrete native Git/REEL providers, consumed
recipe fingerprints, model parity and final asset/custody adoption remain OPEN.

## Native Git resolver progress

EVID-034 qualifies 13 real Git/CLI fixtures and the generic AssetResolver consumer
port. Metadata snapshots support current IDs and explicit old captures without
working-file reads. REEL consumer wiring, opaque reference extraction, full schema/
recipe/variant parity, durable source/job adoption and custody stay OPEN.

## Existing byte-reader progress

EVID-036/037 connect asset IDs to verified existing bytes with held read/seek
sessions and explicit path/cooperative protection. Strict Windows, child exit,
corrupt fallback and actual Git LFS coexistence are tested. Native REEL rendering,
full job/custody adoption, compaction and final eviction remain OPEN.

Consumer status maps: EVID-038 exposes immutable project values at exact record
identity. Native Git views and privacy checks pass; no project vocabulary,
producer eligibility or custody/eviction policy is introduced into the core.

Map collection extension: EVID-039 explicitly traverses declared object values
with stable escaped keys. Forty graph cases and eighteen native-source cases
pass; one-cel Git replacement and frozen old metadata are preserved without
parent edits. Full schema/recipe/variant, native retention and custody remain open.

Native metadata archive progress: EVID-040 preserves exact source interpretation
through existing immutable metadata providers, with detached-source restoration,
frozen/current replay, shared sparse updates and bounded failure cases. Explicit
finish/readback remains separate from candidate creation; source/job adoption,
opaque metadata payloads, schema/variant/producer semantics and eviction stay open.

Declared production requirements: EVID-042 includes container selected/named
controls and exact interpretation documents with primary payloads. Synthetic
archive/job-root replay preserves an old job after main changes. Complete opaque
semantics, live job protection, custody/eviction and runtime adoption stay open.

Frozen production context: EVID-043 adds generic metadata-only context roots and
externally admitted replay through existing providers/journal. Independent old
job obligations survive main changes. This is not live job registration, payload
custody, semantic producer completeness, eviction or runtime adoption.

Declared recipe progress: EVID-044 computes consumed-field/default and recursive
qualified-input fingerprints while preserving exact provenance. Synthetic report,
frozen graph and temporary Git/archive cases execute. This is not complete native
producer/model/variant/translation/result/scheduler adoption or storage authority.

Produced correspondence progress: EVID-045 separates unbuilt/unproven history,
matching declared recipes and stale selected outputs under exact subjects. Native
context replay verifies original metadata/parameters after store reopen without
Git. Actual producer/output verification, model/variant and custody adoption remain open.

Job resource context progress: EVID-047 binds original source/recipe, declared
runtime and optional checked unsigned output/evidence through a small linked
metadata tree. Explicit existing journal ownership survives independent main and
peer-job changes/restart. Native managed execution, capacity/byte availability,
complete source/job coverage, custody/retirement and reviewed adoption remain open.

Git record proposal progress: EVID-048 adds a generic exact-base small-file commit/
ref API without optional typed dependencies. Git handles separate cherry-pick/
merge application and conflicts; proposing preserves dirty HEAD/index/worktree.
Native candidate integration, semantic merge gates, source/claim lifecycle and
full consumer project acceptance remain open.

Verified-cache proposal CLI: EVID-049 adds explicit check/create commands using
the same preflight capability and existing byte readers. Standalone consumers
need no REEL dependency. Native claim/schema acceptance, source/job lifecycle and
full placement/custody/consumer acceptance remain open.

Explicit restoration pilot: EVID-065 adds default preview and explicit no-overwrite copying of one Git-selected stream through existing guarded readers. The optional project helper orchestrates bounded committed manifests. Synthetic tests stay here; private consumer media and verification receipts stay consumer-side. Full migration, custody and eviction remain open.

Retained-subset prerequisite: EVID-097 and RETAINED-SUBSET.md qualify exact owned
historical binding settlement with finished metadata, same-class survivor history
and conditional original retirement. Atomic history adoption avoids transient
active-use suppression. Focused/CLI/old-reader checks and six actual new exits
pass; corrected locked default/all-feature workspace and explicit artifact
qualification pass. Hosted current-head checks remain separate. No payload deletion, full
bulk selector, metadata retirement, native/provider adoption or full VAL closure.

Final-retirement claim prerequisite: EVID-098 / FINAL-RETIREMENT.md adds live
content-wide membership clearance with exact namespace/policy/minimum pins.
All source classes/roles, another-place use, actual restored history and process
fences execute without grant release or payload mutation. Final-runtime locked
default/all-feature workspace, focused/vector/old-reader, strict Clippy and L0
pass. Hosted exact head and the actual final-provider/bulk/lifetime executor,
native adoption and complete VAL outcomes remain open.
