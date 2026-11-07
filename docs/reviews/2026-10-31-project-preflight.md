# Project host and space preflight: producer review

Controls: [PROJECT-PREFLIGHT](../../specs/PROJECT-PREFLIGHT.md),
WP-005/006/011/015, REQ-001/003/004/006/007/010, IF-005/008/010/012/013,
PKG-003/004/008. Entry `93e57ee8c4eb15b0f0ea7fdc47cb83b094d76874`.
Allowed scope: optional committed host/space requirements, explicit local bindings,
bounded observations and synthetic fixtures. No private font/tool/media is added,
executed or enrolled into a new store. The shared copy engine is unchanged.

Eight additional project cases cover unbound/missing/wrong host outcomes before
source reads, view/render scope, relocated host identity, apply revalidation,
budgets/duplicates/unknown fields/unsupported integers, missing/offline/unrelated
capacity roots, insufficient observed threshold, aliases versus new destination
bytes, advisory estimate shortfalls and logged failure/explicit retry after repair.
All 28 project cases and eight existing restore cases pass. An initial fixture
used integers above the existing strict safe-integer domain; supported large
thresholds replaced those fixture values. Parser limits were not relaxed.
Full locked all-feature workspace tests pass, strict all-feature/all-target
Clippy passes (24.08 s), and standalone Git/LFS check passes (0.96 s). Exact-head
hosted qualification and actual consumer adoption remain separate gates.

Producer-applied role lenses (not independent reviewers):

- Identity/namespace: host Objects are exact externally declared requirements;
  no ID-to-path inference. Raw committed requirements and local bindings are
  already part of the exact plan. Host path changes preserve Object identity.
- Provider/privacy: up to sixteen explicit files, per-file/aggregate byte bounds,
  existing ordinary path verification, no PATH/font discovery or execution, no
  raw host paths in results/errors. A declared font hash does not establish rights.
- Storage/placement: new logical copies, free-space thresholds and scratch
  estimates stay separate. Existing targets cost zero new logical copy bytes;
  aliases still consume separate destination space. Explicit capacity ancestry
  cannot silently fall back to a different filesystem. Observation is not quota
  or ledger admission; allocated overhead/concurrent writers remain caller risks.
- Concurrency/recovery: no global lock; each invocation repeats host/space checks.
  Preview holds neither host-file protection nor capacity. Existing verified copy,
  progress logging, no-replace publication and explicit retry remain intact.
- Git/adoption: definitions stay small and operational results stay asset-side.
  The result schema advances to experimental-3; consumers must deliberately adopt
  it. No runtime binding, asset selection or publication authority changes.

Disposition: pass_with_risk for this scoped preflight seam, subject to remaining
local/hosted gates. Unix device mismatch refusal is implemented; forcing a real
mount/device transition and power-loss/capacity races are not claimed by the
current Windows synthetic fixtures. Full environment/license semantics, durable
admission, orphan classification, source/custody coverage and consumer cutover
remain open. No deletion authority is inferred from successful observations.
