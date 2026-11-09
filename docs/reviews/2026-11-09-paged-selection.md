# Paged historical selection review

Inpute995b604e2218bc71897f51e874f65c59d3f5476;
implementation3ee9ff3aa229ded83a82116bccc52196c4b6cf11.
Controls, scope, entry/exit and RUST/PROVIDER L0/L1/L2:
[PAGED-RETIREMENT](../../specs/PAGED-RETIREMENT.md).
Targets: asset-side exact request/manifest records, complete hypothetical
projection, canonical inspection and original-binding pages through existing
Tree/metadata/source ports. No source mutation, provider changes or consent API.
Reviewed target hashes: foundation-targets.json.

## Method and producer findings

One producer applies all six [.roles](../../.roles/ROLE.md) checklists. Owner of
every finding: Tig maintainer using the named lens. These are producer checks,
not independent people, human acceptance or publication/deletion authority.
Compare actual code, source/metadata effects and independent framing with the
existing bounded bulk/source/provider boundaries. Keep NB-07 operationally open:
this is a prerequisite for the durable paged controller, not that controller.

| Finding / lens | Severity | Disposition and evidence | Remaining obligation |
| --- | --- | --- | --- |
| PG-01 Identity and Namespace | High | Exact request records bind namespace/source/predecessor and expected binding Tree. Cohorts pin actual original root/context, owner/class/policy. Missing paths, substituted objects, duplicate occurrences, foreign/Job/Retention/current-only selections refuse. Cohort and cursor identities are domain separated; point lookup binds each emitted cohort to its frozen request. Ten independent Python framing cases pin all new records. | Native/age selection adapter proof admission and actual source writer/authority adoption remain separate. |
| PG-02 Placement Policy | High | Projection excludes only exact whole-selection historical bindings across the complete admitted roster. A17-root fixture observes no surviving claim while the existing16-root projection still holds through the seventeenth root. Unselected predecessors/aliases, current/draft/job/retention and other owners preserve protection. Private roster and local complete-closure limits are intersected. A strengthened local-limit regression fits both selected roots, then fails only on an additional unselected current root. | Independently approved historical-claim scope and original provider-held/eligible copy set; no automatic eligibility growth. |
| PG-03 Storage and Custody | High | No source/provider effect or removal/claim-release capability exists. Unique bytes/bindings/objects describe selected identities; they are not copy presence, custody, allocation or reclaim. Constructor outputs are candidates; actual finished metadata is reopened and inspected in a1100-binding fixture. | Global intent/decision, per-page/action recovery admission and actual final-provider fences/retained custody, real providers/large files and measured reclaim. |
| PG-04 Concurrency and Recovery | High | Source vectors are captured/reobserved around metadata work; an actual separate handle's prepare/abort during compilation invalidates the result even though no new use remains. Missing/corrupt dependencies, malformed positions and resource exhaustion refuse without source mutation. One root spans byte/count-bounded pages with complete coverage. A cursor confers no execution progress; original pages are not executable stale subset plans. | Durable exact page intent/completion/cancellation, acknowledged survivor/age lineage and live owned-history checks before physical actions. No controller crash/recovery claim is made. |
| PG-05 Provider and Privacy | High | Existing NodeStore/Tree/MetadataStore ports store immutable request/cohort/subject/index records outside Git. Record/page bytes are bounded; all metadata gets/writes share a call budget. Payload bodies are absent from fixture metadata; compilation uses exact references and declares no payload verification. Portable records contain no physical root paths/credentials or authored media semantics. | Explicit provider finish/readback/custody/adoption, dependency-aware GC/expiry and bounded operational continuation. New roots/metadata are not automatically enrolled. |
| PG-06 Git and Adoption | Medium | The layer is generic Rust and has no typed-asset/Git dependency or new SQL/event/cache engine. Project labels/categories/age remain admitted adapter meaning. Large inventories stay asset-side; source event bytes and existing bounded bulk exclusion limits remain unchanged. | Native/age/UI/CLI/agent/Action surfaces, exact trusted decisions, hosted checks and independent/human acceptance. |

