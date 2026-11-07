# Tig agent instructions

Read CONTEXT.md, README.md, PRODUCT_PLAN.md, context/waves/PHASES.md and the
active wave/pulse. Before contract changes read specs/, docs/vtrace/, .pitfall/
and applicable .roles checklists.

The full engineering router is docs/vtrace/README.md. Before nontrivial code,
name the WP/REQ/SPEC/IF/PKG controls, input SHA, allowed scope, entry/exit and
L0/L1/L2 profile. Close verification/validation/trace/review separately from
implementation; producer review is not independent or human approval.

- Tig is generic Rust byte/asset infrastructure. docs/UNIFIED-DESIGN.md folds
  Art's value into an optional typed-asset layer. Keep low-level placement usable
  without it; native media semantics remain admitted adapter responsibilities.
- Logical paths resolve in an exact bundle snapshot. Physical paths are provider-owned.
- No cache initialization, daemon, census, hydration, transfer, upload or deletion
  on resume. Foundation work uses synthetic fixtures only.
- Desired, planned and observed state are separate. Missing coverage is unknown.
- Never delete solely because an object lost a label or claim. Revalidate current
  protection and independently retained custody before authorized removal.
- No global cache lock. Use scoped resource admission and stale-operation fencing.
- Keep records bounded; expanding inventories/journals/receipts belong asset-side.
- Do not copy private assets, prompts, credentials, screenshots or voice credits.
- Roles are producer checklists, not independent people or publication authority.
- Run pwsh -NoProfile -File tools/validate.ps1 and git diff --check before commits.
  Rust fmt/locked tests/Clippy become required when Rust implementation is added.
- No Cargo dependencies on TRACKER-relative paths. Reuse existing providers through
  adapters after fixtures; do not create another project-specific cache engine.
