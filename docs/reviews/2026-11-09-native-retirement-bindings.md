# Native retirement binding review

Inputc241593f8432a2bde9cd3f00c83f9b91fd6213db;
implementation00c229d04e091388cee9b5a0922fdbc5617e950a.
Controls, scope, entry/exit and RUST/GIT/PROVIDER L0/L1/L2:
[NATIVE-RETIREMENT-BINDINGS](../../specs/NATIVE-RETIREMENT-BINDINGS.md).
WP-003/004/007/008/009/010/013/015;
REQ-001/002/003/004/005/006/007/009/010;
SPEC-002/003/004/005/008/009/010/013/014;
IF-002/004/005/006/008/010/012/013; PKG-001/003/004/006/008/009.
Targets: optional graph storage view, native binding/header/index/profile ports,
block/category selector, explicit metadata-only CLI and real Git/source/bulk
fixtures. Reviewed hashes: foundation-targets.json.

## Method and six producer lenses

One producer applies all six [.roles checklists](../../.roles/ROLE.md), without
delegation, independent review or human acceptance. Owner of each finding: Tig
maintainer using the named lens. This qualifies the declared native-to-flat seam;
it does not make an arbitrary flat namespace a complete native protection source.
Adopters must admit the actual native roots/profiles and enforce their writer
boundary before claiming complete scope. No existing root is silently migrated.

| Finding / lens | Severity | Disposition and evidence | Remaining obligation |
| --- | --- | --- | --- |
| NB-01 Identity and Namespace | High | Header pins exact native archive, canonical profiles and root index. Per-asset paths bind raw record/object digests. Inspection recomputes the full original mapping and accepts only actual unchanged subset bindings; private header admission survives owned subset context changes. Forged header/foreign paths and conflicting cross-query sizes refuse. Four independent framing/path cases pin the new records. | Native source-adoption/writer qualification, live migration and eventual metadata dependency retirement. |
| NB-02 Placement Policy | High | Independent logical aliases preserve another asset's shared demand. Categories match only exact privately admitted definition References; block/category intersection never uses names, status or file dates as meaning. Unsupported closure and oversized categories refuse without truncation. Age fields are rejected rather than inventing evidence. | Record and validate conservative supersession age for each historical occurrence; reactivation/recent obligations must remain protected. Full paged selection and UI/Action. |
| NB-03 Storage and Custody | High | A real native block composes with existing bulk final retirement: six unique payload bytes are actually removed, while six shared bytes are read back exactly because an unselected native asset retains its alias. The surviving native root remains inspectable with three of four original bindings for the partially retired asset. Adapter reports no payload verification or removal authority. | Large-file/real-provider/custody qualification and measured volume reclaim; native mapping alone grants none. |
| NB-04 Concurrency and Recovery | High | Selector reobserves the source after metadata work; its result remains a preview, not a lease. Existing bulk live fences remain final authority. Archive acquisition initially bypassed the CLI construction budget; build_from_archive now shares one counter/time boundary with compilation/writes. Counted provider stops at one read and zero writes. Independent persisted readback is a second bounded phase. | Full source-adoption interruption/age-witness recovery, current authority/restore and journal continuation. |
| NB-05 Provider and Privacy | High | CLI uses existing roots/minima only; bind defaults disabled, finishes and independently reads back metadata. Select opens MetadataReader and leaves newly reopened metadata/source checkpoints unchanged. Real CLI works with Git detached; private authored/metadata fields stay absent from output. Header/archive/index/profile references remain asset-side. | Metadata GC must retain every new dependency edge or hold expiry. Native private enrollment, unmanaged writers, actual providers and authority adoption stay open. |
| NB-06 Git and Adoption | Medium | Exact captured old metadata stays independent of moving Git names. Explicit source owner/class/predecessor/header checks gate emitted bulk selections. New storage_objects includes explicitly declared output bytes without changing production_requirements or output-as-primary refusal. Schema-feature CLI stays optional; low-level Git/LFS has no new typed dependency. | Complete native semantic/schema parity, automatic native lifecycle adoption and human acceptance; hosted exact-head checks remain separate. |

## Verification

Additional planning finding NB-07 (High, placement/recovery lenses; owner Tig
maintainer): an object can occur in more historical roots than MAX_ROOTS. Every
bounded batch sees the unselected roots as protection and therefore preserves all
selected paths. Repeating batches cannot retire that whole block. Disposition:
retain an explicit entry gate for paged global selection/consent and separately
authorized historical-claim-only settlement, followed by final deletion limited
to the original approved content. Do not change existing held-member semantics
silently. Required proof: more-than-MAX_ROOTS sharing, age inheritance through
subsets/pages, active/foreign/unselected holds and interrupted/budget-exhausted
progress. This is a future design blocker, not a defect in the bounded contract.

Eight focused native integration entries and one framing entry pass. Four
independent Python/json/hashlib canonical/path cases cover header, profiles and
distinct per-asset aliases of shared bytes. They pin framing, not authority or
custody. Tests use owned external temporary metadata/Git/provider roots only.