## Verification

Eight initial focused entries and strict store all-target Clippy pass (19322).
Three later tests add deterministic reconstruction/unselected history, wrong
object/path and foreign/Job/Retention/current-only rejection. Final eleven-entry
focus/strict store Clippy pass (65113 terminal zero). One entry covers ten
independent Python compact ASCII JSON/hashlib framing cases: enrollment,
selection, origin, cohort, request, held/unprotected subject, manifest, cursor and
page. These pin framing, not authority or semantic coverage.

Producer review strengthened the local-budget test after implementation: the
earlier fixture could fail its selected-binding count before exercising projection.
It now admits two selected roots and adds an unselected current job root, so only
the local full-projection root cap refuses while the roster's cap remains larger.
The corrected exact test and strict store all-target Clippy pass (19626 terminal
zero). Runtime remains3ee9ff3; only the test was strengthened.

Actual synthetic SourceJournal/MetadataStore fixtures cover:

- Seventeen historical occurrences,51 exact bindings and one deduplicated six-byte
  identity. Thirty-four pages cover every original binding exactly once under the
  frozen two-binding/8192-byte policy; source checkpoint stays unchanged.
- A1100-binding original root persists, reopens through MetadataReader and spans
  multiple canonical byte-bounded pages without changing original identity.
- Exact repeated request/build produces the same roots with no new metadata
  checkpoint. Omitting one predecessor retains its shared-content hold.
- Complete current/draft/job/retention/foreign/unselected protection, source drift,
  missing/corrupt records/indexes/binding metadata, inconsistent counts, stale
  cursor identity/position, private/local limits and unsupported source ownership.

Locked default workspace/500k/standalone Git chain72249 and all-feature workspace/
strict all-feature/all-target Clippy/second500k chain77055 terminate zero. Their
runtime is the final implementation; the later three test-only additions are
qualified by the focused run above. Fresh ordinary CLI cancellation, bulk
inspection/execution and pinned pre-final writer refusal with accepted legacy
control pass (89270 terminal zero). All local processes are terminal. Fmt, L0/pins
and diff checks pass before implementation; refreshed checks close this review.
Hosted exact-head qualification remains separate. Pre-pair/frontend/other-consumer
evidence is historical, not newly rerun here.

Commands use explicit external build/temp roots and finite test threads:

```text
cargo test -p tig-store --lib --locked final_retirement::paged::tests
cargo clippy -p tig-store --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo test --workspace --all-features --locked
cargo clippy --workspace --all-features --all-targets --locked -- -D warnings
cargo check -p tig-git --no-default-features --locked
pwsh -NoProfile -File tools/validate.ps1
git diff --check
```

## Synthetic validation, trace and disposition

This rehearses parts of VAL-002/003/004: a large exact block can be represented,
inspected and paged without redefining original membership, pretending missing
coverage is empty or treating inspection progress as completed retirement. It
does not validate actual paged claim settlement/deletion or end-user workflows.

WP-003/004/006/007/008/009/013/015;
REQ-001/002/003/004/005/006/007/009 ->
SPEC-002/003/004/005/006/008/009/013/014,
IF-002/004/005/006/008/010/012/013, PKG-002/003/004/006/008/009 ->
PAGED-RETIREMENT -> final_retirement::paged request/build/inspect/page and separate
admission::bulk::remaining_selected -> EVID-107.

Disposition: pass_with_risk for bounded metadata compilation/inspection/paging;
NB-07 execution remains open. Global durable decisions and page controller,
conditional owned-survivor translation, verified age inheritance, original copy
eligibility and final live fences/cancellation still need implementation/proof.
Native/writer/current-authority/time/provider/large-file adoption, UI/Action/agent,
journal continuation, eventual metadata retirement and human acceptance remain
required. No full requirement/VAL or live-operation approval closes. Goal active.
