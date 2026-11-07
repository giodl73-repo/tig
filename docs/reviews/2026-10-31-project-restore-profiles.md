# Rust project restore profiles: producer review

Controls: [PROJECT-RESTORE](../../specs/PROJECT-RESTORE.md), WP-005/011/015,
REQ-001/003/006/010, IF-005/008/010/013, PKG-003/004/008. Entry code is Tig
`e184ef7`, now qualified as main `5b4735e5ee304358260610f55247b52dc2d02f86`
with identical whole tree. Scope is the first orchestration slice from the
[portable replay feedback](../../feedbacks/README.md).

Extract the existing verified copy into `tig-git::restore` and keep the original
single-object CLI protocol. The new `tig-project` reads a bounded manifest from
one exact Git listing, captures the declared native namespace once and keeps a
frozen in-memory plan. Apply needs no subsequent Git/working-file reads. Nested
scopes, fresh explicit roots, selected/literal-stream selectors and a compact
catalog use the same source guards, private copy/readback and no-replace publish.

Preflight resolves all entries and checks destinations/source availability
before directory creation. Case-conservative aliases and file/parent collisions
refuse. Budgets account separately for unique source objects, requested
destinations and both source-verification phases. An expected plan pins the exact
profile/source/manifest/metadata/entries. Physical relocation can change the plan
without changing immutable payload identity. No quota or hostile-namespace
sandbox is claimed.

Thirteen project harness tests pass (including the actual child-exit driver),
alongside all eight existing single-restore cases. Coverage includes nested
preview/apply/retry, two byte objects at three destinations, literal `selected`
versus root selection, checkout unavailable after planning, view/render scope
separation, stale plan, dirty working files, missing definitions/duplicate JSON,
corrupt source/conflicting target, budget/case/parent refusals, explicit stop and
real process exit between entries. A sub-1-KiB catalog manifest selects 1,000
synthetic invoice records with one unique object and no preview directories.

The first apply run stopped before copying because directory creation tried an
already-existing Windows drive ancestor. Fix: inspect existing ordinary parents
and create only missing ones, retaining link/reparse refusal. The complete
focused set then passes (13 project cases in 13.93 s, eight restore cases in
6.61 s). Full locked all-feature workspace tests, strict workspace/all-target/
all-feature Clippy (9.05 s) and standalone Git/LFS check (1.57 s) pass. No tests
are removed, ignored or weakened to obtain these results.

Producer lenses: identity/namespace checks exact source/profile/plan and explicit
selectors; provider/privacy keeps raw paths out of fixed errors and declares
detailed local results private; storage/custody distinguishes verification at an
operation from currently held guards or backup; concurrency/recovery checks
no-replace, source reopen, partial success and explicit retry; Git/adoption keeps
catalog membership authoritative in small records rather than a generated list.
The progress observer is trusted Rust code, not a data-loaded executable hook.

Disposition: pass_with_risk for this bounded orchestration. Open high, integration
owner: consumer acceptance, persistent per-entry handoff/journal design, orphan
classification and host requirements. A killed process may lack a final report;
verified targets survive and explicit retry checks them. No durable multi-file
transaction, automatic cleanup/resume, render, runtime switch, held batch source
protection, independent custody or eviction authority follows. The Python helper
is unchanged compatibility tooling; it is not yet migrated to this profile.
