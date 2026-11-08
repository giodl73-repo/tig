# Tig resume state

Current focus: finish catalogue qualification and build generic Rust storage
orchestration. PRODUCT_PLAN.md is the ordered roadmap; its six-role review is
docs/reviews/2026-11-08-storage-roadmap-roles.md. Live Git/PR status overrides
historical handoff text. Do not depend on any prior consumer-repository session.

Recent source slices: object-to-owner Overview/Compare/Impact navigation and
record-inspector/dependency request isolation. Relevant reviews are in
docs/reviews/2026-11-08-object-record-workflows.md and
docs/reviews/2026-11-08-record-inspector-fences.md. Their fixture evidence is
scoped; source merge and running viewer deployment are separate operations.
User intends to test UX separately; do not start browser automation on resume.

Next engine task: define the input-bound plan/operation and provider contract,
including complete scoped protection admission before any removal. Then implement
recoverable copy-only orchestration across temporary local stores using existing
tig-store publication/retention/ledger primitives. Follow with guarded eviction,
then remote/LFS upload adapters; no second cache engine or project-specific script.
Inspect existing code/tests first. The roadmap's lifecycle fixture is proposed,
not implemented; component tests do not qualify end-to-end removal.

Known boundary: catalogue storage actions are held previews; download-only LFS
exists. Cloud upload, generic payload eviction and full automatic Git lifecycle
reconciliation remain open. Unknown source/job coverage blocks destructive work,
while copy-only progress can continue. No global cache/render lock.

Use docs/DEVELOPMENT.md for startup/check commands, .roles/ROLE.md for all six
producer checklists and docs/vtrace/README.md for package controls. Public source
stays generic and small. Machine enrollment, runtime bindings, logs, credentials,
inventories and build/temp roots stay outside Git.

User feedback expands the target: merge-to-main reconciliation under an enabled
policy puts new released bytes at working/backup roles, superseded unused bytes
at history/backup roles, then automatically evicts eligible working copies.
Golden/category vocabulary and store bindings remain external configuration.
Backup retention is finite: roadmap priorities 7-8 add block/age-based final
content deletion and eventual metadata retirement, separate from cache eviction.
First define coverage authority/invalidation, cross-store retirement fencing,
publication replay after authorized absence, large-object qualification and
journal lifetime. No new runtime behavior or live deletion is authorized here.

Second six-role pass on input7979190 clarifies owner-authorized claim settlement,
trusted merge-trigger/policy admission, same-hash re-ingest versus stale deletion,
pre-removal durable intent and recovery budget, provider-version deletion scope,
and audit-window/aggregate-fence compatibility before metadata forgetting.
Journal lifetime design starts in priority 2; do not defer safe exhaustion until
priority 8. These are planned acceptance conditions, not implemented guarantees.