Real Git capture supplies three exact native records. Per-asset maps retain two
bindings to shared content; removing one asset's bindings cannot erase the other.
Unknown category/record, changed definition identity, wrong header and foreign
paths refuse or match no assets rather than substituting a newer revision.
Retained subsets with changed source context stay inspectable under independently
admitted header identity; removed members remain missing. Aggregate node budget,
zero writes, unsupported payload scope and conflicting sizes refuse before new
metadata publication. A1,000-record fixture preserves all asset identities while
oversized category selection refuses; an exact ten-asset block has forty logical
bindings and one deduplicated shared payload identity.

The actual schema-feature CLI defaults bind disabled and preserves the existing
checkpoint on refusal. Enabled bind finishes metadata and independently reopens a
reader before reporting its candidate. Git's .git directory is actually moved
offline after capture; bind/select use only admitted archived metadata. Select
validates exact historical namespace/source/owner/class/predecessor and header,
emits a real bulk::Selection, preserves newly reopened source/metadata checkpoints
and redacts private authored metadata. Unsupported age fields refuse at decoding.

The native-to-bulk fixture publishes/retains shared and unique synthetic payloads
with the existing accounted provider. Selecting assets A and C leaves asset B's
shared alias protected; only C's unique six-byte payload becomes a removal action.
Actual final-provider observation is absent for C; shared bytes read back exactly.
The owned retained-subset source preserves native interpretation and partial asset
membership after execution. No new provider or claim engine is introduced.

An output-root fixture explicitly selects stored output bytes through the new
storage view while production_requirements still refuses them as inputs. Existing
production rules and record/provider event encodings remain unchanged. New native
header metadata is separately versioned; unknown/malformed framing has no fallback.

Final strict all-feature/all-target Clippy, focused/native CLI/vector tests, fmt
and portable L0/diff pass. Full locked default/all-feature workspace chains
terminated zero (22223/15766), including both500k metadata fixtures, standalone
Git and explicit fresh ordinary cancellation/bulk inspection/execution/pinned
old-writer refusal with accepted legacy control. No runtime changed after00c229d.
Producer review strengthened the output fixture with a complete Production-only
query, so refusal cannot pass merely because Composition was requested. It also
adds an explicit wrong-record-revision negative. Both later assertions pass in
the final eight-entry focused run and final strict Clippy (48378 terminal zero);
broad suites cover the unchanged runtime. Hosted exact-head results remain pending.

```text
cargo fmt --all -- --check
cargo test -p tig-git --features schema --test native_source retirement_cases --locked
cargo test -p tig-git --features schema --lib independent_native_binding --locked
cargo clippy --workspace --all-features --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo check -p tig-git --no-default-features --locked
cargo build --workspace --all-features --bins --locked
cargo test --workspace --all-features --locked
cargo test -p tig-store --lib --locked retirement::tests::explicit_cli_pending_cancellation_qualification -- --ignored --exact
cargo test -p tig-store --lib --locked final_retirement::bulk::tests::explicit_cli_bulk_inspection_qualification -- --ignored --exact
cargo test -p tig-store --lib --locked final_retirement::bulk::tests::operator::explicit_cli_bulk_execution_qualification -- --ignored --exact
cargo test -p tig-store --test retirement_cli --locked pre_final_writer_refuses_new_manifest_and_accepts_legacy_control -- --ignored --exact
```

Native CLI integration uses Cargo's explicitly built CARGO_BIN_EXE_tig-retirement,
not an installed runtime. TIG_RETIREMENT_CLI/TIG_BULK_CLI name the fresh external
all-feature artifact; TIG_PRE_FINAL_CLI names pinned273e639. Ignored suite output
is not explicit qualification. Frontend/restore and source pre-pair artifact
evidence remain historical unless rerun; CI and human acceptance remain separate.

## Synthetic validation, trace and residual scope

EVID-105 contributes bounded VAL-002/003/004 rehearsal: an operator/application
can map exact old native blocks/categories to source obligations without confusing
shared payload identity with ownership. Captured interpretation survives Git
outage and partial retirement. Existing bulk protection/consent remains decisive;
the new adapter neither authorizes adoption nor deletes bytes itself.

Trace: controls above -> NATIVE-RETIREMENT-BINDINGS.md -> Graph::storage_objects,
native::retirement build/inspect/select and tig-retirement -> eight integration
entries, one framing entry/four independent cases, real Git/CLI,1,000-root limits
and actual bulk byte effects -> EVID-105. Implementation, verification, synthetic
validation and six producer lenses close separately. Disposition: pass_with_risk
for bounded declared mapping; hosted exact-head results remain pending.

Age filtering is explicitly unimplemented. Its next protocol must record a
conservative observed supersession time for an exact historical occurrence,
through admitted clock/publication/ledger ports, preserving unknown-age holds
and newer/current/recent reactivation obligations. Native source writer/adoption
coverage, full UI/Action/paged scope, current authority/host trust/unseen restore,
large files/real providers, continuation, new metadata dependency retention and
eventual expiry, independent review and human acceptance remain required. The
full product goal stays active and incomplete; no live cache, source adoption,
provider initialization, migration, hook, runner, source merge or deployment.
